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

#[derive(Clone, Debug)]
pub struct BlockHeader<Number: Copy + Into<u64>, Hash: HasherT> {
    pub parent_hash: Hash::Output,
    pub number: Number,
    pub state_root: Hash::Output,
}

impl<Number, Hash> BlockHeaderT for BlockHeader<Number, Hash>
where
    Number: Copy + Into<u64> + Send + Sync + Debug + Clone + 'static,
    Hash: HasherT + Debug + Clone + 'static,
{
    type Number = Number;
    type Hash = Hash::Output;
    type Hashing = Hash;

    fn new(number: Number, state_root: Hash::Output, parent_hash: Hash::Output) -> Self {
        Self {
            number,
            state_root,
            parent_hash,
        }
    }

    fn number(&self) -> &Self::Number {
        &self.number
    }

    fn state_root(&self) -> &Self::Hash {
        &self.state_root
    }

    fn parent_hash(&self) -> &Self::Hash {
        &self.parent_hash
    }

    fn encode(&self) -> &[u8] {
        todo!()
    }
}
