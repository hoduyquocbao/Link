use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use async_trait::async_trait;

use crate::core::error::{Error, Result};
use crate::core::State;
use crate::core::link::{Linkable, Settings};
use crate::core::state::Mode;
use crate::net::Socket;

/// Mục trong bảng định tuyến
#[derive(Debug, Clone)]
pub struct Entry {
    /// Địa chỉ của điểm đến
    pub addr: String,
    /// Trọng số của đường đi
    pub weight: u32,
}

/// Triển khai định tuyến mạng
pub struct Route {
    /// Bảng định tuyến lưu trữ các mục
    table: Arc<RwLock<HashMap<String, Entry>>>,
    /// Cài đặt cho định tuyến
    settings: Arc<Settings>,
    /// Trạng thái của định tuyến
    state: Arc<State>,
}

impl Route {
    /// Tạo một thể hiện định tuyến mới
    ///
    /// # Arguments
    /// * `settings` - Cài đặt cho định tuyến
    ///
    /// # Returns
    /// * `Self` - Thể hiện định tuyến mới được tạo
    pub fn new(settings: Settings) -> Self {
        Self {
            table: Arc::new(RwLock::new(HashMap::new())),
            settings: Arc::new(settings),
            state: Arc::new(State::new()),
        }
    }

    /// Thêm một mục vào bảng định tuyến
    ///
    /// # Arguments
    /// * `name` - Tên của mục định tuyến
    /// * `entry` - Mục định tuyến cần thêm
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả thêm mục định tuyến
    pub async fn add(&self, name: String, entry: Entry) -> Result<()> {
        let mut table = self.table.write().await;
        table.insert(name, entry);
        Ok(())
    }

    /// Xóa một mục khỏi bảng định tuyến
    ///
    /// # Arguments
    /// * `name` - Tên của mục định tuyến cần xóa
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả xóa mục định tuyến
    pub async fn remove(&self, name: &str) -> Result<()> {
        let mut table = self.table.write().await;
        table.remove(name);
        Ok(())
    }

    /// Lấy một mục từ bảng định tuyến
    ///
    /// # Arguments
    /// * `name` - Tên của mục định tuyến cần lấy
    ///
    /// # Returns
    /// * `Result<Entry>` - Mục định tuyến được tìm thấy hoặc lỗi nếu không tồn tại
    pub async fn get(&self, name: &str) -> Result<Entry> {
        let table = self.table.read().await;
        table.get(name)
            .cloned()
            .ok_or_else(|| Error::Net("route not found".into()))
    }

    /// Liệt kê tất cả các mục trong bảng định tuyến
    ///
    /// # Returns
    /// * `Result<Vec<(String, Entry)>>` - Danh sách các mục định tuyến
    pub async fn list(&self) -> Result<Vec<(String, Entry)>> {
        let table = self.table.read().await;
        Ok(table.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    /// Kết nối tới một điểm đến thông qua định tuyến
    ///
    /// # Arguments
    /// * `name` - Tên của mục định tuyến cần kết nối
    ///
    /// # Returns
    /// * `Result<Socket>` - Socket đã được kết nối
    pub async fn connect(&self, name: &str) -> Result<Socket> {
        let entry = self.get(name).await?;
        Socket::connect(entry.addr, (*self.settings).clone()).await
    }
}

#[async_trait]
impl Linkable for Route {
    /// Khởi động dịch vụ định tuyến
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả khởi động dịch vụ
    async fn start(&mut self) -> Result<()> {
        self.state.set_mode(Mode::Ready).await?;
        Ok(())
    }

    /// Dừng dịch vụ định tuyến
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả dừng dịch vụ
    async fn stop(&mut self) -> Result<()> {
        self.state.set_mode(Mode::Close).await?;
        Ok(())
    }

    /// Lấy trạng thái hiện tại của dịch vụ định tuyến
    ///
    /// # Returns
    /// * `Result<Mode>` - Trạng thái hiện tại của dịch vụ
    async fn state(&self) -> Result<Mode> {
        self.state.mode().await
    }
} 