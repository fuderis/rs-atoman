#![cfg(all(feature = "trace", feature = "logger"))]
use atoman::{Logger, Trace, info, prelude::*};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(".logs", 20)?;
    let log_path = Logger::get_path().unwrap();

    // start log file tracing:
    let trace_handle = tokio::spawn(async move {
        let mut trace = Trace::open(log_path, Duration::from_millis(50))
            .await
            .expect("Failed to open trace");

        let mut count = 0;
        while count < 10 {
            if let Some(line) = trace.next_line().await {
                println!("Traced line: {}", line);
                count += 1;
            }
        }
    });

    // wait for spawn thread:
    sleep(Duration::from_millis(100)).await;

    for i in 1..=15 {
        info!("Test log #{}", i);
        sleep(Duration::from_millis(100)).await;
    }

    // waiting trace thread:
    let _ = trace_handle.await;

    Ok(())
}
