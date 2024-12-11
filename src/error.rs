use std::fmt::{self, Display, Formatter};
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Os { cause: io::Error },
    UnexpectedArg { arg: String },
}

impl Error {
    pub fn os() -> Error {
        Error::Os {
            cause: io::Error::last_os_error(),
        }
    }

    pub fn unexpected_arg(arg: &str) -> Error {
        Error::UnexpectedArg {
            arg: arg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Os { cause } => write!(f, "I/O error: {cause}"),
            Error::UnexpectedArg { arg } => write!(f, "{arg}: unexpected argument"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::Os { cause }
    }
}
