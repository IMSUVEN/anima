# 参与贡献

[English](CONTRIBUTING.md) | 中文

感谢你对 harn 的关注！本文档涵盖了人类贡献者和 AI 智能体贡献者需要了解的要点。

## AI 智能体

请先阅读 [AGENTS.md](AGENTS.md)——其中包含项目地图、构建命令、工作流与约束条件。（AGENTS.md 为英文，这是智能体工作的通用语言。）

## 人类贡献者

### 环境搭建

```bash
git clone https://github.com/imsuven/harn.git
cd harn
cargo build
cargo test
```

环境要求：Rust 1.70+（stable 工具链）。

### 开发流程

1. Fork 本仓库，从 `main` 创建功能分支。
2. 进行修改。请遵循 [AGENTS.md](AGENTS.md) 中的约束——它们同样适用于所有贡献者。
3. 提交前运行完整检查：

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

4. 使用清晰的提交信息。尽量遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范（如 `feat:`、`fix:`、`docs:`、`refactor:`、`test:`）。
5. 向 `main` 发起 Pull Request。

### 代码规范

- **零警告。** `cargo clippy -- -D warnings` 必须通过，无例外。
- **必须有测试。** 新功能需要测试，Bug 修复需要回归测试。
- **优先使用 Newtype。** 领域概念应使用 `src/types.rs` 中的类型包装器，而非裸字符串。
- **错误信息包含修复建议。** 每条面向用户的错误信息必须同时说明发生了什么以及如何处理。
- **遵守架构规则。** 依赖方向单向向下。详见 [ARCHITECTURE.md](ARCHITECTURE.md)。

### 可以做什么

- 查看 [Issues](https://github.com/imsuven/harn/issues) 中标有 `good first issue` 或 `help wanted` 的任务。
- 模板改进始终欢迎——将 `templates/` 输出与本仓库中手工编写的文档对比，作为质量基准。
- 减少歧义或提升智能体可用性的文档改进具有很高的价值。

### 报告问题

请提交 Issue 并包含：
- 你期望的行为
- 实际发生的行为
- 复现步骤
- `harn` 版本（`harn --version`）与操作系统

### 许可证

参与贡献即表示你同意你的贡献将以 [MIT 许可证](LICENSE) 发布。
