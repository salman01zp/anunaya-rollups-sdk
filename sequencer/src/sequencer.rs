use tokio::sync::{broadcast, Mutex};

pub struct SequencerConfig;

pub struct TransactionStore;

pub struct SequencerContext {
    config : SequencerConfig,
    notify_shutdown: broadcast::Sender<()>,
    store: TransactionStore,
}

