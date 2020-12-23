use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SimpleError(String);

impl SimpleError {
    pub fn new<T>(error: T) -> SimpleError
        where T: Display {
        SimpleError(error.to_string())
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SimpleError {}
