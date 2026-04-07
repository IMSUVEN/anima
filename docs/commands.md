# harn — Command Reference

## Global Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--verbose` | `-v` | Show detailed output |
| `--quiet` | `-q` | Suppress non-essential output |
| `--no-color` | | Disable colored output |
| `--dir <path>` | `-C` | Operate on a different project directory (default: cwd) |

---

## `harn init`

Bootstrap harness structure for a new or existing project.

### Usage

```
harn init [OPTIONS]
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--name <name>` | `-n` | Project name | Directory name |
| `--tools <tools>` | `-t` | AI tools (comma-separated): `claude-code`, `codex` | Detected, or `codex,claude-code` |
| `--stack <stack>` | `-s` | Stack hint: `rust`, `node`, `python`, `go`, `generic` | Detected from package manager |
| `--interactive` | `-i` | Prompt for AI tools even if detectable | Off |
| `--minimal` | | Only generate essential core | Off |
| `--template-dir <path>` | | Use custom external templates | Built-in |
| `--force` | `-f` | Overwrite existing files without confirmation | Off |
| `--dry-run` | | Show what would be generated, don't write | Off |

### Flow

```
┌─────────────────────────────────────────┐
│  1. DETECT                              │
│  Git? Package manager? AI tool configs? │
│  Existing docs?                         │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  2. PROMPT (only unresolved questions)  │
│  AI tools? (multiselect, if not         │
│    detected and TTY or --interactive)    │
│  Non-TTY without -i: defaults to both   │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  3. RENDER                              │
│  Populate templates with context        │
│  Compute file hashes for gc tracking    │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  4. WRITE                               │
│  Create directories                     │
│  Write files (skip existing)            │
│  Write .agents/harn/config.toml         │
│  Report results and next steps          │
└─────────────────────────────────────────┘
```

### Generated File Tree

See [generated-content.md](generated-content.md) for the content of each file.

```
project-root/
├── AGENTS.md                           # Universal agent entry point
├── CLAUDE.md                           # Claude Code thin wrapper → AGENTS.md
├── ARCHITECTURE.md                     # Architecture skeleton
├── .agents/
│   └── harn/
│       └── config.toml                 # harn configuration
└── docs/
    ├── design-docs/
    │   ├── index.md                    # Design doc registry
    │   └── core-beliefs.md             # Golden principles (10 rules)
    ├── exec-plans/
    │   ├── active/                     # (empty dir, for plans and sprints)
    │   └── completed/                  # (empty dir, for archived plans)
    ├── product-specs/                  # (empty dir)
    ├── references/                     # (empty dir)
    ├── templates/
    │   ├── exec-plan.md
    │   ├── sprint-contract.md
    │   └── handoff.md
    └── evaluation/
        └── criteria.md                 # 5-dimension quality grading
```

10 files, 9 directories. Every file has substantive content.

### Example Output

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

---

## `harn check`

Validate harness structure integrity.

### Usage

```
harn check [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--fix` | Auto-fix simple issues (create missing dirs) | Off |
| `--ci` | Exit code 1 on warnings, 2 on errors (for CI pipelines) | Off |

### Checks

`check` is purely structural — it validates shape and integrity without requiring git history. Time-based staleness analysis belongs to `harn gc`.

| Category | Check | Severity |
|----------|-------|----------|
| Structure | Required files exist (AGENTS.md, ARCHITECTURE.md, criteria.md) | Error |
| Structure | Required directories exist (exec-plans/active/, exec-plans/completed/, templates/) | Error |
| Content | Required files have substantive content (not just headers) | Warning |
| Content | Init-generated files were customized (hash differs from init) | Warning |
| References | Cross-references in AGENTS.md resolve to existing files | Error |
| Config | `.agents/harn/config.toml` is valid and parseable (fail-fast: exits code 3 before report) | Error |
| Quality | AGENTS.md exceeds 150 lines (should be a concise map) | Warning |
| Quality | ARCHITECTURE.md missing dependency direction statement | Warning |
| Quality | `docs/QUALITY_SCORE.md` does not exist | Warning |

### Example

When all checks pass:

```
$ harn check

Harness integrity check: my-project

  ✓ AGENTS.md exists and has content
  ✓ ARCHITECTURE.md exists and has content
  ✓ docs/evaluation/criteria.md exists and has content

All checks passed.
```

When issues are found (only warnings/errors are shown):

```
$ harn check

Harness integrity check: my-project

  ⚠ ARCHITECTURE.md still matches init template (not customized)
  ✗ AGENTS.md references docs/references/api-spec.md which does not exist

Result: 1 error, 1 warning
Tip: run `harn gc` for time-based staleness analysis.
```

---

## `harn status`

Show current project state at a glance.

### Usage

```
harn status
```

### Output

Aggregates information from config, active plans, and current sprint into a single view. No options — designed to be the first command you run each day.

### Example

```
$ harn status

Project: my-project (Rust)
Harness: harn v0.1.0
Tools: codex, claude-code

Sprint: implement login page (2/5 acceptance criteria)
  └─ plan: user-auth-oauth2
Active plans: 2
  • user-auth-oauth2 (0/3 milestones)
  • api-v2-migration (2/5 milestones)
```

```
$ harn status

Project: my-project (Rust)
Harness: harn v0.1.0
Tools: codex, claude-code

Sprint: none active
Active plans: 0
```

---

## `harn plan`

Manage execution plans.

### Plan-Sprint Relationship

Plans and sprints have an **optional parent-child relationship**:

- A sprint may optionally link to a parent plan via `--plan <name>`.
- A plan can have zero or many sprints over its lifetime.
- A sprint can also exist independently (for small, standalone tasks).
- `harn plan list` shows linked sprint progress under each plan.

This keeps things simple for small tasks (standalone sprint, no plan) while enabling structured decomposition for complex work (plan with milestones, broken into sprints).

### Subcommands

#### `harn plan new <description> [--slug <slug>]`

Create a new execution plan from template in `docs/exec-plans/active/`.

**File naming**: `YYYY-MM-DD-<slug>.md`

Slug generation:
- If `--slug` is provided, use it directly.
- Otherwise, extract ASCII characters from the description and slugify.
- If no usable characters remain, generate `plan-NNN` (sequential).

#### `harn plan list`

List active and recently completed plans with milestone and sprint progress.

#### `harn plan complete <name>`

Move plan from `active/` to `completed/`. Fails if the plan has an active linked sprint (complete the sprint first).

### Example

```
$ harn plan new "user auth OAuth2 integration"

Created: docs/exec-plans/active/2026-04-03-user-auth-oauth2-integration.md

Edit this file to fill in:
  - Purpose and context
  - Scope (in/out)
  - Milestones with observable acceptance
  - Validation and acceptance criteria

$ harn plan new "payment processing" --slug payments

Created: docs/exec-plans/active/2026-04-03-payments.md

$ harn plan list

Active plans:
  1. 2026-04-03-user-auth-oauth2-integration (0/3 milestones)
     └─ sprint: implement-login-page (2/5 acceptance criteria)
  2. 2026-03-28-api-v2-migration (2/5 milestones)

Completed:
  3. 2026-03-25-initial-setup
```

---

## `harn sprint`

Manage sprint contracts.

### Subcommands

#### `harn sprint new <description> [--slug <slug>] [--plan <plan-name>]`

Create a new sprint contract. Writes two files:
- `.agents/harn/current-sprint.toml` — machine-readable sprint state (includes `plan` field if linked)
- `docs/exec-plans/active/sprint-YYYY-MM-DD-<slug>.md` — human/agent-readable contract

Only one sprint can be active at a time. If a sprint is already active, this command fails with guidance to complete or discard it first.

If `--plan` is provided, the sprint is linked to that plan. The plan name must match an active plan in `docs/exec-plans/active/`.

Slug generation follows the same rules as `harn plan new`.

#### `harn sprint status`

Show current sprint state: description, acceptance criteria, and their checked/unchecked status.

#### `harn sprint done`

Complete the current sprint.
- Archive the sprint contract to `docs/exec-plans/completed/`
- Remove `.agents/harn/current-sprint.toml`
- Optionally generate a handoff artifact in `docs/exec-plans/completed/` (alongside the archived sprint)

### Example

```
$ harn sprint new "implement login page" --plan user-auth-oauth2-integration

Created sprint contract:
  Contract: docs/exec-plans/active/sprint-2026-04-03-implement-login-page.md
  State: .agents/harn/current-sprint.toml
  Linked to plan: user-auth-oauth2-integration

Fill in deliverables and acceptance criteria before starting work.

$ harn sprint new "quick bugfix"

Created sprint contract:
  Contract: docs/exec-plans/active/sprint-2026-04-03-quick-bugfix.md
  State: .agents/harn/current-sprint.toml

Fill in deliverables and acceptance criteria before starting work.

$ harn sprint done

Sprint "quick bugfix" completed.
Generate handoff artifact for context reset? [y/N] y

Created: docs/exec-plans/completed/handoff-2026-04-03-quick-bugfix.md
Edit the handoff to record:
  - Completed work
  - Current state
  - Known issues
  - Next steps
```

---

## `harn score`

View and update quality grades.

### Subcommands

#### `harn score show`

Parse and display `docs/QUALITY_SCORE.md` in a formatted table. If the file doesn't exist yet, report that no assessments have been made.

#### `harn score update`

Interactive workflow: walk through each domain (from ARCHITECTURE.md), prompt for a grade. Creates `docs/QUALITY_SCORE.md` on first run.

---

## `harn gc`

Detect stale documentation using git history analysis. While `harn check` validates structural integrity (does it exist? is it valid?), `gc` answers: **is it still current?**

### Usage

```
harn gc [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--days <n>` | Staleness threshold in days | 14 (from config) |
| `--report` | Output report only, no suggestions | Off |
| `--json` | Output in JSON format (for tooling integration) | Off |
| `--ci` | Exit code 1 on warnings, 2 on errors (for CI pipelines) | Off |

### Analysis

1. **Age scan**: Flag docs not modified in >N days.
2. **Code-doc divergence**: For configured mappings, check if related code paths have newer commits than the doc.
3. **Template customization**: Check if init-generated files still match their original template hash.
4. **Reference integrity**: Verify cross-references still resolve.

### Example

When issues are found:

```
$ harn gc

Scanning documentation freshness...

  ℹ docs/product-specs/onboarding.md — not modified in 32 days
  ⚠ docs/design-docs/api-design.md — not modified since related code changed
    (14 commit(s) since last doc update)
  ⚠ ARCHITECTURE.md — still matches init template (not customized)

Found 3 potentially stale documents.
Consider reviewing with your AI coding tool, or updating manually.
```

When all documentation is current:

```
$ harn gc

Scanning documentation freshness...

  ✓ All documentation is current.
```

---

## `harn assess`

Evaluate harness maturity against HARNESS-SPEC levels. Runs automated checks across Level 1 (Single Agent) and Level 2 (Multi-Agent) requirements, producing a maturity score and actionable report.

### Usage

```
harn assess [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--json` | Output in JSON format (for CI integration) | Off |

### Checks

14 checks across two maturity levels:

| Level | Section | Check |
|-------|---------|-------|
| 1 | §1.1 Repository Knowledge | AGENTS.md at repository root |
| 1 | §1.1 Repository Knowledge | ARCHITECTURE.md with module boundaries |
| 1 | §1.1 Repository Knowledge | Structured knowledge in versioned docs/ |
| 1 | §1.2 Codebase Legibility | Language with build-time type checking |
| 1 | §1.4 Mechanical Enforcement | CI pipeline configuration |
| 1 | §1.4 Mechanical Enforcement | Linter configured with enforced rules |
| 1 | §1.5 Testing | Full test suite executable in ≤1 minute |
| 1 | §1.6 Dev Environment | Dev environment spinnable in one command |
| 1 | §1.7 Safety | No secrets committed to repository |
| 1 | §4.1 Entropy Management | Active entropy detection and correction |
| 2 | §2.2 Execution Plans | Self-contained ExecPlans for complex work |
| 2 | §2.3 Sprint Contracts | Negotiated sprint contracts |
| 2 | §2.5 Quality Criteria | Explicit grading criteria |
| 2 | §4.2 Knowledge Hygiene | Documentation-to-code mappings |

### Example

```
$ harn assess

Harness Maturity Assessment: my-project

Overall: Level 2 (Level 1: 94%, Level 2: 91%)

Level 1 — Single Agent + Loop (94%)

  ✓ [MUST] Repository Knowledge: AGENTS.md at repository root as agent entry point
  ✓ [SHOULD] Repository Knowledge: ARCHITECTURE.md with domain layering and module boundaries
  ✓ [MUST] Repository Knowledge: Structured knowledge in versioned docs/
  ✓ [MUST] Codebase Legibility: Language with build-time type checking
  ✓ [MUST] Mechanical Enforcement: CI pipeline enforcing lint/test on every change
  ✓ [SHOULD] Mechanical Enforcement: Linter configured with enforced rules
  ◐ [MUST] Testing: Full test suite executable in ≤1 minute
         Test directory found. Run your test suite to verify it completes in ≤60s.
  ✓ [MUST] Dev Environment: Dev environment spinnable in one command
  ✓ [SHOULD] Safety: No secrets committed to repository
  ✓ [MUST] Entropy Management: Active entropy detection and correction

Level 2 — Multi-Agent + Planning (91%)

  ✓ [MUST] Execution Plans: Self-contained ExecPlans for complex work
  ◐ [SHOULD] Sprint Contracts: Negotiated sprint contracts with testable acceptance criteria
         Sprint template exists but no sprint active. Use `harn sprint new`.
  ✓ [MUST] Quality Criteria: Explicit grading criteria shared with generators and evaluators
  ✓ [SHOULD] Knowledge Hygiene: Documentation-to-code mappings for drift detection

All MUST requirements met. Focus on SHOULD items to strengthen the harness.

$ harn assess --json
{"project":"my-project","level":2,"level1_pct":94,"level2_pct":91,"checks":[...]}
```

---

## `harn upgrade`

Update harness structure when harn version changes. Compares embedded templates against the files generated at init time using stored content hashes to determine what's changed.

### Usage

```
harn upgrade [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--dry-run` | Show what would change, don't write | Off |
| `--template-dir <path>` | Use custom external templates instead of built-in | Built-in |

### Behavior

- **Unchanged from template** (hash matches `[init.file_hashes]`): safe to overwrite with new version.
- **Customized** (hash differs): write new content as `.harn-upgrade` sidecar next to the existing file for manual merge.
- **No hash record**: treated as customized — sidecar only, never overwrite.
- **New template paths** in newer harn version: created directly (no sidecar needed).
- Config `harn_version` is updated after a successful upgrade.

### Example

```
$ harn upgrade

  ✓ docs/evaluation/criteria.md (already up to date)
  updated docs/templates/exec-plan.md (updated from template)
  sidecar AGENTS.md (modified — sidecar created)

Upgrade complete. Updated 1, created 0, sidecar 1 file(s).

Review .harn-upgrade files and merge changes manually or with your AI tool.
```

---

## Exit Codes

| Code | Meaning | Used by |
|------|---------|---------|
| 0 | Success / no issues | All commands |
| 1 | Warnings found (with `--ci`) | `harn check`, `harn gc` |
| 2 | Errors found | `harn check`, `harn gc` |
| 3 | Configuration error (missing config, invalid TOML) | `harn check`, `harn gc`, `harn status`, `harn upgrade` |

---

## Command Phase Map

| Command | Phase 1 (MVP) | Phase 2 | Phase 3 |
|---------|:---:|:---:|:---:|
| `harn init` | ✓ | | |
| `harn check` | ✓ | | |
| `harn plan` | | ✓ | |
| `harn sprint` | | ✓ | |
| `harn status` | | ✓ | |
| `harn gc` | | ✓ | |
| `harn score` | | | ✓ |
| `harn upgrade` | | | ✓ |
| `harn assess` | | | ✓ |
