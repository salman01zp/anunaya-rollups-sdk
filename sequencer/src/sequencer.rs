use crate::{error::Result, store::TransactionStore, transaction::SignedTransaction};

#[derive(Debug, Clone, Default)]
pub struct SequencerConfig;

#[derive(Debug, Clone)]
pub struct SequencerContext {
    pub config: SequencerConfig,
    pub store: TransactionStore,
}

pub trait SequencerRpcMethods {
    fn accept_tx(&self, tx: Vec<u8>) -> Result<()>;
    fn publish_batch(&self) -> Result<()>;
}

impl SequencerContext {
    pub fn new(config: SequencerConfig, store: TransactionStore) -> Result<Self> {
        Ok(Self { config, store })
    }
}

impl SequencerRpcMethods for SequencerContext {
    fn accept_tx(&self, tx: Vec<u8>) -> Result<()> {
        tracing::info!("Accepting tx: 0x{}", hex::encode(&tx));

        // Decode the transaction
        let signed_tx = SignedTransaction::decode(&tx)
            .ok_or_else(|| crate::error::SequencerError::Generic("Failed to decode transaction"))?;

        // Push to mempool
        self.store.push(signed_tx)?;
        Ok(())
    }

    fn publish_batch(&self) -> Result<()> {
        todo!()
    }
}
