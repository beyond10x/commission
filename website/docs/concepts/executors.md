---
title: Executors and Loom
description: One executor contract, many executors. Loom is the native one.
status: shipped
lede: Commission does not depend on any particular executor. The AgentExecutor contract is meant to fit very different workers.
source: The AgentExecutor port, crates/commission/src/ports/executor.rs
source_url: https://github.com/beyond10x/commission/blob/main/crates/commission/src/ports/executor.rs
---

| Executor | Role |
|---|---|
| **Loom** | Commission's native, default executor. Loom depends on Commission's shared execution contracts and implements them; Commission core has no dependency on Loom. See the [Loom documentation](https://beyond10x.github.io/loom/). |
| **Other harnesses and workflows** | External coding harnesses and workflow executors can implement the same contract. Commission is not an LLM harness itself. |
| **People** | A human executor can answer a frontier the same way. |
| **Test fakes** | Fakes implement it too. The test kit has a scripted fake executor, along with fakes of the governor and the authority provider. |

Whatever the executor, it only proposes. It never marks a case complete, and under the decided
design in [A run](./a-run.md#effect-invocation-decided-design-not-shipped-code) it does not invoke
effects either.

The contract itself is sketched in [AgentExecutor](./contracts/executor.md).
