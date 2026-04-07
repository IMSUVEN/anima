# harn

[English](README.md) | [中文](README.zh-CN.md)

Help humans harness AI agents — not just use them.

## The Problem

Most engineers today use AI agents as faster typists. They prompt, they get code, they paste it in. This works — until it doesn't. The agent drifts from intent, replicates bad patterns, confidently produces broken code, and the human spends more time debugging the output than they saved generating it.

The bottleneck was never the model. It's the environment.

## What Harness Engineering Is

A **harness** is everything around the model: the project structure that orients the agent, the type system that constrains its guesses, the tests that close the feedback loop, the safety boundaries that prevent damage, and the escalation protocols that pull it back when it's stuck.

**Harness engineering** is the discipline of designing these environments deliberately — so the agent's power works *for* you instead of *against* you.

This isn't a new idea. Between late 2024 and early 2026, [OpenAI](https://openai.com/index/harness-engineering/), [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps), and several independent practitioners converged on remarkably similar conclusions — not by coordination, but because the problem structure admits only a narrow band of workable solutions.

## What harn Will Be

`harn` is a project to make harness engineering accessible. Setting up the right environment — discoverable knowledge, architectural constraints, feedback loops, workflow artifacts — is tedious and inconsistent when done by hand. `harn` aims to change that.

The project's final form is deliberately undecided. The guiding principles are clear; the product shape will follow.

## Guiding Documents

The design is grounded in a formal specification and a companion guide. Together they establish three structural contradictions of human-agent systems, three axioms for managing them, and a maturity model (Level 1–3) for progressive adoption.

- [Harness Specification](docs/HARNESS-SPEC.md) — what a harness is, its components, obligations, and lifecycle
- [Harness Guide](docs/HARNESS-GUIDE.md) — how to think about harness design: rationale, examples, and decision frameworks

## License

MIT
