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
scope:
- confidence: inferred
  path: crates/commission/src/admission.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: inferred
  path: crates/commission/tests/frontier_admission.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 8
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
current frontier/admissible set" (`docs/contracts/frontier.md:57`). The check lives in a new module,
`crates/commission/src/admission.rs`, and takes the generated `Frontier` through
`b10x-commission`'s re-export of the generated crate.

## Shared surface

This story is link 3 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:ess-hard-gate`, and `story:governor-port`
depends on it. The whole order is in `story:ess-hard-gate` § Shared surface.
`crates/commission/src/lib.rs` (one `mod` line) is edited along the same chain.

## ESS

Change `ess/domains/responsibility.yaml` before any code: declare the admission result as a model
type, a `union` with the variants admissible, needs-authority carrying the capability, and refused
carrying the action and the reasons. Then pass `task ess-gate` and regenerate with
`task generate`.

The frontier's contents are not this story's: `story:ess-hard-gate` declares `ActionStatus`,
`FrontierClaim`, `FrontierObligation` and `FrontierAction` and closes the frontier marker. This
story adds no command, so it adds no conformance scenario.

## Domain relations

- Frontier -> Case, many-to-one, references, carrying `case_revision`:
  `ess/domains/responsibility.yaml:195-199`, `commission.responsibility.Frontier` relation `case`.
- Frontier -> claims, obligations and actions: fields of Commission-owned value types declared by
  `story:ess-hard-gate` (`claims`, `obligations`, `actions`), not entities and not relations. The
  check reads `actions` only.

## Scope

- `crates/commission/src/admission.rs` (new)
- `crates/commission/src/lib.rs` (module line)
- `crates/commission/tests/frontier_admission.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

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
- **`ApprovalRequired` is kept apart.** It is sorted needs-authority, never admissible.
- **Blocked reasons stay strings.** A `FrontierAction`'s `reasons` are `List<String>`, as
  `story:ess-hard-gate` declares them. Typed reasons (`docs/contracts/frontier.md:45-48` shows a
  claim, whether it is required and its actual value) are not needed here.
- **Test frontiers are hand-built.** Frontiers in this test are built by hand, not by a governor
  (Atlas `epic:ga-commission-core`, Shared surface). The fake governor arrives in the next link,
  `story:governor-port`.

## Source

TASKBOARD M-002 (build pack `TASKBOARD.md` § Commission); `docs/contracts/frontier.md`; Atlas
ADRs 0072, 0076.


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
