mod error;
mod logger;
mod sequencer;
mod store;
mod transaction;

use logger::setup_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger(2, "Sequencer")?;
    Ok(())
}
