use core::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub enum ErrorType {
    /// I/O erorr when reading or writing to file, e.g. file not found etc etc.
    Io,
    /// An error specific to attempting to serialize or deserialize the data.
    Serialization,
}

pub struct Error {
    err_code: ErrorCode,
}

/// Alias for a `Result` with the error type [Error](struct.Error.html).
pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub(crate) fn new(err_code: ErrorCode) -> Error {
        Error { err_code }
    }

    pub fn get_type(&self) -> ErrorType {
        match self.err_code {
            ErrorCode::Io(_) => ErrorType::Io,
            ErrorCode::Serialization(_) => ErrorType::Serialization,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_code {
            ErrorCode::Io(ref err) => fmt::Display::fmt(err, f),
            ErrorCode::Serialization(ref err_str) => f.write_str(err_str),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!(
            "Error {{ msg: {} }}",
            match self.err_code {
                ErrorCode::Io(ref err) => err.to_string(),
                ErrorCode::Serialization(ref err_str) => err_str.to_string(),
            }
        ))
    }
}

impl std::error::Error for Error {}

pub(crate) enum ErrorCode {
    Io(io::Error),
    Serialization(String),
}
