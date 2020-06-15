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

    fn msg(&self) -> &str {
        match &self.err_type {
            GameErrorType::TerminalSize => "terminal size isn't fit for the game",
            GameErrorType::Io(err) => err.description(),
        }
    }
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        GameError::new(GameErrorType::Io(Box::new(err)))
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg())
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        self.msg()
    }

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
