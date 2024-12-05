use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::time::Instant;

use crate::core::error::Result;

/// Trạng thái hiện tại của một liên kết
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Init,
    Ready,
    Active,
    Pause,
    Close,
}

/// Các chỉ số để theo dõi
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Measure {
    pub send: usize,
    pub receive: usize,
    pub error: usize,
    #[serde(skip)]
    start_instant: Option<Instant>,
    pub start_timestamp: Option<i64>,
}

/// Container trạng thái an toàn cho luồng
#[derive(Debug)]
pub struct State {
    mode: Arc<RwLock<Mode>>,
    measure: Arc<RwLock<Measure>>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            mode: Arc::clone(&self.mode),
            measure: Arc::clone(&self.measure),
        }
    }
}

impl State {
    /// Tạo một thể hiện mới của `State`
    ///
    /// # Returns
    /// * `Self` - Một thể hiện mới của `State`
    pub fn new() -> Self {
        Self {
            mode: Arc::new(RwLock::new(Mode::Init)),
            measure: Arc::new(RwLock::new(Measure::default())),
        }
    }

    /// Lấy trạng thái hiện tại của `State`
    ///
    /// # Returns
    /// * `Result<Mode>` - Trạng thái hiện tại của `State`
    pub async fn mode(&self) -> Result<Mode> {
        Ok(*self.mode.read().await)
    }

    /// Đặt trạng thái mới cho `State`
    ///
    /// # Arguments
    /// * `mode` - Trạng thái mới để đặt
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả của việc đặt trạng thái mới
    pub async fn set_mode(&self, mode: Mode) -> Result<()> {
        *self.mode.write().await = mode;
        Ok(())
    }

    /// Lấy các chỉ số hiện tại của `State`
    ///
    /// # Returns
    /// * `Result<Measure>` - Các chỉ số hiện tại của `State`
    pub async fn measure(&self) -> Result<Measure> {
        Ok(self.measure.read().await.clone())
    }

    /// Ghi nhận số lượng byte đã gửi
    ///
    /// # Arguments
    /// * `bytes` - Số lượng byte đã gửi
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả của việc ghi nhận số lượng byte đã gửi
    pub async fn record_send(&self, bytes: usize) -> Result<()> {
        let mut measure = self.measure.write().await;
        measure.send += bytes;
        if measure.start_instant.is_none() {
            measure.start_instant = Some(Instant::now());
            measure.start_timestamp = Some(chrono::Utc::now().timestamp());
        }
        Ok(())
    }

    /// Ghi nhận số lượng byte đã nhận
    ///
    /// # Arguments
    /// * `bytes` - Số lượng byte đã nhận
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả của việc ghi nhận số lượng byte đã nhận
    pub async fn record_receive(&self, bytes: usize) -> Result<()> {
        let mut measure = self.measure.write().await;
        measure.receive += bytes;
        if measure.start_instant.is_none() {
            measure.start_instant = Some(Instant::now());
            measure.start_timestamp = Some(chrono::Utc::now().timestamp());
        }
        Ok(())
    }

    /// Ghi nhận lỗi
    ///
    /// # Returns
    /// * `Result<()>` - Kết quả của việc ghi nhận lỗi
    pub async fn record_error(&self) -> Result<()> {
        let mut measure = self.measure.write().await;
        measure.error += 1;
        Ok(())
    }
} 