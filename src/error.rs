use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::ops::Deref;

#[derive(Debug)]
pub enum GameError {
    TerminalSize,
    Io(Box<dyn Error>),
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        GameError::Io(Box::new(err))
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let msg: Box<dyn Display> = match &self {
            GameError::TerminalSize => Box::new("terminal size isn't fit for the game"),
            GameError::Io(err) => Box::new(err),
        };
        write!(f, "{}", msg)
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            GameError::TerminalSize => None,
            GameError::Io(err) => Some(err.deref()),
        }
    }
}
