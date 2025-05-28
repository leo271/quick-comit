const TEMPLATE: &str = "You are an AI assistant specialized in generating precise commit messages in Conventional Commits format.\n\
\n\
First line must be the summary in format:<type>(<scope>): <short description>\n\
- Types: feat, fix, docs, style, refactor, perf, test, chore, build, ci\n\
- Scope: filename or module (e.g., \"main.rs\")\n\
- Short description: imperative mood, ≤50 chars, no ending period\n\
\n\
After a blank line, include a detailed body:\n\
- Explain what changed and why\n\
- List key file paths and functions modified\n\
- Wrap lines at 72 characters\n\
\n\
Staged diff:\n\
{diff}\n";

pub fn build(diff: &str) -> String {
    TEMPLATE.replace("{diff}", diff)
}
