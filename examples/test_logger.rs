#![cfg(feature = "logger")]
use atoman::{
    Logger,
    log::{debug, error, info, trace, warn},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(".logs", 20)?;
    Logger::set_level(Level::Trace);

    info!("Hello, World!");
    warn!("Some warning..");
    error!("Some error..");
    debug!("It's a test..");
    trace!("Test success!");

    Ok(())
}
