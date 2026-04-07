# harn

[English](README.md) | [中文](README.zh-CN.md)

A CLI tool that gives projects the structure and workflows for effective AI-agent-driven development — based on harness engineering principles from [OpenAI](https://openai.com/index/harness-engineering/) and [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps).

## Problem

AI coding agents are context-dependent. Their output quality is bounded by the **environment**, not the model. Setting up the right project structure — discoverable knowledge, architectural constraints, quality criteria, workflow artifacts — is tedious and inconsistent when done manually.

## What harn Does

`harn` is a harness lifecycle tool. It bootstraps and maintains the knowledge and workflow layer that makes agents effective. It does not orchestrate agents — that's your AI coding tool's job (Cursor, Codex, Claude Code, etc.). `harn` gives those tools a well-structured environment to operate in.

```
harn init          # scaffold harness structure
harn check         # validate structural integrity
harn status        # show current project state at a glance
harn plan          # manage execution plans
harn sprint        # manage sprint contracts
harn gc            # detect stale docs via git history
harn score         # view and update quality grades
harn upgrade       # update templates to latest harn version
```

## Supported AI Tools

- **Claude Code** — generates `CLAUDE.md` entry point
- **Codex** — generates `AGENTS.md` entry point

Both point to the same `docs/` knowledge structure.

## Key Properties

- **Single binary** — Rust, no runtime dependencies
- **Offline-first** — no network calls for core operations
- **Non-destructive** — never overwrites without confirmation
- **Tool-agnostic** — universal knowledge layer, tool-specific entry points

## Quick Start

```bash
cargo install harn
cd my-project
harn init
```

`harn init` detects your stack and AI tools, then scaffolds the complete harness structure. Customize the generated files, then use `harn check` to validate integrity.

## Configuration

Config lives at `.agents/harn/config.toml`, following the community `.agents/` convention.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines. For agents, see [AGENTS.md](AGENTS.md).

## License

MIT
