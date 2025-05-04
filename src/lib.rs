use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

pub trait SignedTransactionT: Clone + Send + Sync + Debug + 'static {}

pub trait BlockT: Clone + Send + Sync + Debug + 'static {
    type Transaction: SignedTransactionT;
    type BlockHeader: BlockHeaderT;

    fn header(&self) -> &Self::BlockHeader;
    fn transactions(&self) -> &[Self::Transaction];
    fn new(header: Self::BlockHeader, transactions: Vec<Self::Transaction>) -> Self;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block<BlockHeader, Transaction> {
    // The block header.
    pub header: BlockHeader,
    // Transactions in block
    pub transactions: Vec<Transaction>,
}

impl<BlockHeader, Transaction> BlockT for Block<BlockHeader, Transaction>
where
    BlockHeader: BlockHeaderT,
    Transaction: SignedTransactionT,
{
    type BlockHeader = BlockHeader;
    type Transaction = Transaction;

    fn header(&self) -> &Self::BlockHeader {
        &self.header
    }

    fn transactions(&self) -> &[Self::Transaction] {
        &self.transactions[..]
    }

    fn new(header: Self::BlockHeader, transactions: Vec<Self::Transaction>) -> Self {
        Block {
            header,
            transactions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;

    // Test implementation of SignedTransactionT
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestTransaction {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for TestTransaction {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }
    impl SignedTransactionT for TestTransaction {}

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct KeccakHasher;

    impl HasherT for KeccakHasher {
        type Output = B256;

        fn hash(s: &[u8]) -> Self::Output {
            alloy::primitives::keccak256(s)
        }
    }

    #[test]
    fn test_block_serialization() {
        // Create test data
        let parent_hash = KeccakHasher::hash(b"parent");
        let state_root = KeccakHasher::hash(b"state");
        let number = 1u64;

        // Create a block header
        let header = BlockHeader::<u64, KeccakHasher>::new(number, state_root, parent_hash);

        // Create some transactions
        let transactions = vec![
            TestTransaction {
                data: vec![1, 2, 3],
            },
            TestTransaction {
                data: vec![4, 5, 6],
            },
        ];

        // Create a block
        let block = Block::new(header, transactions);

        // Serialize to JSON
        let serialized = serde_json::to_string(&block).unwrap();

        // Deserialize back
        let deserialized: Block<BlockHeader<u64, KeccakHasher>, TestTransaction> =
            serde_json::from_str(&serialized).unwrap();

        // Verify the deserialized block matches the original
        assert_eq!(block.header.number, deserialized.header.number);
        assert_eq!(block.header.parent_hash, deserialized.header.parent_hash);
        assert_eq!(block.header.state_root, deserialized.header.state_root);
        assert_eq!(block.transactions.len(), deserialized.transactions.len());

        // Verify transaction data
        for (original, deserialized) in block
            .transactions
            .iter()
            .zip(deserialized.transactions.iter())
        {
            assert_eq!(original.data, deserialized.data);
        }
    }
}
