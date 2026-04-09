# anima check as the spirit's first sense

`anima check` is the first command beyond `init`. It observes four signals in
the project's harness files — state phase, architecture documentation, decision
records, and conventions — and reports which areas have grown and which remain
dormant.

## Why this, why now

After `init` plants the seed, there is no feedback mechanism. The cultivation
protocol tells agents *what* to do but gives them no way to *perceive* what has
already been done. `check` closes this gap: an agent can run it at session start
and immediately understand the project's cultivation state without reading and
interpreting every file manually.

## Design choices

- **Observation, not judgment.** Output describes what exists ("3 decisions
  recorded", "architecture: empty"), not what's good or bad. There is no score.
- **Agent-first, human-readable.** The output is compact and structured for
  agent consumption, but any human can read it too.
- **Four signals only.** State phase, architecture, decisions, conventions —
  these correspond directly to the four cultivation directives in the seed's
  `AGENTS.md`. No additional metrics in v1.
- **No configuration.** `check` takes no arguments. It reads what `init`
  planted and reports what it finds.

## Alternatives considered

- A JSON output mode for machine parsing was considered but deferred. Plain
  text is sufficient for current AI coding tools, and adding `--format json`
  is trivial if needed later.
- Scoring or grading the harness health was rejected. The philosophy is
  cultivation, not examination. Observations enable the agent to act;
  scores would impose a value system.
