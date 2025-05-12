#[derive(Debug, thiserror::Error)]
pub enum ErasureCodeError {
    // Insufficient Shares
    #[error("{}", _0)]
    InsufficientSharesError(String),
}
