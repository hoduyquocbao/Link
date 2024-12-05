use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore, mpsc};
use tokio::time::{timeout, Duration};
use async_trait::async_trait;

use crate::core::error::{Error, Result};
use crate::core::State;
use crate::core::link::{Linkable, Settings};
use crate::core::state::Mode;
use crate::net::Socket;

/// Trạng thái bên trong của nhóm kết nối
struct Inner {
    /// Danh sách các socket trong nhóm
    sockets: Vec<Socket>,
    /// Semaphore để kiểm soát số lượng kết nối đồng thời
    semaphore: Arc<Semaphore>,
    /// Cài đặt cho nhóm kết nối
    #[allow(dead_code)]
    settings: Arc<Settings>,
    /// Trạng thái của nhóm kết nối
    state: Arc<State>,
}

/// Cài đặt nhóm kết nối
pub struct Group {
    /// Trạng thái bên trong được bảo vệ bởi mutex
    inner: Arc<Mutex<Inner>>,
    /// Kênh gửi socket trả về
    return_tx: mpsc::UnboundedSender<Socket>,
    /// Kênh nhận socket trả về
    return_rx: mpsc::UnboundedReceiver<Socket>,
}

impl Group {
    /// Tạo một nhóm kết nối mới
    ///
    /// # Arguments
    /// * `size` - Kích thước tối đa của nhóm
    /// * `settings` - Cài đặt cho nhóm
    ///
    /// # Returns
    /// * `Self` - Nhóm kết nối mới được tạo
    pub fn new(size: usize, settings: Settings) -> Self {
        println!("Creating new group with size {}", size);
        let (return_tx, return_rx) = mpsc::unbounded_channel();
        let inner = Inner {
            sockets: Vec::with_capacity(size),
            semaphore: Arc::new(Semaphore::new(size)),
            #[allow(dead_code)]
            settings: Arc::new(settings),
            state: Arc::new(State::new()),
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
            return_tx,
            return_rx,
        }
    }

    /// Thêm một socket vào nhóm
    ///
    /// # Arguments
    /// * `socket` - Socket cần thêm vào
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả thêm socket
    pub async fn add(&self, socket: Socket) -> Result<()> {
        println!("Adding socket to group");
        let mut inner = self.inner.lock().await;
        if inner.sockets.len() >= inner.semaphore.available_permits() {
            println!("Group is full");
            return Err(Error::Net("group is full".into()));
        }
        inner.sockets.push(socket);
        println!("Socket added successfully");
        Ok(())
    }

    /// Lấy một socket từ nhóm
    ///
    /// # Returns
    /// * `Result<Holder>` - Kết quả lấy socket kèm holder
    pub async fn get(&mut self) -> Result<Holder> {
        println!("Getting socket from group");
        
        // Xử lý các socket được trả về trước với timeout
        let mut processed = 0;
        while processed < 10 {  // Giới hạn số socket được xử lý
            match timeout(Duration::from_millis(100), self.return_rx.recv()).await {
                Ok(Some(socket)) => {
                    let mut inner = self.inner.lock().await;
                    inner.sockets.push(socket);
                    processed += 1;
                }
                _ => break,
            }
        }
        if processed > 0 {
            println!("Processed {} returned sockets", processed);
        }

        // Lấy permit trước khi giữ khóa
        println!("Acquiring permit");
        let permit = {
            let inner = self.inner.lock().await;
            match timeout(
                Duration::from_millis(100),
                inner.semaphore.clone().acquire_owned()
            ).await {
                Ok(Ok(permit)) => {
                    println!("Permit acquired");
                    permit
                }
                Ok(Err(_)) => {
                    println!("Failed to acquire permit");
                    return Err(Error::Net("failed to acquire semaphore".into()));
                }
                Err(_) => {
                    println!("Permit acquisition timed out");
                    return Err(Error::Net("permit acquisition timeout".into()));
                }
            }
        };

        // Sau đó thử lấy socket với timeout
        println!("Getting socket");
        let socket = {
            let mut inner = self.inner.lock().await;
            if let Some(socket) = inner.sockets.pop() {
                println!("Socket obtained successfully");
                socket
            } else {
                println!("No available sockets");
                drop(permit);
                return Err(Error::Net("no available sockets".into()));
            }
        };

        Ok(Holder {
            socket: Some(socket),
            permit: Some(permit),
            return_tx: self.return_tx.clone(),
        })
    }
}

impl Clone for Group {
    /// Tạo bản sao của nhóm kết nối
    ///
    /// # Returns
    /// * `Self` - Bản sao của nhóm kết nối
    fn clone(&self) -> Self {
        println!("Cloning group");
        let (return_tx, return_rx) = mpsc::unbounded_channel();
        Self {
            inner: self.inner.clone(),
            return_tx,
            return_rx,
        }
    }
}

#[async_trait]
impl Linkable for Group {
    /// Khởi động nhóm kết nối
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả khởi động
    async fn start(&mut self) -> Result<()> {
        println!("Starting group");
        let inner = self.inner.lock().await;
        inner.state.set_mode(Mode::Ready).await?;
        println!("Group started");
        Ok(())
    }

    /// Dừng nhóm kết nối
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả dừng
    async fn stop(&mut self) -> Result<()> {
        println!("Stopping group");
        
        // 1. Đặt trạng thái thành Close
        {
            let inner = self.inner.lock().await;
            inner.state.set_mode(Mode::Close).await?;
        }
        
        // 2. Xử lý các socket được trả về
        while let Ok(socket) = self.return_rx.try_recv() {
            println!("Processing returned socket during stop");
            let mut inner = self.inner.lock().await;
            inner.sockets.push(socket);
        }
        
        // 3. Lấy tất cả socket
        let sockets_to_stop = {
            let mut inner = self.inner.lock().await;
            std::mem::take(&mut inner.sockets)
        };
        println!("Got {} sockets to stop", sockets_to_stop.len());
        
        // 4. Dừng tất cả socket đồng thời
        let mut handles = Vec::new();
        for mut socket in sockets_to_stop {
            handles.push(tokio::spawn(async move {
                match socket.stop().await {
                    Ok(_) => println!("Socket stopped successfully"),
                    Err(e) => println!("Error stopping socket: {}", e),
                }
            }));
        }
        
        // 5. Đợi tất cả socket dừng với timeout
        for handle in handles {
            match timeout(Duration::from_millis(100), handle).await {
                Ok(Ok(_)) => println!("Socket stop completed"),
                Ok(Err(e)) => println!("Socket stop error: {}", e),
                Err(_) => println!("Socket stop timed out"),
            }
        }
        
        println!("Group stopped");
        Ok(())
    }

    /// Lấy trạng thái của nhóm
    ///
    /// # Returns
    /// * `Result<Mode>` - Trạng thái hiện tại
    async fn state(&self) -> Result<Mode> {
        let inner = self.inner.lock().await;
        inner.state.mode().await
    }
}

/// Holder RAII cho việc mượn socket
pub struct Holder {
    /// Socket đang được giữ
    socket: Option<Socket>,
    /// Permit cho phép truy cập
    permit: Option<tokio::sync::OwnedSemaphorePermit>,
    /// Kênh để trả socket về
    return_tx: mpsc::UnboundedSender<Socket>,
}

impl Holder {
    /// Lấy tham chiếu đến socket
    ///
    /// # Returns
    /// * `&mut Socket` - Tham chiếu có thể thay đổi đến socket
    pub fn socket(&mut self) -> &mut Socket {
        self.socket.as_mut().unwrap()
    }

    /// Kiểm tra tính hợp lệ của holder
    ///
    /// # Returns
    /// * `bool` - true nếu holder hợp lệ
    pub fn is_valid(&self) -> bool {
        self.socket.is_some() && self.permit.is_some()
    }

    /// Giải phóng holder và trả socket về nhóm
    pub fn release(mut self) {
        println!("Releasing holder");
        
        // Lấy quyền sở hữu socket và permit
        let socket = self.socket.take();
        let permit = self.permit.take();
        
        // Giải phóng permit trước
        if let Some(permit) = permit {
            println!("Releasing permit");
            drop(permit);
        }
        
        // Sau đó thử trả socket về với timeout
        if let Some(socket) = socket {
            println!("Returning socket to group");
            match self.return_tx.send(socket) {
                Ok(_) => println!("Socket returned successfully"),
                Err(_) => println!("Failed to return socket"),
            }
        }
        
        println!("Holder released");
    }
}

impl Drop for Holder {
    /// Giải phóng tài nguyên khi holder bị hủy
    fn drop(&mut self) {
        println!("Dropping holder");
        
        // Lấy quyền sở hữu socket và permit
        let socket = self.socket.take();
        let permit = self.permit.take();
        
        // Giải phóng permit trước
        if let Some(permit) = permit {
            println!("Releasing permit in drop");
            drop(permit);
        }
        
        // Sau đó thử trả socket về
        if let Some(socket) = socket {
            println!("Returning socket to group in drop");
            match self.return_tx.send(socket) {
                Ok(_) => println!("Socket returned successfully in drop"),
                Err(_) => println!("Failed to return socket in drop"),
            }
        }
        
        println!("Holder dropped");
    }
}