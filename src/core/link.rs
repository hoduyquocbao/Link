use std::sync::Arc;
use async_trait::async_trait;

use crate::core::error::{Error, Result};
use crate::core::state::{Mode, State};

/// Trait cốt lõi cho chức năng liên kết
#[async_trait]
pub trait Linkable {
    /// Bắt đầu liên kết
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả của việc bắt đầu liên kết
    async fn start(&mut self) -> Result<()>;

    /// Dừng liên kết
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả của việc dừng liên kết
    async fn stop(&mut self) -> Result<()>;

    /// Trạng thái của liên kết
    ///
    /// # Returns
    /// * `Result<Mode>` - Trạng thái hiện tại của liên kết
    async fn state(&self) -> Result<Mode>;
}

#[async_trait]
pub trait Movable {
    /// Gửi dữ liệu
    ///
    /// # Arguments
    /// * `data` - Dữ liệu cần gửi
    ///
    /// # Returns
    /// * `Result<usize>` - Số lượng byte đã gửi thành công
    async fn send(&mut self, data: &[u8]) -> Result<usize>;

    /// Nhận dữ liệu
    ///
    /// # Arguments
    /// * `buf` - Bộ đệm để nhận dữ liệu
    ///
    /// # Returns
    /// * `Result<usize>` - Số lượng byte đã nhận thành công
    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize>;
}

#[async_trait]
pub trait Guardable {
    /// Bảo vệ dữ liệu
    ///
    /// # Arguments
    /// * `data` - Dữ liệu cần bảo vệ
    ///
    /// # Returns
    /// * `Result<Vec<u8>>` - Dữ liệu đã được bảo vệ
    async fn protect(&self, data: &[u8]) -> Result<Vec<u8>>;

    /// Tiết lộ dữ liệu
    ///
    /// # Arguments
    /// * `data` - Dữ liệu cần tiết lộ
    ///
    /// # Returns
    /// * `Result<Vec<u8>>` - Dữ liệu đã được tiết lộ
    async fn expose(&self, data: &[u8]) -> Result<Vec<u8>>;
}

/// Cấu hình cho liên kết
#[derive(Debug, Clone)]
pub struct Settings {
    pub name: String,
    pub size: usize,
    pub wait: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: String::from("link"),
            size: 65536,
            wait: 30,
        }
    }
}

/// Thực hiện chính của liên kết
pub struct Link {
    settings: Arc<Settings>,
    state: Arc<State>,
    handlers: Vec<Box<dyn Handler>>,
}

impl Link {
    /// Tạo một liên kết mới
    ///
    /// # Arguments
    /// * `settings` - Cấu hình cho liên kết
    ///
    /// # Returns
    /// * `Self` - Liên kết mới được tạo
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: Arc::new(settings),
            state: Arc::new(State::new()),
            handlers: Vec::new(),
        }
    }

    /// Thêm một bộ xử lý vào liên kết
    ///
    /// # Arguments
    /// * `handler` - Bộ xử lý cần thêm
    pub fn add_handler<H: Handler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }
}

#[async_trait]
impl Linkable for Link {
    async fn start(&mut self) -> Result<()> {
        self.state.set_mode(Mode::Ready).await?;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        self.state.set_mode(Mode::Close).await?;
        Ok(())
    }

    async fn state(&self) -> Result<Mode> {
        self.state.mode().await
    }
}

#[async_trait]
impl Movable for Link {
    async fn send(&mut self, data: &[u8]) -> Result<usize> {
        // Kiểm tra trạng thái trước khi gửi
        let mode = self.state.mode().await?;
        if mode != Mode::Ready {
            return Err(Error::State("link is not ready".into()));
        }

        if data.len() > self.settings.size {
            return Err(Error::Net("data too large".into()));
        }

        // Chạy dữ liệu qua các bộ xử lý
        let mut processed = data.to_vec();
        for handler in &self.handlers {
            processed = handler.handle(&processed).await?;
        }

        self.state.record_send(processed.len()).await?;
        Ok(processed.len())
    }

    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Kiểm tra trạng thái trước khi nhận
        let mode = self.state.mode().await?;
        if mode != Mode::Ready {
            return Err(Error::State("link is not ready".into()));
        }

        if buf.len() > self.settings.size {
            return Err(Error::Net("buffer too large".into()));
        }

        self.state.record_receive(buf.len()).await?;
        Ok(buf.len())
    }
}

/// Trait cho bộ xử lý dữ liệu
#[async_trait]
pub trait Handler: Send + Sync {
    /// Xử lý dữ liệu
    ///
    /// # Arguments
    /// * `data` - Dữ liệu cần xử lý
    ///
    /// # Returns
    /// * `Result<Vec<u8>>` - Dữ liệu đã được xử lý
    async fn handle(&self, data: &[u8]) -> Result<Vec<u8>>;
} 