use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum LinearizeError {
    NotFound,
    BadHead { base: String, this: String },
    Circular,
}

pub type Result<T> = std::result::Result<T, LinearizeError>;

impl Display for LinearizeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LinearizeError::NotFound => write!(f, "cannot find C3-linearization for input"),
            LinearizeError::BadHead { .. } => write!(f, "cannot find C3-linearization for input"),
            LinearizeError::Circular => write!(f, "Circular dependency found"),
        }
    }
}
