mod error;
mod sequencer;
mod transaction;
mod logger;
mod store;

use logger::setup_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger(2, "Sequencer")?;
    Ok(())
}
