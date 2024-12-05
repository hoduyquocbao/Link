/// Module lỗi chứa các loại lỗi cốt lõi trong hệ thống.
/// Module này được sử dụng để phân loại và xử lý các lỗi có thể xảy ra trong quá trình vận hành hệ thống.
pub mod error;

/// Module liên kết chứa các chức năng liên kết cốt lõi.
/// Module này được sử dụng để quản lý các kết nối và trạng thái của hệ thống.
pub mod link;

/// Module trạng thái chứa các trạng thái cốt lõi trong hệ thống.
/// Module này được sử dụng để quản lý trạng thái của hệ thống.
pub mod state;

/// Sử dụng lỗi từ module `error`.
pub use error::Error;

/// Sử dụng liên kết từ module `link`.
pub use link::Link;

/// Sử dụng trạng thái từ module `state`.
pub use state::State;