# Contributing to WOPR TUI 2026

Thanks for your interest in contributing! This project welcomes all kinds of contributions — bug fixes, new features, documentation, and ideas.

## Getting Started

1. Fork the repo and clone your fork
2. Install Rust (1.85+ required for edition 2024): https://rustup.rs
3. Build and run:
   ```bash
   cargo run
   ```
4. Make your changes on a feature branch

## Development

```bash
cargo build          # Debug build
cargo run            # Run the TUI
cargo test           # Run tests
cargo clippy         # Lint
cargo fmt            # Format
```

## Pull Requests

- Keep PRs focused — one feature or fix per PR
- Add a clear description of what changed and why
- Make sure `cargo build` and `cargo clippy` pass
- Follow the existing code style (run `cargo fmt`)

## What to Work On

Check the [issues](https://github.com/ankurCES/WOPR_TUI_2026/issues) for open tasks. Good first contributions:

- New stub scenarios (see `src/llm/stub.rs`)
- UI improvements and new widgets
- Additional LLM provider integrations
- Documentation and examples
- Bug reports with reproduction steps

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold it.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
