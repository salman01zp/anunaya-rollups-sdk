use crate::error::Result;
use tokio::sync::{Mutex, broadcast};

pub struct SequencerConfig;

pub struct TransactionStore;

pub struct SequencerContext {
    config: SequencerConfig,
    notify_shutdown: broadcast::Sender<()>,
    store: TransactionStore,
}

impl SequencerContext {
    pub async fn new(config: SequencerConfig, store: TransactionStore) -> Result<Self> {
        let (notify_shutdown, _) = broadcast::channel(2);

        Ok(Self {
            config,
            store,
            notify_shutdown,
        })
    }

    /// Returns a broadcast receiver handle for the shutdown signal.
    pub fn shutdown_signal(&self) -> Shutdown {
        Shutdown::new(self.notify_shutdown.subscribe())
    }

    /// Sends a shutdown signal to all subscribed tasks/connections.
    pub fn shutdown(&self) {
        let _ = self.notify_shutdown.send(());
    }
}

/// Listens for the server shutdown signal.
///
/// Shutdown is signalled using a `broadcast::Receiver`. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, the server
/// should shutdown.
///
/// The `Shutdown` struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
#[derive(Debug)]
pub struct Shutdown {
    /// `true` if the shutdown signal has been received
    shutdown: bool,

    /// The receive half of the channel used to listen for shutdown.
    notify: broadcast::Receiver<()>,
}
impl Shutdown {
    /// Create a new `Shutdown` backed by the given `broadcast::Receiver`.
    pub fn new(notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown {
            shutdown: false,
            notify,
        }
    }

    /// Receive the shutdown notice, waiting if necessary.
    pub async fn recv(&mut self) {
        // If the shutdown signal has already been received, then return
        // immediately.
        if self.shutdown {
            return;
        }

        // Cannot receive a "lag error" as only one value is ever sent.
        let _ = self.notify.recv().await;

        // Remember that the signal has been received.
        self.shutdown = true;
    }
}
