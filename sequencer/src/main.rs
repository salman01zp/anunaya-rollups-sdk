mod error;
mod logger;
mod routes;
mod sequencer;
mod store;
mod transaction;

use logger::setup_logger;
use routes::build_api_services;
use sequencer::{SequencerConfig, SequencerContext};
use store::TransactionStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger(2, "Sequencer")?;

    // Sequencer configuration
    let config = SequencerConfig::default();
    // Transaction storage for sequencer
    let store = TransactionStore::new(100);
    // SequencerContext takes a configuration and populates object that are necessary througout the lifetme of sequencer
    let ctx = SequencerContext::new(config, store)?;

    // Build API services
    build_api_services(ctx.clone()).await?;

    Ok(())
}
