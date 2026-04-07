# Core Beliefs

These are the operating principles for developing harn. Agents should treat these as hard constraints unless explicitly overridden in a specific execution plan.

## 1. Repository Is the Source of Truth

All knowledge lives in the repository. Decisions made in conversations, chat threads, or meetings must be captured as versioned artifacts (markdown, code, config) or they effectively don't exist. If it isn't in the repo, it isn't real.

## 2. Every Generated File Must Be Worth Reading

No empty placeholders or TODO-only tables. If a file exists in the generated harness, an agent reading it should learn something. Files that would be empty at init time are deferred — created on demand when they have content. An absent file is a clearer signal than a file pretending to have content.

## 3. Non-Destructive by Default

Never overwrite existing user files without explicit `--force`. When a file exists, skip it and report. Partial failures leave already-written files in place — the user can fix the issue and re-run idempotently. This is not optional; it is the core trust contract between harn and its users.

## 4. Detect, Don't Ask

Auto-detect the project environment (git, package manager, existing AI tool configs). Only prompt for what cannot be inferred. If all required information can be detected, `harn init` runs with zero prompts. The fewer questions asked, the faster the user reaches a working harness.

## 5. Offline-First

All subcommands are offline — none require network access. harn is a local tool that reads and writes files. No telemetry, no package registry calls, no API hits.

## 6. Type Safety via Newtypes

Use Rust's type system as the primary enforcement mechanism. Domain identifiers (slugs, file paths, dates, plan names) get typed wrappers — not raw `String`. When the compiler rejects a misuse, the agent doesn't need to debug it at runtime. Parse external input (CLI args, TOML config, file content) into typed representations at the boundary; trust the types within.

## 7. No Silent Failures

Every error path must produce a user-visible message with two parts: what happened, and what to do about it. No exit code 0 on failure. No swallowed errors. No "should never happen" panics in non-catastrophic paths. Use `anyhow` for application errors with context chains that trace back to the root cause.

## 8. Boring Technology

Favor well-documented, API-stable crates with large user bases: clap, serde, minijinja, walkdir. Agents reason better about tools with extensive documentation and predictable behavior. When a dependency is opaque or unstable, consider reimplementing the needed subset. Every dependency is a liability; justify each one.

## 9. Enforce Boundaries Mechanically

Architectural constraints (module dependency direction, naming conventions, error handling patterns) are enforced by `cargo clippy -- -D warnings` and `rustfmt`. If a rule can be checked by a machine, it should be. CI blocks merge on lint failures. Structural tests validate invariants that linters can't express (e.g., "no module in `init/` imports from `gc.rs`").

## 10. Embedded Templates, Single Binary

All templates compile into the binary via `include_dir!`. Zero external file dependencies for distribution. The `--template-dir` flag is the escape hatch for custom templates. This means harn can be installed with `cargo install harn` and immediately works — no setup, no config, no companion files.
