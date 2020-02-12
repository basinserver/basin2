
pub use std::io::Error as IoError;
pub use std::io::ErrorKind; 

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_io_error(e: Error) -> IoError {
    return IoError::new(ErrorKind::Other, e);
}
