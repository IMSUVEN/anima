# harn — Technical Design

## 1. Why Rust CLI (Not Alternatives)

| Alternative | Verdict |
|-------------|---------|
| GitHub template repo | Only covers `init`. No lifecycle commands. Can't adapt to project context. |
| Shell script | Poor cross-platform support. Fragile for structured logic. |
| Python / Node CLI | Adds runtime dependency. Slower startup. Distribution complexity. |
| **Rust CLI** | Single binary, fast startup (<10ms), strong CLI ecosystem, grows with lifecycle scope. |

Rust is justified because `harn` is a lifecycle tool, not a one-shot scaffolder.

## 2. Architecture

### 2.1 Crate Structure

```
harn/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # clap argument parsing, subcommand dispatch
│   ├── config.rs            # .agents/harn/config.toml read/write
│   ├── types.rs             # Newtypes: Slug, ProjectName, HarnDate, etc.
│   ├── detect.rs            # Project environment detection
│   ├── init/
│   │   ├── mod.rs           # Init orchestration
│   │   └── render.rs        # Template rendering (minijinja + include_dir!)
│   ├── plan.rs              # Execution plan management
│   ├── sprint.rs            # Sprint contract management
│   ├── check.rs             # Structural validation
│   ├── status.rs            # Project state aggregation
│   ├── score.rs             # Quality score management
│   ├── gc.rs                # Staleness detection (git2)
│   ├── upgrade.rs           # Hash-based template upgrade
│   ├── util.rs              # Shared utilities (sha256_hex, extract_md_links)
│   ├── assess/
│   │   ├── mod.rs           # Assessment entry point and orchestration
│   │   ├── checks.rs        # Individual HARNESS-SPEC check implementations
│   │   └── report.rs        # Report formatting (text and JSON)
├── templates/               # Embedded template files
│   ├── AGENTS.md.j2
│   ├── CLAUDE.md.j2
│   ├── ARCHITECTURE.md.j2
│   └── docs/
│       ├── design-docs/     # index.md.j2, core-beliefs.md.j2
│       ├── evaluation/      # criteria.md.j2
│       └── templates/       # exec-plan.md, sprint-contract.md, handoff.md
├── Cargo.toml
└── README.md
```

### 2.2 Data Flow

```
User invokes `harn init`
  │
  ├─→ detect.rs: scan project root for signals
  │     (git, package managers, existing AI tool configs)
  │
  ├─→ mod.rs: resolve what can't be detected
  │     (AI tools via CLI args; --interactive overrides detection;
  │      fallback to interactive multiselect on TTY, or defaults)
  │
  ├─→ render.rs: populate templates with context
  │     (project name, description, date, stack, selected tools)
  │     filter_by_tools: omit CLAUDE.md if claude-code not selected
  │     filter_minimal: keep only essentials if --minimal
  │
  ├─→ init_directories: create docs/ tree (including docs/references/)
  │
  ├─→ write files: skip existing, create missing; compute SHA-256 hashes
  │
  └─→ config.rs: write .agents/harn/config.toml (with file_hashes)
```

### 2.3 Config Path: `.agents/harn/`

All harn operational data lives under `.agents/harn/`:

```
.agents/
└── harn/
    ├── config.toml            # Project configuration (committed to git)
    └── current-sprint.toml    # Active sprint state (committed to git)
```

This aligns with the community `.agents/` convention. The directory should be committed to git.

The knowledge layer lives separately, where agents naturally look:

```
project-root/
├── AGENTS.md                  # Universal entry point (map + workflow + tooling)
├── CLAUDE.md                  # Thin wrapper → AGENTS.md
├── ARCHITECTURE.md            # Architecture map
└── docs/                      # Full knowledge structure
```

## 3. Key Design Decisions

### 3.1 Template Engine: minijinja

**Decision**: Use `minijinja` over `tera`.

| | minijinja | tera |
|---|-----------|------|
| Binary size impact | ~100KB | ~500KB |
| Compile time | Faster | Slower |
| Features | Sufficient (variables, conditionals, loops) | More (macros, inheritance) |
| Jinja2 compatibility | High | High |

`harn` templates are relatively simple — variable substitution and conditional sections. minijinja is sufficient and keeps the binary small.

### 3.2 Template Embedding

**Decision**: Embed templates in the binary using `include_dir!`.

Templates are compiled into the binary. No external file dependencies for distribution.

**Escape hatch**: `harn init --template-dir <path>` allows using custom external templates.

### 3.3 Detection Priority

When signals conflict, use this priority:

1. Explicit CLI flags (`--tools`, `--stack`)
2. Existing file detection (CLAUDE.md exists → claude-code)
3. Hardcoded defaults

### 3.4 Deferred File Generation

Not all harness files are generated during `init`. Files that would be empty placeholders are created on demand by the corresponding command:

| File | Created by | Why deferred |
|------|-----------|-------------|
| `docs/QUALITY_SCORE.md` | `harn score update` | Empty quality table wastes agent context |
| `docs/exec-plans/tech-debt-tracker.md` | Created manually or by an agent | Empty debt table provides no information |
| `docs/product-specs/*.md` | User or agent | No spec content exists at init time |

AGENTS.md only links to files that exist. When a deferred file is first created, the creating command does NOT retroactively modify AGENTS.md — the file is simply discoverable by directory traversal or by future manual edits to AGENTS.md.

### 3.5 Plan-Sprint Relationship

Plans and sprints are loosely coupled by design:

```
Plan (optional parent)
 └── Sprint (optionally linked via --plan)
```

**State tracking**: When a sprint is linked to a plan, `current-sprint.toml` stores a `plan` field with the plan's slug. This enables `harn plan list` to show sprint progress under the parent plan and `harn plan complete` to block if a linked sprint is still active.

**Why optional**: Many tasks are small enough that a standalone sprint (no plan) is the right level of structure. Forcing plan creation for every sprint adds bureaucracy without value. The hierarchy is available when complexity warrants it.

### 3.6 Filename Slug Generation

Plan and sprint filenames use ASCII-only slugs for cross-platform safety.

**Strategy**:

1. If `--slug <slug>` is provided, use it directly.
2. Otherwise, extract ASCII-range characters from the description, lowercase, replace spaces/punctuation with hyphens, collapse consecutive hyphens.
3. If no usable characters remain, use a sequential fallback: `plan-001`, `plan-002`, etc.

**Examples**:
- `"implement login page"` → `implement-login-page`
- `"OAuth2 integration"` → `oauth2-integration`
- `"add feature" --slug auth-flow` → `auth-flow` (explicit slug)

Full filename format: `YYYY-MM-DD-<slug>.md`

### 3.7 `harn gc` — Staleness Detection Design

`gc` uses `git2` to analyze commit history and detect documentation that may be outdated.

#### Staleness Signals

| Signal | Severity | Logic |
|--------|----------|-------|
| Doc not modified in >N days | Info | Simple timestamp check (configurable, default 14 days) |
| Related code changed since doc was last modified | Warning | Map docs to code paths; if code path has newer commits than doc, flag it |
| Template never customized | Warning | Compare file content hash against the original template hash (stored in config at init time) |
| Broken cross-references | Error | Links in AGENTS.md point to non-existent files |

#### Code-to-Doc Mapping

This is the hardest part. A simple heuristic for v1:

- `docs/product-specs/auth.md` ↔ `src/auth/` (name-based matching)
- `ARCHITECTURE.md` ↔ any structural change (new top-level directories)
- `docs/evaluation/criteria.md` ↔ rarely changes, low staleness risk

The mapping is configurable in `.agents/harn/config.toml`:

```toml
[gc]
stale_threshold_days = 14
ignore_paths = ["docs/HARNESS-SPEC.md", "docs/HARNESS-GUIDE.md"]

[[gc.mappings]]
doc = "docs/product-specs/auth.md"
code = ["src/auth/", "src/middleware/auth.rs"]

[[gc.mappings]]
doc = "ARCHITECTURE.md"
code = ["src/"]
```

The `ignore_paths` field excludes files from gc analysis entirely (e.g., reference documents that are not expected to track code changes).

For v1, the name-based heuristic plus manual mappings should be sufficient. ML-based or semantic mapping is out of scope.

#### Template Hash Tracking

During `harn init`, store a hash of each generated file:

```toml
# .agents/harn/config.toml
[init.file_hashes]
"AGENTS.md" = "a1b2c3d4e5f6..."    # full SHA-256 hex
"docs/evaluation/criteria.md" = "f7g8h9..."
```

`harn gc` compares current file hashes against these. If unchanged, the file was never customized — a warning worth flagging.

#### GC Output

Each finding is a `GcFinding { path, severity, message }`. Severities: `"info"`, `"warning"`, `"error"`.

- Text output: each finding is printed with a severity icon (ℹ / ⚠ / ✗).
- JSON output (`--json`): a pretty-printed JSON array of finding objects.
- CI mode (`--ci`): exit code 1 if any warnings, exit code 2 if any errors.

### 3.8 `harn assess` — Harness Maturity Assessment

`assess` evaluates the project's harness maturity against HARNESS-SPEC levels. It is standalone — it does not depend on `config.rs` or `types.rs`.

#### Module Structure

```
assess/
├── mod.rs       # Entry point: run_assessments → print_report or print_json
├── checks.rs    # 14 individual check functions, one per HARNESS-SPEC requirement
└── report.rs    # Text and JSON report formatting
```

#### Data Types

```rust
struct Assessment {
    category: &'static str,     // e.g. "Repository Knowledge"
    requirement: &'static str,  // e.g. "AGENTS.md at repository root"
    level: u8,                  // 1 or 2
    obligation: Obligation,     // Must | Should
    status: Status,             // Pass | Partial | Fail
    detail: String,             // Human-readable explanation
}
```

#### Scoring

Each assessment has a weight: MUST = 2.0, SHOULD = 1.0. Pass = full weight, Partial = 50%, Fail = 0%. Scores are computed per level:

- Overall Level 2 if `l1_score >= 70% && l2_score >= 50%`
- Overall Level 1 if `l1_score >= 50%`
- Overall Level 0 otherwise

#### JSON Output

```json
{
  "project": "my-project",
  "level": 2,
  "level1_pct": 94,
  "level2_pct": 91,
  "checks": [
    {
      "category": "Repository Knowledge",
      "requirement": "AGENTS.md at repository root as agent entry point",
      "level": 1,
      "obligation": "must",
      "status": "pass",
      "detail": "AGENTS.md exists (61 lines)."
    }
  ]
}
```

#### Checks (14 total)

| Level | Spec Section | Check |
|-------|-------------|-------|
| 1 | §1.1 | AGENTS.md exists and is ≤150 lines |
| 1 | §1.1 | ARCHITECTURE.md with dependency/module info |
| 1 | §1.1 | Structured knowledge in `docs/` |
| 1 | §1.2 | Language with build-time type checking |
| 1 | §1.4 | CI pipeline configuration |
| 1 | §1.4 | Linter configured |
| 1 | §1.5 | Test suite infrastructure |
| 1 | §1.6 | One-command dev setup |
| 1 | §1.7 | No committed secrets |
| 2 | §2.2 | Active or completed ExecPlans |
| 2 | §2.3 | Sprint contract infrastructure |
| 2 | §2.5 | Evaluation criteria and quality scores |
| 1 | §4.1 | Entropy management (gc config) |
| 2 | §4.2 | Doc-code mappings configured |

`assess` always exits 0. It reports findings; it does not gate CI.

## 4. Dependencies

| Crate | Purpose | Phase |
|-------|---------|-------|
| `clap` (derive) | CLI argument parsing | 1 |
| `dialoguer` | Interactive prompts (select, confirm, input) | 1 |
| `console` | Terminal formatting (colors, styles) | 1 |
| `serde` + `toml` | Config serialization | 1 |
| `minijinja` | Template rendering | 1 |
| `include_dir` | Embed templates in binary | 1 |
| `chrono` | Date handling in templates and config | 1 |
| `walkdir` | Directory traversal for check | 1 |
| `sha2` | File hash computation for gc template tracking | 2 |
| `git2` | Git history analysis for gc | 2 |
| `anyhow` | Application error handling with context chains | 1 |
| `serde_json` | JSON output for gc and assess commands | 2 |

## 5. Phased Delivery

### Phase 1: Core (MVP)

- `harn init` — interactive scaffolding with environment detection
- `harn check` — structural validation (file existence, cross-refs, template customization)
- `.agents/harn/config.toml` management
- Embedded templates for full harness structure
- Entry point generation for Claude Code (`CLAUDE.md`) and Codex (`AGENTS.md`)

### Phase 2: Workflow & Maintenance

- `harn plan new|list|complete`
- `harn sprint new|status|done`
- `harn status` — project state overview
- Handoff artifact generation
- `harn gc` — staleness detection with git history

### Phase 3: Scoring & Upgrade

- `harn score show|update` — quality score management
- `harn upgrade` — update harness structure when harn version changes (with conflict handling for modified files)
- `harn assess` — harness maturity assessment (HARNESS-SPEC levels)

### Phase 4: Extension (future, not committed)

- Additional AI tool entry points (Aider, Windsurf, etc.)
- `harn eval` — lightweight evaluation invocation
- i18n / multi-language template support
- Plugin/preset system for shared configurations

## 6. Build & Distribution

- **Build**: Standard `cargo build --release`.
- **Cross-compilation**: Use `cross` or `cargo-zigbuild` for Linux/macOS/Windows targets.
- **Distribution**: `cargo install harn` + GitHub releases with prebuilt binaries.
- **CI**: GitHub Actions for testing, linting (`clippy`), and release builds.

## 7. Test Strategy

### Test Categories

| Category | Scope | Examples |
|----------|-------|---------|
| Unit | Pure functions, no I/O | Slug generation, config parsing/serialization, template variable extraction, detection logic (given these files exist, infer this stack) |
| Integration | Single command, real filesystem | `harn init` produces correct file tree, `harn check` validates/fails correctly, `harn plan new` creates expected file |
| End-to-end | Multi-command workflow | init → plan → sprint → done lifecycle; init → gc detects stale files |

### Isolation

All tests that touch the filesystem use `tempfile::TempDir`. No shared state between tests. No tests depend on the host machine's home directory, git config, or installed tools beyond `git` (for gc tests).

Integration and e2e tests create a fresh temp directory, optionally run `git init` inside it, then invoke the command under test. After assertion, the temp directory is dropped automatically.

### Speed Target

**≤ 60 seconds** for the full suite, matching the AGENTS.md and HARNESS-SPEC budgets. Achieved via:

- Parallel test execution (`cargo test` runs tests in parallel by default)
- No network calls in any test
- No sleep/delay-based tests — use filesystem state, not timing
- Temp directories are fast; no database fixtures or container setup

### Directory Structure

```
tests/
├── architecture.rs  # Structural tests for ARCHITECTURE.md dependency rules
├── init.rs          # Integration tests for harn init
├── check.rs         # Integration tests for harn check
├── plan.rs          # Integration tests for harn plan
├── sprint.rs        # Integration tests for harn sprint
├── status_gc.rs     # Integration tests for harn status + gc
├── score.rs         # Integration tests for harn score
├── upgrade.rs       # Integration tests for harn upgrade
├── e2e.rs           # End-to-end multi-command workflow tests
└── helpers/
    └── mod.rs       # Shared test utilities (TempProject, harn binary runner)
```

Unit tests live inline in `src/` modules via `#[cfg(test)] mod tests`.

### What Must Be Tested

| Component | Critical Test Cases |
|-----------|-------------------|
| `init` | Correct file tree generated; existing files skipped; `--force` overwrites; `--dry-run` produces no files; detection heuristics for each stack |
| `check` | Passes on valid harness; fails on missing required files; warns on uncustomized templates; `--fix` recreates missing dirs |
| `plan` | Creates file with correct slug and date; sequential fallback when no ASCII chars; `complete` blocks if linked sprint active |
| `sprint` | Only one active at a time; `--plan` links correctly; `done` archives to completed/ and generates handoff |
| `gc` | Flags stale docs; respects threshold config; detects uncustomized templates via hash comparison |
| `upgrade` | Unchanged files overwritten; customized files get sidecar; new template paths created; `--dry-run` writes nothing |
| `assess` | Assessed via e2e tests: on empty projects, on initialized projects; JSON output schema validation |
| `config` | Roundtrip serialization; missing fields get defaults |
| `slug` | ASCII extraction; consecutive hyphen collapse; explicit `--slug` override; sequential fallback |

## 8. Upgrade Strategy

### How `harn upgrade` works

When `harn` ships a new version with updated templates:

1. **Compare** each harness file's current hash against the `init.file_hashes` stored in `config.toml`.
2. **File unchanged** (hash matches original) → overwrite silently with the new template. The user never customized it, so there's nothing to lose.
3. **File modified** (hash differs) → generate `<filename>.harn-upgrade` alongside the existing file. The user (or their AI tool) can merge.
4. **No hash record** → treated as customized — sidecar only, never overwrite.
5. **New file** (exists in new template set but not in project) → create it. Report as added.

After upgrade, `config.toml` is updated with the new `harn_version` and refreshed `file_hashes` for any files that were overwritten.

**Escape hatch**: `harn upgrade --template-dir <path>` uses custom external templates instead of the built-in ones.

### Why not three-way merge

Three-way merge requires storing the original template content (not just its hash), adds significant complexity, and is error-prone for markdown files where structural changes don't merge cleanly. The `.harn-upgrade` sidecar approach is simple, non-destructive, and delegates the merge decision to the user — who has an AI tool that excels at this.

## 9. Error Handling

### Error Categories

| Category | Trigger | Exit Code | Recovery |
|----------|---------|-----------|----------|
| Config missing | No `config.toml` found | 3 | `harn init` or `harn init --force` |
| Config invalid | Malformed TOML, missing required fields | 3 | Fix manually or `harn init --force` to regenerate |
| State conflict | `harn sprint new` when sprint already active | 1 | `harn sprint done` first, or delete `current-sprint.toml` |
| File conflict | `harn init` when files already exist | 0 (skip) | Use `--force` to overwrite |
| Missing harness | `harn check`/`harn gc` in a non-harn project | 3 | `harn init` |
| Git unavailable | `harn gc` without a usable git repository | 0 | Git-based analysis is skipped; info-level findings note the skip. Initialize git to enable age and code-doc divergence checks. |

### Error Message Principles

1. **Say what happened.** "Sprint already active: implement-login-page"
2. **Say what to do.** "Run `harn sprint done` to complete it first, or delete `.agents/harn/current-sprint.toml`."
3. **Never fail silently.** Every error produces output. No exit code 0 on failure.

### Partial Failure in `harn init`

If `harn init` fails midway (e.g., permission error writing one file):
- Files already written are kept (not rolled back).
- Error is reported with the failing file path.
- User can fix the issue and re-run `harn init` (idempotent — skips existing files).

### Recovery Path: `harn check --fix`

`--fix` handles recoverable issues:
- Recreates missing required directories.
- Does NOT overwrite existing files.

## 10. Remaining Technical Questions

| ID | Question | Notes |
|----|----------|-------|
| T1 | ~~Should `harn upgrade` support `--dry-run`?~~ | Resolved: Yes, `--dry-run` is implemented. |
| T2 | How should `harn check --fix` handle files that were customized then deleted? | The hash exists in config but the file is gone. Probably regenerate from template and warn. |
