#[derive(Debug, thiserror::Error)]
pub enum TokenDappRollupError {
    // Insufficient Shares
    #[error("{}", _0)]
    Generic(String),
}
