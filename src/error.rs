use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::ops::Deref;

#[derive(Debug)]
pub struct GameError {
    err_type: GameErrorType,
}

impl GameError {
    pub fn new(err_type: GameErrorType) -> GameError {
        GameError { err_type: err_type }
    }
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        GameError::new(GameErrorType::Io(Box::new(err)))
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let msg: Box<dyn Display> = match &self.err_type {
            GameErrorType::TerminalSize => Box::new("terminal size isn't fit for the game"),
            GameErrorType::Io(err) => Box::new(err),
        };
        write!(f, "{}", msg)
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.err_type {
            GameErrorType::TerminalSize => None,
            GameErrorType::Io(err) => Some(err.deref()),
        }
    }
}

#[derive(Debug)]
pub enum GameErrorType {
    TerminalSize,
    Io(Box<dyn Error>),
}
