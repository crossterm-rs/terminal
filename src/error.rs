use std::{
    fmt::{self, Display, Formatter},
    io,
};

/// The `terminal` result type.
pub type Result<T> = std::result::Result<T, ErrorKind>;

/// Wrapper for all errors that can occur in `terminal`.
#[derive(Debug)]
pub enum ErrorKind {
    FlushingBatchFailed,
    /// Attempt to lock the terminal failed.
    AttemptToAcquireLock(String),
    /// Action is not supported by the current backend.
    ActionNotSupported(String),
    /// Atribute is not supported by the current backend.
    AttributeNotSupported(String),
    /// IO error occurred
    IoError(io::Error),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<io::Error> for ErrorKind {
    fn from(error: io::Error) -> Self {
        ErrorKind::IoError(error)
    }
}

impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorKind::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match &*self {
            ErrorKind::FlushingBatchFailed => {
                write!(fmt, "An error occurred with an attempt to flush the buffer")
            }
            ErrorKind::AttemptToAcquireLock(reason) => write!(
                fmt,
                "Attempted to acquire lock mutably more than once. {}",
                reason
            ),
            ErrorKind::ActionNotSupported(action_name) => {
                write!(fmt, "Action '{}' is not supported by backend.", action_name)
            }
            ErrorKind::AttributeNotSupported(attribute_name) => write!(
                fmt,
                "Attribute '{}' is not supported by backend.",
                attribute_name
            ),
            _ => write!(fmt, "Some error has occurred"),
        }
    }
}
