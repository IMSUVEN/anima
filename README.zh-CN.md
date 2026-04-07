# harn

[English](README.md) | 中文

一个 CLI 工具，为项目搭建 AI 智能体高效协作所需的结构与工作流——基于 [OpenAI](https://openai.com/index/harness-engineering/) 和 [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps) 提出的 Harness 工程理念。

## 问题

AI 编程智能体的表现高度依赖上下文。它的输出质量由**工作环境**决定，而非模型本身。为项目搭建合理的结构——可检索的知识库、架构约束、质量标准、工作流模板——如果全靠手动完成，既繁琐又难以一致。

## harn 做什么

`harn` 是一个 Harness 生命周期管理工具。它负责搭建并维护让智能体高效工作所需的知识层与工作流层。它**不负责编排智能体**——那是你的 AI 编程工具（Cursor、Codex、Claude Code 等）的职责。`harn` 为这些工具提供一个结构良好的工作环境。

```
harn init          # 生成 harness 结构
harn check         # 校验结构完整性
harn status        # 查看当前项目状态
harn plan          # 管理执行计划
harn sprint        # 管理冲刺合约
harn gc            # 通过 git 历史检测过期文档
harn score         # 查看与更新质量评分
harn upgrade       # 将模板更新至最新版本
harn assess        # 评估 harness 成熟度（对标 HARNESS-SPEC）
```

## 支持的 AI 工具

- **Claude Code** — 生成 `CLAUDE.md` 入口文件
- **Codex** — 生成 `AGENTS.md` 入口文件

两者指向同一套 `docs/` 知识结构。

## 核心特性

- **单一可执行文件** — Rust 编写，无运行时依赖
- **离线优先** — 核心操作不需要网络访问
- **非破坏性** — 未显式指定 `--force` 绝不覆盖已有文件
- **工具无关** — 通用知识层 + 工具专属入口文件

## 快速开始

```bash
cargo install harn
cd my-project
harn init
```

`harn init` 会自动检测技术栈和 AI 工具，然后生成完整的 harness 结构。定制生成的文件后，运行 `harn check` 校验完整性。

## 配置

配置文件位于 `.agents/harn/config.toml`，遵循社区 `.agents/` 目录约定。

## 深入了解

- [Harness 工程规范（中文）](docs/HARNESS-SPEC.zh-CN.md) — 完整的 harness 设计规范
- [Harness 实践指南（中文）](docs/HARNESS-GUIDE.zh-CN.md) — 配套指南：思路、示例与决策框架

## 参与贡献

参见 [CONTRIBUTING.zh-CN.md](CONTRIBUTING.zh-CN.md) 了解开发环境搭建与贡献指南。

## 许可证

MIT
