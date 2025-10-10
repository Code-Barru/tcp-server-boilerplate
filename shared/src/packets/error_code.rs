use bincode::{Decode, Encode};
use thiserror::Error;

#[derive(Debug, Clone, Error, Encode, Decode, PartialEq, Eq)]
pub enum ErrorCode {
    #[error("Network communication error")]
    NetworkError,

    #[error("File system operation failed")]
    FileSystemError,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Command execution failed")]
    CommandFailed,

    #[error("Operation timed out")]
    Timeout,

    #[error("Resource not found")]
    NotFound,

    #[error("Resource already exists")]
    AlreadyExists,

    #[error("Invalid operation")]
    InvalidOperation,

    #[error("Internal error")]
    InternalError,

    #[error("Operation cancelled")]
    Cancelled,
}
