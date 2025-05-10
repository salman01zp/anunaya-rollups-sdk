use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
use crate::error::ErasureCodeError;

/// Erasure code trait
/// `T` is the input data type
pub trait ErasureCode<T> {
    /// Type for each data shares (usually depends on `T`)
    /// Why so many trait bounds on `Share`? <https://github.com/rust-lang/rust/issues/20671>
    type Share: Debug
        + Clone
        + Eq
        + PartialEq
        + Sync
        + Send
        + Serialize
        + DeserializeOwned;

    /// Encode `data` into `data.len() + parity_size` shares.
    fn encode(data: &[T], parity_size: usize) -> Result<Vec<Self::Share>, ErasureCodeError>;

    /// Decode `shares` into `data_size` data elements.
    /// Return `Result::Err` if `shares.len() < data_size`.
    fn decode(shares: &[Self::Share], data_size: usize) -> Result<Vec<T>, ErasureCodeError>;

}