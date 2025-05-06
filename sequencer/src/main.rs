mod error;
mod sequencer;
mod transaction;
mod logger;

use logger::setup_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger(2, "Sequencer")?;
    Ok(())
}
