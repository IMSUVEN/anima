# harn

> A Rust CLI that bootstraps and maintains harness structures for AI-agent-driven development. Single binary, offline-first, non-destructive, idempotent.

## Quick Start

```bash
cargo build                    # debug build
cargo test                     # full suite — target ≤60s
cargo clippy -- -D warnings    # lint — zero warnings policy
cargo fmt --check              # format check
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for crate structure, module boundaries, and dependency rules.

## Workflow

1. **Orient.** Read the active plan or sprint in [`docs/exec-plans/active/`](docs/exec-plans/active/). Run `cargo test` to confirm the tree is green.
2. **Know the quality bar.** Read [`docs/evaluation/criteria.md`](docs/evaluation/criteria.md). Target grade B or above.
3. **Respect the architecture.** Modules have strict dependency direction — see [ARCHITECTURE.md](ARCHITECTURE.md). CLI parses and dispatches; domain modules never depend on CLI types.
4. **Validate before handing off.** Run `cargo test && cargo clippy -- -D warnings && cargo fmt --check`. Fix any failures before declaring work complete.
5. **Update docs with code.** When changing code mapped in `.agents/harn/config.toml` `[[gc.mappings]]`, update the corresponding documentation in the same change. CI runs `harn check --ci` and `harn gc` to enforce this.
6. **Record decisions.** Non-obvious choices go in the execution plan's Decision Log or in [`docs/design-docs/`](docs/design-docs/).

## Key Constraints

- **Type safety.** Rust's type system is the primary enforcement mechanism. Use newtypes for domain identifiers (slugs, paths, dates). No `String` where a typed wrapper communicates intent.
- **No silent failures.** Every error path produces a user-visible message with what happened and what to do. Use `anyhow` for application errors with context chains.
- **Non-destructive by default.** Never overwrite existing user files without explicit `--force`. Skip and report.
- **Offline-first.** All subcommands are offline — none require network access.
- **Templates are embedded.** All templates compile into the binary via `include_dir!`. The `--template-dir` flag is the escape hatch.
- **Test speed.** Test suite must stay under 60 seconds. Use temp directories, not real filesystem fixtures that accumulate.
- **Stop on ambiguity.** When encountering destructive operations, unclear requirements, or security-sensitive changes, report what you found and wait for guidance rather than proceeding.

## Documentation Map

**Implementation docs** — read these for every task:

| Topic | Location |
|-------|----------|
| Architecture & modules | [ARCHITECTURE.md](ARCHITECTURE.md) |
| Technical design | [docs/design.md](docs/design.md) |
| Command reference | [docs/commands.md](docs/commands.md) |
| Generated file specs | [docs/generated-content.md](docs/generated-content.md) |
| Core beliefs | [docs/design-docs/core-beliefs.md](docs/design-docs/core-beliefs.md) |
| Evaluation criteria | [docs/evaluation/criteria.md](docs/evaluation/criteria.md) |
| Product specification | [docs/product-spec.md](docs/product-spec.md) |

**Reference & workflow** — consult when needed:

| Topic | Location |
|-------|----------|
| Harness specification | [docs/HARNESS-SPEC.md](docs/HARNESS-SPEC.md) |
| Harness guide | [docs/HARNESS-GUIDE.md](docs/HARNESS-GUIDE.md) |
| Design decisions | [docs/design-docs/](docs/design-docs/) |
| Workflow templates | [docs/templates/](docs/templates/) |
| Active plans | [docs/exec-plans/active/](docs/exec-plans/active/) |
| Completed plans | [docs/exec-plans/completed/](docs/exec-plans/completed/) |
