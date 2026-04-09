# anima

[English](README.md) | [中文](README.zh-CN.md)

种下种子，而非模板。培育智能体，勿以配置代之。

> **状态：扎根期。** anima 尚年轻，正在积极生长。种子格式可能随版本演进——`anima check` 会检测变化并引导你的智能体完成更新。

## anima 做什么

anima 向你的项目植入一颗 **种子**——三个文件，让你的 AI 编程智能体成为培育者，而不仅仅是执行者：

```
AGENTS.md              — 智能体定向 + 培育协议
docs/ARCHITECTURE.md   — 架构浮现时填充
docs/decisions/        — 随手记录决定
```

种子包含一套 **培育协议**：四条指引，告诉智能体主动沉淀知识——记录决定、更新架构、编纂约定——无需等人催促。该协议已在真实项目中验证：读到它的智能体会从「完成任务」转变为项目生长的积极参与者。

不规定技术栈。不预设 linter 规则。不指定测试框架。合宜的规则从你的项目实践中自行涌现。

## 安装

```bash
cargo install --git https://github.com/IMSUVEN/anima
```

## 使用

```bash
# 在当前目录播种
anima init

# 或指定项目名
anima init --name my-project

# 观察项目的培育状态
anima check
```

`anima init` 播下种子。`anima check` 观察种子的生长——哪些区域已发展、哪些仍休眠。智能体可在会话开始时运行 `check`，感知项目所需。

然后 anima 退场。种子通过你与 AI 编程工具的协作而生长——Cursor、Codex、Claude Code，或任何你使用的工具。

## 核心理念

AI 辅助工程中的瓶颈不在模型，而在环境。

应对之道，多见诸控制：约束智能体、防其失误、以精密规则配置。此举有效——却往往催生出标准化、脆弱、与所服项目脱节的 harness。

anima 立足另一前提：**智能体是尚待涵养的初生协作者，而非须严加防范的危险工具。** harness 不是囚笼——而是土、光与水。每一次将失误升格为 linter 规则，每一次记下决断，每一次从实践中发现约定——这些都不是维护琐事。它们是生长。

## 理论基础

anima 扎根于 [harness engineering](https://openai.com/index/harness-engineering/)——2024 至 2026 年间，[OpenAI](https://openai.com/index/harness-engineering/)、[Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps) 与独立实践者关于「何以使 AI 编程智能体奏效」的汇聚性发现。

| 文档 | 用途 |
|---|---|
| [产品哲学](docs/PHILOSOPHY.zh-CN.md) | anima 的立场：培育优先于控制，种子优先于模板，精灵 |
| [Harness 规范](docs/HARNESS-SPEC.zh-CN.md) | 学科之维：当建何物，以及义务层级 |
| [Harness 指南](docs/HARNESS-GUIDE.zh-CN.md) | 推理之维：如何思考 harness 设计 |

## 许可

MIT
