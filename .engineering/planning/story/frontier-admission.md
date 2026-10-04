---
format: aep.planning-md/3
id: story:frontier-admission
kind: story
status: proposed
title: Admission check for proposed actions over Commission's own frontier
summary: Sort a proposed action as admissible, needs-authority or refused against the generated Frontier's FrontierAction values from story:ess-hard-gate; no Canon type.
refs:
- provider: taskboard
  reference: M-002
relations:
- decomposes: epic:commission-core
- depends_on: story:generated-responsibility-model
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:ess-hard-gate
- depends_on: story:port-skeleton
scope:
- confidence: cited
  path: crates/commission/src/admission.rs
- confidence: inferred
  path: crates/commission/tests/frontier_admission.rs
- confidence: inferred
  path: docs/contracts/frontier.md
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 12
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":1}}}
---
## Outcome

This story adds the admission check Commission applies to a proposed action against the frontier
issued for one case revision. The frontier is Commission's own: the generated
`commission.responsibility.Frontier` with its `actions` list of `FrontierAction` values, which
`story:ess-hard-gate` declares. No Canon type is used. The check sorts a proposed action into one
of three results:

- admissible, when the frontier lists the action with status `Admissible`;
- needs authority, naming the capability, when the frontier lists it with status
  `ApprovalRequired` and a `capability`;
- refused, naming the action, when the frontier lists it with status `Blocked` (the refusal
  carries the action's `reasons`), lists it with status `ApprovalRequired` and no `capability`, or
  does not list it.

An `ApprovalRequired` action with no capability is refused rather than sent for authority, because
no capability can be asked for and the check fails toward less authority (`AGENTS.md:33-34`).

This is the frontier contract's invariant: "An executor may not invoke an action absent from the
current frontier/admissible set" (`docs/contracts/frontier.md:57`). The check lives in the module
`crates/commission/src/admission.rs`, which `story:port-skeleton` creates empty with its `mod`
line, and takes the generated `Frontier` through `b10x-commission`'s re-export of the generated
crate.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on `story:port-skeleton` (it fills
`admission.rs`, which that story creates) and runs beside `story:governor-port`,
`story:agent-executor-port` and `story:authority-provider-port`, none of which uses the admission
check.

It still edits `ess/domains/responsibility.yaml` and regenerates `generated/rust/commission/` (the
admission result, and the contract-drift settlement below), and it is the only story in its wave
that does. The drift settlement may change the frontier's item types (`FrontierClaim`,
`FrontierObligation`, `FrontierAction`, `ActionStatus`, `Truth`) but not `Frontier`'s own fields;
the stories beside it carry `Frontier` values without building or reading items, so the change
cannot break them. `story:stale-revision-action-request` (which reuses the check) and
`story:run-outcomes` (which reads the items) depend on this story.

The admission result is not declared by `story:port-skeleton`: its refusal carries an action's
reasons, and their type is one of the differences settled below.

## ESS first

Atlas ADR 0080: the first commit changes only the specification, a named test is red on it, and
later commits make it pass.

- **Specification change (first commit, `ess/domains/responsibility.yaml` only).** Declare the
  admission result as a model type, a `union` with the variants admissible, needs-authority
  carrying the capability, and refused carrying the action and the reasons. Where § Contract drift
  to settle first is settled by growing the specification, the changed frontier item types go in
  the same commit; where it is settled by rewriting `docs/contracts/frontier.md`, that file is not
  specification and changes in a later commit.
- **Red on that commit.** `drift_passes_on_the_committed_tree`
  (`crates/commission-xtask/tests/checks.rs`, and `task drift`) fails: the committed
  `generated/rust/commission/` no longer matches a fresh synthesis of `ess/`.
- **Then.** `task generate`, the admission check, and the test `admission_sorts_proposed_actions`;
  `task ess-gate` passes throughout.

`story:ess-hard-gate` declared `ActionStatus`, `FrontierClaim`, `FrontierObligation` and
`FrontierAction`; this story changes them only as the drift settlement requires. It adds no
command, so it adds no conformance scenario.

## Domain relations

- Frontier -> Case, many-to-one, references, carrying `case_revision`:
  `ess/domains/responsibility.yaml:195-199`, `commission.responsibility.Frontier` relation `case`.
- Frontier -> claims, obligations and actions: fields of Commission-owned value types declared by
  `story:ess-hard-gate` (`claims`, `obligations`, `actions`), not entities and not relations. The
  check reads `actions` only.

## Scope

- `crates/commission/src/admission.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission/tests/frontier_admission.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (the admission result and the
  drift settlement; the only story in its wave that edits them)
- `docs/contracts/frontier.md` (only if the drift is settled by rewriting the contract)

## Acceptance

The test `admission_sorts_proposed_actions` in `crates/commission/tests/frontier_admission.rs`
passes. It builds generated `Frontier` values by hand and checks these expectations:

1. An action listed with status `Admissible` is sorted admissible.
2. An action listed with status `ApprovalRequired` and capability `Some(c)` is sorted
   needs-authority and names `c`.
3. An action listed with status `Blocked` and reasons `R` is refused; the refusal names the action
   and carries `R`.
4. An action listed with status `ApprovalRequired` and capability `None` is refused and the refusal
   names the action.
5. An action the frontier does not list is refused and the refusal names the action.
6. The admission result is the generated union: the test imports it through `b10x-commission`'s
   re-export, and `std::any::type_name` of it begins with the generated crate's name,
   `commission::`.

## Notes

- **Canon:** no change, and no Canon type is used. Canon's wave-1 change deletes its bootstrap
  `Frontier`, `ActionId`, `ActionStatus` and `ActionCandidate`; the admission check never depended
  on them.
- **No Canon dependency to work around.** `story:port-skeleton` removes `b10x-canon` and the
  bootstrap contracts from `crates/commission/src/lib.rs`.
- **`ApprovalRequired` is kept apart.** It is sorted needs-authority, never admissible.
- **Blocked reasons stay strings.** A `FrontierAction`'s `reasons` are `List<String>`, as
  `story:ess-hard-gate` declares them. Typed reasons (`docs/contracts/frontier.md:45-48` shows a
  claim, whether it is required and its actual value) are not needed here.
- **Test frontiers are hand-built.** Frontiers in this test are built by hand, not by a governor
  (Atlas `epic:ga-commission-core`, Shared surface). The fake governor is `story:governor-port`'s,
  which runs in the same wave.

## Source

TASKBOARD M-002 (build pack `TASKBOARD.md` § Commission); `docs/contracts/frontier.md`; Atlas
ADRs 0072, 0076, 0080.


## Contract drift to settle first

Wave 2026-10-04-w2 (story:ess-hard-gate, adversary pass 2, finding F3) found that
`docs/contracts/frontier.md` no longer matches the value types the specification now declares:

- the contract keys frontier items by `id`; the specification names them `claim`, `obligation`, `action`;
- the contract gives obligations `status: open` plus `priority`; the specification has `open: Boolean`;
- the contract's `reasons` are structured (`claim`, `required`, `actual`); the specification has `List<String>`;
- the contract's action statuses have no approval-required value; the specification has `ApprovalRequired`;
- the contract says a claim may be "unknown or contradicted"; `Truth` cannot say contradicted.

This story decides each difference before it writes the admission check: either the specification
grows to the contract (through `ess/` and regeneration) or the contract is rewritten to the
specification. Both end with the two in agreement.
