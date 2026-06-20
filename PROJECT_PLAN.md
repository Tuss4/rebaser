# rebaser — Project Plan

## What it does

`rebaser` is a Rust CLI tool that syncs a source branch with its remote and rebases a target branch onto it, eliminating the manual multi-step git workflow.

## CLI

```
rebaser <source> <target>
```

- `source` — branch to sync first (e.g. `main`)
- `target` — branch to rebase onto source (e.g. `feature/my-feature`)

## Prerequisites

On startup, the tool checks that `git` is installed and available in `PATH`. If not, it exits immediately with an error.

## Execution Flow

1. `git checkout <source>`
2. `git fetch origin`
3. `git reset --hard origin/<source>`
4. `git checkout <target>`
5. `git rebase <source>`

All git output is streamed to the terminal. If rebase exits non-zero (conflicts), the tool exits 1, prints a message with next steps, and leaves the rebase state intact for the user to resolve manually.

## Architecture

```
src/
  main.rs   — arg parsing (clap), calls run()
  git.rs    — run_git() helper + one function per git step
```

`run_git()` takes a `&[&str]` of args, runs the command with inherited stdio, and returns an error with the command and exit code on failure.

## Dependencies

- `clap` (derive feature) — CLI argument parsing
- `std::process::Command` — shell out to git (no git library needed)

## Error Handling

- Each step is checked; on failure, print which step failed and exit 1
- Rebase conflicts: exit 1 with a clear message pointing the user to `git rebase --continue`
