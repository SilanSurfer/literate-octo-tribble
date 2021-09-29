use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("No field \"type\" in JSON: {0}")]
    NoFieldTypeInJson(String),
    #[error("Field \"type\" is not a String, JSON: {0}")]
    FieldTypeIsNotString(String),
}
