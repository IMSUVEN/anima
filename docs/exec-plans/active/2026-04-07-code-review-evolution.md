# ExecPlan: Code Review-Driven Evolution

Living document. Update Progress, Surprises, Decision Log, and Retrospective
as work proceeds. This plan MUST be self-contained — a reader with only this
file and the working tree must be able to succeed.

## Purpose

After a comprehensive code review of the entire harn codebase (src, tests, templates, docs), this plan addresses all findings in priority order — from critical bugs to strategic evolution — to bring harn to release-quality and beyond.

## Context and Orientation

harn is at v0.1.0, pre-release. All tests pass (98 at plan start, 113 at completion), clippy and fmt are clean. The codebase is well-structured but has several issues discovered during review:

- **Critical**: logic bugs (milestone counting), production panics (unwrap), architecture violations (process::exit in cli.rs, duplicate detect calls)
- **Medium**: broken cross-references (claude-only init, minimal mode), silent failures, missing anyhow context, DRY violations
- **Testing**: score command barely tested, uninitialized project paths untested, exit code assertions weak
- **Strategic**: HARNESS-SPEC.md is an untapped asset; multi-tool support would expand reach

Key files:
- `src/plan.rs` — milestone counting bug (lines ~216-234)
- `src/sprint.rs` — unwrap panic (line ~166)
- `src/cli.rs` — process::exit (line ~209), duplicate detect (line ~192)
- `src/check.rs`, `src/gc.rs`, `src/init/mod.rs`, `src/upgrade.rs` — duplicated sha256_hex
- `tests/score.rs` — minimal coverage

## Scope

### In Scope

- Fix all High and Medium severity code issues
- Extract shared utilities (sha256_hex, extract_md_links)
- Harden test suite for critical gaps
- Prepare for 0.1.0 quality bar

### Out of Scope

- `harn adopt` (gradual adoption for existing projects) — future plan
- Multi-tool support (`--tools cursor,aider,gemini`) — future plan
- CI/CD setup (GitHub Actions configuration)
- Performance optimization (suite already at ~0.4s)

## Milestones

### Milestone 1: Critical Fixes (Phase 1)

- **Scope**: Fix all High severity issues
- **Observable acceptance**: `cargo test && cargo clippy -- -D warnings && cargo fmt --check` passes; milestone counting is scoped correctly; no unwrap() in production code; no process::exit outside main.rs; detect called exactly once per init
- **Commands to verify**: `cargo test && cargo clippy -- -D warnings`

### Milestone 2: Robustness (Phase 2)

- **Scope**: Fix Medium severity issues — broken links, silent failures, weak error context, DRY violations
- **Observable acceptance**: claude-only init produces valid cross-refs; all IO errors have actionable context messages; sha256_hex and extract_md_links extracted to shared module
- **Commands to verify**: `cargo test && cargo clippy -- -D warnings`

### Milestone 3: Test Hardening (Phase 3)

- **Scope**: Fill testing gaps identified in review
- **Observable acceptance**: score update tested; uninitialized project behavior tested; check failures assert exit codes; slug collision covered
- **Commands to verify**: `cargo test`

### Milestone 4: Strategic Evolution (Phase 4)

- **Scope**: `harn assess` — evaluate harness maturity against HARNESS-SPEC levels
- **Observable acceptance**: `harn assess` runs 14 checks across Level 1-2, outputs maturity score; `harn assess --json` produces machine-readable output; `docs/commands.md` documents the command; `ARCHITECTURE.md` lists `assess.rs`; `config.toml` mappings include `assess.rs`
- **Commands to verify**: `cargo test && harn assess && harn check && harn gc`

## Progress

- [x] Phase 1: Critical Fixes — milestone counting logic, unwrap panic, process::exit, duplicate detect
- [x] Phase 2: Robustness — shared util.rs, anyhow context, silent failures, deduplicated code
- [x] Phase 3: Test Hardening — 15 new tests (98→113), uninitialized projects, exit codes, corrupt state
- [x] Phase 4: Strategic Evolution — `harn assess` command with 14 checks across HARNESS-SPEC Level 1-2

## Decision Log

- Decision: Fix critical bugs before adding features
  Rationale: Release quality bar requires zero known High-severity issues
  Date: 2026-04-07

- Decision: Extract shared utilities in Phase 2 (not Phase 1)
  Rationale: DRY cleanup is Medium severity; Critical fixes come first
  Date: 2026-04-07

- Decision: Bring `harn assess` into scope as Phase 4
  Rationale: HARNESS-SPEC.md is harn's unique differentiator; turning it into an automated diagnostic creates immediate user value and dogfoods the spec itself
  Date: 2026-04-07

- Decision: Single commit for all phases (not split per phase)
  Rationale: Changes are interleaved across shared files (cli.rs, main.rs); splitting would create intermediate commits that don't compile
  Date: 2026-04-07

## Outcomes & Retrospective

**Outcomes:**
- 6 High-severity bugs fixed (milestone counting, 3× unwrap panics, process::exit violation, redundant detect)
- 8 Medium-severity issues resolved (DRY violations, missing error context, silent failures, corrupt state handling)
- Test suite grew from 98 → 113 tests, covering previously untested error paths
- New `harn assess` command: 14 automated checks against HARNESS-SPEC Level 1-2
- `src/util.rs` established as shared utility module, reducing duplication across 4 files
- All documentation updated in lockstep: ARCHITECTURE.md, docs/commands.md, config.toml mappings

**What went well:**
- Parallel subagent review covered source, tests, templates, and external research simultaneously
- Phased approach (critical → robust → tests → strategic) ensured each step built on a green tree
- The `harn assess` command immediately dogfoods the project's own HARNESS-SPEC

**What to improve:**
- Dogfood discipline: initial commit attempt missed updating docs/commands.md and config.toml mappings — caught on review
- Execution plan's Out of Scope section contradicted Phase 4 scope — scope changes should be recorded in Decision Log immediately when they happen
