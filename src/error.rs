use thiserror;

pub type Result<T> = std::result::Result<T, GenAiError>;

#[derive(thiserror::Error, Debug)]
pub enum GenAiError {
    #[error("Internal error: {0}")]
    Internal(String),
}
