# anima

> Plant seeds, not templates. Cultivate agents, don't configure them.

## State

**Phase: Rooting.** Seed design validated. First CLI command (`anima init`) implemented in Rust.

anima plants growth-capable seeds into new projects — minimal structures from which a harness grows through practice, not templates that prescribe one. The `anima init` command is working: it plants three seed files with `{project-name}` replaced. The seed has been validated in two real projects (see `docs/decisions/006-seed-validation-results.md`). Next: `anima check` — a growth health assessment command that agents can call to evaluate knowledge sedimentation.

## Map

| Path | Purpose |
|---|---|
| [docs/PHILOSOPHY.md](docs/PHILOSOPHY.md) | Product philosophy: cultivation over control, the spirit concept |
| [docs/HARNESS-SPEC.md](docs/HARNESS-SPEC.md) | Prescriptive spec for harness engineering (discipline-level) |
| [docs/HARNESS-GUIDE.md](docs/HARNESS-GUIDE.md) | Reasoning guide for harness design (discipline-level) |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | How the documents and seed relate to each other |
| [docs/decisions/](docs/decisions/) | Recorded technical decisions (see README inside) |
| [seed/](seed/) | The concrete seed that `anima init` plants |
| [src/main.rs](src/main.rs) | CLI implementation (Rust) |
| [Cargo.toml](Cargo.toml) | Rust project manifest |

Each document in `docs/` has a Chinese translation (`*.zh-CN.md`) alongside.

## Conventions

- English is the primary language; Chinese translations maintained for all documents
- English written first; Chinese translated to match (信达雅 standard)
- All project knowledge lives in the repository — if it's not in the repo, it doesn't exist
- Guiding documents live in `docs/`
- The Philosophy interprets the Spec/Guide but does not override them

## Cultivation

This project practices what it preaches. As you work on anima:

- **Decisions**: When a significant choice is made — about philosophy, seed
  design, tool architecture, or product direction — record it in
  `docs/decisions/`. Suggest this when you see it happen; don't wait to be asked.

- **Architecture**: When the project's structure changes (new documents, new
  directories, new relationships), update `docs/ARCHITECTURE.md`. Describe what
  exists and why, not what might exist someday.

- **Conventions**: When a pattern proves its worth, add it to the Conventions
  section above. A convention is a pattern promoted from "this worked" to "this
  is how we do it."

- **This file**: Keep this file honest. When the project's state changes, update
  the State section. When new knowledge locations appear, update the Map. This
  file should always reflect what is, not what should be.
