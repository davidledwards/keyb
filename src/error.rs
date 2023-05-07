use std::io;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Options(String),
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}
