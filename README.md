# anima

[English](README.md) | [中文](README.zh-CN.md)

Plant seeds, not templates. Cultivate agents, don't configure them.

## What anima Does

anima plants a **seed** into your project — three files that make your AI coding agent a cultivator, not just an executor:

```
AGENTS.md              — orients the agent + cultivation protocol
docs/ARCHITECTURE.md   — fills as architecture emerges
docs/decisions/        — record decisions as you go
```

The seed contains a **cultivation protocol**: four directives that tell the agent to proactively sediment knowledge — record decisions, update architecture, codify conventions — without waiting to be asked. The protocol has been validated in real projects: agents that read it shift from "task completer" to active participant in the project's growth.

No technology stack is prescribed. No linter rules. No test framework. The right rules emerge from your project's own practice.

## Install

```bash
cargo install --git https://github.com/IMSUVEN/anima
```

## Usage

```bash
# Plant a seed in the current directory
anima init

# Or specify a project name
anima init --name my-project

# Observe the project's cultivation state
anima check
```

`anima init` plants the seed. `anima check` observes how it's growing — which areas have developed and which remain dormant. Agents can run `check` at session start to sense what the project needs.

Then anima gets out of the way. The seed grows through your collaboration with your AI coding tool — Cursor, Codex, Claude Code, or whatever you use.

## The Belief

The bottleneck in AI-assisted engineering is not the model. It's the environment.

Most approaches reach for control: constrain the agent, guard against its mistakes, configure it with precise rules. This works — but it produces harnesses that are standardized, brittle, and disconnected from the projects they serve.

anima starts from a different premise: **the agent is a nascent collaborator, not a dangerous tool.** The harness is not a cage — it's soil, light, and water. Each mistake promoted to a linter rule, each decision recorded, each convention discovered — these are not maintenance tasks. They are growth.

## Theory

anima is rooted in [harness engineering](https://openai.com/index/harness-engineering/) — the convergent discoveries of [OpenAI](https://openai.com/index/harness-engineering/), [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps), and independent practitioners between 2024–2026 about what makes AI coding agents effective.

| Document | Purpose |
|---|---|
| [Product Philosophy](docs/PHILOSOPHY.md) | anima's stance: cultivation over control, seeds over templates, the spirit |
| [Harness Specification](docs/HARNESS-SPEC.md) | The discipline: what to build, with obligation levels |
| [Harness Guide](docs/HARNESS-GUIDE.md) | The reasoning: how to think about harness design |

## License

MIT
