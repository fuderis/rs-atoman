#![cfg(feature = "logger")]
use atoman::{Logger, debug, error, info, prelude::*, trace, warn};
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(".logs", 1000).await?;
    Logger::set_level(Level::Trace).await;

    info!("Hello, World!");
    warn!("Some warning..");
    error!("Some error..");
    debug!("It's a test..");
    trace!("Test success!");

    // ensures file I/O completes before exit:
    time::sleep(time::Duration::from_millis(10)).await;

    Ok(())
}
