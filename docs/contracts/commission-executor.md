# Contract Sketch — Commission Executor

Commission must not depend on Loom specifically.

A generic executor contract should be able to support:

- Loom;
- external coding harnesses;
- workflow executors;
- human executors;
- test fakes.

Conceptual Rust:

```rust
pub trait AgentExecutor {
    fn run(
        &self,
        commission: &CommissionContext,
        frontier: &Frontier,
    ) -> Result<ExecutorOutcome, ExecutorError>;
}
```

Production will likely be async.

Possible output:

```rust
pub enum ExecutorOutcome {
    ProposedAction {
        action: ActionId,
        arguments_json: String,
    },

    NeedsHumanJudgment {
        request: HumanDecisionRequest,
    },

    Suspended {
        reason: SuspensionReason,
    },

    NoUsefulAction,

    CompletedLocalReasoning,
}
```

The executor does not mark the case complete.

Completion remains a governor/protocol determination.
