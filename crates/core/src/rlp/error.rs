use thiserror::Error;

#[derive(Debug, Error)]
pub enum RLPDecodeError {
    #[error("InvalidLength")]
    InvalidLength,
    #[error("MalformedData")]
    MalformedData,
    #[error("MalformedBoolean")]
    MalformedBoolean,
    #[error("UnxpectedList")]
    UnxpectedList,
    #[error("UnxpectedString")]
    UnxpectedString,
    #[error("{0}")]
    Custom(String)
}