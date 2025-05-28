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

    // Get staged file statuses (A=added, M=modified, D=deleted)
    let files_status_output = Command::new("git")
        .args(&["diff", "--cached", "--name-status"])
        .output()?;
    let files_status = String::from_utf8(files_status_output.stdout)?
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut diff = String::new();
    for entry in files_status {
        let mut parts = entry.split('\t');
        let status = parts.next().unwrap();
        let file = parts.next().unwrap();
        diff.push_str(&format!("=== {} ===\n", file));
        // For deletions, show a simple notice and skip patch content
        if status == "D" {
            diff.push_str(&format!("deleted file: {}\n", file));
            continue;
        }
        // Existing diff logic for added/modified files
        let file_diff_output = Command::new("git")
            .args(&["diff", "--cached", &file])
            .output()?;
        let file_diff = String::from_utf8(file_diff_output.stdout)?;
        for line in file_diff.lines().take(100) {
            diff.push_str(line);
            diff.push('\n');
        }
    }

    // Exit early if no staged changes
    if diff.trim().is_empty() {
        eprintln!("No staged changes detected.");
        return Ok(());
    }

    if Command::new("git")
        .args(&["hook", "run", "pre-commit", "--ignore-missing"])
        .status()?
        .success()
    {
        eprintln!("pre-commit hook OK");
    } else {
        eprintln!("pre-commit hook failed — aborting");
        std::process::exit(1);
    }

    // AIプロンプト組み立て: main.rs を含むステージ済み変更の diff を渡す
    let user_message = format!(
        "You are an assistant that generates “exact” commit messages following Conventional Commits.\n\
        Input: Git diff of all staged changes, including changes in src/main.rs.\n\n
        Output: Commit message in Conventional Commits format:\n\n\np
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

    let api_key = env::var("COHERE_API_KEY")?;
    let client = Client::new();

    let response: Value = client
        .post("https://api.cohere.com/v2/chat")
        .bearer_auth(api_key)
        .header("X-Client-Name", "quick-commit")
        .header("Content-Type", "application/json")
        .json(&json!({
            "stream": false,
            "model": "command-a-03-2025",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        { "type": "text", "text": user_message }
                    ]
                }
            ]
        }))
        .send()
        .await?
        .json()
        .await?;


    // 4. レスポンスから候補をパース（ここでは choices[].message.content が \n 区切りと仮定）
    let text = response["message"]["content"][0]["text"].as_str().unwrap_or_default();
    let text = text.replace("```markdown", "").replace("```", "").trim().to_string();
    
    // 6. 選択したメッセージを出力（ここを `git commit -m` に流すなども可）
    println!("{}", text);

    // 7. 実際にgit commitを実行
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&text)
        .arg("--no-verify")
        .output()?;
    // 実行結果を表示（必要に応じて）
        // git は多くの場合標準エラーに結果を出すので両方表示する
    eprintln!(
        "out: {}{}",
        String::from_utf8_lossy(&commit_output.stdout),
        String::from_utf8_lossy(&commit_output.stderr)
    );


    // 実行時間を表示
    let elapsed = start.elapsed();
    eprintln!("Cohere-Commit execution time: {:.3?}", elapsed);

    Ok(())
}
