# Architecture

## System Overview

harn is a single-binary Rust CLI. It scaffolds a harness structure for a project, then provides lifecycle commands to maintain that structure over time. It does **not** orchestrate agents — it gives agents a well-structured environment.

## Crate Structure

```
harn/
├── src/
│   ├── main.rs          # Entry point: parse args, dispatch to commands
│   ├── cli.rs           # clap derive definitions, subcommand routing
│   ├── config.rs        # .agents/harn/config.toml read/write (serde + toml)
│   ├── types.rs         # Newtypes: Slug, ProjectName, HarnDate, HarnPath, Stack, AiTool
│   ├── detect.rs        # Project environment detection (git, package managers, AI tools)
│   ├── init/
│   │   ├── mod.rs       # Init orchestration: detect → resolve → render → write
│   │   └── render.rs    # Template rendering (minijinja + include_dir!)
│   ├── check.rs         # Structural validation (file existence, cross-refs, hashes)
│   ├── plan.rs          # Execution plan management (new, list, complete)
│   ├── sprint.rs        # Sprint contract management (new, status, done)
│   ├── status.rs        # Project state aggregation and display
│   ├── gc.rs            # Staleness detection via git history (git2)
│   ├── score.rs         # Quality score display and interactive update
│   ├── upgrade.rs       # Hash-based template upgrade with sidecar strategy
│   ├── util.rs          # Shared utilities (sha256_hex, extract_md_links)
│   └── assess.rs        # Harness maturity assessment (HARNESS-SPEC levels)
├── templates/           # Embedded at compile time via include_dir!
│   ├── AGENTS.md.j2
│   ├── CLAUDE.md.j2
│   ├── ARCHITECTURE.md.j2
│   └── docs/            # Mirrors the generated docs/ tree
├── tests/
│   ├── helpers/mod.rs   # TempProject: temp dir + git init + harn binary runner
│   ├── init.rs          # Integration tests for harn init
│   ├── check.rs         # Integration tests for harn check
│   ├── plan.rs          # Integration tests for harn plan
│   ├── sprint.rs        # Integration tests for harn sprint
│   ├── status_gc.rs     # Integration tests for harn status + gc
│   ├── score.rs         # Integration tests for harn score
│   ├── upgrade.rs       # Integration tests for harn upgrade
│   └── e2e.rs           # End-to-end multi-command workflow tests
└── Cargo.toml
```

## Module Dependency Rules

Dependencies flow **downward only**. No module may import from a module above it.

```
main.rs
  └── cli.rs
        ├── init/       → config, detect, types
        ├── check.rs    → config, util
        ├── plan.rs     → types
        ├── sprint.rs   → types
        ├── status.rs   → config, sprint
        ├── gc.rs       → config, util
        ├── score.rs    → types
        ├── upgrade.rs  → config, init/render, util
        └── assess.rs   → (standalone, no crate imports)
```

- `cli.rs` dispatches to command modules. Command modules depend on `config.rs` and domain-specific crates.
- `config.rs` is a shared dependency for all commands. It owns the `Config` type and all config I/O.
- `types.rs` defines the newtype vocabulary (`Slug`, `ProjectName`, `HarnDate`, `HarnPath`, `Stack`, `AiTool`). Used across all modules.
- `detect.rs` is used only by `init/`. No other module should call detection logic.
- `util.rs` provides shared pure functions (`sha256_hex`, `extract_md_links`). It is a leaf dependency like `types.rs` — it must not import other crate modules.
- `templates/` is a compile-time asset directory, not a runtime module. Accessed via `include_dir!` in `init/render.rs`.

## Common Mistakes

1. **Importing `config.rs` types in `cli.rs` dispatch logic.** Config belongs to command modules. `cli.rs` should only parse arguments and call command functions — never read or interpret config values directly.
2. **Adding detection logic outside `detect.rs` / `init/`.** All environment-sensing code (git detection, stack detection, AI tool detection) belongs in `detect.rs`. Other modules should receive detection results as parameters.
3. **Using raw `String` where a newtype exists.** If a value has domain meaning (slugs, project names, dates, paths, stacks, AI tools), use the corresponding type from `types.rs`. Raw strings bypass validation.
4. **Calling `std::process::exit()` in command modules.** Return `Result<()>` and let errors propagate to `main()`. Only `main.rs` should determine exit codes.
