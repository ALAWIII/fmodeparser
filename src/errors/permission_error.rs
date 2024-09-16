use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]

/// the error that can be returned when parsing the permission.
pub struct FullPermissionError {
    message: String,
}
impl FullPermissionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
impl Display for FullPermissionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for FullPermissionError {}

impl From<Box<dyn Error>> for FullPermissionError {
    fn from(value: Box<dyn Error>) -> Self {
        FullPermissionError::new(value.to_string())
    }
}
impl From<std::io::Error> for FullPermissionError {
    fn from(value: std::io::Error) -> Self {
        FullPermissionError::new(value.to_string())
    }
}
