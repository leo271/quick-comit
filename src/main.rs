// src/main.rs
use anyhow::Result;
use quick_commit::cli;

#[tokio::main]
async fn main() -> Result<()> {
    // 計測開始
    cli::run().await
}
