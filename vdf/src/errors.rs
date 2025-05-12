#[derive(Debug, thiserror::Error)]
pub enum VDFError {
    // Generic Error
    #[error("{}", _0)]
    GenericError(String),
    // VerificationError
    #[error("{}", _0)]
    VerificationError(String),
}
