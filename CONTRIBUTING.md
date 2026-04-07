# Contributing to harn

[English](CONTRIBUTING.md) | [中文](CONTRIBUTING.zh-CN.md)

Thank you for your interest in contributing to harn! This document covers the essentials for both human and AI-agent contributors.

## For AI Agents

Read [AGENTS.md](AGENTS.md) first — it contains the project map, build commands, workflow, and constraints. Everything you need is there.

## For Human Contributors

### Getting Started

```bash
git clone https://github.com/imsuven/harn.git
cd harn
cargo build
cargo test
```

Requirements: Rust 1.70+ (stable toolchain).

### Development Workflow

1. Fork the repository and create a feature branch from `main`.
2. Make your changes. Follow the constraints in [AGENTS.md](AGENTS.md) — they apply to all contributors.
3. Run the full check suite before submitting:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

4. Commit with clear messages. Use [conventional commits](https://www.conventionalcommits.org/) when possible (e.g., `feat:`, `fix:`, `docs:`, `refactor:`, `test:`).
5. Open a pull request against `main`.

### Code Standards

- **Zero warnings.** `cargo clippy -- -D warnings` must pass. No exceptions.
- **Tests required.** New features need tests. Bug fixes need regression tests.
- **Newtypes over raw strings.** Use typed wrappers from `src/types.rs` for domain concepts.
- **Error messages include remediation.** Every user-facing error must say what happened AND what to do.
- **Architecture rules hold.** Dependencies flow downward. See [ARCHITECTURE.md](ARCHITECTURE.md).

### What to Work On

- Check [open issues](https://github.com/imsuven/harn/issues) for tasks labeled `good first issue` or `help wanted`.
- Template improvements are always welcome — compare `templates/` output against the hand-authored docs in this repo for quality benchmarks.
- Documentation improvements that reduce ambiguity or improve agent usability have high impact.

### Reporting Issues

Open an issue with:
- What you expected to happen
- What actually happened
- Steps to reproduce
- `harn` version (`harn --version`) and OS

### License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
