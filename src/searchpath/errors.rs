use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum FinderError {
    NotFound(String)
}

impl Error for FinderError {}

impl Display for FinderError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FinderError::NotFound(filename) => write!(f, "couldn't find file {}.", filename)
        }
    }
}

impl From<&str> for FinderError {
    fn from(filename: &str) -> Self {
        FinderError::NotFound(filename.to_string())
    }
}
