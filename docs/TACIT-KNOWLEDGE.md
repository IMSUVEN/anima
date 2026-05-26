# anima and the Tacit Knowledge Substrate

[English](TACIT-KNOWLEDGE.md) | [中文](TACIT-KNOWLEDGE.zh-CN.md)

## 1. The Problem

Many projects do not fail because they lack code, documents, or tools. They fail because the knowledge that actually determines success never enters the environment an agent can perceive.

This knowledge is usually not explicit API documentation. It is:

- Why one path was chosen instead of another.
- What past failure sits behind a constraint that looks strange today.
- Which judgment criteria live only in the experience of senior members.
- What is theoretically possible but wrong for this organization, business, dataset, or codebase.
- How the team actually trades off speed, quality, cost, safety, and maintainability when they conflict.

This is the project's **tacit knowledge**. Humans acquire it through long collaboration, oral transmission, and repeated mistakes. Agents that can only read the current task and local code will keep repeating the same errors.

anima does not try to replace human experience. It helps that experience sediment into the repository, where agents can read it, follow it, and improve it.

## 2. Why Tacit Knowledge Is Hard to Transfer

Tacit knowledge has three properties.

**It is often recognized only after the fact.** Teams rarely know all important rules on day one. The rules that matter usually come from incidents, rework, debates, and repeated choices.

**It is bound to context.** General best practices provide direction, but they cannot replace a project's history. A quantitative system, an embodied data production system, and an internal knowledge base may all use agents, but their real constraints and judgment standards differ sharply.

**It changes as the project evolves.** Old constraints may expire. New patterns may appear. A one-time manual quickly becomes burden.

So the answer is not just to write a handbook. A better answer is a continuous sedimentation mechanism: knowledge is discovered, recorded, validated, pruned, and reorganized through practice.

## 3. anima's Position

anima can be understood as a project's **tacit knowledge substrate**. It places the background, memory, decisions, and feedback loops that agents need into the repository, so each collaboration improves the environment for the next one.

This differs from a traditional template.

A template tries to provide the right structure up front. anima plants a seed: `AGENTS.md`, `docs/ARCHITECTURE.md`, and `docs/decisions/`. These files do not need to be perfect at the beginning. Their value is that they receive the knowledge that grows out of later practice.

When an agent discovers an important decision, it should record it in a decision document.

When an agent repeatedly hits a class of mistakes, it should suggest promoting the lesson into a test, linter, type constraint, or runtime check.

When an agent understands that the architecture has changed, it should update the architecture document instead of only finishing the immediate code change.

When old knowledge stops being true, it should be pruned rather than left to mislead future sessions.

## 4. From Prompt to Environment

Many agent adoption problems look like prompt problems. At the root, they are environment problems.

A prompt tells the agent what to do now. An environment tells the agent how this project thinks, judges, and avoids repeating history.

If tacit knowledge stays only in people's heads, every new session becomes a cold start. Humans must repeat context, and agents must rediscover old mistakes. anima changes the shape of the problem: the project itself becomes memory, and the repository carries part of the organization's learning capacity.

This is also central to harness engineering: not to trap the agent inside a more complex cage of rules, but to provide a richer, truer, more responsive working environment.

## 5. Where This Applies

anima is not limited to internal engineering projects. Any long-lived project with implicit judgment and continuous evolution can benefit from a tacit knowledge substrate.

### Software Engineering

A codebase's crucial knowledge is often not in function signatures. It lives in architecture tradeoffs, historical debt, edge cases, and team conventions. anima helps this knowledge enter the agent's working surface.

### Enterprise Agent Adoption

Enterprise workflows depend heavily on veteran experience: customer preferences, delivery boundaries, exception handling, quality standards, and compliance lines. Agents that cannot perceive this knowledge can only automate the surface. anima provides a way to sediment experience into an executable environment.

### Embodied Data Production

Embodied intelligence data production contains many judgments that are hard to write down all at once: what data is valuable, which simulation distributions are trustworthy, which labeling rules pollute training, and which anomalies should be preserved. anima helps teams move these judgments from people's heads into project memory.

### Quantitative Research

Quant systems learn from failed strategies, data source anomalies, backtest bias, and risk events. anima can turn those lessons into strategy lifecycle states, validation gates, risk constraints, and research logs.

### Personal Knowledge Bases

Personal knowledge management also has tacit knowledge: why a topic matters, how a note connects to long-running questions, and which materials have been validated or abandoned. anima's seed approach can help an agent and a knowledge base grow together.

## 6. Minimal Practice

A project does not need a complex system to start using anima for tacit knowledge. Four habits are enough:

1. **Record decisions as they happen.** If a choice will affect future maintenance, write it into `docs/decisions/`.
2. **Promote repeated errors into mechanisms.** If the same class of mistake recurs, turn it into a test, lint rule, type constraint, script, or check command.
3. **Update architecture when understanding changes.** When humans or agents learn a more accurate picture of the system, update `docs/ARCHITECTURE.md`.
4. **Prune obsolete knowledge.** More documentation is not always better. Wrong, stale, or useless knowledge should be deleted or rewritten.

These habits are not documentation hygiene for its own sake. Their purpose is to make the project more suitable for the next round of collaboration.

## 7. Signals of Success

A project is truly sedimenting tacit knowledge when:

- New agent sessions understand the project faster instead of asking for the same context repeatedly.
- Important decisions can be traced back to their original situation, not just their result.
- Common mistakes are increasingly blocked by mechanisms rather than human memory.
- Documents describe the project's real state instead of template aspirations.
- As the project develops, the agent's effective behavior increasingly resembles that of someone familiar with the project.

When these signals appear, anima is not merely generating files. It is helping the project form transferable memory.

## 8. Conclusion

Tacit knowledge is not an asset that can be extracted once. It is an environment that grows through action, feedback, and pruning.

anima's core value is to stop treating the agent as an external tool that must be retrained every session. Instead, the project gradually grows memory, identity, and judgment that agents can read.

That is the difference between a seed and a template: a template gives a project a starting point; a seed gives it the ability to grow its own experience.
