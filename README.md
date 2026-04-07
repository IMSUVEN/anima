# harn

[English](README.md) | [中文](README.zh-CN.md)

A CLI tool that gives projects the structure and workflows for effective AI-agent-driven development — based on harness engineering principles from [OpenAI](https://openai.com/index/harness-engineering/) and [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps).

## Background

AI coding agents are context-dependent. Their output quality is bounded by the **environment**, not the model. Setting up the right project structure — discoverable knowledge, architectural constraints, quality criteria, workflow artifacts — is tedious and inconsistent when done manually.

`harn` aims to solve this by providing a CLI that bootstraps and maintains the knowledge and workflow layer that makes agents effective.

## Specification

The design is grounded in a formal specification:

- [Harness Specification](docs/HARNESS-SPEC.md) — defines what a harness is, its components, and lifecycle
- [Harness Guide](docs/HARNESS-GUIDE.md) — companion guide with rationale, examples, and decision frameworks

## License

MIT
