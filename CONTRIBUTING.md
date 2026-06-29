# Contributing to Project Aurora

## Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development Workflow

- All code goes through PR review
- Tests must pass before merging
- Follow the coding style of the surrounding code
- Keep commits atomic and well-described

## Code Style

- **Rust:** `cargo fmt` + `cargo clippy`
- **Shell:** Use `shellcheck` — no bashisms in POSIX scripts
- **Markdown:** Wrap at 80 columns, one sentence per line

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add browser terminal
fix: correct battery percentage calculation
docs: update README with quick start
```

## Adding a New Package

See `packages/CONTRIBUTING.md` for the package format spec.
