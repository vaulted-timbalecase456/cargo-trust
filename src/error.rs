use std::path::PathBuf;
use thiserror::Error;

/**
 * Custom error types for cargo-trust operations
 */

#[derive(Error, Debug)]
pub enum TrustError {
    #[error("Failed to read file {path}: {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Failed to parse Rust code in {path}: {message}")]
    Parse {
        path: PathBuf,
        message: String,
    },
    
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),
    
    #[error("Invalid project structure: {0}")]
    InvalidProject(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}