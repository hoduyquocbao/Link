//! # Ngrok P2P Library
//! 
//! Thư viện này cung cấp các thành phần cốt lõi cho hệ thống tunnel P2P phân tán.
//! 
//! ## Tính năng chính
//! 
//! - Kết nối P2P an toàn và hiệu quả
//! - Xử lý NAT traversal tự động
//! - Mã hóa end-to-end
//! - Ghi nhật ký toàn diện
//! 
//! ## Cấu trúc module
//! 
//! - `core`: Module lõi chứa các thành phần cơ bản
//! - `net`: Module mạng xử lý kết nối và truyền dữ liệu
//! - `guard`: Module bảo mật và mã hóa
//! - `store`: Module lưu trữ và cache
//! - `log`: Module ghi nhật ký và theo dõi

pub mod core;
pub mod net;
pub mod guard;
pub mod store;
pub mod log;

pub use core::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 