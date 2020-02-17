use std::fmt;
pub use std::io::Error as IoError;
pub use std::io::ErrorKind;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_io_error(e: Error) -> IoError {
    return IoError::new(ErrorKind::Other, e);
}

#[derive(Debug)]
pub struct BasinError {
    message: String,
}

impl BasinError {
    pub fn new(message: String) -> BasinError {
        BasinError { message }
    }
}

impl fmt::Display for BasinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BasinError {{ {} }}", self.message)
    }
}

impl std::error::Error for BasinError {}

#[macro_export]
macro_rules! basin_err {
    ($($arg:tt)*) => { Box::new(BasinError::new(format!($($arg)*))) }
}
