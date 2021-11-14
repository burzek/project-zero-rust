use std::fmt;

pub struct GameError {
    message : String,
}

impl GameError {
    pub fn new(msg : &str) -> Self {
        Self {message : msg.into()}
    }
    pub fn new_formatted(msg : String) -> Self {
        Self {message : msg}
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
