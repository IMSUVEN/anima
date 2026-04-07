# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-03

### Added

- `harn init` — scaffold a complete harness structure with stack detection (Rust, Node, Python, Go), AI tool detection (Claude Code, Codex), and Jinja2 template rendering. Supports `--force`, `--dry-run`, `--minimal`, and `--template-dir`.
- `harn check` — validate harness integrity: required files, directory structure, content substantiveness, template customization, cross-reference validation, AGENTS.md length, ARCHITECTURE.md dependency direction, and QUALITY_SCORE.md existence. Supports `--fix` and `--ci` modes.
- `harn status` — display aggregated project state including active sprint, plan progress, and configuration summary.
- `harn plan` — manage execution plans with `new`, `list`, and `complete` subcommands. Supports slug generation from descriptions and explicit `--slug`.
- `harn sprint` — manage sprint contracts with `new`, `status`, and `done` subcommands. Enforces one-active-at-a-time. Supports `--plan` linking and handoff generation.
- `harn gc` — detect stale documentation via git2 commit history analysis, age-based checks, and code-doc divergence detection. Supports `--json` output and `--ci`.
- `harn assess` — run harness health checks and produce a structured assessment report.
- `harn score` — display and interactively update quality grades with `show` and `update` subcommands.
- `harn upgrade` — hash-based template upgrade with sidecar strategy for user-modified files. Supports `--dry-run`.
- 9 embedded templates (AGENTS.md, CLAUDE.md, ARCHITECTURE.md, core-beliefs, criteria, design-docs index, exec-plan, sprint-contract, handoff) with stack-aware conditional rendering.
- 138 tests (46 unit, 85 integration, 7 architecture) running in <1 second.
- GitHub Actions CI pipeline (fmt, clippy, test) on Linux, macOS, and Windows.
