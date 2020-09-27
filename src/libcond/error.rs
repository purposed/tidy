use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ExecutionError {
    message: String,
}

impl ExecutionError {
    pub fn new<T>(msg: T) -> Self
    where
        String: From<T>,
    {
        ExecutionError {
            message: String::from(msg),
        }
    }
}

impl<T: Error> From<T> for ExecutionError {
    fn from(e: T) -> Self {
        ExecutionError {
            message: e.to_string(),
        }
    }
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
