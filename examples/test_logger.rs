use atoman::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(any(feature = "logger"))]
    {
        Logger::init(".test/logs", 20)?;
        
        info!("Hello, World!");
    }

    Ok(())
}
