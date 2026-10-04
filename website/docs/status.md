---
title: Status
description: What the Commission repository holds today, and what is planned.
---

# Status: bootstrap

The model, the ports and the checks a run needs are in the repository. The loop that runs them
together is not written yet, and there is no release. This table is what the repository holds
today.

| Part | State | Detail |
|---|---|---|
| **Responsibility model** | Exists | Agent, AgentRevision, Case, Commission, Run, Frontier, Observation, Evidence and AuthorityDecision, the Run commands and events, and the value types the ports speak. Specified in ESS under `ess/`, generated into Rust and re-exported as `b10x_commission::model`. Never written by hand. See the [domain model](./reference/domain-model.md). |
| **Ports**: `Governor`, `AgentExecutor`, `AuthorityProvider`, observation and evidence | Exists | Traits in `b10x_commission::ports`. An authority provider that fails is turned into a refusal, never an allow. Evidence enters only through one function that sets the producer the trusted caller supplies. |
| **Frontier admission** | Exists | `admission::admit` decides whether the current frontier admits an action, taking the least-authority entry when an action is listed more than once. |
| **Run outcomes** | Exists | `outcome::derive` decides how a run ends, or that it does not end yet. `RunStore` backs the `StartRun`, `SuspendRun` and `ResumeRun` commands; it is in memory, so a suspended run does not survive a process restart. |
| **Test fakes**: governor, executor, authority provider | Exists | In `b10x-commission-testkit`. |
| **Repository gates** | Exists | The specification must validate strictly, compile and synthesize a conformance suite with no refusals; the generated code must match a fresh synthesis; the hand-written crates may not redefine a generated type; `b10x-commission` may not depend on Loom or a model-provider crate; and these reference pages must match a fresh generation. |
| **Action-request revalidation** | Exists | `action_request::revalidate` checks a request again immediately before use: stale if the case revision moved, not admitted if the current frontier refuses it, needs authority if the action is `ApprovalRequired`. It changes nothing. |
| **Local runtime loop** | Planned | The module exists and is empty. |
| **Adapter conformance kits** for governor and authority adapters | Planned | The modules exist and are empty. |
| **Effect invocation** | Decided design, not shipped code | Atlas ADR 0082. See [A run](./concepts/a-run.md#effect-invocation-decided-design-not-shipped-code). |

Commission ships no command-line tool. The repository's two command lines, `commission-xtask` and
`commission-docs`, are unpublished repository tools.
