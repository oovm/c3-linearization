mod algorithm;
mod errors;

pub use algorithm::merge;
pub use errors::{Error, Result};

pub struct C3 {
    reverse: bool,
    python: bool,
}

impl Default for C3 {
    fn default() -> Self {
        Self { reverse: false, python: false }
    }
}
