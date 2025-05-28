use anyhow::Result;

use crate::ai::generate_commit_message;
use crate::prompt;
use crate::timer::Timer;
use crate::git::{collect_diff, commit, run_pre_commit_hook};

pub async fn run() -> Result<()> {
    let timer = Timer::start();
    let diff = collect_diff()?;

    if diff.is_empty() {
        eprintln!("No staged changes detected.");
        eprintln!("Quick-Commit execution time: {:.3?}", timer.elapsed());
        return Ok(());
    }
    eprintln!("Quick-Commit execution time: {:.3?}", timer.elapsed());

    run_pre_commit_hook()?;
    eprintln!("Quick-Commit execution time: {:.3?}", timer.elapsed());
    let prompt = prompt::build(&diff);
    let ai_review = generate_commit_message(&prompt).await?;
    eprintln!("Quick-Commit execution time: {:.3?}", timer.elapsed());
    println!("{}", ai_review);
    commit(&ai_review)?;

    eprintln!("Quick-Commit execution time: {:.3?}", timer.elapsed());
    Ok(())
}


