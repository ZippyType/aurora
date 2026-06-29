# Project Aurora — Agent Instructions

## Tools & Commands

- **Rust**: installed via rustup at `$HOME/.cargo/bin/rustc`
- **Build**: `./scripts/build.sh`
- **Deploy**: `./scripts/deploy.sh`
- **Debug**: `./scripts/debug.sh {logs|shell|restart}`
- **Format**: `cargo fmt` (Rust)
- **Lint**: `cargo clippy` (Rust)

## Project Structure

```
scripts/           # Build/deploy/debug toolchain scripts
dashboard/         # Rust backend (axum) + web frontend
services/          # Linux background services
packages/          # Package manager
widgets/           # FbInk widget framework
apps/              # Native Qt applications
nickel/            # Nickel reverse engineering & hooks
sdk/               # Plugin SDK
docs/              # Documentation
cross-toolchain/   # ARM cross-compiler (gitignored)
```

## Conventions

- Rust backend in `dashboard/` uses axum for HTTP
- All scripts in `scripts/` are POSIX sh + bash, shellcheck-valid
- Never use `tail` or `head` commands — use grep/rg instead
- Commit messages follow Conventional Commits
- Wrap markdown at 80 columns
