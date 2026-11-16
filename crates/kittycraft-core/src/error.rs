use thiserror::Error;

#[derive(Debug, Error)]
pub enum KittyCraftError {
    #[error("SQLite error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("LLM error: {0}")]
    Llm(String),
}
