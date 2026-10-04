---
title: AgentExecutor (sketch)
description: The design sketch of the executor contract. The executor proposes; the governor decides.
---

# AgentExecutor

:::note[Design sketch]

This page renders `docs/contracts/commission-executor.md`, a design sketch. The trait below is the
sketch, not the code. The `AgentExecutor` port that exists in `b10x_commission::ports::executor`
differs: it takes the assigned commission and the issued frontier and returns an `ExecutorOutcome`
with no error channel; an executor that fails returns `Suspended`. `ExecutorOutcome` is generated
from the specification: see [value types](../../reference/types.md#executoroutcome).

:::

Commission must not depend on Loom specifically. One executor contract should serve Loom, external
coding harnesses, workflow executors, human executors and test fakes.

```rust title="conceptual"
pub trait AgentExecutor {
    fn run(
        &self,
        commission: &CommissionContext,
        frontier: &Frontier,
    ) -> Result<ExecutorOutcome, ExecutorError>;
}
```

The sketch notes that production will likely be async. The port today is synchronous.

| Outcome | Meaning |
|---|---|
| `ProposedAction` | An action from the frontier, with arguments as the model wrote them. Identity, authority and case revision are not part of a proposal. |
| `NeedsHumanJudgment` | A request for a person to decide. |
| `Suspended` | A reason to pause: authority, a human, evidence, time, a dependency on other cases, budget, or external availability. |
| `NoUsefulAction` | Nothing on the frontier is worth doing now. |
| `CompletedLocalReasoning` | The executor finished its own reasoning. That is not the case being complete. |

:::info[The executor does not mark the case complete]

Completion remains a governor and protocol determination. In the generated model the governor's
answer is a `CompletionDetermination`, either open or complete with the governor's outcome
identifier.

:::

Under Atlas ADR 0082 (decided design, not shipped code) a proposed action is invoked by the
Commission runtime after it rechecks frontier, case revision and authority, never by the executor.
See [A run](../a-run.md).
