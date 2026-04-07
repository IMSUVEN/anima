# harn

> Help humans harness AI agents — not just use them.

## What This Project Is

`harn` is a project that helps engineers adopt **harness engineering** — the discipline of designing environments, feedback loops, and control systems that make AI coding agents effective. The name comes from "harness": the structure around the model, not the model itself.

Many engineers today treat AI agents as autocomplete with extra steps. Harness engineering is the recognition that **the environment bounds the agent's output quality, not the model**. `harn` exists to make that environment easy to build and maintain.

The project's final form — CLI tool, library, framework, or something else — is deliberately undecided. The guiding principles are settled; the product shape will emerge from them.

## Guiding Documents

These two documents are the intellectual foundation of the project. All design decisions trace back to them.

- [Harness Specification](docs/HARNESS-SPEC.md) — prescriptive spec: what to build, with obligation levels (MUST/SHOULD/MAY)
- [Harness Guide](docs/HARNESS-GUIDE.md) — companion guide: how to think about harness design, with rationale and decision frameworks

The spec and guide are built on three structural contradictions inherent to human-agent systems (intent transfer, self-evaluation, entropy) and three axioms derived from them (governance, strategy, mechanism). Read the guide's §1–2 for the full analysis.

## Project Status

**Phase: foundational thinking.** The guiding documents are written and stable. No implementation exists. The next step is determining what form the tool takes and how it delivers the spec's principles to working engineers.

## Conventions

- English is the primary language; Chinese translations are maintained alongside
- Guiding documents live in `docs/`
- All project knowledge is stored in the repository (if it's not in the repo, it doesn't exist)
