---
format: aep.planning-md/3
id: story:agent-executor-port
kind: story
status: proposed
title: AgentExecutor port over generated outcomes, with no model-provider or Loom dependency
summary: Executor returns a generated ExecutorOutcome that cannot complete a case; declares SuspensionReason and ProposedActionArguments; task deps-guard refuses model-provider crates and b10x-loom.
refs:
- provider: taskboard
  reference: M-004
relations:
- decomposes: epic:commission-core
- depends_on: story:frontier-admission
- depends_on: story:governor-port
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/commission-testkit/src/fake_executor.rs
- confidence: inferred
  path: crates/commission-testkit/src/lib.rs
- confidence: inferred
  path: crates/commission-testkit/tests/executor_port.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: crates/commission/src/ports/executor.rs
- confidence: cited
  path: crates/commission/src/ports/mod.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
- confidence: inferred
  path: model-provider-deny.txt
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

The `AgentExecutor` port gets its own module, `crates/commission/src/ports/executor.rs`. Given the
commission and the current frontier, an executor returns one generated `ExecutorOutcome`. The
outcome is one of five:

- a proposed action with arguments;
- a need for human judgment;
- a suspension with a typed reason;
- no useful action;
- completed local reasoning.

No variant marks the case complete. The generated enum replaces the bootstrap one
(`crates/commission/src/lib.rs:24-37`), which was written by hand and has no
`CompletedLocalReasoning`. The generated enum also replaces the bootstrap trait (`lib.rs:53-55`).

A scripted fake executor goes in `crates/commission-testkit/src/fake_executor.rs`, in the testkit
crate created by `story:governor-port`. It returns whatever outcome its script names.

Commission stays an SDK, not an LLM harness. The dependency guard is Rust, inside this story's
test (`AGENTS.md:35`). The test reads `cargo tree -p b10x-commission -e normal --prefix none` and
fails when the listing names `b10x-loom` or a crate on the repository's model-provider deny list.
The deny list is `model-provider-deny.txt` at the repository root, beside `Taskfile.yml`. A new
task, `deps-guard`, runs that test file (`cargo test -p b10x-commission-testkit --test
executor_port`), and `check` lists it as its own step. The rule comes from Atlas ADR 0075: Loom
depends on Commission, never the reverse.

**Type ownership.** This story declares two types, and later stories only consume them:

- **`SuspensionReason`.** The payload of `ExecutorOutcome::Suspended`
  (`docs/contracts/commission-executor.md:40-42`). `story:run-outcomes` uses it for the run's
  suspended outcome and for the Run's `Suspended` state.
- **`ProposedActionArguments`.** The arguments type of a proposed action.
  `story:stale-revision-action-request` carries it on the action request.

## Shared surface

This story is link 4 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:governor-port`, and
`story:authority-provider-port` depends on it. The full order is in
`story:generated-responsibility-model` § Shared surface.

The same chain also orders the edits to these files:
- `Taskfile.yml` (one line in `check`, plus the new `deps-guard` task)
- `crates/commission/src/lib.rs` and `ports/mod.rs`
- `crates/commission-testkit/src/lib.rs`

## ESS

`commission.responsibility.ExecutorOutcome` (`ess/domains/responsibility.yaml:65-68`) is declared
as an enum with bare variants. Make these changes in `ess/` first, then pass
`ess specify validate --path ess` and regenerate with `task generate`.

1. **Turn it into a `union`.** Each variant carries the payload that
   `docs/contracts/commission-executor.md:29-47` gives it.
2. **Declare the proposed action.** A proposed action names an action and its
   `ProposedActionArguments`, which is a `newtype` of `Json` in place of the contract's
   `arguments_json: String`. The action's identifier is a String newtype, converted to Canon's
   `ActionId` at the boundary. Identity, authority and case revision are not part of it
   (`AGENTS.md:25-27`).
3. **Declare `SuspensionReason`** as a `union` whose variants are the ones in the history design
   § 35 (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1477-1485`): authority,
   human, evidence, time, dependency, budget, external availability. Where a payload's type is
   declared nowhere in Commission's sources, carry it as String or Json. Do not invent an entity
   for it.
4. **Declare the human-judgment request** that `NeedsHumanJudgment` carries in the same way.

These are types, not commands, so they add no conformance scenario.

## Domain relations

None beyond those `story:generated-responsibility-model` cites. The executor receives a commission
and a frontier and owns neither.

## Scope

- `crates/commission/src/ports/executor.rs` (new)
- `crates/commission/src/ports/mod.rs`
- `crates/commission/src/lib.rs`
- `crates/commission-testkit/src/fake_executor.rs` (new)
- `crates/commission-testkit/src/lib.rs`
- `crates/commission-testkit/tests/executor_port.rs` (new)
- `model-provider-deny.txt` (new)
- `Taskfile.yml` (task `deps-guard`; one line in `check`)
- `ess/domains/responsibility.yaml` and `generated/rust/commission/` (chain surface)

## Acceptance

The test `executor_port_contract` in `crates/commission-testkit/tests/executor_port.rs` passes. It
checks these expectations:

1. For each of the five generated `ExecutorOutcome` variants, the scripted fake executor, called
   through `AgentExecutor`, returns that variant with its payload unchanged.
2. A `Suspended` outcome carries a generated `SuspensionReason`. A `ProposedAction` outcome carries
   generated `ProposedActionArguments`.
3. The real `cargo tree -p b10x-commission -e normal --prefix none` listing names neither
   `b10x-loom` nor any crate in `model-provider-deny.txt`.
4. The guard's matcher is given a canned listing that contains the line `b10x-loom v0.0.0`, and a
   second one that contains the first crate in `model-provider-deny.txt`. For each listing it
   reports a violation that names that crate.

## Notes

- Canon: no change.
- The deny list is the repository's own and names each crate it refuses.

## Source

TASKBOARD M-004 (build pack `TASKBOARD.md` § Commission); `docs/contracts/commission-executor.md`;
Atlas ADRs 0070, 0075; `epic:commission-core` Acceptance ("`cargo tree` shows no model-provider
crate").
