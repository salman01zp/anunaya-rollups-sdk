#[derive(thiserror::Error, Debug)]
pub enum VidError {
    /// Caller provided an invalid argument
    #[error("invalid arguments: {0}")]
    Argument(String),

}

pub type VidResult<T> = Result<T, VidError>;