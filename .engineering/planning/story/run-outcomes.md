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
- depends_on: story:stale-revision-action-request
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: crates/commission-testkit/tests/run_outcomes.rs
- confidence: cited
  path: crates/commission-xtask/
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: inferred
  path: crates/commission/src/outcome.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 11
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

The run's outcome becomes a generated type, together with the rule that derives it and the Run's
`Suspended` state. The derivation lives in a new module, `crates/commission/src/outcome.rs`.

A run ends with one of six outcomes. The variants come from the bootstrap
`crates/commission/src/lib.rs:39-47` and are moved onto generated types:

- completed, carrying the governor's outcome;
- suspended, carrying a typed reason;
- needs authority, carrying the provider's approval request;
- needs human judgment, carrying the executor's request;
- needs external evidence;
- no admissible action.

**Generated, not hand-written.** `RunOutcome` is declared in `ess/domains/responsibility.yaml` and
reaches the code only through `b10x-commission`'s re-export of the generated crate. The bootstrap
`pub enum RunOutcome` (`crates/commission/src/lib.rs:39-47`) is deleted. This story adds
`RunOutcome` to the names the `no-hand-model` check in `crates/commission-xtask/` refuses, and
widens that check to refuse an `enum` of a listed name as well as a `struct`.

The derivation takes three inputs: the governor's determination (`story:governor-port`), the
executor's outcome (`story:agent-executor-port`) and any authority decision
(`story:authority-provider-port`). It may also return "continue", meaning the run does not end.

Only the governor completes a case. An executor's `CompletedLocalReasoning` never does
(`docs/contracts/commission-executor.md:50-52`).

`SuspensionReason` and the human-judgment request are declared by `story:agent-executor-port`;
this story only consumes them.

**Suspension and resume.** This story owns the Run's `Suspended` state and the commands that move a
run into it and out of it. The operator decided on 2026-10-04
(`decision-blocker:suspended-run-continuity`, cleared) that resuming after a restart continues the
same run. The run id survives and run state is durable
(`ess/domains/responsibility.yaml:172-173`). So resume moves the same run from `Suspended` back to
`Running`, keeping its id and its `case_revision`. It does not create a new Run.

Resume does not reuse the frontier the run held before it suspended. The next request is
revalidated against the current revision (`story:stale-revision-action-request`).

Persisting a suspended run across a real process restart is not in this story. That belongs to
`story:approval-suspend-resume-slice` (TASKBOARD I-003). Where an AEP-governed suspension is kept
is still open as `decision-blocker:suspension-durable-record`.

## Shared surface

This story is link 9 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:stale-revision-action-request`, and
`story:local-runtime-loop` depends on it. The whole order is in
`story:ess-hard-gate` § Shared surface.

## ESS

Make these changes in `ess/domains/responsibility.yaml` first:

- Declare the run outcome as a `union` with the six variants above.
- Give `commission.responsibility.Run` (`:165-180`) a second state, `Suspended`.
- Add two transitions, suspend (`Running` → `Suspended`) and resume (`Suspended` → `Running`).
- Add a command for each transition. The suspend command carries a `SuspensionReason`.
- Replace the note at `:172-173` ("the Suspended state arrives with its command outcome in story
  M-007") with the state itself.

Neither state is terminal, and each has an outgoing transition. Then pass
`ess specify validate --path ess` and regenerate with `task generate`.

The two commands add conformance scenarios. `story:commission-ess-conformance`, later in the chain,
answers them through its Rust target or names them in `ess/SKIPPED.md`.

## Domain relations

- Commission -> Run, one-to-many, the commission owns its runs: `ess/domains/responsibility.yaml:155-159`,
  `commission.responsibility.Commission` relation `runs`. A run ending does not end the commission
  or the case.
- A suspended run stays the same Run of the same commission across resume (`:172-173`). No new
  relation.

## Scope

- `crates/commission/src/outcome.rs` (new)
- `crates/commission/src/lib.rs`
- `crates/commission-xtask/` (`RunOutcome` added to the `no-hand-model` list)
- `crates/commission-testkit/tests/run_outcomes.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

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
  `story:ess-hard-gate`) onto the bootstrap `NeedsExternalEvidence { requirements }`
  (`crates/commission/src/lib.rs:44`), one requirement per open obligation's `obligation` string.
  No Canon type is used.
- **Budget exhaustion is left out.** The history design § 14 also lists `BudgetExhausted`
  (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:650-658`). The bootstrap does
  not carry it, and neither does this story.

## Source

TASKBOARD M-007 (build pack `TASKBOARD.md` § Commission); `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md`
§§ 14, 35; operator decision of 2026-10-04 on run continuity.
