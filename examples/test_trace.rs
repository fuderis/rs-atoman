#![cfg(all(feature = "trace", feature = "logger"))]
use atoman::{Logger, Trace, info, prelude::*};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(".logs", 1000).await?;
    info!("Logger initialized!");
    let log_path = Logger::path().unwrap();

    info!("Tracing file: {}", log_path.display());
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // start log file tracing:
    let trace_handle = tokio::spawn(async move {
        let trace = Trace::open(log_path, Duration::from_millis(50), false)
            .await
            .expect("Failed to open trace");

        // fast check (without blocking thread):
        for _ in 0..5 {
            if let Some(lines) = trace.check().await {
                for line in lines {
                    println!("Traced line: {line}");
                }

                sleep(Duration::from_millis(120)).await;
            }
        }

        // read next lines (with blocking thread):
        let mut count = 0;
        while count < 5 {
            if let Some(lines) = trace.next().await {
                for line in lines {
                    println!("Traced line: {line}");
                }
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
