use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdventError {
    #[error("Empty slice")]
    EmptySlice,
    #[error("No items left in iterator")]
    EndOfIterator,
    #[error("{0} not found")]
    NotFound(String),
    #[error("Failed to parse int")]
    ParseInt(ParseIntError),
    #[error("Failed to split {0} at {1}")]
    SplitOnce(String, String),
    #[error("Failed to resolve enum type from {0}")]
    StringToEnum(String),
    #[error("Expected {0}, found {1}")]
    UnexpectedValue(String, String),
    #[error("Unknown pattern: {0}")]
    UnknownPattern(String),
}

impl From<ParseIntError> for AdventError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
