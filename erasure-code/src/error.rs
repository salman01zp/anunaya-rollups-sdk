
#[derive(Debug, thiserror::Error)]
pub enum ErasureCodeError {
        /// Generic error.
        #[error("{}", _0)]
        Generic(&'static str),
}