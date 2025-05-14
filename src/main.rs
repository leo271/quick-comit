// src/main.rs
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::process::Command

#[tokio::main]
async fn main() -> Result<()> {
    let git_output = Command::new("git").args(&["diff", "--name-only", "--cached"])
    
    let api_key = env::var("COHERE_API_KEY")?;
    let client = Client::new();

    let response: Value = client
        .post("https://api.cohere.com/v2/chat")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "command-r7b-12-2024",
            "messages": [{"role":"user","content":"Generate commit message"}]
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("{}", response);
    Ok(())
}
