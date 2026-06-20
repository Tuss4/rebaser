# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`rebaser` is a Rust CLI tool that helps quickly rebase a branch against another.

## Commands

```bash
cargo build              # compile (debug)
cargo build --release    # compile (optimized)
cargo run -- <args>      # run the CLI
cargo test               # run all tests
cargo test <name>        # run a single test by name
cargo clippy             # lint
cargo fmt                # format
cargo fmt --check        # check formatting without modifying
```

## Architecture

Two source files:

- `src/main.rs` — arg parsing via `clap` derive, git prerequisite check, and the `run()` function that sequences all steps
- `src/git.rs` — `run_git(&[&str])` helper (spawns git with inherited stdio) and one function per operation: `checkout`, `fetch_origin`, `reset_hard_origin`, `rebase`

`run_git` returns a `GitError` (contains the args and exit status) on non-zero exit. The rebase step gets special handling in `main.rs`: on failure it prints conflict resolution instructions and exits 1, leaving the rebase state intact for the user to continue manually.
