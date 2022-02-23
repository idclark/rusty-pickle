use std::io;
use std::result;

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

impl std::error::Error for Error {}

pub(crate) enum ErrorCode {
    Io(io::Error),
    Serialization(String),
}
