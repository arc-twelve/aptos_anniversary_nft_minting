use eyre::{Context, Result};
use rand::Rng;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::instrument;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use crate::constants::TIME_SLEEP;

#[instrument]
pub async fn get_account_list() -> Result<Vec<String>> {
    let file = File::open("data/wallets.txt")
        .await
        .context("Error reading wallets.txt file")?;

    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    let mut lines_iter = reader.lines();

    while let Some(line) = lines_iter.next_line().await? {
        lines.push(line);
    }

    if lines.is_empty() {
        return Err(eyre::eyre!("No wallets in the file"));
    }

    Ok(lines)
}

#[instrument]
pub fn get_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 3)
}

#[instrument]
pub fn set_logger() {
    // Create a file appender for logging
    let file_appender = tracing_appender::rolling::daily("logs", "app.json");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Create a subscriber that logs in JSON format to file
    let file_layer = fmt::layer()
        .json() // Use JSON formatter for file output
        .with_writer(non_blocking)
        .with_target(false); // Optional: hide target info

    // Create a subscriber that logs in pretty format to terminal
    let terminal_layer = fmt::layer()
        //.pretty() // Use pretty formatter for terminal output
        .with_writer(std::io::stdout) // Log to terminal
        .with_target(false); // Optional: hide target info

    let subscriber = Registry::default()
        .with(EnvFilter::new("info")) // Set log level filter
        .with(file_layer) // Add JSON logging to file
        .with(terminal_layer); // Add pretty logging to terminal

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    // Drop the guard to flush logs before exiting
    std::mem::forget(guard);
}

#[instrument]
pub async fn sleep() {
    let mut rng = rand::thread_rng();
    let sec = rng.gen_range(TIME_SLEEP[0], TIME_SLEEP[0]);
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
}
