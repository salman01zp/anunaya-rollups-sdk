use crate::error::{Result, TxStoreError};
use crate::transaction::SignedTransaction;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct TransactionStore {
    mempool: Arc<Mutex<VecDeque<SignedTransaction>>>,
    mempool_max_txs_count: usize,
}

impl TransactionStore {
    pub fn new(mempool_max_txs_count: usize) -> Self {
        Self {
            mempool: Arc::new(Mutex::new(VecDeque::new())),
            mempool_max_txs_count,
        }
    }

    /// Push a transaction to the mempool if there's space available
    pub fn push(&self, transaction: SignedTransaction) -> Result<()> {
        let mut mempool = self.mempool.lock().map_err(|_| TxStoreError::LockError)?;

        if mempool.len() >= self.mempool_max_txs_count {
            return Err(TxStoreError::MempoolFull.into());
        }

        mempool.push_back(transaction);
        Ok(())
    }

    /// Remove a transaction from the mempool by its index
    pub fn remove(&self, index: usize) -> Result<SignedTransaction> {
        let mut mempool = self.mempool.lock().map_err(|_| TxStoreError::LockError)?;

        if index >= mempool.len() {
            return Err(TxStoreError::IndexOutOfBounds.into());
        }

        Ok(mempool.remove(index).unwrap())
    }

    /// Get the current size of the mempool
    pub fn size(&self) -> Result<usize> {
        let mempool = self.mempool.lock().map_err(|_| TxStoreError::LockError)?;
        Ok(mempool.len())
    }

    /// Get the next transaction from the front of the mempool without removing it
    pub fn peek_front(&self) -> Result<Option<SignedTransaction>> {
        let mempool = self.mempool.lock().map_err(|_| TxStoreError::LockError)?;
        Ok(mempool.front().cloned())
    }

    /// Remove and return the next transaction from the front of the mempool
    pub fn pop_front(&self) -> Result<Option<SignedTransaction>> {
        let mut mempool = self.mempool.lock().map_err(|_| TxStoreError::LockError)?;
        Ok(mempool.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SequencerError;
    use crate::transaction::{SignedTransaction, Transaction};
    use alloy::signers::{Signer, local::PrivateKeySigner};

    // Helper function to create a test transaction
    async fn create_test_transaction() -> SignedTransaction {
        let signer = PrivateKeySigner::random();
        let transaction = Transaction {
            amount: 100,
            destination: signer.address(),
            nonce: 1,
        };
        let signature = signer.sign_message(&transaction.encode()).await.unwrap();
        SignedTransaction {
            transaction,
            signature,
        }
    }

    #[tokio::test]
    async fn test_new_store() {
        let store = TransactionStore::new(10);
        assert_eq!(store.size().unwrap(), 0);
    }

    #[tokio::test]
    async fn test_push_transaction() {
        let store = TransactionStore::new(2);
        let tx = create_test_transaction().await;

        // Test successful push
        assert!(store.push(tx.clone()).is_ok());
        assert_eq!(store.size().unwrap(), 1);

        // Test pushing to full mempool
        let tx2 = create_test_transaction().await;
        assert!(store.push(tx2.clone()).is_ok());
        assert_eq!(store.size().unwrap(), 2);

        // Test pushing when mempool is full
        let tx3 = create_test_transaction().await;
        assert!(matches!(
            store.push(tx3),
            Err(SequencerError::TxStoreError(TxStoreError::MempoolFull))
        ));
        assert_eq!(store.size().unwrap(), 2);
    }

    #[tokio::test]
    async fn test_remove_transaction() {
        let store = TransactionStore::new(2);
        let tx1 = create_test_transaction().await;
        let tx2 = create_test_transaction().await;

        // Push two transactions
        store.push(tx1.clone()).unwrap();
        store.push(tx2.clone()).unwrap();

        // Test removing first transaction
        store.remove(0).unwrap();
        assert_eq!(store.size().unwrap(), 1);

        // Test removing second transaction
        store.remove(0).unwrap();
        assert_eq!(store.size().unwrap(), 0);

        // Test removing from empty mempool
        assert!(matches!(
            store.remove(0),
            Err(SequencerError::TxStoreError(TxStoreError::IndexOutOfBounds))
        ));
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let store = Arc::new(TransactionStore::new(100));
        let mut handles = vec![];

        // Spawn multiple tasks that push transactions
        for _ in 0..10 {
            let store_clone = Arc::clone(&store);
            let handle = tokio::spawn(async move {
                let tx = create_test_transaction().await;
                store_clone.push(tx)
            });
            handles.push(handle);
        }

        // Wait for all pushes to complete
        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        // Verify final size
        assert_eq!(store.size().unwrap(), 10);
    }

    #[tokio::test]
    async fn test_remove_out_of_bounds() {
        let store = TransactionStore::new(1);
        let tx = create_test_transaction().await;

        // Test removing from empty mempool
        assert!(matches!(
            store.remove(0),
            Err(SequencerError::TxStoreError(TxStoreError::IndexOutOfBounds))
        ));

        // Push a transaction
        store.push(tx).unwrap();

        // Test removing with invalid index
        assert!(matches!(
            store.remove(1),
            Err(SequencerError::TxStoreError(TxStoreError::IndexOutOfBounds))
        ));
    }

    #[tokio::test]
    async fn test_peek_and_pop_front() {
        let store = TransactionStore::new(2);
        let tx1 = create_test_transaction().await;
        let tx2 = create_test_transaction().await;

        // Push two transactions
        store.push(tx1.clone()).unwrap();
        store.push(tx2.clone()).unwrap();

        // Test peek front
        assert!(store.peek_front().unwrap().is_some());
        assert_eq!(store.size().unwrap(), 2); // Size should remain unchanged

        // Test pop front
        let popped = store.pop_front().unwrap().unwrap();
        assert_eq!(store.size().unwrap(), 1);
        assert_eq!(popped.transaction.amount, tx1.transaction.amount);

        // Test pop front again
        let popped2 = store.pop_front().unwrap().unwrap();
        assert_eq!(store.size().unwrap(), 0);
        assert_eq!(popped2.transaction.amount, tx2.transaction.amount);

        // Test pop front on empty mempool
        assert!(store.pop_front().unwrap().is_none());
    }
}
