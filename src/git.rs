use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct GitError {
    pub args: Vec<String>,
    pub status: std::process::ExitStatus,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`git {}` failed with {}", self.args.join(" "), self.status)
    }
}

impl std::error::Error for GitError {}

pub trait GitOps {
    fn checkout(&self, branch: &str) -> Result<(), GitError>;
    fn fetch_origin(&self) -> Result<(), GitError>;
    fn reset_hard_origin(&self, branch: &str) -> Result<(), GitError>;
    fn rebase(&self, onto: &str) -> Result<(), GitError>;
}

pub struct RealGit {
    dir: PathBuf,
}

impl RealGit {
    pub fn new() -> Self {
        Self {
            dir: std::env::current_dir().expect("failed to get current dir"),
        }
    }

    pub fn with_dir(dir: PathBuf) -> Self {
        Self { dir }
    }
}

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

fn run_git(args: &[&str], dir: &Path) -> Result<(), GitError> {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
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

impl GitOps for RealGit {
    fn checkout(&self, branch: &str) -> Result<(), GitError> {
        run_git(&["checkout", branch], &self.dir)
    }

    fn fetch_origin(&self) -> Result<(), GitError> {
        run_git(&["fetch", "origin"], &self.dir)
    }

    fn reset_hard_origin(&self, branch: &str) -> Result<(), GitError> {
        let refspec = format!("origin/{branch}");
        run_git(&["reset", "--hard", &refspec], &self.dir)
    }

    fn rebase(&self, onto: &str) -> Result<(), GitError> {
        run_git(&["rebase", onto], &self.dir)
    }
}

#[cfg(test)]
pub fn failing_error() -> GitError {
    run_git(&["notavalidsubcommand"], &std::env::temp_dir()).unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_error_display() {
        let err = failing_error();
        let s = err.to_string();
        assert!(s.contains("`git notavalidsubcommand` failed with"));
    }

    #[test]
    fn git_error_debug() {
        let err = failing_error();
        assert!(format!("{err:?}").contains("GitError"));
    }

    #[test]
    fn git_error_is_std_error() {
        let err = failing_error();
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn check_git_installed_succeeds() {
        assert!(check_git_installed().is_ok());
    }

    #[test]
    fn run_git_success() {
        assert!(run_git(&["--version"], &std::env::temp_dir()).is_ok());
    }

    #[test]
    fn run_git_failure_captures_args() {
        let err = run_git(&["notavalidsubcommand"], &std::env::temp_dir()).unwrap_err();
        assert_eq!(err.args, vec!["notavalidsubcommand"]);
    }

    #[test]
    fn real_git_new_uses_current_dir() {
        let git = RealGit::new();
        assert!(git.dir.exists());
    }

    #[test]
    fn real_git_ops_fail_outside_repo() {
        let tmp = std::env::temp_dir();
        let git = RealGit::with_dir(tmp);
        assert!(git.checkout("main").is_err());
        assert!(git.fetch_origin().is_err());
        assert!(git.reset_hard_origin("main").is_err());
        assert!(git.rebase("main").is_err());
    }
}
