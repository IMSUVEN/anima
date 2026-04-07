# harn

[English](README.md) | [中文](README.zh-CN.md)

帮助人们驾驭 AI Agent——而不止于使用它们。

## 问题所在

如今多数工程师把 AI Agent 当作更快的打字员：提问、收代码、粘贴。一时可行——直到不再可行：Agent 偏离意图、复刻不良模式、笃定地写出坏代码，人花在调试其输出上的时间，反而超过了生成代码所省下的时间。

瓶颈从来不在模型，而在环境。

## 何谓 Harness 工程

**harness** 是围绕模型的一切：为 Agent 指明方向的项目结构、约束其猜测的类型系统、闭合反馈环的测试、防止损害的安全边界，以及在 Agent 陷入困境时将其拉回正轨的升级协议。

**Harness 工程** 是有意设计这些环境的学科——使 Agent 的力量*为你所用*，而非*与你为敌*。

这并非新见。2024 年末至 2026 年初，[OpenAI](https://openai.com/index/harness-engineering/)、[Anthropic](https://www.anthropic.com/engineering/harness-design-long-running-apps) 与多位独立实践者殊途同归，得出了惊人相似的结论——并非事先协调，而是问题结构本身只留下一窄条可行解。

## harn 将是什么

`harn` 旨在让 Harness 工程变得可及。手工搭好合适的环境——可被发现的知识、架构约束、反馈环、工作流产物——既繁琐又难以一贯；`harn` 想要改变这一点。

项目的最终形态刻意未定。指导原则已然清晰；产品形态将随之成形。

## 指导性文档

设计植根于正式规范与配套指南。二者共同确立人机协同系统的三重结构矛盾、应对它们的三条公理，以及渐进采纳的成熟度模型（Level 1–3）。

- [Harness 规范](docs/HARNESS-SPEC.zh-CN.md)——harness 是什么，其组成、义务与生命周期
- [Harness 指南](docs/HARNESS-GUIDE.zh-CN.md)——如何思考 harness 设计：理据、示例与决策框架

## 许可

MIT
