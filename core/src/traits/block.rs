#![allow(dead_code)]
use super::*;
use std::fmt::Debug;

pub trait BlockT: Clone + Send + Sync + Debug + 'static {
    type Transaction: SignedTransactionT;
    type BlockHeader: BlockHeaderT;

    fn header(&self) -> &Self::BlockHeader;
    fn transactions(&self) -> &[Self::Transaction];
    fn new(header: Self::BlockHeader, transactions: Vec<Self::Transaction>) -> Self;
}

pub trait BlockHeaderT: Clone + Send + Sync + Debug + 'static {
    // Header number
    type Number: Into<u64>;
    // Header hash type
    type Hash: AsRef<[u8]>;
    // Hashing algorithm;
    type Hashing: HasherT<Output = Self::Hash>;

    // Creates new header
    fn new(number: Self::Number, state_root: Self::Hash, parent_hash: Self::Hash) -> Self;

    // Return reference to the header number.
    fn number(&self) -> &Self::Number;

    // Returns a reference to the state root.
    fn state_root(&self) -> &Self::Hash;

    // Returns a reference to the parent hash.
    fn parent_hash(&self) -> &Self::Hash;

    // Returns a vector of encoded values
    fn encode(&self) -> Vec<u8>;

    // Returns the hash of the header.
    fn hash(&self) -> Self::Hash {
        Self::Hashing::hash(&self.encode())
    }
}
