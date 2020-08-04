use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
    ops::Deref,
    result,
};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TerminalSize,
    Io(Box<dyn error::Error>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(Box::new(err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let msg: Box<dyn Display> = match &self {
            Error::TerminalSize => Box::new("terminal size isn't fit for the game"),
            Error::Io(err) => Box::new(err),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::TerminalSize => None,
            Error::Io(err) => Some(err.deref()),
        }
    }
}
