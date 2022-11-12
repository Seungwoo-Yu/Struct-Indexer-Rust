use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ErrorCodes {
    NoIdFound
}

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCodes
}

impl Display for ErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.code {
            ErrorCodes::NoIdFound => {
                write!(f, "No item found for Id ({})", self.code)
            }
        }
    }
}