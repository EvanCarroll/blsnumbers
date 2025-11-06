use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlsError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("BLS API error: {0}")]
    Api(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("No data returned for series {0}")]
    NoData(String),
}

pub type Result<T> = std::result::Result<T, BlsError>;
