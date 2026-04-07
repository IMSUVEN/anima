# Harness Specification for Agentic Software Engineering

[English](HARNESS-SPEC.md) | [中文](HARNESS-SPEC.zh-CN.md)

> Three structural contradictions are inherent to human-agent systems. They are permanent — not bugs to be fixed or limitations to be outgrown — and any harness must manage all three. (The companion [Harness Guide](HARNESS-GUIDE.md), §1–2, provides the full analysis.)
>
> - *The intent-transfer contradiction:* human intent is ambiguous and contextual; agent execution is literal and context-bound. The gap is structural.
> - *The self-evaluation contradiction:* the same system that produces work cannot objectively evaluate that work. Self-serving bias is a property of self-evaluation itself.
> - *The entropy contradiction:* pattern replication is simultaneously the agent's greatest strength and its greatest vulnerability.
>
> The following axioms are principles for managing these contradictions.
>
> **Axiom 1. Humans steer, agents execute.**
> *Manages the intent-transfer and self-evaluation contradictions.* The scarce resource is human attention. Invest it in designing the environment, not writing code. This axiom governs the division of authority — humans define intent and boundaries; agents realize them within those boundaries.
>
> **Axiom 2. Start simple; earn complexity.**
> *Governs how aggressively the management apparatus is deployed.* Every harness component encodes an assumption about what the model cannot do. If the model can do it alone, the component is waste. Stress-test assumptions with each model generation. This axiom governs growth strategy — begin with one agent in a loop; add structure only when the current level demonstrably fails.
>
> **Axiom 3. Everything is a loop.**
> *Manages the entropy and self-evaluation contradictions.* Agent acts, environment responds, agent adjusts. Design the loop well and the agent converges; design it poorly and the agent drifts. When a failure recurs, resolve it structurally — add a rule, a test, a tool — so the loop self-corrects. This axiom governs the fundamental mechanism of all agentic work.

---

## 0. Preamble

### 0.1 What This Document Is

This is a **prescriptive specification** for building harnesses around coding agents. A harness is the environment, feedback loops, and control systems that shape how an agent works. The harness is not the model; it is everything around the model.

**Scope.** This spec applies to software engineering agents — agents that read, write, test, and deploy code. It covers both greenfield projects and incremental adoption on existing codebases. It is agent-platform-agnostic: the requirements apply whether you use an IDE-integrated agent, a self-hosted orchestrator, or a SaaS agent platform. Where a requirement is impractical for an existing codebase (e.g., full type coverage on a legacy untyped project), the spec notes a progressive adoption path. Non-software agents (data analysis, content generation, robotics) are out of scope.

This spec is organized by **maturity level**: Level 1 (single agent), Level 2 (multi-agent), Level 3 (clusters and fleets). Each level subsumes the previous. Start at Level 1; upgrade only when you hit a ceiling that the current level cannot address.

### 0.2 Obligation Language

This document uses RFC 2119 keywords:

- **MUST**: Mandatory. Violating this will cause predictable failure.
- **SHOULD**: Strongly recommended. Deviate only with explicit justification.
- **MAY**: Optional. Use when it adds value for your situation.

This specification contains two kinds of obligations:

- **Structural obligations** derive from the permanent contradictions of human-agent systems. They hold regardless of model capability and are not expected to change. Examples: "store knowledge in the repository" (§1.1), "validate input at system edges" (§1.2), "sandbox execution environments" (§1.7).
- **Calibrated obligations** encode current engineering judgment about specific thresholds. The underlying principle is durable, but the specific value reflects the state of the art and should be stress-tested with each model generation. Where a specific number appears (≤ 1 minute, ≤ 2 seconds, ~100 lines), the obligation is calibrated: challenge the threshold, not the principle.

### 0.3 Workflow Pattern Menu

Before adding agents, choose the simplest pattern that fits the task:

| Pattern | Use When | Level |
|---------|----------|-------|
| **Single agent + loop** | Most tasks; agent has good tools and a clear prompt | 1 |
| **Prompt chaining** | Task decomposes into fixed sequential subtasks | 1 |
| **Routing** | Distinct categories need different treatment (e.g., easy → fast model) | 1 |
| **Parallelization** | Independent subtasks; or vote N times for confidence | 1–2 |
| **Orchestrator-workers** | Can't predict subtasks in advance | 2 |
| **Evaluator-optimizer** | Clear evaluation criteria; iterative refinement adds measurable value | 2 |
| **Planner + Generator + Evaluator** | Complex multi-hour builds requiring planning, execution, and QA | 2–3 |

**The default is a single agent in a loop.** Escalate only when it demonstrably improves outcomes.

---

## 1. Level 1 — Single Agent + Loop

Level 1 is the foundation. A single agent operates in a single repository, performing one task per loop iteration. All higher levels build on this base.

**Adoption priority.** If adopting incrementally, the highest-leverage MUSTs to implement first are: repository knowledge (§1.1) — so the agent can orient itself; testing (§1.5) — so the feedback loop is fast and honest; and safety (§1.7) — so the agent can't cause damage. Once these are in place, add codebase legibility (§1.2), mechanical enforcement (§1.4), tool design (§1.3), and dev environment speed (§1.6) in whatever order addresses your most frequent failure mode.

### 1.1 Repository Knowledge

The agent can only reason about what it can see. Knowledge outside the repository effectively does not exist.

**MUST**: Maintain an `AGENTS.md` at the repository root as the agent's entry point. This file is a **map** (~100 lines), not an encyclopedia. It points to deeper sources of truth elsewhere in the repo. *Because: a monolithic instruction file crowds out task context and rots quickly; a map stays stable and lets the agent navigate to relevant detail on demand.*

**MUST**: Store all authoritative project knowledge — architecture, design decisions, conventions — in versioned files in the repository. *Because: conversations, chat threads, and wikis are invisible to agents. If it's not in the repo, it doesn't exist for the agent.*

**SHOULD**: Use nested `AGENTS.md` files in monorepos, one per subproject. The closest file to the edited path takes precedence. *Because: different subprojects have different build commands, conventions, and constraints; a single root file cannot capture this without bloating.*

**SHOULD**: Maintain an `ARCHITECTURE.md` that describes domain layering and key module boundaries. *Because: the agent needs a mental model of how the system is organized to make changes that fit the existing structure.*

### 1.2 Codebase Legibility

The agent navigates code via the filesystem, the type system, and tool interfaces. These surfaces MUST be designed for agent comprehension.

**MUST**: Use a language with **build-time type checking** — a language where types are verified by the compiler or a build-step type checker before execution (e.g., TypeScript, Go, Rust, Python with mypy/pyright in strict mode, Kotlin, C#). Achieve end-to-end type coverage across system boundaries (API clients, database schemas, inter-module contracts). For existing dynamically-typed codebases, adopt progressive typing: enable a strict type checker, enforce full type coverage on new and changed files, and expand coverage incrementally. *Because: types eliminate entire categories of illegal states at build time, shrinking the agent's search space. An untyped boundary forces the agent to guess data shapes, leading to silent failures.*

**MUST**: Validate external input into typed representations at system edges (parse, don't validate). Trust the type system within the boundary. *Because: this concentrates validation logic at a small number of well-tested points, rather than scattering defensive checks through the codebase where agents inevitably miss some.*

**SHOULD**: Prefer many small, well-scoped files over few large ones. *Because: agents truncate or summarize large files, losing context. A short file can be loaded in full.*

**SHOULD**: Use semantic directory paths (`billing/invoices/compute.ts`) and semantic type names (`UserId`, `SignedWebhookPayload`). *Because: the agent infers purpose from names. Generic names (`utils/helpers.ts`, `T`, `data`) create ambiguity that compounds.*

### 1.3 Tool Design (ACI)

The Agent-Computer Interface deserves as much design effort as a Human-Computer Interface.

**MUST**: Design tools to be hard to misuse (poka-yoke). Example: require absolute file paths, not relative — relative paths break when the agent changes directories. *Because: agents don't recover well from ambiguous or silent tool failures; the error compounds across subsequent steps.*

**SHOULD**: Write tool documentation as prompt engineering — include example usage, edge cases, input format requirements, and boundaries from other tools. *Because: the tool description is the only context the agent has for deciding how to use it.*

**SHOULD**: Keep tool input/output formats close to naturally-occurring text (markdown, plain code). Avoid formats with high cognitive overhead (diffs with chunk headers, JSON-escaped code). *Because: the model produces these formats more reliably when they resemble its training distribution.*

### 1.4 Mechanical Enforcement

**MUST**: Enforce architectural invariants via linters and structural tests, not documentation alone. *Because: agents replicate existing patterns, including violations. A documented rule that isn't mechanically enforced will be ignored within days.*

**MUST**: Write custom linter error messages that include **remediation instructions**. *Because: the error message is injected directly into the agent's context; a clear remediation turns a failure into self-correction.*

**SHOULD**: Enforce dependency direction between layers (e.g., `Types → Config → Repo → Service → Runtime → UI`). *Because: without directional constraints, agents create circular dependencies that make the codebase progressively harder to reason about.*

**SHOULD**: Encode subjective quality preferences ("taste") as mechanical rules where possible. When a preference is violated repeatedly, promote it from documentation into a linter rule. *Because: documentation is advisory; code is mandatory. Taste expressed as code applies uniformly to every line.*

### 1.5 Testing

**MUST**: Keep the full test suite executable in **≤ 1 minute**. *Because: the agent runs tests as feedback in its loop. A slow suite means the agent iterates without checking, accumulating errors between checks. Invest in test speed via concurrency, isolation, and third-party call caching.*

**SHOULD**: Target 100% code coverage for agent-generated codebases. *Because: at 100%, the coverage report is an unambiguous TODO list — no judgment required about what to test. Below 100%, agents must decide what's "important enough," introducing a decision surface where they consistently under-test. This is a phase change, not a marginal improvement. For incremental changes to large existing codebases, enforce coverage on changed lines instead.*

### 1.6 Dev Environment

**MUST**: Make the dev environment spinnable in **one command, ≤ 2 seconds**. *Because: agents (and agent orchestrators) create and destroy environments constantly. Manual setup steps become a bottleneck that prevents parallelization.*

**MUST**: Isolate environments so multiple instances run concurrently without conflict (ports, databases, caches, background jobs). *Because: even at Level 1, a developer often runs one agent while manually testing in another worktree. At Level 2+, concurrency is mandatory.*

**SHOULD**: Use git worktrees for environment isolation, with per-worktree observability (logs, metrics, traces) that is torn down on completion. *Because: worktrees provide filesystem isolation with shared git history, and ephemeral observability prevents stale data from polluting future runs.*

### 1.7 Safety Basics

**MUST**: Run agents in sandboxed environments with no access to production credentials or customer data. *Because: agents execute arbitrary code; an unsandboxed agent with production access is a security incident waiting to happen.*

**MUST**: Use allowlists for tool permissions (network hosts, shell commands, file paths). Deny by default. *Because: agents will use any tool available to them. Unrestricted access means an agent can accidentally call production APIs, delete files outside the repo, or exfiltrate data.*

**SHOULD**: Never commit secrets to the repository. Inject them via environment variables in the sandboxed worktree. *Because: agents read the entire repository; secrets in the repo become part of the agent's context and may leak into outputs.*

At Level 1, the human is assumed to be directly supervising. The agent's default behavior on encountering destructive or ambiguous operations SHOULD be to report and wait rather than proceed. (Level 2 formalizes this into an explicit escalation protocol — see §2.6.)

---

## 2. Level 2 — Multi-Agent + Planning

Upgrade to Level 2 when a single agent consistently fails to complete tasks within its context window, when output quality requires iterative evaluation that the agent cannot reliably self-perform, or when task scope requires upfront decomposition.

### 2.1 Agent Roles

Level 2 introduces three specialized roles. Each exists to address a specific failure mode of single-agent operation.

**Planner**: Takes a short (1–4 sentence) prompt and expands it into a full product spec with milestones. *Because: without a planner, agents under-scope — they start building immediately without thinking through the full feature set, leading to incoherent designs.*

- MUST focus on product context and high-level technical design, not granular implementation details. *Because: if the planner over-specifies implementation and gets something wrong, errors cascade into the build.*

**Generator (Builder)**: Implements features one at a time against the plan.

- MUST self-evaluate at the end of each unit of work, but MUST NOT be the final arbiter of quality. *Because: self-evaluation catches mechanical errors but exhibits self-serving bias for subjective quality and completeness.*

**Evaluator (QA)**: Structurally separated from the generator. Tests the live application and grades output against explicit criteria.

- MUST be a **different agent session** from the generator. *Because: LLMs exhibit self-serving bias when evaluating their own output — they skew positive and rationalize flaws. A structurally separate evaluator is far more tractable to calibrate toward skepticism, for the same reason that code review works better than self-review.*
- MUST exercise the **running artifact** — not just read the source code. For web applications, this means browser automation and API testing. For CLI tools, this means running commands and verifying outputs. For libraries, this means executing integration tests against the public API. *Because: code that compiles and looks correct can still be non-functional. Only live interaction reveals integration failures, broken workflows, and behavioral bugs.*
- SHOULD be calibrated via few-shot examples with detailed score breakdowns. *Because: without calibration, evaluators drift toward leniency over time.*

**Agent-to-Agent Review Loop**: After the generator completes a unit of work, a separate agent session (typically the evaluator or another generator instance) reviews the code before merge. The flow is: Generator builds → generator self-reviews → separate agent reviews → iterate on feedback → merge. Human review is reserved for genuine judgment calls. *Because: agent review throughput far exceeds human review capacity. Reserving humans for judgment keeps them focused on the highest-leverage decisions.*

### 2.2 Execution Plans (ExecPlans)

ExecPlans are the primary coordination artifact at Level 2. They replace informal task descriptions with structured, durable, self-contained documents.

**MUST**: Make every ExecPlan fully self-contained. A novice with only the plan and the working tree must be able to succeed. *Because: context resets (§2.4) destroy all session state. The ExecPlan is the only thing that survives. If it references external docs that aren't in the new session's context, the plan breaks.*

**MUST**: Treat ExecPlans as living documents. Update Progress, Decision Log, and Surprises sections as work proceeds. *Because: the plan is consumed by future agent sessions that have no other context. Stale plans cause agents to repeat work or contradict completed decisions.*

**MUST**: Phrase acceptance criteria as **observable behavior** ("navigating to `/health` returns HTTP 200"), not code structure ("added a HealthCheck struct"). *Because: agents optimize for the literal acceptance criterion. If the criterion is structural, the agent will produce the structure without ensuring it works.*

**MUST**: Design steps to be idempotent — re-runnable without causing damage. *Because: agent sessions crash mid-step. The next session will re-execute the step; if it's not idempotent, it produces duplicates or corruption.*

See [Appendix A](#appendix-a-templates) for the full ExecPlan template.

### 2.3 Sprint Contracts

Before each unit of work, the generator and evaluator SHOULD **negotiate a sprint contract**: an explicit agreement on what "done" looks like, including testable acceptance criteria and verification method.

*Because: the product spec is intentionally high-level. Sprint contracts bridge the gap between user stories and testable implementation, preventing the generator from building the wrong thing or the evaluator from grading against unstated expectations.*

See [Appendix A](#appendix-a-templates) for the Sprint Contract template.

### 2.4 Context Strategy

**SHOULD**: Use context resets (clear window, spawn fresh agent with structured handoff) over compaction when the model begins degrading — wrapping up prematurely, losing track of the plan, or declining in quality. *Because: compaction preserves continuity but doesn't give the agent a clean slate. Models that exhibit "context anxiety" continue to exhibit it after compaction.*

**MUST**: When performing context resets, the outgoing agent MUST produce a structured handoff artifact containing: completed work, current state (app status, passing tests, known issues), ordered next steps, and key context the successor needs. *Because: without a handoff, the successor agent starts from zero, repeating completed work and contradicting prior decisions.*

See [Appendix A](#appendix-a-templates) for the Handoff Artifact template.

### 2.5 Quality Criteria

**MUST**: Define explicit grading criteria and give the same criteria to both generator and evaluator. Criteria MAY be defined in a sprint contract (§2.3), in the ExecPlan's milestone definitions, or in a standalone document — the mechanism is flexible, but the criteria themselves MUST exist and MUST be shared. *Because: the generator needs to know the target; the evaluator needs to know the scorecard. Misaligned criteria produce work that passes generation but fails evaluation, wasting iteration cycles.*

**MUST**: Set hard pass/fail thresholds per criterion. If any criterion fails, the sprint fails and the generator receives detailed, actionable feedback. *Because: soft thresholds let evaluators rationalize marginal passes. Hard thresholds force the generator to actually fix the issue.*

**Iterative refinement**: Generator builds → Evaluator tests live app → scores + critique → iterate (1–5 rounds). If the approach is fundamentally flawed, the generator SHOULD pivot entirely rather than incrementally polishing a broken direction.

Recommended criteria for full-stack applications:

| Criterion | Weight | What Failure Looks Like |
|-----------|--------|------------------------|
| **Product Depth** | High | Features are display-only stubs that don't actually work |
| **Functionality** | High | Bugs when exercised as a real user; broken workflows |
| **Visual Design** | Medium | Inconsistent identity, broken layout, non-responsive |
| **Code Quality** | Medium | No tests, no error handling, architectural violations |

### 2.6 Safety at Level 2

**MUST**: Define a human escalation protocol — explicit triggers for when the agent stops and asks:
- Destructive operations (data-dropping migrations, force-pushes, production deployments)
- Ambiguous or contradictory requirements the agent cannot resolve from the plan
- Repeated failure (3+ attempts at the same step without progress)
- Security-sensitive changes (auth, payments, access control)
- Cost or duration ceiling exceeded

*Because: without explicit stop-conditions, agents either stall silently or improvise on high-stakes operations. Both are worse than asking.*

The agent MUST report: what it attempted, what blocked it, and what it recommends. Not just "I'm stuck."

---

## 3. Level 3 — Clusters & Fleets

Upgrade to Level 3 when the work backlog consistently exceeds what a single agent pipeline (Planner → Generator → Evaluator) can process, and you need parallel execution across multiple agents.

### 3.1 Workflow Durability

Level 2 introduced durability primitives: ExecPlans (§2.2), handoff artifacts (§2.4), and idempotent steps. At Level 3, these primitives are *industrialized*. Sessions routinely expire, multiple agents work concurrently on different steps of the same plan, and any agent must be able to resume any workflow at any point. This requires **nondeterministic idempotence**: the path through a workflow is nondeterministic (agents decide how), but the outcome is deterministic (acceptance criteria are met), because the work state is durable.

**MUST**: Store all work state outside the agent session — in git (ExecPlans, sprint contracts, handoffs) or in an issue tracker. *Because: agent sessions are ephemeral by nature. If the work state is only in the session, a crash or context exhaustion loses all progress.*

**MUST**: Give each workflow step explicit completion criteria. The agent claims a step, executes it, and marks it complete. Progress is always visible in the plan's Progress section. *Because: without explicit completion markers, the next agent cannot determine where to resume, and work is repeated or skipped.*

**MUST**: Make step claiming atomic — only one agent works on a given step at a time. *Because: without exclusive claims, two agents may work on the same step concurrently, producing conflicting changes.*

**Sessions are cattle; work artifacts are persistent.** An agent session is interchangeable — any agent can pick up any in-progress workflow by reading the durable state.

### 3.2 Scaling Architecture

- **Clusters** (5–30 concurrent agents): Multiple agents work in parallel from the same backlog. MUST have a supervisor/witness agent monitoring worker health. *Because: unsupervised agents get stuck, silently stall, or loop; a supervisor detects and restarts them.*
- **Fleets** (100+ agents): Supervisory agents manage pods of coding agents, escalating to humans only when genuinely stuck.

### 3.3 Merge Queue

**MUST**: Operate a dedicated merge agent (or "refinery") that processes the merge queue sequentially, one PR at a time. *Because: when multiple agents work in parallel, the baseline changes dramatically during a swarm. Naive concurrent merging produces constant conflicts and broken builds.*

**MUST**: The merge agent MUST understand both sides of a conflict semantically, not blindly rebase. *Because: late-arriving agents may need to reimagine their changes against a substantially different HEAD. Mechanical rebase loses intent.*

**SHOULD**: Track parallel agent output in convoy-style work orders that show what's in flight, what's merged, and what's stuck. *Because: at fleet scale, visibility into delivery status is essential for the human overseer to prioritize and unblock.*

### 3.4 Advanced Safety

**MUST**: Every deployment produced by the harness MUST be rollback-capable (blue-green, canary, or feature-flagged). *Because: at fleet scale, the volume of changes makes manual verification impossible. Automated rollback is the safety net.*

**SHOULD**: Run automated smoke tests immediately after deployment. On failure, roll back automatically or escalate. *Because: the latency between deploy and detection determines blast radius.*

**SHOULD**: Tag each merge with a revert-ready commit. *Because: a single-command revert is the fastest possible recovery path.*

---

## 4. Cross-Cutting Concerns

These concerns apply at all maturity levels. The *principle* is universal; the *implementation* scales with your level.

### 4.1 Entropy Management

Agent-generated code replicates existing patterns — including suboptimal ones. Without active maintenance, the codebase drifts toward incoherence.

**MUST**: Actively manage entropy — detect pattern deviations and correct them before they compound. *Because: technical debt compounds. Small, continuous corrections are far cheaper than periodic large cleanups.*

- *At Level 1*: Review agent output for pattern drift during normal human supervision. When a deviation recurs, promote the fix to a linter rule (§1.4).
- *At Level 2+*: Run background agents on a regular cadence to scan for deviations from golden principles, update quality grades per domain, and open targeted refactoring PRs.

**SHOULD**: Track known debt in a `tech-debt-tracker.md`; maintain quality grades in a `QUALITY_SCORE.md`. *Because: without measurement, drift is invisible until it causes a crisis.*

### 4.2 Knowledge Hygiene

**MUST**: Keep documentation current with the codebase. Outdated docs actively mislead agents, which is worse than no docs at all. *Because: documentation rots faster than code. An agent following stale instructions produces confidently wrong output.*

- *At Level 1*: Review docs as part of related code changes. When you change behavior, update the corresponding doc in the same commit.
- *At Level 2+*: Run a recurring doc-gardening agent to scan for stale documentation and open fix-up PRs.

**SHOULD**: When a team conversation produces an architectural decision, capture it as a repository artifact. *Because: decisions made in chat are invisible to agents and to future team members. If it's not in the repo, it will be contradicted.*

### 4.3 Harness Evolution

Every harness component encodes an assumption about model limitations. When a new model ships:

1. Remove components one at a time.
2. Measure impact on output quality.
3. Keep only what is load-bearing.
4. Add new components to exploit new capabilities.

Two categories of harness components have different lifecycles:

**Scaffolding** — compensates for current model weaknesses. Drop when the model improves:

| Component | Drop When |
|-----------|-----------|
| Sprint decomposition | Model sustains coherence across full builds |
| Context resets | Compaction alone suffices |
| Separate evaluator | Model self-evaluation becomes reliable |

**Engineering fundamentals** — valuable regardless of model capability. Keep permanently:

| Component | Why It's Permanent |
|-----------|--------------------|
| Mechanical enforcement (linters, structural tests) | Entropy is inherent in any system, not a model limitation |
| 100% test coverage | The constraint eliminates decision burden, regardless of how good the model is at testing |
| ACI poka-yoke | Ambiguous interfaces cause errors for any agent, including superhuman ones |
| Type-driven boundaries | Types are documentation and compiler-enforced correctness simultaneously |

The space of interesting harness combinations doesn't shrink as models improve. Instead, it moves.

### 4.4 Technology Selection

**SHOULD**: Favor "boring" technologies — composable, API-stable, well-represented in training data. *Because: agents model familiar technologies more reliably. Exotic dependencies with thin documentation or unstable APIs increase the error rate per task.*

**MAY**: Reimplement a needed subset of an opaque dependency, tightly integrated and fully tested. *Because: a legible, tested in-repo implementation is more maintainable by agents than an opaque upstream library.*

---

## Appendix A: Templates

### A.1 ExecPlan Template

```markdown
# ExecPlan: <Short, Action-Oriented Title>

Living document. Update Progress, Surprises, Decision Log, and Retrospective
as work proceeds. This plan MUST be self-contained — a reader with only this
file and the working tree must be able to succeed.

## Purpose
What someone gains after this change. How they can see it working.

## Context and Orientation
Current state as if the reader knows nothing. Key files by full path.
Define non-obvious terms.

## Milestones
Each: scope, what exists at the end, commands to run, observable acceptance.
Include prototyping milestones to de-risk unknowns.

### Milestone 1: <Name>
...

## Validation and Acceptance
How to exercise the system. Observable behavior with specific inputs/outputs.
Exact test commands and expected results.

## Progress
- [x] (YYYY-MM-DD HH:MMZ) Completed step.
- [ ] Incomplete step.

## Surprises & Discoveries
- Observation: ...
  Evidence: ...

## Decision Log
- Decision: ...
  Rationale: ...
  Date: ...

## Outcomes & Retrospective
Outcomes, gaps, lessons learned.

## Interfaces and Dependencies
Libraries, types, function signatures that must exist post-implementation.
```

### A.2 Sprint Contract Template

```markdown
# Sprint Contract: <Sprint Name>

## Deliverables
1. ...

## Acceptance Criteria (observable behavior)
- [ ] Criterion 1
- [ ] Criterion 2

## Verification Method
How the evaluator will test each criterion.
```

### A.3 Handoff Artifact Template

```markdown
# Handoff: Session N → Session N+1

## Completed Work
- ...

## Current State
- App status: running / broken / partial
- Passing tests: ...
- Known issues: ...

## Next Steps (ordered)
1. ...

## Key Context
Decisions, constraints, or patterns the successor must know.
```

## Appendix B: Anti-Patterns

| Anti-Pattern | Failure Mode | Fix |
|-------------|-------------|-----|
| Giant monolithic AGENTS.md | Crowds out task context; rots within weeks | Map (~100 lines) + structured `docs/` |
| Agent self-evaluation as sole QA | Self-serving bias; agents praise their own work | Structurally separate evaluator (Level 2) |
| Over-specified implementation plans | Planner errors cascade into the build | High-level spec + negotiated sprint contracts |
| Work state only in agent context | Session crash = all progress lost | Durable artifacts in git |
| Premature multi-agent | Nondeterminism compounds across agent boundaries | Single agent loop first; scale when provably needed |
| Large files + generic names | Agent truncates content; can't infer purpose from path | Small files, semantic paths and type names |
| Untyped system boundaries | Agent guesses data shapes; silent failures | Parse at boundary; end-to-end types |
| Slow test suite (>1 min) | Agent doesn't run checks often enough; errors accumulate | Target ≤1 min via concurrency and caching (§1.5) |
| No human escalation triggers | Agent improvises on destructive or ambiguous operations | Define explicit stop-and-ask conditions |
| Static harness design | Assumptions about model limits go stale | Re-evaluate components with each model generation |

## Appendix C: Benchmarks

*Snapshots circa early 2026. Use for order-of-magnitude planning, not as current pricing or performance guarantees. Model versions and costs change frequently.*

**Anthropic (Opus 4.5 → Opus 4.6):**

| Configuration | Duration | Cost |
|--------------|----------|------|
| Solo agent, no harness | 20 min | $9 |
| Full harness (3-agent, sprints) | 6 hr | $200 |
| Simplified harness (no sprints) | 4 hr | $125 |

**OpenAI (GPT-5 + Codex, 5-month internal product):**

| Metric | Value |
|--------|-------|
| Codebase | ~1M lines, 0 manually written |
| PRs | ~1,500 over 5 months |
| Throughput | 3.5 PRs/engineer/day |
| Single-run duration | up to 6 hours |

**Community (Logic.inc, Gas Town):**

| Metric | Value |
|--------|-------|
| Agent burn rate | ~$10–12/hr per agent |
| Cluster size | 5–30 concurrent per developer |
| Test suite (Logic.inc) | 10k+ assertions in ~1 min |

## Appendix D: Sources

**Primary**

| Source | Key Contribution |
|--------|-----------------|
| [OpenAI — Harness Engineering](https://openai.com/index/harness-engineering/) | Repo as system of record; progressive disclosure; architectural enforcement; entropy management |
| [Anthropic — Harness Design for Long-Running Apps](https://www.anthropic.com/engineering/harness-design-long-running-apps) | Generator/evaluator separation; context resets; grading criteria; simplification principle |
| [Anthropic — Building Effective Agents](https://www.anthropic.com/research/building-effective-agents) | Five workflow patterns; ACI design; poka-yoke; simplicity-first |

**Community**

| Source | Key Contribution |
|--------|-----------------|
| [Geoffrey Huntley — Everything Is a Ralph Loop](https://ghuntley.com/loop/) | Loop primitive; monolithic-first |
| [OpenAI Cookbook — Using PLANS.md](https://cookbook.openai.com/articles/codex_exec_plans) | Self-contained ExecPlan format; living documents; observable outcomes |
| [AGENTS.md (Linux Foundation)](https://agents.md/) | Universal agent instruction standard; nested files; cross-tool compatibility |
| [Logic.inc — AI Is Forcing Us To Write Good Code](https://bits.logic.inc/p/ai-is-forcing-us-to-write-good-code) | 100% coverage; filesystem as interface; fast/ephemeral/concurrent envs; end-to-end types |
| [Steve Yegge — Welcome to Gas Town](https://steve-yegge.medium.com/welcome-to-gas-town-4f25ee16dd04) | Orchestration at scale; merge queue; nondeterministic idempotence |
