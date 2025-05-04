#![allow(dead_code)]
use std::fmt::Debug;

pub trait HasherT: Sync + Send {
    type Output: AsRef<[u8]> + Send + Sync + Clone + Debug + Copy;
    fn hash(s: &[u8]) -> Self::Output;
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

    fn encode(&self) -> &[u8] {
        todo!()
    }
    // Returns the hash of the header.
    fn hash(&self) -> Self::Hash {
        Self::Hashing::hash(self.encode())
    }
}

pub trait SignedTransactionT: Clone + Send + Sync + Debug + 'static {}

pub trait BlockT: Clone + Send + Sync + Debug + 'static {
    type Transaction: SignedTransactionT;
    type BlockHeader: BlockHeaderT;

    fn header(&self) -> &Self::BlockHeader;
    fn transactions(&self) -> &[Self::Transaction];
    fn new(header: Self::BlockHeader, transactions: Vec<Self::Transaction>) -> Self;
}
