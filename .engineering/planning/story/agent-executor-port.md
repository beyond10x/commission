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
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
scope:
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/commission-testkit/src/fake_executor.rs
- confidence: inferred
  path: crates/commission-testkit/tests/executor_port.rs
- confidence: cited
  path: crates/commission/src/ports/executor.rs
- confidence: inferred
  path: model-provider-deny.txt
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

The `AgentExecutor` port gets its own module, `crates/commission/src/ports/executor.rs`, which
`story:port-skeleton` creates empty with its `mod` line. Given the commission and the current
frontier, an executor returns one generated `ExecutorOutcome`. The outcome is one of five:

- a proposed action with arguments;
- a need for human judgment;
- a suspension with a typed reason;
- no useful action;
- completed local reasoning.

No variant marks the case complete. The generated union (declared by `story:port-skeleton`)
replaces the bootstrap enum, which was written by hand and had no `CompletedLocalReasoning`;
`story:port-skeleton` deletes that enum and the bootstrap trait from `crates/commission/src/lib.rs`,
and this story writes the trait anew over generated types.

A scripted fake executor goes in `crates/commission-testkit/src/fake_executor.rs`, created empty by
`story:port-skeleton`. It returns whatever outcome its script names.

Commission stays an SDK, not an LLM harness. The dependency guard is Rust, inside this story's
test (`AGENTS.md` § Rules). The test reads `cargo tree -p b10x-commission -e normal --prefix none`
and fails when the listing names `b10x-loom` or a crate on the repository's model-provider deny
list. The deny list is `model-provider-deny.txt` at the repository root, beside `Taskfile.yml`. A
new task, `deps-guard`, runs that test file (`cargo test -p b10x-commission-testkit --test
executor_port`), and `check` lists it as its own step. The rule comes from Atlas ADR 0075: Loom
depends on Commission, never the reverse.

**The Canon dependency is gone before this story starts.** The executor receives the generated
`commission.responsibility.Frontier` (`story:ess-hard-gate`). The removal of the bootstrap
signatures that named Canon types, of the `use b10x_canon` line, of the `b10x-canon` dependency
and its `Cargo.lock` entries, and of the `AGENTS.md` § Work bullet about the Canon pin moved to
`story:port-skeleton`, because each of those edits lands on a file several stories shared. This
story's test still holds the result (Acceptance 5). A later story that needs Canon adds it back and
says why.

**Type use.** `SuspensionReason`, `ProposedActionArguments` and `HumanDecisionRequest` are declared
by `story:port-skeleton` from this story's former § ESS. `story:run-outcomes` uses
`SuspensionReason` for the run's suspended outcome; `story:stale-revision-action-request` carries
`ProposedActionArguments` on the action request. Neither depends on this story for the types.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on `story:port-skeleton` (its module,
its fake file and the executor types). It runs beside `story:frontier-admission`,
`story:governor-port` and `story:authority-provider-port`; it uses none of their behaviour, so its
old ordering edges on `story:frontier-admission` and `story:governor-port` are gone.

`Taskfile.yml` (the `deps-guard` task and one line in `check`) is still shared with
`story:commission-ess-conformance`, which runs two waves later. `story:frontier-admission`
regenerates the model in the same wave and may change the frontier's item types; this story's fake
and test carry `Frontier` values without building or reading items, so that change cannot break it.

`story:run-outcomes` depends on this story for the scripted fake executor.

## ESS first

- **Specification change: none in this story.** It relies on the declarations
  `story:port-skeleton` lands from this story's former § ESS: `ExecutorOutcome` as a union
  (`ProposedAction { action: String, arguments: ProposedActionArguments }`,
  `NeedsHumanJudgment { request: HumanDecisionRequest }`, `Suspended { reason: SuspensionReason }`,
  `NoUsefulAction`, `CompletedLocalReasoning`), `ProposedActionArguments` (newtype of `Json`),
  `HumanDecisionRequest` and `SuspensionReason` (seven variants from the history design § 35,
  `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1477-1485`). None is a command,
  so there is no conformance scenario to add.
- **First commit, red.** The test `executor_port_contract` alone, in
  `crates/commission-testkit/tests/executor_port.rs`, with `model-provider-deny.txt`. It is red
  because `ports::executor` declares no `AgentExecutor` trait, `fake_executor` holds no fake and the
  guard's matcher does not exist: the test does not compile.
- **Then.** The trait, the scripted fake, the guard and the `deps-guard` task, which make it pass.

## Domain relations

None beyond those `story:generated-responsibility-model` cites. The executor receives a commission
and a frontier and owns neither.

## Scope

- `crates/commission/src/ports/executor.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-testkit/src/fake_executor.rs` (created empty by `story:port-skeleton`; filled
  here)
- `crates/commission-testkit/tests/executor_port.rs` (new)
- `model-provider-deny.txt` (new)
- `Taskfile.yml` (task `deps-guard`; one line in `check`)

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
5. The same real listing names no `b10x-canon`, and `AgentExecutor::run` takes the generated
   `Frontier` imported through `b10x-commission`'s re-export.

## Notes

- Canon: no change to Canon. Commission stops depending on it in `story:port-skeleton`; this story's
  guard keeps it out.
- The deny list is the repository's own and names each crate it refuses.

## Source

TASKBOARD M-004 (build pack `TASKBOARD.md` § Commission); `docs/contracts/commission-executor.md`;
Atlas ADRs 0070, 0075, 0080; `epic:commission-core` Acceptance ("`cargo tree` shows no
model-provider crate").
