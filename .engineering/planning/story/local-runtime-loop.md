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
- depends_on: story:observation-evidence-ports
scope:
- confidence: inferred
  path: crates/commission-testkit/tests/runtime_loop.rs
- confidence: cited
  path: crates/commission/src/runtime.rs
revision: 13
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

This story adds the local runtime loop, `run_until_blocked`, which runs over fakes. It lives in the
module `crates/commission/src/runtime.rs`, which `story:port-skeleton` creates empty with its `mod`
line.

A commission works through these steps:

1. It loads its case through the governor.
2. It obtains the frontier.
3. It invokes the executor.
4. It turns a proposed action into an action request (`story:stale-revision-action-request`).
5. It revalidates the request.
6. It consults authority where the frontier requires it.
7. It either continues or ends the run with a derived run outcome (`story:run-outcomes`).

This is the chain in Atlas `docs/design/governed-autonomy/projects/commission-TASKS.md` (the build pack's `projects/commission/TASKS.md`):
`CaseRef -> Governor.frontier() -> AgentExecutor.run() -> revalidate proposed action -> block / suspend / continue`.
Each loop creates a Run of the commission at the case revision it started against. A suspension
leaves that Run in `Suspended` with its id unchanged, so a resume continues the same run
(`story:run-outcomes`; operator decision of 2026-10-04).

This story executes no effect. The sources put execution bindings in Loom (TASKBOARD L-004, L-014,
L-015) and name no Commission effect port. The loop records each admitted, revalidated request
where the test can read it, then reads the frontier again.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on these, each a real dependency:

- `story:run-outcomes`, for the derived outcome and the suspend behaviour;
- `story:stale-revision-action-request`, for the action request and its revalidation;
- `story:observation-evidence-ports`, for the observation port of Acceptance 10 (before the
  re-plan this was reached only through the chain).

The governor, executor and authority fakes come through those three. This story no longer touches
`ess/`, `generated/` or `crates/commission/src/lib.rs`, so it runs beside
`story:commission-ess-conformance` and `story:adapter-conformance-suites`; the old edge from
`story:commission-ess-conformance` onto this story was ordering only and is gone.

## ESS first

- **Specification change: none in this story.** It adds no noun. The `Run` states and commands it
  drives (`Running`, `Suspended`, suspend, resume) are declared by `story:port-skeleton`, and
  `ActionRequest` by `story:stale-revision-action-request`. A specification change this story finds
  necessary is filed as a `decision-blocker` and taken by its own story with its own
  `## ESS first`, so this story stays off `ess/`.
- **First commit, red.** The test `run_until_blocked_over_fakes` alone, in
  `crates/commission-testkit/tests/runtime_loop.rs`. It is red because `runtime` declares no
  `run_until_blocked`: the test does not compile.
- **Then.** The loop, which makes it pass.

## Domain relations

- Commission -> Run, one-to-many, the commission owns its runs: `ess/domains/responsibility.yaml:192-197`,
  `commission.responsibility.Commission` relation `runs`.
- Commission -> Case, many-to-one, references: `:187-191`, relation `case`. A case may hold many
  commissions at once (`:164-165`). The loop runs one commission and does not assume it is the
  case's only commission.
- Frontier -> Case, many-to-one, references, carrying `case_revision`: `:239-243`,
  `commission.responsibility.Frontier` relation `case`.

## Scope

- `crates/commission/src/runtime.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-testkit/tests/runtime_loop.rs` (new)

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

TASKBOARD M-009 (build pack `TASKBOARD.md` § Commission); Atlas `docs/design/governed-autonomy/projects/commission-TASKS.md`;
`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` §§ 13-14; Atlas ADR 0080.


## From wave 2026-10-04-w4 (governor-port, agent-executor-port adversary passes)

- `FakeGovernor::calls()` and `ScriptedExecutor::calls()` are separate per-fake logs with no
  shared sequence. Acceptance 1 (case loaded and frontier obtained before the executor runs, per
  iteration) needs the order across fakes: read `governor.calls()` from inside a test executor
  wrapper, or add a shared sequence in this story's test file.
- All three governor methods consume one answer queue, so a loop script counts every governor
  call per iteration; an added `completion()` check shifts later revisions (acceptance 3, 5, 8).
- Its "execution bindings are in Loom" reason for running no effect is replaced by Atlas ADR
  0082: Commission makes the effect invocation; this story still runs none (commission
  story:effect-invocation).


## From wave 2026-10-04-w5 (run-outcomes, adversary pass 2, F4)

`outcome::derive` returns Continue for `NoUsefulAction` when the frontier admits an action. The
loop decides what Continue after a NoUsefulAction means: it must not re-ask the same executor on an
unchanged frontier without bound. Settle a rule here (for example: NoUsefulAction twice on the same
frontier revision ends the run as no admissible action) and test it.
