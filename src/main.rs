// src/main.rs
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::process::Command;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    // 計測開始
    let start = Instant::now();

    // ステージされたファイルごとに diff を取得し行数を制限
    let files_output = Command::new("git")
        .args(&["diff", "--cached", "--name-only"])
        .output()?;
    let files = String::from_utf8(files_output.stdout)?
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut diff = String::new();
    for file in files {
        diff.push_str(&format!("=== {} ===\n", file));
        let file_diff_output = Command::new("git")
            .args(&["diff", "--cached", &file])
            .output()?;
        let file_diff = String::from_utf8(file_diff_output.stdout)?;
        for line in file_diff.lines().take(100) {
            diff.push_str(line);
            diff.push('\n');
        }
    }

    // AIプロンプト組み立て: main.rs を含むステージ済み変更の diff を渡す
    let user_message = format!(
        "You are an assistant that generates “exact” commit messages following Conventional Commits.\n\
        Input: Git diff of all staged changes, including changes in src/main.rs.\n\
        Output: Commit message in Conventional Commits format:\n\n\
          <type>(<scope>): <short description>\n\n\
          <detailed body>\n\n\
        Rules:\n\
        1. Types: feat, fix, docs, style, refactor, perf, test, chore, build, ci.\n\
        2. Scope: file or module name (use \"main.rs\" for src/main.rs).\n\
        3. Short description: imperative mood, ≤50 chars, no period.\n\
        4. Body: Explain what and why, list affected files/functions, wrap at 72 chars.\n\
        5. Must reference additions/deletions and key file paths.\n\n\
        Staged diff:\n{}\n",
        diff
    );

    println!("{}", user_message);

    let messages = vec![json!({"role": "user", "content": user_message})];

    let api_key = env::var("COHERE_API_KEY")?;
    let client = Client::new();

    let response: Value = client
        .post("https://api.cohere.com/v2/chat")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "command-r7b-12-2024",
            "messages": messages,
        }))
        .send()
        .await?
        .json()
        .await?;

    // 4. レスポンスから候補をパース（ここでは choices[].message.content が \n 区切りと仮定）
    let text = response["message"]["content"][0]["text"].as_str().unwrap_or_default();
    let text = text.replace("```markdown", "").trim().to_string();
    
    // 6. 選択したメッセージを出力（ここを `git commit -m` に流すなども可）
    println!("{}", text);

    // 7. 実際にgit commitを実行
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&text)
        .output()?;
    // 実行結果を表示（必要に応じて）
    eprintln!("{}", String::from_utf8_lossy(&commit_output.stdout));

    // 実行時間を表示
    let elapsed = start.elapsed();
    eprintln!("Cohere-Commit execution time: {:.3?}", elapsed);

    Ok(())
}
