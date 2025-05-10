use crate::errors::ErasureCodeError;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{fmt::Debug, vec::Vec};

/// Erasure code trait
/// `T` is the input data type
pub trait ErasureCode<T> {
    /// Type for each data shares (usually depends on `T`)
    type Share: Debug
        + Clone
        + Eq
        + PartialEq
        + Sync
        + Send
        + CanonicalSerialize
        + CanonicalDeserialize;

    /// Encode `data` into `data.len() + parity_size` shares.
    fn encode(data: &[T], parity_size: usize) -> Result<Vec<Self::Share>, ErasureCodeError>;

    /// Decode `shares` into `data_size` data elements.
    /// Return `Result::Err` if `shares.len() < data_size`.
    fn decode(shares: &[Self::Share], data_size: usize) -> Result<Vec<T>, ErasureCodeError>;
}
