use std::fmt;

pub enum Error {
    ExecutionError { message: String },
    FieldTypeError,
}

impl<T: std::error::Error> From<T> for Error {
    fn from(e: T) -> Self {
        Error::ExecutionError {
            message: e.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ExecutionError { message } => write!(f, "{}", message),
            Error::FieldTypeError => write!(f, "Field type error"),
        }
    }
}
