use std::env;

use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

pub async fn generate_commit_message(prompt: &str) -> Result<String> {
    let messages = vec![json!({"role": "user", "content": prompt})];

    let api_key = env::var("COHERE_API_KEY")?;
    let client = Client::new();

    let response: Value = client
        .post("https://api.cohere.com/v2/chat")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "command-light",
            "messages": messages,
        }))
        .send()
        .await?
        .json()
        .await?;

    // 4. レスポンスから候補をパース（ここでは choices[].message.content が \n 区切りと仮定）
    let text = response["message"]["content"][0]["text"]
        .as_str()
        .unwrap_or_default();
    let text = text
        .replace("```markdown", "")
        .replace("```", "")
        .trim()
        .to_string();
    Ok(text)
}
