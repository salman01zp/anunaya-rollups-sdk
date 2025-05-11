#[derive(Debug, thiserror::Error)]
pub enum VIDError {
    // Generic Error
    #[error("{}", _0)]
    GenericError(String),
}