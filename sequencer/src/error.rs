use alloy::primitives::SignatureError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum RollupError {
    SignatureError(#[from] SignatureError),
}

impl std::fmt::Display for RollupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
