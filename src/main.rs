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

    let ops = git::RealGit::new();
    if let Err(e) = run(&ops, &args.source, &args.target) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run(ops: &impl git::GitOps, source: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("==> Checking out source branch '{source}'");
    ops.checkout(source)?;

    println!("==> Fetching origin");
    ops.fetch_origin()?;

    println!("==> Resetting '{source}' to origin/{source}");
    ops.reset_hard_origin(source)?;

    println!("==> Checking out target branch '{target}'");
    ops.checkout(target)?;

    println!("==> Rebasing '{target}' onto '{source}'");
    if let Err(e) = ops.rebase(source) {
        eprintln!("\nConflicts detected during rebase.");
        eprintln!("Resolve the conflicts, stage the files, then run:");
        eprintln!("  git rebase --continue");
        eprintln!("\n(or run `git rebase --abort` to cancel)");
        return Err(e.into());
    }

    println!("\nDone. '{target}' has been rebased onto '{source}'.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    struct MockGit {
        results: RefCell<VecDeque<Result<(), git::GitError>>>,
    }

    impl MockGit {
        fn new(results: Vec<Result<(), git::GitError>>) -> Self {
            Self {
                results: RefCell::new(results.into()),
            }
        }

        fn next(&self) -> Result<(), git::GitError> {
            self.results
                .borrow_mut()
                .pop_front()
                .expect("unexpected git call in mock")
        }
    }

    impl git::GitOps for MockGit {
        fn checkout(&self, _: &str) -> Result<(), git::GitError> {
            self.next()
        }
        fn fetch_origin(&self) -> Result<(), git::GitError> {
            self.next()
        }
        fn reset_hard_origin(&self, _: &str) -> Result<(), git::GitError> {
            self.next()
        }
        fn rebase(&self, _: &str) -> Result<(), git::GitError> {
            self.next()
        }
    }

    fn ok() -> Result<(), git::GitError> {
        Ok(())
    }

    fn err() -> Result<(), git::GitError> {
        Err(git::failing_error())
    }

    #[test]
    fn run_happy_path() {
        let mock = MockGit::new(vec![ok(), ok(), ok(), ok(), ok()]);
        assert!(run(&mock, "main", "feature").is_ok());
    }

    #[test]
    fn run_fails_on_checkout_source() {
        let mock = MockGit::new(vec![err()]);
        assert!(run(&mock, "main", "feature").is_err());
    }

    #[test]
    fn run_fails_on_fetch() {
        let mock = MockGit::new(vec![ok(), err()]);
        assert!(run(&mock, "main", "feature").is_err());
    }

    #[test]
    fn run_fails_on_reset() {
        let mock = MockGit::new(vec![ok(), ok(), err()]);
        assert!(run(&mock, "main", "feature").is_err());
    }

    #[test]
    fn run_fails_on_checkout_target() {
        let mock = MockGit::new(vec![ok(), ok(), ok(), err()]);
        assert!(run(&mock, "main", "feature").is_err());
    }

    #[test]
    fn run_fails_on_rebase_conflict() {
        let mock = MockGit::new(vec![ok(), ok(), ok(), ok(), err()]);
        let result = run(&mock, "main", "feature");
        assert!(result.is_err());
    }
}
