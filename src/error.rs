use glfw::InitError;
use std::io;

#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    Glfw(InitError),
    Other(String),
}

impl From<InitError> for CustomError {
    fn from(e: InitError) -> Self {
        CustomError::Glfw(e)
    }
}

impl From<io::Error> for CustomError {
    fn from(e: io::Error) -> Self {
        CustomError::Io(e)
    }
}

pub type CustomResult = Result<(), CustomError>;
