use alloy::primitives::SignatureError;

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
