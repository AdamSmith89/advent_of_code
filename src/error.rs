use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdventError {
    #[error("Empty slice")]
    EmptySlice,
    #[error("No items left in iterator")]
    EndOfIterator,
    #[error("{0} not found")]
    NotFound(String),
    #[error("Failed to parse int: {0}")]
    ParseInt(ParseIntError),
    #[error("Failed to convert {0} to digit")]
    ParseDigit(char),
    #[error("Failed to split {0} at {1}")]
    SplitOnce(String, String),
    #[error("Failed to resolve enum type from {0}")]
    StringToEnum(String),
    #[error("Expected {0}, found {1}")]
    UnexpectedValue(String, String),
    #[error("Unknown pattern: {0}")]
    UnknownPattern(String),
    #[error("Failed to solve puzzle: {0}")]
    LogicError(String),
}

impl From<ParseIntError> for AdventError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
