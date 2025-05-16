const TEMPLATE: &str = "You are an assistant that generates “exact” commit messages following Conventional Commits.\n\
Input: Git diff of all staged changes, including changes in src/main.rs.\n\n\
Output: Commit message in Conventional Commits format:\n\n\n\
    <type>(<scope>): <short description>\n\n\
    <detailed body>\n\n\
Rules:\n\
1. Types: feat, fix, docs, style, refactor, perf, test, chore, build, ci.\n\
2. Scope: file or module name (use \"main.rs\" for src/main.rs).\n\
3. Short description: imperative mood, ≤50 chars, no period.\n\
4. Body: Explain what and why, list affected files/functions, wrap at 72 chars.\n\
5. Must reference additions/deletions and key file paths.\n\n\
Staged diff:\n{diff}\n";

pub fn build(diff: &str) -> String {
    TEMPLATE.replace("{diff}", diff)
}
