use std::sync::Arc;
use std::collections::VecDeque;
use crate::transaction::SignedTransaction;
use tokio::sync::Mutex;

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
    pub async fn push(&self, transaction: SignedTransaction) -> Result<(), String> {
        let mut mempool = self.mempool.lock().await;
        
        if mempool.len() >= self.mempool_max_txs_count {
            return Err("Mempool is full".to_string());
        }

        mempool.push_back(transaction);
        Ok(())
    }

    /// Remove a transaction from the mempool by its index
    pub async fn remove(&self, index: usize) -> Result<SignedTransaction, String> {
        let mut mempool = self.mempool.lock().await;
        
        if index >= mempool.len() {
            return Err("Index out of bounds".to_string());
        }

        Ok(mempool.remove(index).unwrap())
    }

    /// Get the current size of the mempool
    pub async fn size(&self) -> usize {
        let mempool = self.mempool.lock().await;
        mempool.len()
    }

    /// Get the next transaction from the front of the mempool without removing it
    pub async fn peek_front(&self) -> Option<SignedTransaction> {
        let mempool = self.mempool.lock().await;
        mempool.front().cloned()
    }

    /// Remove and return the next transaction from the front of the mempool
    pub async fn pop_front(&self) -> Option<SignedTransaction> {
        let mut mempool = self.mempool.lock().await;
        mempool.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(store.size().await, 0);
    }

    #[tokio::test]
    async fn test_push_transaction() {
        let store = TransactionStore::new(2);
        let tx = create_test_transaction().await;

        // Test successful push
        assert!(store.push(tx.clone()).await.is_ok());
        assert_eq!(store.size().await, 1);

        // Test pushing to full mempool
        let tx2 = create_test_transaction().await;
        assert!(store.push(tx2.clone()).await.is_ok());
        assert_eq!(store.size().await, 2);

        // Test pushing when mempool is full
        let tx3 = create_test_transaction().await;
        assert!(store.push(tx3).await.is_err());
        assert_eq!(store.size().await, 2);
    }

    #[tokio::test]
    async fn test_remove_transaction() {
        let store = TransactionStore::new(2);
        let tx1 = create_test_transaction().await;
        let tx2 = create_test_transaction().await;

        // Push two transactions
        store.push(tx1.clone()).await.unwrap();
        store.push(tx2.clone()).await.unwrap();

        // Test removing first transaction
        store.remove(0).await.unwrap();
        assert_eq!(store.size().await, 1);

        // Test removing second transaction
        store.remove(0).await.unwrap();
        assert_eq!(store.size().await, 0);

        // Test removing from empty mempool
        assert!(store.remove(0).await.is_err());
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
                store_clone.push(tx).await
            });
            handles.push(handle);
        }

        // Wait for all pushes to complete
        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        // Verify final size
        assert_eq!(store.size().await, 10);
    }

    #[tokio::test]
    async fn test_remove_out_of_bounds() {
        let store = TransactionStore::new(1);
        let tx = create_test_transaction().await;

        // Test removing from empty mempool
        assert!(store.remove(0).await.is_err());

        // Push a transaction
        store.push(tx).await.unwrap();

        // Test removing with invalid index
        assert!(store.remove(1).await.is_err());
    }

    #[tokio::test]
    async fn test_peek_and_pop_front() {
        let store = TransactionStore::new(2);
        let tx1 = create_test_transaction().await;
        let tx2 = create_test_transaction().await;

        // Push two transactions
        store.push(tx1.clone()).await.unwrap();
        store.push(tx2.clone()).await.unwrap();

        // Test peek front
        store.peek_front().await.unwrap();
        assert_eq!(store.size().await, 2); // Size should remain unchanged

        // Test pop front
        let popped = store.pop_front().await.unwrap();
        assert_eq!(store.size().await, 1);
        assert_eq!(popped.transaction.amount, tx1.transaction.amount);

        // Test pop front again
        let popped2 = store.pop_front().await.unwrap();
        assert_eq!(store.size().await, 0);
        assert_eq!(popped2.transaction.amount, tx2.transaction.amount);

        // Test pop front on empty mempool
        assert!(store.pop_front().await.is_none());
    }
}