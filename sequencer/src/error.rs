use alloy::primitives::SignatureError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, SequencerError>;

#[derive(Debug, thiserror::Error)]
pub enum SequencerError {
    /// An Io error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Generic error.
    #[error("{}", _0)]
    Generic(&'static str),
    /// Transaction verification
    #[error(transparent)]
    SignatureError(#[from] SignatureError),
    /// Transaction store error
    #[error(transparent)]
    TxStoreError(#[from] TxStoreError),
}

#[derive(Debug, thiserror::Error)]
pub enum TxStoreError {
    /// Mempool is full
    #[error("Mempool is full")]
    MempoolFull,
    /// Index out of bounds
    #[error("Index out of bounds")]
    IndexOutOfBounds,
    /// Failed to acquire lock
    #[error("Failed to acquire lock")]
    LockError,
}

impl From<SequencerError> for ApiError {
    fn from(value: SequencerError) -> Self {
        ApiError(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

/// Error type for HTTP handlers
pub struct ApiError(
    /// HTTP status code for response
    pub StatusCode,
    /// Response message
    pub String,
);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}
