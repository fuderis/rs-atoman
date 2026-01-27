#![cfg(feature = "logger")]
use atoman::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(".logs", 20)?;
    info!("Hello, World!");

    Ok(())
}
