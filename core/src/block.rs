use crate::traits::{BlockHeaderT, BlockT, HasherT, SignedTransactionT};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use hex::decode;

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

    fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(68);
        encoded.extend_from_slice(self.parent_hash.as_ref());
        let number_le: u32 = self.number.into() as u32;
        encoded.extend_from_slice(&number_le.to_le_bytes());
        encoded.extend_from_slice(self.state_root.as_ref());
        encoded
    }
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
    use anyhow::Ok;

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
    #[test]
    fn test_header_encoding() -> Result<(), anyhow::Error> {
        let parent_hash = KeccakHasher::hash(b"parent");
        let state_root = KeccakHasher::hash(b"state");
        let number = 1u64;
        let header = BlockHeader::<u64, KeccakHasher>::new(number, state_root, parent_hash);


        let encoded = header.encode();
        assert_eq!(encoded.len(), 68);

        // Check parent_hash (first 32 bytes)
        let expected = hex::decode("ff483e972a04a9a62bb4b7d04ae403c615604e4090521ecc5bb7af67f71be09c")?;
        assert_eq!(&encoded[..32], expected.as_slice());
        Ok(())
    }
}
