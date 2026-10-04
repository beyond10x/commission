---
format: aep.planning-md/3
id: story:run-outcomes
kind: story
status: proposed
title: Run outcomes and suspension reasons, with completion only from the governor
summary: Generated RunOutcome, the rule deriving a run's outcome, and the Run's Suspended state with suspend and resume continuing the same run.
refs:
- provider: taskboard
  reference: M-007
relations:
- decomposes: epic:commission-core
- depends_on: story:governor-port
- depends_on: story:agent-executor-port
- depends_on: story:authority-provider-port
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
- depends_on: story:frontier-admission
scope:
- confidence: inferred
  path: crates/commission-testkit/tests/run_outcomes.rs
- confidence: cited
  path: crates/commission-xtask/
- confidence: cited
  path: crates/commission/src/outcome.rs
revision: 14
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

The rule that derives a run's outcome, and the behaviour of the Run's `Suspended` state, over the
generated `RunOutcome`. The derivation lives in the module `crates/commission/src/outcome.rs`, which
`story:port-skeleton` creates empty with its `mod` line.

A run ends with one of six outcomes. The variants come from the bootstrap `RunOutcome` and are
moved onto generated types:

- completed, carrying the governor's outcome;
- suspended, carrying a typed reason;
- needs authority, carrying the provider's approval request;
- needs human judgment, carrying the executor's request;
- needs external evidence;
- no admissible action.

**Generated, not hand-written.** `RunOutcome` is declared in `ess/domains/responsibility.yaml` by
`story:port-skeleton` and reaches the code only through `b10x-commission`'s re-export of the
generated crate; `story:port-skeleton` also deletes the bootstrap `pub enum RunOutcome` from
`crates/commission/src/lib.rs`. This story adds `RunOutcome` to the names the `no-hand-model`
check in `crates/commission-xtask/` refuses, and widens that check to refuse an `enum` of a listed
name as well as a `struct`, so a hand-written one cannot come back.

The derivation takes three inputs: the governor's determination (`CompletionDetermination`, through
`story:governor-port`), the executor's outcome (`ExecutorOutcome`, through
`story:agent-executor-port`) and any authority verdict (`AuthorityVerdict`, through
`story:authority-provider-port`). It may also return "continue", meaning the run does not end.

Only the governor completes a case. An executor's `CompletedLocalReasoning` never does
(`docs/contracts/commission-executor.md:50-52`).

`SuspensionReason` and `HumanDecisionRequest` are declared by `story:port-skeleton`; this story only
consumes them.

**Suspension and resume.** The Run's `Suspended` state, its suspend and resume transitions and their
commands are declared by `story:port-skeleton` from this story's former § ESS; this story builds the
behaviour behind those commands. The operator decided on 2026-10-04
(`decision-blocker:suspended-run-continuity`, cleared) that resuming after a restart continues the
same run. The run id survives and run state is durable. So resume moves the same run from
`Suspended` back to `Running`, keeping its id and its `case_revision`. It does not create a new Run.

Resume does not reuse the frontier the run held before it suspended. The next request is
revalidated against the current revision; that revalidation is `story:stale-revision-action-request`'s
and is wired into the loop by `story:local-runtime-loop`, not tested here.

Persisting a suspended run across a real process restart is not in this story. That belongs to
`story:approval-suspend-resume-slice` (TASKBOARD I-003). Where an AEP-governed suspension is kept
is still open as `decision-blocker:suspension-durable-record`.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on:

- `story:port-skeleton`, for its module file and the declarations in § ESS first;
- `story:governor-port`, `story:agent-executor-port` and `story:authority-provider-port`, real
  dependencies: every row is scripted through their fakes;
- `story:frontier-admission`, a real dependency: rows 3, 5 and 6 read the frontier's action statuses
  and open obligations, whose types that story may change when it settles its contract drift.

Its old edge on `story:stale-revision-action-request` was ordering only (the shared `ess/` file) and
is gone. It runs beside `story:observation-evidence-ports` and
`story:stale-revision-action-request`; `crates/commission-xtask/` is this story's alone in that
wave. `story:local-runtime-loop` and `story:commission-ess-conformance` depend on it.

## ESS first

- **Specification change: none in this story.** It relies on the declarations
  `story:port-skeleton` lands from this story's former § ESS: the union
  `commission.responsibility.RunOutcome` with the six variants above; `Run`'s second state
  `Suspended`, neither state terminal; the transitions suspend (`Running` → `Suspended`) and resume
  (`Suspended` → `Running`); and a command for each, the suspend command carrying a
  `SuspensionReason`.
- **First commit, red.** The test `run_outcome_derivation` alone, in
  `crates/commission-testkit/tests/run_outcomes.rs`. It is red because `outcome` declares no
  derivation and nothing handles the suspend and resume commands: the test does not compile.
- **Then.** The derivation, the suspend and resume behaviour, and the `no-hand-model` change, which
  make it pass.

The two Run commands carry conformance scenarios from `story:port-skeleton` on.
`story:commission-ess-conformance` answers them through this story's behaviour, or names them in
`ess/SKIPPED.md`.

## Domain relations

- Commission -> Run, one-to-many, the commission owns its runs:
  `ess/domains/responsibility.yaml:192-197`, `commission.responsibility.Commission` relation `runs`.
  A run ending does not end the commission or the case.
- A suspended run stays the same Run of the same commission across resume. No new relation.

## Scope

- `crates/commission/src/outcome.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-xtask/` (`RunOutcome` added to the `no-hand-model` list; enums refused)
- `crates/commission-testkit/tests/run_outcomes.rs` (new)

## Acceptance

The table-driven test `run_outcome_derivation` in `crates/commission-testkit/tests/run_outcomes.rs`
passes. Each row is scripted from a governor determination, an executor outcome and an authority
decision, using the fakes. These are its expectations:

1. **Completed.** The governor reports the case complete with outcome `X`, so the derived outcome
   is completed carrying `X`. `CompletedLocalReasoning` on a case the governor reports open derives
   continue, not completed.
2. **Suspended.** The executor returns `Suspended` with a `SuspensionReason`, so the derived outcome
   is suspended carrying the same reason.
3. **Needs authority.** The executor proposes an action the frontier marks `ApprovalRequired`, and
   the provider answers approval required with request `Q`. The derived outcome is needs authority
   carrying `Q`.
4. **Needs human judgment.** The executor returns `NeedsHumanJudgment` with request `H`, so the
   derived outcome is needs human judgment carrying `H`.
5. **Needs external evidence.** The frontier admits no action and lists open obligations `O`, so
   the derived outcome is needs external evidence carrying `O`.
6. **No admissible action.** The frontier admits no action and lists no obligation, so the derived
   outcome is no admissible action.
7. **Resume.** A run suspended through the suspend command and then resumed through the resume
   command is the same run. It has the same run id and the same `case_revision`, and its state is
   `Running`.
8. **Generated type.** `ess/domains/responsibility.yaml` declares `RunOutcome` as a `union` with the
   six variants above; the test reads the file and finds the declaration. The test imports
   `RunOutcome` through `b10x-commission`'s re-export of the generated crate, and
   `std::any::type_name::<RunOutcome>()` begins with the generated crate's name, `commission::`,
   not with `b10x_commission::`.

## Notes

- **Canon:** no change. The completed outcome carries the governor's outcome identifier as the
  governor reports it through `Governor`. Canon's own outcome model (TASKBOARD C-007) is not needed
  while frontiers come from fakes.
- **Row 5 is inferred.** No source says when a run needs external evidence. The row maps the
  generated `Frontier`'s `obligations` whose `open` is true (`FrontierObligation`,
  `story:ess-hard-gate`) onto `NeedsExternalEvidence { requirements }`, one requirement per open
  obligation's `obligation` string. No Canon type is used.
- **Budget exhaustion is left out.** The history design § 14 also lists `BudgetExhausted`
  (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:650-658`). The bootstrap does
  not carry it, and neither does this story.

## Source

TASKBOARD M-007 (build pack `TASKBOARD.md` § Commission); `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md`
§§ 14, 35; operator decision of 2026-10-04 on run continuity; Atlas ADR 0080.
