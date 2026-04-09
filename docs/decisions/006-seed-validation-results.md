# 006: Seed Validation Results

**Date**: 2026-04-08

## Context

The seed design (three files: AGENTS.md with cultivation protocol,
docs/decisions/README.md, docs/ARCHITECTURE.md) was tested in two real projects
before building the CLI tool.

## Validation 1: Project with existing harness infrastructure

A project already built on community harness principles. The agent was also
told by the human "this is your home."

Results:
- The agent internalized the cultivation protocol as a shift from "task
  completer" to "long-term inhabitant who cultivates the environment"
- It would proactively suggest recording decisions in docs/decisions/ without
  being asked
- It demonstrated the full pattern-to-convention pipeline: notice drift →
  judge if intentional → choose default → converge locally → sediment as
  convention or mechanical rule
- It identified a genuine gap: the seed provides values but not prioritization
  heuristics for when multiple valid actions compete

Caveats: The positive results cannot be fully attributed to the seed alone.
The project already had harness infrastructure (docs/handoff.md, docs/quality.md,
docs/plans/) and the human framed the agent's role explicitly.

## Validation 2: Project with no prior harness experience

A Python ML library (real code, ruff + pyright, no test suite, no harness
history). The seed was planted as the only harness structure.

Results:
- The agent read AGENTS.md and correctly identified the project's phase as
  "Early Growth"
- When asked what knowledge to sediment first, it chose the project's most
  important architectural question: "which designs are core identity vs
  experimental" — applying the Cultivation criteria (costly to reverse,
  non-obvious, worth recording)
- It created docs/decisions/004-core-vs-experimental.md with evidence-based
  reasoning drawn from actual code (public APIs, Phase 2 markers, optional
  switches)
- It proactively updated docs/ARCHITECTURE.md without being prompted, adding
  "Stable Architectural Biases" and "Questions To Revisit Later" sections
- It respected existing file numbering (004, not 001)
- It honestly noted what it didn't do (no quality checks)

This validation is stronger: no prior harness infrastructure, no human
framing of the agent's role. The seed alone drove effective cultivation behavior.

## Conclusions

1. The cultivation protocol changes agent behavior — not just "read and
   acknowledge" but genuine internalization of the cultivator role
2. Structural gravity works — docs/decisions/ with naming convention is
   sufficient to guide recording behavior
3. Four directives are sufficient for basic cultivation
4. The seed works in projects with no harness history
5. Prioritization heuristics are a natural growth-phase need but should not
   be added to the seed (they are project-specific and should emerge from
   practice as a convention)
