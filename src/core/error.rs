use thiserror::Error;

/// Enum for defining core error types within the system.
/// This enum is used to categorize and handle errors that may occur during system operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    Net(String), // Represents errors related to network operations.

    #[error("Guard error: {0}")]
    Guard(String), // Represents errors related to guard operations, such as authentication or encryption.

    #[error("State error: {0}")]
    State(String), // Represents errors related to state management, such as transitioning between states.

    #[error("Store error: {0}")]
    Store(String), // Represents errors related to data storage or retrieval operations.

    #[error("System error: {0}")]
    System(#[from] std::io::Error), // Represents errors related to system-level operations, such as I/O operations.
}

/// Represents the result of an operation that may fail with an `Error`.
/// This type alias is used to simplify the handling of operations that may result in an error.
pub type Result<T> = std::result::Result<T, Error>;