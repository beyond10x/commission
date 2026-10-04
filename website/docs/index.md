---
title: What Commission is
slug: /
description: Commission is the Rust SDK and runtime model for governed autonomous workers.
---

# What Commission is

**Build agents you can give responsibility to.**

Commission is the Rust SDK and runtime model for governed autonomous workers. It binds a reusable
agent revision to a durable case, under a governor that decides what the agent may do next.

An agent is configuration. What you hand over is a case, and the thing that carries responsibility
for it is a commission. Seven nouns cover it.

| Noun | What it is |
|---|---|
| **Agent** | Reusable worker configuration. An agent has revisions (`AgentRevision`), and a commission names one revision, not the agent in general. |
| **Case** | Durable work under a protocol. Commission holds a reference to the case; the governor holds its truth. |
| **Commission** | Binds one agent revision to one case, with a principal and an authority context. A case may hold several commissions at once. |
| **Run** | One bounded period of execution of a commission. A run records the case revision it started against, so a proposal made on another revision is stale. |
| **Governor** | Decides what the agent may do next and whether the case is complete. AEP is one governor implementation; Commission itself is domain-neutral. |
| **Frontier** | What the governor returns for one case revision: what is known, unknown or contradicted, which obligations are open, which actions are admissible, which need authority, and what is blocked and why. See [the frontier sketch](./concepts/contracts/frontier.md). |
| **AgentExecutor** | Proposes an action from the frontier. It does not act on its own say-so: Commission is built to revalidate every proposal first (the loop that does so is planned; see [A run](./concepts/a-run.md)). See [the executor sketch](./concepts/contracts/executor.md). |

:::info[The executor never marks a case complete]

Completion is the governor's determination. An executor can propose an action, ask for human
judgment, suspend, report that nothing useful is left to do, or say it finished its own reasoning.
None of those closes the case.

:::

## Where to go next

- [A run](./concepts/a-run.md): propose, recheck, then act.
- [Rules](./concepts/rules.md): what a model may and may not supply.
- [Executors and Loom](./concepts/executors.md).
- [Status](./status.md): what exists in the repository today, and what is planned. Commission is at
  the bootstrap stage and has no release.
- [Domain model](./reference/domain-model.md) and [value types](./reference/types.md), generated
  from the ESS specification.
