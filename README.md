# Quick-Commit

AIでコミットを自動生成するツール

使い方としてはシンプルに
``` zsh
git add . # 変更をステージする
quick-commit # 差分を見てコミット文を生成→実際にコミット
git push origin # リモートに反映
```

僕はVSCodeの`tasks.json`に以下のようなタスクを記述して、`Ctrl+S`で呼び出せるようにしてあります。(個人的にはかなり便利)
``` json tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Quick Commit",
            "type": "shell",
            "command": "source ~/.zshrc && git add . && quick-commit && git push origin",
            "options": {
                "cwd": "${workspaceFolder}"
            },
            "problemMatcher": []
        }
    ]
}
```

# Parameters


# Architecture

cohere-commit/  
├── Cargo.toml  
└── src/  
    ├── main.rs        ← エントリポイント  
    ├── lib.rs         ← 各モジュールをまとめる  
    ├── cli.rs         ← CLI 周りの設定・引数パース  
    ├── git.rs         ← Git 操作（ステージ取得・diff 取得・フック・コミット）  
    ├── ai.rs          ← Cohere API クライアント  
    ├── prompt.rs      ← プロンプト生成ロジック  
    ├── timer.rs       ← 実行時間計測ユーティリティ  
    └── errors.rs      ← 共通エラー型・Result エイリアス  
