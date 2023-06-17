use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Error {
    NotFound,
    BadHead,
    Circular,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::NotFound => write!(f, "cannot find C3-linearization for input"),
            Error::BadHead => write!(f, "cannot find C3-linearization for input"),
            Error::Circular => write!(f, "Circular dependency found"),
        }
    }
}
