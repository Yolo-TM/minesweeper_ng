use std::fmt;

#[derive(Debug)]
pub enum FieldError {
    InvalidMineConfig { reason: String },
    OutOfBounds { x: u32, y: u32, width: u32, height: u32 },
    InvalidFileData(String),
    IoError(std::io::Error),
    SerializationError(String),
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldError::InvalidMineConfig { reason } => {
                write!(f, "Invalid mine configuration: {}", reason)
            }
            FieldError::OutOfBounds { x, y, width, height } => {
                write!(f, "Position ({}, {}) out of bounds for {}x{} field", x, y, width, height)
            }
            FieldError::InvalidFileData(msg) => {
                write!(f, "Invalid file data: {}", msg)
            }
            FieldError::IoError(err) => write!(f, "I/O error: {}", err),
            FieldError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
        }
    }
}

impl std::error::Error for FieldError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FieldError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FieldError {
    fn from(err: std::io::Error) -> Self {
        FieldError::IoError(err)
    }
}
