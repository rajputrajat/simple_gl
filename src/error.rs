use glfw::InitError;

#[derive(Debug)]
pub enum CustomError {
    Glfw(InitError),
    Other(String),
}

impl From<InitError> for CustomError {
    fn from(e: InitError) -> Self {
        CustomError::Glfw(e)
    }
}

pub type CustomResult = Result<(), CustomError>;
