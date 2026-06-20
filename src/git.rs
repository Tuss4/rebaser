use std::process::{Command, ExitStatus};
use std::fmt;

#[derive(Debug)]
pub struct GitError {
    pub args: Vec<String>,
    pub status: ExitStatus,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`git {}` failed with {}",
            self.args.join(" "),
            self.status
        )
    }
}

impl std::error::Error for GitError {}

pub fn check_git_installed() -> Result<(), String> {
    Command::new("git")
        .arg("--version")
        .output()
        .map_err(|_| "git is not installed or not found in PATH".to_string())
        .and_then(|out| {
            if out.status.success() {
                Ok(())
            } else {
                Err("git --version failed".to_string())
            }
        })
}

pub fn run_git(args: &[&str]) -> Result<(), GitError> {
    let status = Command::new("git")
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn git: {e}"));

    if status.success() {
        Ok(())
    } else {
        Err(GitError {
            args: args.iter().map(|s| s.to_string()).collect(),
            status,
        })
    }
}

pub fn checkout(branch: &str) -> Result<(), GitError> {
    run_git(&["checkout", branch])
}

pub fn fetch_origin() -> Result<(), GitError> {
    run_git(&["fetch", "origin"])
}

pub fn reset_hard_origin(branch: &str) -> Result<(), GitError> {
    let refspec = format!("origin/{branch}");
    run_git(&["reset", "--hard", &refspec])
}

pub fn rebase(onto: &str) -> Result<(), GitError> {
    run_git(&["rebase", onto])
}
