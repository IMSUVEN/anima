# harn — Generated Content Specification

This document specifies every file `harn init` generates. The design principle is: **every generated file must be worth reading**. No empty tables, no placeholder-only files. Files that only become useful later are created on demand by the corresponding command.

Template variables are shown as `{{variable}}`.

---

## File Generation Strategy

### Always generated (has substantive default content)

| File | Why it must exist from day one |
|------|-------------------------------|
| `AGENTS.md` | Agent entry point. The map, workflow, and constraints. |
| `CLAUDE.md`* | Claude Code entry point. Thin wrapper referencing AGENTS.md. |
| `ARCHITECTURE.md` | Structural truth. Agents need this to understand where code belongs. |
| `docs/design-docs/index.md` | Registry with at least one entry (core-beliefs). |
| `docs/design-docs/core-beliefs.md` | Golden principles. Substantive from day one. |
| `docs/evaluation/criteria.md` | Quality grading system. Substantive from day one. |
| `docs/templates/exec-plan.md` | Reference template for workflow commands. |
| `docs/templates/sprint-contract.md` | Reference template for workflow commands. |
| `docs/templates/handoff.md` | Reference template for workflow commands. |
| `.agents/harn/config.toml` | Operational config. Required by all harn commands. |

### Directories created (empty, ready for content)

| Directory | Populated by |
|-----------|-------------|
| `docs/exec-plans/active/` | `harn plan new`, `harn sprint new` |
| `docs/exec-plans/completed/` | `harn plan complete`, `harn sprint done` |
| `docs/product-specs/` | User or agent creates specs as needed |
| `docs/references/` | User adds external doc references as needed |
| `docs/templates/` | Populated by init with workflow templates (exec-plan, sprint-contract, handoff) |

### Created on demand (by harn commands or user)

| File | Created by | Trigger |
|------|-----------|---------|
| `docs/QUALITY_SCORE.md` | `harn score update` | First quality assessment |
| `docs/exec-plans/tech-debt-tracker.md` | Created manually or by an agent | First debt entry |
| `docs/product-specs/*.md` | User or agent | When a spec is written |

**Rationale**: An agent navigating to an empty `QUALITY_SCORE.md` full of TODO rows wastes context window and provides zero information. It's better for that file to not exist yet — the agent sees it's absent, which is itself a clear signal ("quality hasn't been assessed yet"), rather than a file pretending to have content.

---

## Root-Level Files

### AGENTS.md

**Purpose**: Universal agent entry point. The map, the workflow, and the constraints — all in ~80 lines. Every agent tool reads this first.

**Audience**: All agents (Codex primary), humans.

**Design principles**:
- **Map, not manual.** Pointers to deeper docs, not lengthy instructions.
- **Workflow, not just index.** Tell agents what to DO, not just where to look.
- **Stack-aware Quick Start.** Build/test/lint commands are inline Jinja conditionals per stack — no generic placeholders.
- **Two-tier Documentation Map.** "Implementation docs" (read always) vs "Reference & workflow" (consult when needed).
- **Safety by default.** Includes a "Stop on ambiguity" constraint.

**Template variables**: `project_name`, `project_description`, `stack` (used in `{% if stack == "rust" %}` conditionals).

The template renders stack-specific Quick Start commands, constraints, and a split Documentation Map. See the full template in `templates/AGENTS.md.j2`. Key sections:

- **Quick Start**: Stack-conditional code block (Rust: `cargo build/test/clippy/fmt`; Node: `npm install/test/lint`; Python: `pip/pytest/ruff`; Go: `go build/test/vet`; Generic: TODO placeholder).
- **Architecture**: Pointer to ARCHITECTURE.md for crate structure and dependency rules.
- **Workflow**: 5 steps (Orient → Quality bar → Architecture → Validate → Record decisions).
- **Key Constraints**: Type safety, no silent failures, dependency direction, test speed, stop on ambiguity.
- **Documentation Map**: Split into "Implementation docs" and "Reference & workflow".
- **Tooling**: harn command reference table (8 commands).

---

### CLAUDE.md

**Purpose**: Claude Code entry point. A thin wrapper referencing AGENTS.md to avoid duplication.

**Audience**: Claude Code agent.

**Design principle**: One source of truth. All shared knowledge lives in AGENTS.md. CLAUDE.md only contains Claude Code-specific guidance that differs from the universal instructions.

```markdown
# {{project_name}}

This project uses a structured harness for agent-driven development.

All project context — architecture, workflow, constraints, evaluation criteria, and documentation map — lives in [AGENTS.md](AGENTS.md). Read it first.

## Claude Code-Specific Notes

<!-- Add Claude Code-specific instructions here if needed.
     Examples: preferred tool usage patterns, Claude-specific coding conventions.
     Everything shared across all AI tools belongs in AGENTS.md. -->
```

---

### ARCHITECTURE.md

**Purpose**: Top-level architecture map. Defines crate/module structure, dependency rules, and common mistakes.

**Audience**: Agents and humans.

**Design principle**: The structural truth of the project. Provides TODO-driven scaffolding for the user to fill in their actual module tree and dependency graph. Stack-specific hints guide boundary enforcement tooling. Includes a "Common Mistakes" section stub — the guide recommends calling out the 3-4 things a newcomer is most likely to get wrong.

**Template variables**: `stack` (used in `{% if stack == "..." %}` conditionals for boundary enforcement hints).

The template renders these sections. See the full template in `templates/ARCHITECTURE.md.j2`:

- **System Overview**: TODO placeholder for 2-3 sentence high-level description.
- **Project Structure**: Stack-specific example tree in a fenced code block (Rust: `src/`, `tests/`, `Cargo.toml`; Node: `src/`, `package.json`; Python: project package, `tests/`; Go: `cmd/`, `internal/`; Generic: basic layout).
- **Module Dependency Rules**: "Dependencies flow downward only" invariant, example dependency graph in a code block, stack-specific enforcement hints (Rust: `cargo clippy`; Node: ESLint import rules; Python: import linting; Go: `go vet`).
- **Common Mistakes**: 3 TODO items prompting the user to document the most common dependency direction violation, misplaced responsibility, and type safety violation.
- **Cross-Cutting Concerns**: TODO placeholders for error handling, logging, and configuration strategies.

---

## docs/design-docs/

### docs/design-docs/index.md

**Purpose**: Registry of design documents.

```markdown
# Design Documents

| Document | Status | Date | Summary |
|----------|--------|------|---------|
| [Core Beliefs](core-beliefs.md) | Active | {{date}} | Agent-first operating principles |

<!-- Add new design documents here as they are created.
     Include: context, options considered, decision, and consequences. -->
```

### docs/design-docs/core-beliefs.md

**Purpose**: The "golden principles" — opinionated rules that keep the codebase legible and reliable for agents. This is the most opinionated generated file.

**Audience**: Agents (primary). These are hard constraints.

**Design principle**: Absorbs reliability and security principles that would otherwise live in near-empty standalone files. Every principle here has real substance.

```markdown
# Core Beliefs

These are the operating principles for agent-driven development in this project. Agents should treat these as hard constraints unless explicitly overridden in a specific execution plan.

## 1. Repository Is the Source of Truth

All knowledge lives in the repository. Decisions made in conversations, chat threads, or meetings must be captured as versioned artifacts (markdown, code, config) or they effectively don't exist. If it isn't in the repo, it isn't real.

## 2. Map, Not Manual

Agent-facing documentation should be concise and navigational. Tell agents where to look, not what to think. A 100-line entry point with pointers is better than a 1000-line instruction manual. Every document an agent reads should be worth reading.

## 3. Parse, Don't Assume

Validate data shapes at system boundaries. Use typed schemas, parse incoming data, and fail explicitly on malformed input. Never build on assumed shapes. Never trust data from outside the system boundary.

## 4. Fail Explicitly, Not Silently

Every error path must produce a clear signal. No swallowed exceptions, no silent fallbacks that hide broken state. If something goes wrong, the system should say so loudly and immediately. Prefer idempotent operations that can be safely retried.

## 5. Shared Utilities Over Hand-Rolled Helpers

Prefer centralized, tested utility modules over ad-hoc helpers scattered across the codebase. When the same pattern appears twice, extract it. This keeps invariants centralized and reduces agent confusion.

## 6. Enforce Boundaries Mechanically

Architectural constraints (dependency direction, naming conventions, layer rules) should be enforced by linters and tests, not by documentation alone. If a rule can be checked by a machine, it should be. Use least-privilege access for all service accounts and credentials.

<!-- TODO: Set up mechanical enforcement for your stack. Examples:
     - Rust: `cargo clippy -- -D warnings` + `rustfmt` in CI
     - TypeScript: eslint with import boundary rules + strict tsconfig
     - Python: ruff/flake8 + mypy strict mode
     - Go: golangci-lint + go vet
     Add a CI step that blocks merge on lint failures.
     Write structural tests for critical invariants (e.g., "no module in layer A imports from layer B"). -->

## 7. Boring Technology Is a Feature

Favor composable, API-stable, well-documented technologies. Agents reason better about tools with extensive documentation and predictable behavior. When a dependency is opaque, consider reimplementing the needed subset — tightly integrated, fully tested, fully legible.

## 8. Every Change Is Reversible

Prefer small, incremental changes that are easy to review and revert. Large, sweeping changes are harder for both humans and agents to verify. A series of small PRs beats one giant refactor.

## 9. Secrets Never Enter the Repository

Use environment variables or dedicated secret management tools. Never commit credentials, API keys, or tokens. Log security-relevant events (auth failures, permission denials, input validation errors) but never log secrets.

## 10. Technical Debt Is a Continuous Process

Track debt explicitly. Pay it down in small, regular increments rather than accumulating it for painful bursts. Background cleanup is preferable to periodic overhauls.
```

---

## docs/evaluation/

### docs/evaluation/criteria.md

**Purpose**: Explicit quality grading criteria given to both generators and evaluators.

**Audience**: Agents and humans.

**Stack-aware rendering**: The criteria set adapts to the detected stack. CLI tools and libraries don't need "Visual & UX Design"; they benefit from "API Ergonomics" instead. The grading scale and core criteria (Functionality, Code Quality) are universal; stack-specific criteria replace or adjust the others.

```markdown
# Evaluation Criteria

These criteria define what "good" means for this project. When evaluating work — whether self-evaluating or reviewing someone else's output — grade against each criterion independently.

## Grading Scale

| Grade | Meaning |
|-------|---------|
| A | Excellent — exceeds expectations, no issues |
| B | Good — meets expectations, minor issues only |
| C | Acceptable — meets minimum bar, notable gaps |
| D | Below bar — significant issues that need addressing |
| F | Failing — does not meet requirements |

**Pass threshold**: Every criterion must be C or above for the work to pass.

---

## Criteria

### 1. Functionality (Weight: High)

Does the implementation do what the spec says? Do core user workflows work end-to-end?

- **A**: All specified features work correctly. Edge cases handled.
- **B**: Core features work. Minor edge cases may be unhandled.
- **C**: Core features work with caveats. Some features may be partial.
- **D**: Key features are broken or incomplete.
- **F**: Primary functionality does not work.

### 2. Product Depth (Weight: High)

Are features fully implemented with real logic, or are they stubs and placeholders?

- **A**: Features are fully implemented with real logic, state management, and feedback.
- **B**: Features work with minor gaps in depth.
- **C**: Some features are skeletal or stub-only.
- **D**: Multiple features are stubs or placeholders.
- **F**: Most features are non-functional facades.

### 3. Code Quality (Weight: Medium)

Is the code maintainable, well-structured, and consistent with architectural constraints?

- **A**: Clean architecture, good error handling, consistent patterns, tested.
- **B**: Generally clean. Minor inconsistencies.
- **C**: Functional but with structural issues (unclear naming, missing error handling).
- **D**: Significant structural problems. Hard to maintain.
- **F**: Spaghetti code. Architectural rules violated.

{% if stack == "node" or stack == "generic" %}
### 4. Visual & UX Design (Weight: Medium)

Does the interface have a coherent identity? Is it usable?

- **A**: Cohesive design language, clear visual hierarchy, intuitive interactions.
- **B**: Clean and functional. Minor design inconsistencies.
- **C**: Usable but generic. No distinct visual identity.
- **D**: Confusing layout or broken visual hierarchy.
- **F**: Unusable interface.
{% else %}
### 4. API Ergonomics (Weight: Medium)

Is the public interface (CLI, library API, config format) intuitive and well-documented?

- **A**: Intuitive naming, clear error messages, discoverable options, well-documented.
- **B**: Generally clean interface. Minor discoverability gaps.
- **C**: Functional but requires reading source to understand usage.
- **D**: Confusing interface, unhelpful errors, undocumented behavior.
- **F**: Unusable without deep source reading.
{% endif %}

### 5. Originality (Weight: Low)

Are there deliberate creative or architectural choices, or is this all defaults and templates?

- **A**: Distinctive choices that serve the product goals.
- **B**: Some deliberate choices visible.
- **C**: Mostly defaults, but appropriate ones.
- **D**: Generic "AI slop" — unmodified templates and library defaults.
- **F**: No evidence of intentional decision-making.

---

## How to Use These Criteria

**For the agent building the feature**: Read these criteria before starting. Target B or above on every criterion. Self-evaluate your work against these before handing off.

**For the reviewing agent or human**: Grade each criterion independently. If any criterion is D or below, the work should be revised with specific feedback on what needs to change.

<!-- TODO: Add few-shot calibration examples in docs/evaluation/examples/ to anchor
     grading consistency. Include 2-3 examples showing what A vs C vs F looks like
     for each criterion in the context of this project. -->
```

---

## docs/templates/

Reference templates copied by `harn plan new`, `harn sprint new`, and `harn sprint done`. Not filled in during init.

### docs/templates/exec-plan.md

**Design principle**: Aligned with HARNESS-SPEC.md Appendix A.1. The template is designed for **self-containedness** (a reader with only this file and the working tree must be able to succeed) and **durability** (updated as work proceeds so it survives context resets).

```markdown
# ExecPlan: [Short, Action-Oriented Title]

Living document. Update Progress, Surprises, Decision Log, and Retrospective
as work proceeds. This plan MUST be self-contained — a reader with only this
file and the working tree must be able to succeed.

## Purpose

<!-- What someone gains after this change. How they can see it working. 2-3 sentences. -->

## Context and Orientation

<!-- Current state as if the reader knows nothing. Key files by full path.
     Define non-obvious terms. Include enough context that a new agent session
     can pick up this plan without any prior conversation history. -->

## Scope

### In Scope
-

### Out of Scope
-

## Milestones

Each milestone: scope, what exists at the end, commands to run, observable acceptance.
Include prototyping milestones to de-risk unknowns.

### Milestone 1: [Name]

- **Scope**: ...
- **Observable acceptance**: ...
- **Commands to verify**: ...

### Milestone 2: [Name]

- **Scope**: ...
- **Observable acceptance**: ...
- **Commands to verify**: ...

## Validation and Acceptance

<!-- How to exercise the system. Observable behavior with specific inputs/outputs.
     Exact test commands and expected results. -->

## Progress

- [ ] Incomplete step.

## Surprises & Discoveries

<!-- Record unexpected findings as work proceeds. These are often the most
     valuable insights for the next session or the next project. -->

- Observation: ...
  Evidence: ...

## Decision Log

- Decision: ...
  Rationale: ...
  Date: ...

## Outcomes & Retrospective

<!-- Filled in when the plan is completed. Outcomes, gaps, lessons learned. -->

## Interfaces and Dependencies

<!-- Libraries, types, function signatures that must exist post-implementation. -->
```

### docs/templates/sprint-contract.md

```markdown
# Sprint Contract: [Title]

## Scope

<!-- What will be built in this sprint? Be specific. -->

## Deliverables

1.
2.
3.

## Acceptance Criteria (observable behavior)

- [ ] Criterion 1: [testable behavior]
- [ ] Criterion 2: [testable behavior]
- [ ] Criterion 3: [testable behavior]

## Verification Method

<!-- How will the evaluator test each criterion? Manual testing? Automated tests? Browser automation? -->

## Dependencies

<!-- What must be in place before this sprint can start? -->

## Risks

<!-- What could go wrong? What's the mitigation? -->
```

### docs/templates/handoff.md

```markdown
# Handoff: [From Context] → [To Context]

## Completed Work

-

## Current State

- Application status: [running / broken / partial]
- Tests: [passing / failing / not yet written]
- Known issues:
  -

## Next Steps (ordered)

1.
2.
3.

## Key Context

<!-- Critical decisions, constraints, patterns, or gotchas the next session must know.
     Include anything that would be lost if the context window were cleared. -->
```

---

## .agents/harn/config.toml

**Purpose**: harn's operational config. Written during init, read by all commands.

```toml
# harn configuration — generated by `harn init`
# This file should be committed to version control.

[project]
name = "{{project_name}}"
created = "{{date}}"
harn_version = "{{harn_version}}"

[tools]
agents = [{{agents_list}}]

[init]
stack = "{{stack}}"

[init.file_hashes]
# SHA-256 hashes of generated files at init time.
# Used by `harn check` and `harn gc` to detect uncustomized templates.
# "AGENTS.md" = "abcdef1234567890..."

[check]
required_files = [
  "AGENTS.md",
  "ARCHITECTURE.md",
  "docs/evaluation/criteria.md",
]

[gc]
stale_threshold_days = 14
ignore_paths = []
# Add paths to exclude from gc analysis:
# ignore_paths = ["docs/HARNESS-SPEC.md", "docs/HARNESS-GUIDE.md"]

# Map documentation to related code for divergence detection:
# [[gc.mappings]]
# doc = "docs/api-reference.md"
# code = ["src/routes/", "src/handlers/"]
```

---

## On-Demand Files

These files are NOT generated by `harn init`. They are created by specific commands when they first become useful.

### docs/QUALITY_SCORE.md (created by `harn score update`)

```markdown
# Quality Scores

Last updated: {{date}}

Grade each domain on the [evaluation criteria](evaluation/criteria.md). Update scores when significant changes land.

| Domain | Functionality | Product Depth | Code Quality | API Ergonomics | Overall | Last Assessed |
|--------|:---:|:---:|:---:|:---:|:---:|:---:|
| {{domain}} | {{grade}} | {{grade}} | {{grade}} | {{grade}} | {{grade}} | {{date}} |

## History

| Date | Domain | Change | Notes |
|------|--------|--------|-------|
| | | | |
```

### docs/exec-plans/tech-debt-tracker.md (created on first debt entry)

```markdown
# Tech Debt Tracker

| ID | Description | Priority | Effort | Impact | Added | Status |
|----|-------------|----------|--------|--------|-------|--------|
| | | | | | | |

## Priority Levels

- **P0**: Blocking — must be addressed before next feature work.
- **P1**: High — address within the current cycle.
- **P2**: Medium — schedule when convenient.
- **P3**: Low — track but no urgency.
```

---

## Summary

### Always Generated

| Category | Files | Dirs |
|----------|-------|------|
| Root entry points | 2–3 (AGENTS.md, ARCHITECTURE.md, CLAUDE.md if Claude Code selected) | 0 |
| Config | 1 (.agents/harn/config.toml) | 2 |
| Design docs | 2 (index.md, core-beliefs.md) | 1 |
| Evaluation | 1 (criteria.md) | 1 |
| Templates | 3 (exec-plan.md, sprint-contract.md, handoff.md) | 1 |
| Empty dirs | 0 | 4 (exec-plans/active/, completed/, product-specs/, references/) |
| **Total** | **9–10 files** | **9 directories** |

*`CLAUDE.md` is only generated when Claude Code is selected. `AGENTS.md` is always generated regardless of tool selection.

### Key Change from Previous Design

Previous: 15 files generated, many with empty TODO tables.
Current: 9–10 files generated (depending on tool selection), every one with substantive content. Remaining files created on demand when content exists. Use `--minimal` to generate only the 4 essential files (AGENTS.md, ARCHITECTURE.md, docs/evaluation/criteria.md, and config.toml).

Principle: **if an agent reads it, it should learn something.**
