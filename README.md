# rebaser

A CLI tool that syncs a source branch with its remote and rebases a target branch onto it — replacing a repetitive multi-step git workflow with a single command.

```
rebaser <source> <target>
```

**Requires:** `git` installed and available in `PATH`.

## What it does

1. Checks out `<source>` and resets it to `origin/<source>`
2. Checks out `<target>` and runs `git rebase <source>`

If there are conflicts, the tool exits and leaves the rebase state intact so you can resolve them and run `git rebase --continue`.

## Build

```bash
# debug build
cargo build

# optimized release binary (output: target/release/rebaser)
cargo build --release
```

To install the binary to your Cargo bin directory:

```bash
cargo install --path .
```
