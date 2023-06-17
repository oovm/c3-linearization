use std::fmt::{self, Display, Formatter};

/// A type alias for the result of a linearization.
pub type LinearizeResult<T> = std::result::Result<T, LinearizeError>;

/// A virtual inheritance.
#[derive(Debug, Clone)]
pub enum LinearizeError {
    /// The input class was not found in the graph.
    NotFound {
        /// The name of the class that was not found.
        base: String,
    },
    /// The input class was found, but it was not the head of any sequence.
    BadHead {
        /// The name of the class that was not the head of any sequence.
        base: String,
        /// The name of the class that was the head of the sequence.
        this: String,
    },
    /// A circular dependency was found in the graph.
    Circular {
        /// The names of the classes that were involved in the circular dependency.
        class: String,
    },
}

impl Display for LinearizeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LinearizeError::NotFound { .. } => write!(f, "cannot find C3-linearization for input"),
            LinearizeError::BadHead { .. } => write!(f, "cannot find C3-linearization for input"),
            LinearizeError::Circular { .. } => write!(f, "Circular dependency found"),
        }
    }
}
