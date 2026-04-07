# Harness Engineering Guide

[English](HARNESS-GUIDE.md) | [中文](HARNESS-GUIDE.zh-CN.md)

A companion to [HARNESS-SPEC.md](HARNESS-SPEC.md). The spec tells you **what** to do and **why** in one sentence. This guide teaches you **how to think** about harness design so you can handle situations the spec doesn't cover.

Read the spec first. Come here when you need to understand the reasoning behind a rule, see it applied to a concrete situation, or make a judgment call at a decision point.

---

## 1. Why Harness Engineering Exists

A coding agent without a harness is like a power tool without a workbench. The tool is capable, but without a stable surface, clamps, guards, and a clear workspace, its power works against you.

The naive approach — "give the agent a task and let it go" — works for small, well-defined problems. It breaks down because of three contradictions inherent to human-agent systems. These are not bugs in a specific model or failure modes to be outgrown — they are permanent tensions that must be continuously managed. A harness that treats them as problems to solve will be perpetually surprised when they persist. A harness that treats them as contradictions to manage will be designed for durability.

**The intent-transfer contradiction.** Human intent is inherently ambiguous and contextual; agent execution is inherently literal and context-bound. The gap between what the human means and what the agent was told cannot be closed by better prompting alone — it is structural. This manifests as **drift**: the agent loses coherence over long sessions, forgets earlier decisions, wraps up prematurely ("context anxiety"), or produces work that contradicts itself. The output looks busy but doesn't add up.

**The self-evaluation contradiction.** The same system that produces work cannot objectively evaluate that work. This isn't a bug in a specific model — it's a structural property of self-evaluation. The same session that made a trade-off will rationalize that trade-off when asked to review it, for the same reason humans are poor judges of their own writing. This manifests as **self-delusion**: confident wrongness that compounds because the feedback signal is dishonest at its source.

**The entropy contradiction.** Pattern replication is simultaneously the agent's greatest strength and its greatest vulnerability. Agents produce code by imitating existing patterns — fast and consistent when patterns are good, a vector for decay when they're bad. A minor shortcut in one file becomes a codebase-wide convention within days. Without active correction, quality degrades monotonically. The contradiction is not that the agent copies badly — it's that the very mechanism that makes agents productive is also the mechanism that degrades the codebase.

| Contradiction | Tension | Corresponding Axiom |
|--------------|---------|---------------------|
| **Intent transfer** | Ambiguity of human intent vs. literalness of agent execution | Axiom 1 (Governance) |
| **Self-evaluation** | Generation and judgment fused in one system | Axiom 1 + Axiom 3 |
| **Entropy** | Pattern replication as productive force vs. decay vector | Axiom 3 (Loop) |

These contradictions cannot be resolved — only managed. The harness is the management apparatus. It provides structure that holds the intent-transfer contradiction in check (execution plans, context resets), separation that holds the self-evaluation contradiction in check (independent evaluator), and mechanical enforcement that holds the entropy contradiction in check (linters, tests, automated cleanup). None of these require the model to be smarter. They require the environment to be designed around the reality that these tensions are permanent.

This framing has a practical consequence: it tells you which harness components are permanent and which are temporary. Components that manage a fundamental contradiction — linters against entropy, separate evaluators against self-delusion, structured plans against drift — are engineering fundamentals. They stay. Components that compensate for a current model weakness — sprint decomposition because context windows are too short, context resets because compaction doesn't work well enough — are scaffolding. They go when the model improves. This is exactly the distinction the spec draws in §4.3.

These contradictions are what make the three axioms necessary. The next section unpacks those axioms and shows how every harness design decision traces back to them.

---

## 2. The Three Axioms

The spec opens with three axioms, each derived from the structural contradictions identified in §1. They are not arbitrary starting points — they are answers to three questions that any agent engineering system must confront.

| Question | Axiom | Domain |
|----------|-------|--------|
| **Who decides?** | Humans steer, agents execute | Governance |
| **How do you grow?** | Start simple; earn complexity | Strategy |
| **How does work happen?** | Everything is a loop | Mechanism |

These three are necessary: skip any one and you have a structural blind spot. Skip governance and you get autonomous agents making high-stakes decisions without oversight. Skip strategy and you get either premature complexity or permanent over-simplification. Skip mechanism and you get agents that act without feedback, drifting undetected.

They are also sufficient: once you've answered who decides, how you grow, and how work happens, every specific harness rule in the spec follows as a derivation. The MUSTs, SHOULDs, and MAYs are not a list of independent best practices — they are consequences of these three principles.

This isn't a framework we invented and then went looking for evidence. Between late 2024 and early 2026, several independent teams — OpenAI, Anthropic, Geoffrey Huntley, Steve Krenzel at Logic.inc, Steve Yegge — arrived at remarkably similar conclusions without coordinating. They converged because they were forced to answer the same three structural questions, and the problem domain admits only a narrow band of workable answers. Section 7 traces the full history.

### 2.1 Axiom 1: Humans Steer, Agents Execute

**The question:** Who holds authority in an agent engineering system?

The answer seems obvious until you watch it break down. The failure mode isn't a dramatic robot takeover — it's a gradual erosion of human intent. The agent makes a reasonable-seeming judgment call. Then another. Then another. By the time a human reviews the output, the accumulated decisions have drifted far from the original intent, and unwinding them is more expensive than starting over.

The axiom draws a bright line: humans define intent, boundaries, and quality criteria. Agents realize that intent within those boundaries. The harness is the mechanism that enforces this separation — not by limiting what the agent can do, but by structuring how human intent flows into the agent's environment.

This means investing human attention at the leverage points: designing the AGENTS.md that orients the agent (Spec §1.1), defining the safety boundaries that constrain it (Spec §1.7), building the escalation protocol that pulls it back when it's stuck or about to do damage (Spec §2.6), and calibrating the evaluator that grades its output (Spec §2.5). These aren't overhead — they're the steering mechanism.

**What violation looks like:**

- **Confident wrongness.** The agent says the feature works; it doesn't. Root cause: no structurally independent evaluation — the agent evaluated its own work and, predictably, approved it. Fix: separate evaluator that exercises the live artifact (Spec §2.1).
- **Scope explosion.** The agent adds features nobody asked for. Root cause: human intent was vague — the prompt or plan didn't define boundaries. Fix: ExecPlan with bounded milestones and explicit acceptance criteria (Spec §2.2).
- **Decision paralysis.** The agent asks for help on something it could decide. Root cause: the steering is contradictory or over-constrained — conflicting instructions in AGENTS.md, or rules that leave no room for legitimate agent judgment. Fix: simplify the instructions; resolve contradictions.

### 2.2 Axiom 2: Start Simple; Earn Complexity

**The question:** How should the harness evolve over time?

The temptation is to build the full harness upfront — planner, generator, evaluator, merge queue, fleet supervisor. But every harness component encodes an assumption about what the model cannot do. If the assumption is wrong — if the model can handle the task without the component — then the component is pure overhead: latency, cost, coordination bugs, and orchestration surface area that obscures rather than helps.

The axiom demands that you start with the simplest viable harness (a single agent in a loop) and add complexity only when the current level demonstrably fails. "Demonstrably" is the key word. Not "might fail," not "feels like it could be better" — actually fails, in a way you can point to.

This principle operates at two levels:

**Level progression.** The spec's L1 → L2 → L3 structure (Spec §1, §2, §3) is a direct expression of this axiom. Each level has explicit upgrade triggers — you don't move to Level 2 until a single agent consistently runs out of context, or output quality requires evaluation the agent can't self-perform. You don't move to Level 3 until the work backlog consistently exceeds pipeline capacity.

**Component lifecycle.** Within any level, the harness contains two types of components. **Scaffolding** compensates for things the model can't do *yet* — sprint decomposition, context resets, separate evaluators. These are temporary by nature. When a model improves enough to handle the task natively, the scaffolding becomes overhead and should be removed. **Fundamentals** are valuable regardless of model capability — linters, types, fast tests. Even a hypothetically perfect model would produce better results in an environment with strong fundamentals, because entropy is inherent in any copy-and-modify system, not a model limitation.

The practical test: "Would a brilliant human engineer also benefit from this constraint?" If yes, it's a fundamental. If no, it's scaffolding. After each model generation, run a simplification sprint (§6.2): disable each scaffolding component, measure impact, keep only what's load-bearing.

**What violation looks like:**

- **Premature multi-agent.** Team jumps to a planner-generator-evaluator pipeline when a single agent in a loop would suffice. Nondeterminism compounds across agent boundaries; orchestration bugs consume more time than the agents save. Fix: single agent loop first; scale only when provably needed (Spec Appendix B).
- **Static harness design.** Team builds the harness once and never revisits it. Scaffolding that was necessary two model generations ago is still running, adding latency and cost without improving output. Fix: simplification sprint after each model upgrade (Spec §4.3).
- **Over-specified plans.** Planner produces granular implementation instructions. When it gets a detail wrong, the error cascades into the build because the generator follows instructions literally. Fix: high-level spec + negotiated sprint contracts (Spec §2.2, §2.3).

### 2.3 Axiom 3: Everything Is a Loop

**The question:** What is the fundamental mechanism by which agentic work produces good output?

Not talent. Not scale. Not prompting cleverness. The answer is feedback. Every agentic workflow is a loop: the agent acts, the environment responds, and the agent adjusts. This is true at every scale — a single test-fix cycle, a multi-sprint build, an overnight fleet run. The quality of the loop determines the quality of the output.

A good loop has three properties:

- **Fast feedback.** The agent learns quickly whether its last action worked. A 1-minute test suite lets the agent check its work after every change. A 20-minute suite means it batches changes and hopes for the best — errors compound unchecked. This is why the spec requires tests in ≤1 minute (Spec §1.5) and dev environments in ≤2 seconds (Spec §1.6) — not as arbitrary thresholds, but as conditions for the loop to function.
- **Honest feedback.** The signal must reflect reality. A linter that ignores violations, a test suite with holes, or a coverage report that excludes files — these create false confidence. The agent thinks it's done when it's not. This is why the spec pushes toward 100% coverage (Spec §1.5) and end-to-end types (Spec §1.2) — not as perfectionism, but as conditions for the signal to be trustworthy.
- **Structural correction.** When the same failure recurs, you don't just fix it — you change the environment so it can't happen again. Add a linter rule. Add a test. Change a tool's interface. Each fix makes the loop smarter, permanently. This is the mechanism behind entropy management (Spec §4.1) — not periodic cleanup, but continuous structural improvement.

Think of the loop as a learning system. Your job is not to fix each output. Your job is to improve the feedback mechanism so the agent fixes its own output.

**What violation looks like:**

- **Premature completion.** Agent declares "done" with half the work finished. Root cause: the loop lacks explicit completion criteria — the agent's "context anxiety" isn't corrected by structural signals. Fix: ExecPlan with observable milestones; context reset when coherence degrades (Spec §2.2, §2.4).
- **Pattern replication.** New code copies a bad pattern from elsewhere in the codebase. Root cause: the loop has no structural correction for this class of error — no linter rule, no golden-principle enforcement. The agent imitates what it sees. Fix: promote the fix to a linter rule; run entropy-detection agents (Spec §1.4, §4.1).
- **Silent tool failure.** Agent proceeds despite a tool error because the tool returned ambiguous output. Root cause: the feedback was dishonest — the tool didn't fail loudly enough. Fix: poka-yoke — redesign the tool to fail clearly and unambiguously (Spec §1.3).
- **Test avoidance.** Agent writes code but skips or skims tests. Root cause: the loop either doesn't enforce coverage or the suite is too slow to run frequently. Fix: 100% coverage target with a fast suite (Spec §1.5).

### 2.4 Reading Failures Through the Axioms

The sections above show characteristic failures for each axiom. In practice, most failures involve more than one. The table below maps every common failure pattern to the axiom(s) it violates — serving as both a diagnostic tool and a demonstration that the three axioms have complete coverage.

| Failure Pattern | What You See | Axiom Violated | Root Cause | Harness Fix |
|----------------|-------------|----------------|------------|-------------|
| **Premature completion** | Agent declares "done" at 50% progress | 3 (Loop) | No completion criteria in feedback | Context reset; ExecPlan with observable milestones |
| **Confident wrongness** | Agent says the feature works; it doesn't | 1 (Governance) | No independent evaluation | Separate evaluator; live-app testing |
| **Pattern replication** | New code copies a bad pattern | 3 (Loop) | No structural correction | Linter rule; golden principle; targeted refactor |
| **Silent tool failure** | Agent proceeds despite a tool error | 3 (Loop) | Dishonest feedback from tool | Poka-yoke: redesign tool to fail loudly |
| **Scope explosion** | Agent adds features nobody asked for | 1 (Governance) + 2 (Strategy) | Vague intent; unbounded scope | Bounded milestones; sprint contract |
| **Boundary corruption** | API sends wrong data shape; DB insert fails | 3 (Loop) | Feedback gap at system edge | Parse at boundary; end-to-end types |
| **Test avoidance** | Agent writes code but skips tests | 3 (Loop) | No coverage enforcement; slow suite | 100% coverage target; fast suite |
| **Decision paralysis** | Agent asks for help on trivial decisions | 1 (Governance) | Contradictory or over-constrained steering | Simplify AGENTS.md; resolve contradictions |
| **Premature multi-agent** | Multi-agent pipeline where single agent suffices | 2 (Strategy) | Complexity not earned | Single agent loop first; scale when provably needed |
| **Static harness** | Harness unchanged across model generations | 2 (Strategy) | Assumptions go stale | Simplification sprint after each model upgrade |

When something goes wrong, don't just fix the output. Identify which axiom was violated, find the structural gap in the harness, and close it. Each fix makes the loop smarter, the steering clearer, or the strategy more adaptive — permanently.

### 2.5 When Axioms Conflict

The three axioms are complementary by design, but in specific decisions they can pull in opposing directions. When they do, **prefer the axiom whose violation produces the least reversible damage.**

Axiom 1 (Governance) violations — the agent making high-stakes decisions without human oversight — produce damage that compounds silently and is expensive to unwind. By the time you notice, accumulated decisions have drifted far from intent.

Axiom 3 (Loop) violations — broken or dishonest feedback — produce damage that accumulates but is detectable once you look. A missing test or a silent tool failure causes real harm, but the harm becomes visible in outputs once you inspect them.

Axiom 2 (Strategy) violations — premature complexity — produce overhead that is visible from the start and removable without lasting damage. An unnecessary harness component adds cost and latency, but you can always take it out.

This gives a natural precedence when axioms conflict: **Governance > Loop > Strategy**, ordered by irreversibility of harm.

**In practice:**

- *Adding human review (Axiom 1) increases complexity (Axiom 2).* If the decision domain is high-stakes — security, data integrity, production deployment — Axiom 1 wins: add the review step. If the domain is low-stakes and the agent has a track record, Axiom 2 wins: skip the overhead.

- *Adding a feedback mechanism (Axiom 3) increases complexity (Axiom 2).* If a feedback gap is causing recurring failures, Axiom 3 wins: the structural correction is worth the added structure. If the improvement is speculative, Axiom 2 wins: wait until the gap demonstrably causes harm.

- *Simplifying the harness (Axiom 2) might reduce feedback quality (Axiom 3).* Measure. Disable the component, run a representative task, compare output. If quality holds, Axiom 2 wins — remove the component. If quality drops, Axiom 3 wins — keep it.

The precedence is not a rigid hierarchy — it is a tiebreaker for genuine conflicts, applied with judgment about the specific stakes involved. Most of the time, the axioms align: a well-designed feedback loop (Axiom 3) is also the simplest intervention that works (Axiom 2), and it supports human oversight by making the agent's state legible (Axiom 1).

---

## 3. Getting Started: Level 1

Level 1 is where everyone starts and where most of the leverage lives. A well-built Level 1 harness often eliminates the need for Level 2 entirely.

**A note on ordering.** The sections below follow a narrative arc — from setting up the repository to running your first loop. This is not an adoption priority order. If you're adopting incrementally, the spec (§1, "Adoption priority") recommends starting with: repository knowledge (§3.1), then fast tests (the testing half of §3.4, Spec §1.5), then safety (§3.5) — because these close the feedback loop and prevent damage. Add codebase legibility (§3.2), tool design (§3.3), mechanical enforcement (Spec §1.4), and dev environment speed (the environment half of §3.4, Spec §1.6) afterward, in whatever order addresses your most frequent failure mode.

### 3.1 Setting Up Your Repository

> Spec reference: §1.1 Repository Knowledge

Start with three files:

**`AGENTS.md`** — Write it as if onboarding a capable but context-free contractor. They know how to code but know nothing about your project. What do they need to find?

A good AGENTS.md for a typical project:

```markdown
# AGENTS.md

## Project
Invoice processing service. TypeScript + Fastify + PostgreSQL.

## Quick Start
pnpm install && pnpm dev     # starts on port 3000
pnpm test                     # full suite, ~45 seconds

## Architecture
See ARCHITECTURE.md for domain map and layer rules.

## Conventions
- Strict TypeScript; no `any`
- All API endpoints defined in OpenAPI (see openapi.yaml)
- DB types generated by Kysely; never write raw SQL types
- Parse external input at the boundary (see docs/boundaries.md)
- Max file size: 300 lines

## Key Directories
src/domains/         # business logic, one dir per domain
src/infrastructure/  # DB, HTTP, messaging adapters
src/api/             # route handlers (thin; delegate to domains)
tests/               # mirrors src/ structure
docs/                # design docs, decision records
```

Notice what's NOT here: no long explanations of why decisions were made, no history, no tutorials. Those belong in `docs/`. The AGENTS.md is a quick-reference card, not a manual.

**`ARCHITECTURE.md`** — A narrative map of how the system is organized (see Spec §1.1). Name every top-level directory. Explain the dependency direction. Call out the three things a newcomer is most likely to get wrong.

```markdown
# Architecture

## Domain Map
Invoice → LineItem → TaxRule → Currency

## Layer Order (dependency flows downward only)
Types → Config → Repository → Service → Runtime → UI

## Directory Structure
src/domains/invoicing/    # core business logic
src/domains/tax/          # tax computation, isolated from invoicing
src/infrastructure/db/    # Kysely adapters; no business logic here
src/infrastructure/http/  # Fastify plugins and middleware
src/api/                  # route handlers — thin, delegate to services

## Common Mistakes
1. Putting business logic in route handlers (belongs in src/domains/)
2. Importing from src/api/ into src/domains/ (violates layer direction)
3. Writing raw SQL instead of using Kysely typed queries
```

**`docs/core-beliefs.md`** — Your golden principles. These are the opinionated rules that define your codebase's identity. Think of them as the rules you'd enforce in every code review.

```markdown
# Core Beliefs

These are non-negotiable. Every PR, human or agent, is judged against them.

1. **Explicit error handling.** No blanket try/catch wrappers. Every error
   path returns a typed result or throws a domain-specific error.
2. **No business logic in route handlers.** Handlers parse input, call a
   service, and format the response. Nothing else.
3. **Every API endpoint has an integration test.** Unit tests are not
   sufficient for endpoints — the route, middleware, and serialization
   must be exercised together.
4. **Parse at the boundary.** External input is validated into typed
   representations at the edge. Inside the boundary, trust the types.
5. **No orphan code.** If a function isn't called, delete it. If a type
   isn't used, delete it. Dead code misleads agents.
```

### 3.2 Making Your Codebase Agent-Friendly

> Spec reference: §1.2 Codebase Legibility

**Adopt build-time type checking.** This is the spec's highest-obligation legibility MUST. If you're starting a new project, choose a language with native type checking (TypeScript, Go, Rust, Kotlin). If you have an existing dynamically-typed codebase, adopt progressive typing: enable a strict type checker (mypy/pyright for Python, TypeScript for JavaScript), enforce full type coverage on new and changed files, and expand coverage over time. The agent uses types as its primary source of truth for data shapes — without them, it guesses, and guesses compound into silent failures at system boundaries.

Two additional changes have high impact with low effort:

**Split large files.** A practical guideline: aim for files under 300 lines. (The spec requires "many small, well-scoped files" but doesn't mandate a specific number — 300 is a useful threshold because it fits comfortably in most models' working context without truncation.) A 1,500-line file either gets truncated (losing critical context) or consumes budget that should go to the task. You don't need to refactor the logic; just extract cohesive sections into their own files with clear names.

**Rename generic paths.** A 20-minute renaming session — `utils/helpers.ts` → `invoicing/tax-calculation.ts`, `types/index.ts` → `invoicing/types.ts` — immediately improves how well agents navigate your codebase. The agent uses directory and file names as its primary heuristic for "where should I look?"

### 3.3 Designing Your Agent Tools

> Spec reference: §1.3 Tool Design (ACI)

The Agent-Computer Interface (ACI) is the set of tools, commands, and scripts the agent calls. You're already designing one even if you don't realize it — every shell script, CLI wrapper, and build command in your repo is part of the ACI.

The core principle is **poka-yoke**: make the tool hard to misuse, rather than relying on the agent to use it correctly.

**Practical examples:**

- A deployment script that accepts `--env production` as a flag is easy to misuse. One that reads the target environment from a config file tied to the current branch is poka-yoke — the agent can't accidentally deploy to production from a feature branch.
- A database migration tool that accepts raw SQL strings invites injection and schema errors. One that generates typed migration files from schema diffs constrains the agent to valid operations.
- A file-manipulation tool that accepts relative paths will break when the agent changes directories (and it will). Require absolute paths.

**Write tool docs like prompts.** The tool's description, help text, and error messages are injected directly into the agent's context. A tool with a clear `--help` output that shows example invocations, valid input ranges, and common mistakes is effectively self-documenting for the agent. A tool that returns cryptic error codes forces the agent to guess.

**Keep I/O formats natural.** Agents produce and parse markdown and plain text more reliably than JSON-escaped code blocks or unified diff format with chunk headers. When designing tool output, prefer formats that look like text the model would encounter in training data.

### 3.4 The Speed Tax

> Spec reference: §1.5 Testing, §1.6 Dev Environment

Two investments pay for themselves within the first week:

**Fast tests.** Measure your suite right now. If it takes more than 60 seconds, that's your top priority. Every minute over 60 reduces the number of feedback cycles the agent gets per session. Common wins: run tests in parallel, mock external services, use an in-memory database for unit tests, cache third-party API responses.

**One-command environment.** Write a single script that creates a fresh, isolated, working dev environment. The target is under 2 seconds. Typical implementation: `git worktree add`, copy `.env`, `pnpm install --frozen-lockfile`, start dev server. If any step requires manual intervention (editing a config, running a migration interactively, clicking something in a UI), automate it or you'll never parallelize.

### 3.5 Safety Before You Start

> Spec reference: §1.7 Safety Basics

Before running your first agent loop, set up the safety boundary. This is easy to skip ("I'll add it later") and dangerous to skip — an unsandboxed agent with your credentials is a security incident waiting for a trigger.

**Sandbox the execution environment.** The agent will execute arbitrary code — that's the point. Run it in a container, a VM, or a sandboxed worktree with no access to production systems. The spec makes this a MUST, and for good reason: agents don't distinguish between "test database" and "production database" unless the environment makes it impossible to reach production.

**Allowlist, don't blocklist.** Define which network hosts, shell commands, and file paths the agent may access. Everything else is denied by default. This is counterintuitive — it feels restrictive — but agents will use any tool available to them. An allowlist means an accidental `curl` to a production API returns a permission error instead of mutating live data.

**Keep secrets out of the repo.** Agents read the entire repository. If a `.env` file or API key is committed, it's in the agent's context and may appear in outputs, logs, or generated code. Inject secrets via environment variables in the sandboxed runtime, never through files in the working tree.

**A minimal setup for a solo developer:**

1. Use Docker or a similar container to run the agent. Mount only the project directory.
2. Set network rules: allow `localhost` and your package registry. Block everything else.
3. Use `.env` files that are in `.gitignore` and copied into the container at startup — never committed.
4. If your project talks to external services (databases, APIs), point them at local or test instances only.

This takes about 30 minutes to set up and is effectively permanent — the same sandbox configuration works for every future agent session.

**Set the agent's default behavior.** Even with a sandbox, the agent may encounter operations that are destructive or ambiguous within the allowed scope (e.g., dropping a test database table, or a requirement that could be interpreted two ways). At Level 1, the human is directly supervising, so the agent's default should be to report and wait rather than proceed. State this explicitly in your AGENTS.md: "When you encounter a destructive operation or an ambiguous requirement, stop and ask." (Level 2 formalizes this into a full escalation protocol — see §4.5.)

### 3.6 Your First Agent Loop

Once the foundation is set, the actual loop is simple:

1. Give the agent a well-scoped task. "Add a `GET /invoices/:id` endpoint that returns the invoice with line items, or 404 if not found." Not "build the invoicing feature."
2. Let it work. The agent reads your AGENTS.md, finds the relevant files, writes code, runs tests.
3. Observe the output. Did it follow your conventions? Did it run the tests? Did the tests pass?
4. If something went wrong, ask: **is this a one-off mistake, or a pattern?** A one-off mistake is fine — just tell the agent what to fix. A pattern means the loop needs improvement: add a linter rule, clarify the AGENTS.md, improve a tool.

Over time, fewer things go wrong because each fix improves the loop permanently. This is the core dynamic of harness engineering: **you're not fixing outputs, you're improving the system that produces outputs.**

### 3.7 Common Level 1 Mistakes

**Over-prompting.** Writing a 2,000-word prompt for every task. If you need that much instruction, the problem is in the environment (missing AGENTS.md, unclear architecture, no conventions), not in the prompt. Fix the environment once instead of re-explaining in every prompt.

**Under-enforcing.** Writing conventions in docs but not in linters. The agent will read and then ignore documented conventions as the context window fills. A linter rule is unforgettable — it fails the build. Promote your most-violated conventions to linter rules.

**Skipping the speed tax.** Accepting a slow test suite because "it works." It works for humans who run tests a few times an hour. An agent runs tests after every change — the spec requires ≤1 minute (§1.5) because every second over that threshold is a feedback cycle the agent doesn't get. Every week you delay, you lose compounding value.

**Reviewing every line.** At Level 1, it's tempting to read every line of agent-generated code. This doesn't scale and isn't necessary if your linters, types, and tests are solid. Review outputs that change critical paths. Trust the mechanical checks for everything else.

---

## 4. Scaling Up: Level 2

### 4.1 Signs You Need Level 2

> Spec reference: §2 Level 2 — Multi-Agent + Planning (upgrade criteria)

Don't upgrade preemptively. Upgrade when you observe specific failures:

- **The agent consistently runs out of context** before finishing tasks, even with well-scoped prompts. The task is inherently larger than one session can handle.
- **Output quality is inconsistent** and the agent can't reliably self-correct. You're catching bugs that should have been caught before you saw the output.
- **Tasks require upfront design** that the agent skips. It starts coding immediately when it should be planning, leading to structural rework.

If your Level 1 loop is working and you're just impatient, resist the urge to upgrade. Level 2 adds real orchestration complexity.

### 4.2 Writing Your First ExecPlan

> Spec reference: §2.2 Execution Plans (ExecPlans), Appendix A.1

The ExecPlan is the hardest artifact to get right, and the one that most determines success at Level 2. The common mistake is treating it as a task list. It's not. It's a **self-contained instruction manual** that a stranger could follow to reproduce your intent.

The litmus test: imagine deleting all chat history and context from every agent session. Can a brand-new agent, reading only the ExecPlan and the current codebase, pick up the work and continue correctly? If not, the plan isn't self-contained enough.

Practical tips:

- **Write the Purpose section first.** If you can't articulate what the user gains in 2-3 sentences, the feature isn't well-defined enough to build.
- **Define milestones by observable outcome, not by code change.** "After Milestone 1, running `curl localhost:3000/health` returns `{"status":"ok"}`" is better than "After Milestone 1, the health endpoint is implemented." The first version is testable by any agent; the second is open to interpretation.
- **Include a prototyping milestone when there's uncertainty.** If you're not sure whether a library works the way you think, or whether an approach is feasible, make the first milestone a proof-of-concept that validates the assumption. This is cheaper than discovering the assumption is wrong in Milestone 4.
- **Keep updating it.** The plan accumulates institutional knowledge as work proceeds. The Surprises section captures things that surprised the agent — these are often the most valuable insights for the next session or the next project.

### 4.3 Calibrating the Evaluator

> Spec reference: §2.1 Agent Roles, §2.5 Quality Criteria

The evaluator is only as good as its calibration. Out of the box, an agent evaluator is overly generous — it identifies real issues and then talks itself into deciding they're not important.

The calibration loop:

1. **Run the evaluator** on a piece of work you've already assessed yourself.
2. **Compare scores.** Where does the evaluator diverge from your judgment? Is it too lenient? Too harsh? Missing a category?
3. **Update the evaluator's prompt.** Add few-shot examples showing your scoring for specific scenarios. "This design scores 3/5 on originality because it uses default library styling with no customization" is more useful than "be strict about originality."
4. **Repeat** until the evaluator's judgment aligns with yours on 3-5 test cases.

This typically takes 3-5 iteration rounds. The investment is front-loaded — once calibrated, the evaluator is stable across runs.

A well-calibrated evaluator catches real issues. From Anthropic's experiments, example evaluator findings:

> "Rectangle fill tool only places tiles at drag start/end points instead of filling the region. fillRectangle function exists but isn't triggered properly on mouseUp."

> "PUT /frames/reorder route defined after /{frame\_id} routes. FastAPI matches 'reorder' as a frame\_id integer and returns 422."

This level of specificity — naming the function, the route, the exact error — is what makes evaluator feedback actionable. Vague feedback ("some features don't work perfectly") gives the generator nothing to work with.

### 4.4 Context Resets in Practice

> Spec reference: §2.4 Context Strategy, Appendix A.3

The theory is simple: when the agent degrades, clear the slate and start fresh. The practice requires judgment.

**How to detect degradation:**
- The agent starts wrapping up ("I've completed the main features") when the plan shows 40% progress.
- Quality drops noticeably — the agent stops running tests, takes shortcuts, or produces code that contradicts earlier decisions.
- The agent begins repeating itself or re-explaining things it already did.

**How to perform a reset:**
1. Tell the agent: "Let's hand off. Please create a handoff artifact summarizing your work."
2. The agent writes the handoff to a file in the repo (e.g., `docs/exec-plans/active/handoff-003.md`).
3. Start a new agent session. Point it to the ExecPlan and the handoff.
4. The new agent reads both, understands the current state, and continues.

**What goes wrong:**
- The outgoing agent writes a handoff that's too terse. It says "implemented the API" without specifying which endpoints, what's tested, what's not. The incoming agent repeats work. **Fix**: review the first few handoffs manually; adjust the handoff template or prompt if the agent consistently under-documents.
- The incoming agent ignores the handoff and starts over. **Fix**: make the ExecPlan's Progress section the authoritative state. The incoming agent sees checked-off milestones and unchecked next steps.

### 4.5 The Human Escalation Protocol

> Spec reference: §2.6 Safety at Level 2

The hardest part of the escalation protocol isn't defining the triggers — it's trusting that the agent will actually stop.

Agents are trained to be helpful. They resist saying "I can't do this." Without explicit stop-conditions, they'll attempt destructive operations, guess at ambiguous requirements, and silently retry failing steps indefinitely.

The fix is to make stopping a first-class action, not a failure. In the evaluator's prompt and the generator's AGENTS.md, frame it as:

> "Escalating to a human is the correct action when [trigger conditions]. When escalating, report: (1) what you attempted, (2) what specifically blocked you, (3) your recommended next step. Then stop and wait."

Test this by giving the agent a deliberately ambiguous task and verifying it escalates instead of guessing.

---

## 5. Going Big: Level 3

### 5.1 Signs You Need Level 3

> Spec reference: §3 Level 3 — Clusters & Fleets (upgrade criteria)

Level 3 is not "Level 2 but more." It's a qualitative shift: from sequential pipelines to parallel swarms. The signals:

- Your work backlog is consistently deeper than your pipeline can drain. Features queue up waiting for the generator/evaluator cycle.
- Tasks are independent enough that multiple agents could work on different ones simultaneously without stepping on each other.
- You've invested in Level 2 infrastructure (ExecPlans, evaluator, context resets) and it's stable.

If your Level 2 pipeline is underutilized — the bottleneck is ideation or specification, not execution — you don't need Level 3. You need better planning.

### 5.2 The Merge Queue Problem

> Spec reference: §3.3 Merge Queue

The moment you have two agents working in parallel, you have the merge queue problem. Agent A finishes and merges. Agent B finishes, but now the baseline has changed. Agent B's changes may conflict with Agent A's — not just textually (git can detect those) but semantically (same function modified for different purposes).

A naive approach — let agents rebase and merge themselves — devolves into a "monkey knife fight" (Yegge's term). Agents spend more time resolving conflicts than building features.

The solution is a dedicated merge agent that:
1. Takes PRs from the queue one at a time.
2. Merges against the current HEAD.
3. If there's a conflict, reads both branches, understands the intent of each, and produces a correct merge.
4. Runs the full test suite on the merge result.
5. If tests pass, merges. If not, sends the PR back with specific failure info.

This is sequential by design. Parallelizing the merge itself is a false optimization — the coordination cost exceeds the time saved.

### 5.3 Keeping Work Durable

> Spec reference: §3.1 Workflow Durability

At Level 3, agents crash, expire, and get replaced constantly. The system works only if progress survives these disruptions.

The key insight is that the **session is not the unit of work** — the **ExecPlan step is**. An agent claims a step, works on it, and marks it complete. If the agent dies mid-step, the step stays unclaimed. The next agent picks it up, checks the current state (is the change partially applied? are tests passing?), and completes or restarts the step.

This is why ExecPlan steps must be idempotent. The agent should be able to re-run any step from scratch without leaving duplicates or corruption.

Step claiming must be atomic — only one agent works on a given step at a time. Without this, two agents may claim the same step concurrently and produce conflicting changes.

Practical pattern: each agent, on startup, reads its assigned ExecPlan, atomically claims the first unclaimed step, and begins. When it finishes, it marks the step complete, commits the progress update, and moves to the next step. If it's about to exhaust its context, it commits a handoff and exits cleanly.

---

## 6. Maintaining the Harness

### 6.1 Entropy in Practice

> Spec reference: §4.1 Entropy Management

You'll notice entropy first as a vague feeling: "this codebase is getting messy." Then as specific symptoms: duplicated utility functions, inconsistent error handling styles, a mix of naming conventions.

**At Level 1**, entropy management is a human responsibility. During normal code review of agent output, watch for pattern drift — the agent copying a shortcut from one file into five new ones. When a deviation recurs, don't just fix it in review: promote the fix to a linter rule (see §3.7 on under-enforcing). Each linter rule you add permanently removes a class of entropy.

**At Level 2+**, automate what you've been doing manually. A background agent runs nightly, scanning for deviations from your golden principles (defined in `docs/core-beliefs.md`). It opens small, focused refactoring PRs — each fixing one pattern deviation. These PRs are reviewable in under a minute and often auto-mergeable.

The key in both cases is frequency. A weekly cleanup that produces a 500-line PR is painful to review and risky to merge. A nightly cleanup that produces five 20-line PRs is almost invisible. Technical debt management follows the same principle as compound interest: small, frequent payments beat rare large ones.

### 6.2 When to Simplify

> Spec reference: §4.3 Harness Evolution

After a model upgrade, schedule a "simplification sprint": systematically test whether each scaffolding component is still load-bearing.

Method:
1. List all scaffolding components (sprint decomposition, context resets, separate evaluator, etc.).
2. For each, disable it and run a representative task.
3. Compare output quality with and without the component.
4. If quality is equivalent, remove the component permanently.

Be honest about the results. It's tempting to keep components that feel valuable but aren't measurably improving output. Every unnecessary component adds latency, cost, and orchestration surface area.

What you'll typically find: some components are clearly still needed, some are clearly obsolete, and a few are in a gray zone where they help on hard tasks but are overhead on easy ones. For the gray zone, consider making the component opt-in rather than default.

### 6.3 Evolving Your Harness

A harness is a living system, not a one-time setup. Expect to make meaningful changes to it every few weeks, driven by three inputs:

**Model changes.** A new model release may obsolete scaffolding or enable new capabilities. Run the simplification sprint (§6.2).

**Failure patterns.** When you see a failure repeat, trace it to a harness gap. The discipline is: don't just fix the output; fix the loop that produced the output. Over time, this makes the harness uniquely adapted to your project.

**Scale changes.** Growing the team, adding a new service, entering a new domain — each may shift the optimal harness configuration. A harness designed for a 10-file microservice won't serve a 500-file monorepo without adjustment.

The sign of a healthy harness: the rate of human intervention decreases over time, while throughput and quality increase. If you're intervening more as time passes, the harness is accumulating unresolved feedback loops.

---

## 7. Background: How We Got Here

This section is context, not prescription. It explains the intellectual lineage of the practices in the spec.

### 7.1 The Convergent Discoveries

Between late 2024 and early 2026, several teams independently arrived at remarkably similar conclusions about what makes agents effective. This wasn't coordination — they were solving the same problems and converging on the same physics. As §2 argues, this convergence is explained by the structure of the problem domain itself: every team was forced to answer the same three questions (who decides, how to grow, how work happens), and the viable answer space is narrow.

**OpenAI's team** built an internal product with zero manually-written code over five months. Their key discovery: the repository itself — not the prompt, not the model — is the primary lever for agent effectiveness. They made the codebase the single source of truth, pushed all context into versioned artifacts, and enforced architectural invariants mechanically. Their "AGENTS.md as table of contents" pattern, progressive disclosure, and doc-gardening practices all emerged from the insight that agent legibility is the bottleneck.

**Anthropic's team** explored two domains: frontend design (subjective quality) and full-stack application building (verifiable correctness). Their central finding: separating generation from evaluation — a technique they describe as inspired by GANs — produced the largest measurable quality improvement in their experiments. They also identified context anxiety as a first-class problem and reported that context resets outperformed compaction for models exhibiting it. Their simplification principle — remove harness components when the model outgrows them — keeps the harness lean over time.

**Geoffrey Huntley** articulated the loop primitive and the monolithic-first principle. His "Ralph Wiggum Loop" — named for the pattern of continuous iteration between an agent and its environment — became a widely-adopted mental model. His core argument: don't reach for multi-agent complexity until a single agent in a loop demonstrably can't solve the problem.

**Steve Krenzel (Logic.inc)** demonstrated that traditional engineering best practices — 100% test coverage, fast suites, typed boundaries, semantic naming — become dramatically more valuable in an agent context. What was optional for humans becomes essential for agents, because agents amplify whatever exists in the codebase, good or bad.

**Steve Yegge** pushed the frontier with Gas Town, demonstrating that agent orchestration at scale (30+ concurrent agents) is feasible with the right infrastructure: durable work state in git, a dedicated merge queue, and patrol agents that monitor worker health. His concept of nondeterministic idempotence — workflows that survive agent crashes by construction — is the foundation of Level 3 durability.

### 7.2 What Remains Unsettled

Harness engineering is young. Several questions remain open. For each, we note the thinking framework the three axioms provide — not as answers, but as constraints on the space of good answers.

- **How does architectural coherence evolve over years?** Current experiments span months. Nobody yet knows what a 5-year-old, fully agent-generated codebase looks like. *Framework: Axiom 3 says structural correction must be continuous, not periodic. The stance this implies: invest in continuous entropy measurement and frequent small corrections rather than periodic large refactors. If coherence is degrading, the feedback loop is too slow or too dishonest — fix the loop, not the symptoms.*

- **Where does human judgment add the most leverage?** The boundary between "agent can decide" and "human must decide" shifts with each model generation. The optimal division of labor is a moving target. *Framework: Axiom 1 orders by irreversibility — human attention belongs at the decisions with the least reversible consequences. As agents earn trust through demonstrated loop performance (Axiom 3), the boundary should shift outward. The practical test: if an agent decision goes wrong, can the loop self-correct? If yes, delegate. If no, retain human governance.*

- **How should harnesses adapt to multi-modal agents?** As agents gain the ability to see screenshots, hear audio, and reason over video, the evaluation surface expands. Current harnesses are primarily text-oriented. *Framework: the three axioms are modality-agnostic. When agents gain a new modality, apply the same principles: design the feedback loop for that modality (Axiom 3), maintain human governance over high-stakes decisions regardless of input channel (Axiom 1), and add multi-modal evaluation only when text-only evaluation demonstrably fails (Axiom 2).*

- **What's the right cost model?** At $10-12/hr per agent, Level 3 operations cost thousands per day. Is this justified by output quality? Under what conditions? The industry doesn't have robust benchmarks yet. *Framework: Axiom 2 says start simple and earn complexity — this applies to spend as well. Scale agent investment the same way you scale the harness: measure output value at the current level, scale up only when that level plateaus, and ensure each increment of cost produces a measurable increment of value.*

These are research questions, not blockers. The frameworks above won't provide answers, but they will keep you oriented while the answers emerge. Build with what works today; let the axioms guide your judgment where the evidence hasn't arrived yet.
