mod git;

use clap::Parser;

#[derive(Parser)]
#[command(about = "Rebase a target branch onto an up-to-date source branch")]
struct Args {
    /// Branch to sync with remote before rebasing (e.g. main)
    source: String,
    /// Branch to rebase onto source (e.g. feature/my-feature)
    target: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = git::check_git_installed() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }

    if let Err(e) = run(&args.source, &args.target) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run(source: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("==> Checking out source branch '{source}'");
    git::checkout(source)?;

    println!("==> Fetching origin");
    git::fetch_origin()?;

    println!("==> Resetting '{source}' to origin/{source}");
    git::reset_hard_origin(source)?;

    println!("==> Checking out target branch '{target}'");
    git::checkout(target)?;

    println!("==> Rebasing '{target}' onto '{source}'");
    if let Err(e) = git::rebase(source) {
        eprintln!("\nConflicts detected during rebase.");
        eprintln!("Resolve the conflicts, stage the files, then run:");
        eprintln!("  git rebase --continue");
        eprintln!("\n(or run `git rebase --abort` to cancel)");
        return Err(e.into());
    }

    println!("\nDone. '{target}' has been rebased onto '{source}'.");
    Ok(())
}
