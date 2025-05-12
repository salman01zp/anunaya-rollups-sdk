use serde::{Serialize, Deserialize};
use crate::traits::HasherT;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeccakHasher;

impl HasherT for KeccakHasher {
    type Output = [u8;32];

    fn hash(s: &[u8]) -> Self::Output {
        alloy::primitives::keccak256(s).0
    }
}


