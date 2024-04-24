use std::io;

pub enum ReaderError {
    Custom(String),
    IOError(io::Error),
    ArgsNotAllowed,
    Unrecognized(String),
    ReaderOrValueNotSet,
}

pub use ReaderError::*;

pub type Result<T> = std::result::Result<T, ReaderError>;

pub fn fail<T>(msg: &str) -> Result<T> {
    Err(Custom(msg.to_string()))
}

pub fn unrecognized_arg<T>(arg: String) -> Result<T> {
    Err(Custom(arg))
}

impl From<io::Error> for ReaderError {
    fn from(value: io::Error) -> Self {
        IOError(value)
    }
}

impl From<String> for ReaderError {
    fn from(value: String) -> Self {
        Custom(value)
    }
}

impl From<&str> for ReaderError {
    fn from(value: &str) -> Self {
        Custom(value.to_string())
    }
}
