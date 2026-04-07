# harn

[English](README.md) | 中文

一个 CLI 工具，为项目搭建 AI 智能体高效协作所需的结构与工作流——基于 [OpenAI](https://openai.com/index/harness-engineering/) 和 [Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps) 提出的 Harness 工程理念。

## 背景

AI 编程智能体的表现高度依赖上下文。它的输出质量由**工作环境**决定，而非模型本身。为项目搭建合理的结构——可检索的知识库、架构约束、质量标准、工作流模板——如果全靠手动完成，既繁琐又难以一致。

`harn` 旨在通过一个 CLI 工具，搭建并维护让智能体高效工作所需的知识层与工作流层，从而解决这一问题。

## 规范文档

设计基于一套正式的规范：

- [Harness 工程规范](docs/HARNESS-SPEC.zh-CN.md) — 定义 harness 的概念、组成部分与生命周期
- [Harness 实践指南](docs/HARNESS-GUIDE.zh-CN.md) — 配套指南：思路、示例与决策框架

## 许可证

MIT
