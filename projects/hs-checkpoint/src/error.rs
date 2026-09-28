use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckpointError {
    #[error("{path}: {message}")]
    Io { path: PathBuf, message: String },
    #[error("invalid checkpoint: {0}")]
    Invalid(String),
    #[error("unsupported target: {0}")]
    UnsupportedTarget(String),
}
