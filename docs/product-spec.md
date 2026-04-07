# harn — Product Specification

## 1. Motivation

AI coding agents (Cursor, Codex, Claude Code, etc.) are powerful but context-dependent. Their output quality is bounded not by the model, but by the **environment** they operate in. Two independent engineering teams (OpenAI, Anthropic) arrived at the same conclusion: the discipline of structuring that environment — "harness engineering" — is what separates mediocre agent output from production-grade results.

The core elements of a good harness are:

1. **Discoverable knowledge** — Agents need a map, not a manual. A short entry point (`AGENTS.md`) pointing to structured docs.
2. **Architectural constraints** — Mechanical enforcement of invariants (dependency direction, naming, boundaries) that prevent drift.
3. **Quality criteria** — Explicit, gradable definitions of "good" that agents can target and evaluators can measure.
4. **Workflow artifacts** — Execution plans, sprint contracts, handoff documents that decompose complex work into tractable units.
5. **Entropy management** — Ongoing detection and correction of codebase drift.

Setting this up manually for each project is tedious and inconsistent. `harn` automates the bootstrapping and ongoing maintenance of this structure.

## 2. What harn Is

`harn` is a **harness lifecycle tool**. It bootstraps the harness structure (`init`), then provides ongoing commands for managing the development workflow (plans, sprints, validation, maintenance).

`harn` is **not** a scaffolding-only tool. If it were, a GitHub template repo would be a shorter path. The lifecycle commands (`plan`, `sprint`, `check`, `gc`) are what justify a dedicated CLI.

`harn` is **not** an agent orchestrator. It does not run planner/generator/evaluator loops, launch coding agents, or interface with AI tool APIs. Agent orchestration is the domain of the AI coding tool itself. `harn` provides the structured environment those tools operate in.

## 3. Scope

### In scope

- Project harness structure scaffolding (`init`)
- Execution plan lifecycle (create, track, complete)
- Sprint contract lifecycle (create, negotiate, close)
- Harness integrity validation (structural checks, freshness detection)
- Quality score tracking
- Stale documentation detection with git history analysis (`gc`)
- Harness maturity assessment against HARNESS-SPEC levels (`assess`)

### Out of scope (now, with extensibility preserved)

- Agent orchestration (no planner → generator → evaluator loop)
- Evaluation invocation (e.g., prompting the user's AI tool to run eval) — reserved for future extension
- Running code or tests
- Interfacing with specific AI tool APIs
- Git operations beyond history reading (for `gc`)
- i18n / multi-language template generation — may be added later

## 4. Target Users

**Primary**: Individual developers who use AI coding agents as their primary development method and want consistent, effective project environments.

**Assumptions**:
- Uses one or more AI coding tools (primarily Claude Code and/or Codex)
- Starts new projects frequently enough that manual setup is painful
- Wants projects to be agent-friendly from day one
- Works solo (team features like profiles/presets are out of scope for now)

## 5. Supported AI Tools

Initial release supports two AI coding tools:

| Tool | Entry File | Location | Notes |
|------|-----------|----------|-------|
| Codex | `AGENTS.md` | Project root | Also serves as universal harness map |
| Claude Code | `CLAUDE.md` | Project root | Thin wrapper referencing AGENTS.md |

Both entry files point to the same underlying `docs/` knowledge structure. `CLAUDE.md` is a thin wrapper to avoid duplication; all shared knowledge lives in `AGENTS.md`.

Tools that follow the `.agents` community convention (e.g., Cursor) already discover `AGENTS.md` without a dedicated entry file. Adding more tools (Aider, Windsurf) later is straightforward — each is essentially a new entry file template.

## 6. Configuration

Configuration lives at `.agents/harn/config.toml`, aligning with the community `.agents/` directory convention.

```toml
[project]
name = "my-project"
created = "2026-04-03"
harn_version = "0.1.0"

[tools]
agents = ["claude-code", "codex"]

[init]
stack = "rust"

[check]
required_files = [
  "AGENTS.md",
  "ARCHITECTURE.md",
  "docs/evaluation/criteria.md",
]

[gc]
stale_threshold_days = 14
ignore_paths = []
```

### Why `.agents/harn/` instead of `.harn/`

- `.agents/` is becoming a community convention for agent-related configs.
- Keeping harn's config in `.agents/harn/` avoids polluting the project root with another dotdir.
- Other agent tools can use sibling directories (`.agents/other-tool/`) without collision.
- The `.agents/` directory should be committed to git (it's project config, not ephemeral state).

### What lives in `.agents/harn/` vs. `docs/`

| Location | Contents | Why |
|----------|----------|-----|
| `.agents/harn/` | harn's operational config and state (config.toml, current-sprint.toml) | harn's own concerns, not agent-facing knowledge |
| `docs/` | Knowledge layer (specs, plans, criteria, templates) | Agent-facing; must be discoverable and readable by any AI tool |
| Project root | Entry files (AGENTS.md, CLAUDE.md, ARCHITECTURE.md) | Maximum discoverability — agents look at root first |

## 7. Init UX Design

### 7.1 Philosophy

**Detect as much as possible, ask as little as possible, allow overriding everything.**

The init flow has two tiers:
1. **Auto-detected** — git, package manager, existing AI tool configs → no question asked.
2. **Interactive override** — `--interactive` forces tool selection prompts even when AI tools are auto-detected. Stack and stale threshold are set via `--stack` and config, not interactively.

If all required information can be detected, `harn init` runs with zero prompts.

### 7.2 Detection Logic

```
Detect:
  ├── Git repo?                     → skip `git init` suggestion
  ├── Cargo.toml?                   → stack = "rust"
  ├── package.json?                 → stack = "node"
  ├── pyproject.toml / setup.py?    → stack = "python"
  ├── go.mod?                       → stack = "go"
  ├── CLAUDE.md exists?             → claude-code already configured
  ├── AGENTS.md exists?             → codex already configured
  ├── .agents/ exists?              → previous harn init or agent config
  └── docs/ exists?                 → partial harness already present
```

### 7.3 Example Flow

```
$ harn init

Detecting project environment...
  ✓ Git repository
  ✓ rust project
  ✗ No AI tool configs detected

Creating harness structure...
  ✓ AGENTS.md
  ✓ CLAUDE.md
  ✓ ARCHITECTURE.md
  ✓ docs/design-docs/index.md
  ✓ docs/design-docs/core-beliefs.md
  ✓ docs/evaluation/criteria.md
  ✓ docs/templates/exec-plan.md
  ✓ docs/templates/sprint-contract.md
  ✓ docs/templates/handoff.md
  ✓ .agents/harn/config.toml

Done! Created 10 files.

Next steps:
  1. Edit AGENTS.md — fill in project overview and key constraints
  2. Edit ARCHITECTURE.md — define domain structure and layer rules
  3. Review docs/evaluation/criteria.md — adjust quality criteria
  4. Run `harn check` to validate structural integrity
```

### 7.4 Non-Destructive Behavior

- If a file already exists, **skip it** (report as skipped).
- If `docs/` exists but is partial, **fill gaps** without touching existing files.
- `--force` overrides this and overwrites everything.
- `--dry-run` shows what would be generated without writing.

## 8. Lifecycle Commands Overview

| Command | Purpose | Phase |
|---------|---------|-------|
| `harn init` | Bootstrap harness structure | 1 |
| `harn check` | Validate structure integrity | 1 |
| `harn plan` | Manage execution plans | 2 |
| `harn sprint` | Manage sprint contracts | 2 |
| `harn status` | Show current project state at a glance | 2 |
| `harn gc` | Detect stale docs via git history | 2 |
| `harn score` | View/update quality grades | 3 |
| `harn upgrade` | Update harness when harn version changes | 3 |
| `harn assess` | Evaluate harness maturity against HARNESS-SPEC | 3 |

See [commands.md](commands.md) for detailed command reference.

## 9. Success Criteria

A project initialized with `harn` should:

1. Be immediately navigable by Claude Code and Codex agents.
2. Have explicit quality criteria the agent can target.
3. Have templates for structured workflows (plans, sprints, handoffs).
4. Pass `harn check` with zero warnings.
5. Feel like a natural starting point — not bureaucratic overhead. Every generated file must be worth reading.
6. Agents should know the development workflow (what to do, not just where to look) and be aware of available `harn` commands.

## 10. Non-Functional Requirements

- **Fast**: CLI should feel instant. No network calls for core operations.
- **Offline-first**: Everything works without internet.
- **Non-destructive**: Never overwrite without explicit confirmation.
- **Idempotent**: Running `harn init` twice is safe (fills gaps, skips existing).
- **Single binary**: No runtime dependencies. `cargo install harn` or download a binary.
- **Minimal footprint**: Generated files are useful, not voluminous.
