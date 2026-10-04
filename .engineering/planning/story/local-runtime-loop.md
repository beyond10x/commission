---
format: aep.planning-md/3
id: story:local-runtime-loop
kind: story
status: proposed
title: Local runtime loop over a fake governor and a fake executor
summary: 'run_until_blocked: load case, frontier, executor, revalidate, authority, derived outcome; no effects executed.'
refs:
- provider: taskboard
  reference: M-009
relations:
- decomposes: epic:commission-core
- depends_on: story:run-outcomes
- depends_on: story:stale-revision-action-request
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: crates/commission-testkit/tests/runtime_loop.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: crates/commission/src/runtime.rs
- confidence: inferred
  path: ess/domains/responsibility.yaml
- confidence: inferred
  path: generated/rust/commission/
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

This story adds the local runtime loop, `run_until_blocked`, which runs over fakes. It lives in a
new module, `crates/commission/src/runtime.rs`.

A commission works through these steps:

1. It loads its case through the governor.
2. It obtains the frontier.
3. It invokes the executor.
4. It turns a proposed action into an action request (`story:stale-revision-action-request`).
5. It revalidates the request.
6. It consults authority where the frontier requires it.
7. It either continues or ends the run with a derived run outcome (`story:run-outcomes`).

This is the chain in the build pack's `projects/commission/TASKS.md`:
`CaseRef -> Governor.frontier() -> AgentExecutor.run() -> revalidate proposed action -> block / suspend / continue`.
Each loop creates a Run of the commission at the case revision it started against. A suspension
leaves that Run in `Suspended` with its id unchanged, so a resume continues the same run
(`story:run-outcomes`; operator decision of 2026-10-04).

This story executes no effect. The sources put execution bindings in Loom (TASKBOARD L-004, L-014,
L-015) and name no Commission effect port. The loop records each admitted, revalidated request
where the test can read it, then reads the frontier again.

## Shared surface

This story is link 9 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:run-outcomes` and on
`story:stale-revision-action-request`, and `story:commission-ess-conformance` depends on it. The
whole order is in `story:generated-responsibility-model` § Shared surface.

## ESS

This story adds no new noun. If it changes the `Run` entity (`ess/domains/responsibility.yaml:165-180`
after `story:run-outcomes`), it makes the change there first, passes `ess specify validate --path ess`,
and regenerates with `task generate`. `story:commission-ess-conformance` comes next in the chain and
answers any scenario this story adds.

## Domain relations

- Commission -> Run, one-to-many, the commission owns its runs: `ess/domains/responsibility.yaml:155-159`,
  `commission.responsibility.Commission` relation `runs`.
- Commission -> Case, many-to-one, references: `:149-153`, relation `case`. A case may hold many
  commissions at once (`:126-127`). The loop runs one commission and does not assume it is the
  case's only commission.
- Frontier -> Case, many-to-one, references, carrying `case_revision`: `:195-199`,
  `commission.responsibility.Frontier` relation `case`.

## Scope

- `crates/commission/src/runtime.rs` (new)
- `crates/commission/src/lib.rs`
- `crates/commission-testkit/tests/runtime_loop.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface; only if `Run` changes)

## Acceptance

The test `run_until_blocked_over_fakes` in `crates/commission-testkit/tests/runtime_loop.rs`
passes. It drives the loop with the scripted fake governor, the scripted fake executor and the
static fake authority provider, and checks these expectations:

1. **Call order.** The fakes' call logs show, per iteration, the case loaded and the frontier
   obtained from the governor before the executor is invoked.
2. **Unlisted action refused.** A proposal whose action the current frontier does not list is
   refused, is not recorded as admitted, and the loop reads the frontier again.
3. **Stale proposal refused.** A proposal made at revision N, after the governor has moved the case
   to N+1, is refused as stale and is not recorded as admitted.
4. **Admitted, not executed.** A proposal that the frontier admits and that revalidates is recorded
   as admitted, and no effect is executed.
5. **Completed.** When the governor reports the case complete with outcome `X`, the run ends
   completed carrying `X`.
6. **Suspended.** When the executor returns `Suspended` with reason `S`, the run ends suspended
   carrying `S`. Its Run is in `Suspended` with the run id it started with.
7. **No admissible action.** When the frontier admits no action and lists no obligation, the run
   ends with no admissible action.
8. **Starting revision.** Each Run's `case_revision` is the case revision its loop started against.
9. **Authority consulted.** The executor proposes an action the frontier marks `ApprovalRequired`,
   and the fake authority provider answers approval required with request `Q`. The provider's call
   log shows it was asked for that action's capability, the action is not recorded as admitted, and
   the run ends as needs authority carrying `Q`.
10. **Observation delivered.** After an executor step whose output says tests passed, the fake
    governor's observation log holds exactly one `Observation` from that step, delivered through
    the observation port of story:observation-evidence-ports, and no Evidence.

## Notes

- Canon: no change.
- `epic:commission-core` Acceptance is met by three things together: this story's acceptance,
  `story:agent-executor-port` (dependency guard), and `story:commission-ess-conformance`.

## Source

TASKBOARD M-009 (build pack `TASKBOARD.md` § Commission); build pack `projects/commission/TASKS.md`;
`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` §§ 13-14.
