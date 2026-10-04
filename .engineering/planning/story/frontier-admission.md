---
format: aep.planning-md/3
id: story:frontier-admission
kind: story
status: proposed
title: 'Frontier: Canon-valued, with an admission check for proposed actions'
summary: Close the Frontier contents marker (Canon values) and sort a proposed action as admissible, needs-authority or refused.
refs:
- provider: taskboard
  reference: M-002
relations:
- decomposes: epic:commission-core
- depends_on: story:generated-responsibility-model
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
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
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":1}}}
---
## Outcome

This is Commission's view of a frontier. One issued frontier covers one case revision, and its
claims, obligations and actions are Canon values. The story adds the admission check Commission
applies to a proposed action against that frontier. The check sorts an action into one of three
results:

- admissible;
- needs authority, naming the capability;
- refused, naming the action, when the frontier blocks it or does not list it.

This is the frontier contract's invariant: "An executor may not invoke an action absent from the
current frontier/admissible set" (`docs/contracts/frontier.md:57`). The check lives in a new module,
`crates/commission/src/admission.rs`, and works on Canon's `Frontier` (`b10x_canon::Frontier`,
imported at `crates/commission/src/lib.rs:9`).

## Shared surface

This story is link 2 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:generated-responsibility-model`, and
`story:governor-port` depends on it. The whole order is in `story:generated-responsibility-model`
§ Shared surface. `crates/commission/src/lib.rs` (one `mod` line) is edited along the same chain.

## ESS

Change `ess/domains/responsibility.yaml` before any code, then pass `ess specify validate --path ess`
and regenerate with `task generate`. The ESS change has three parts:

- **Close the marker.** This story closes the marker at `ess/domains/responsibility.yaml:183-184`:
  "UNMAPPED: claims, obligations and actions inside a frontier are Canon values; whether they are
  entities here or a value type imported from Canon is decided in story M-002". They are Canon
  values, not Commission entities. Delete the marker and say so in the comment on
  `commission.responsibility.Frontier` (`:182-203`).
- **Declare the result type.** Declare the admission result as a model type: a `union` with the
  variants admissible, needs-authority carrying the capability, and refused carrying the action
  and the reason.
- **Remove `Truth` unless something reads it.** Once claims belong to Canon, the domain's own
  `Truth` enum (`:70-73`) duplicates Canon's `Truth` (canon `crates/canon/src/lib.rs:9`). Remove it
  unless a Commission entity still reads it.

None of this adds a command, so the change adds no conformance scenario.

## Domain relations

- Frontier -> Case, many-to-one, references, carrying `case_revision`:
  `ess/domains/responsibility.yaml:195-199`, `commission.responsibility.Frontier` relation `case`.
- Frontier -> claims, obligations and actions: a Canon value held by the frontier, not a
  Commission entity. This is inferred, not declared in ESS. Commission imports and returns
  `b10x_canon::Frontier` (`crates/commission/src/lib.rs:9`, `:50`), and Canon defines claims,
  obligations and action candidates (canon `crates/canon/src/lib.rs:27-54`). After this story the
  Frontier comment says so in place of the marker.

## Scope

- `crates/commission/src/admission.rs` (new)
- `crates/commission/src/lib.rs` (module line)
- `crates/commission/tests/frontier_admission.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

## Acceptance

The test `admission_sorts_proposed_actions` in `crates/commission/tests/frontier_admission.rs`
passes. It builds frontiers by hand from Canon values and checks these expectations:

1. An `Admissible` action is sorted admissible.
2. An `ApprovalRequired { capability }` action is sorted needs-authority and names that capability.
3. A `Blocked` action is refused and the refusal names the action.
4. An action the frontier does not list is refused and the refusal names the action.
5. The test reads `ess/domains/responsibility.yaml`, and the file no longer contains the line
   "UNMAPPED: claims, obligations and actions inside a frontier".

## Notes

- **Canon:** no change. The check builds on Canon's `Frontier`, `ActionCandidate` and
  `ActionStatus` at pin `cf29c4b`.
- **`ApprovalRequired` is kept apart.** Canon's `Frontier::contains_action` (canon `lib.rs:56-65`)
  counts `ApprovalRequired` as contained. Commission's check sorts it as needs-authority instead.
- **Blocked reasons stay as they are.** The frontier's "why blocked" stays the strings Canon
  carries today. Richer reasons are Canon's work (TASKBOARD C-009) and are not needed here.
- **Test frontiers are hand-built.** Frontiers in this test are built by hand, not by Canon
  evaluation (Atlas `epic:ga-commission-core`, Shared surface). The fake governor arrives in the
  next link, `story:governor-port`.

## Source

TASKBOARD M-002 (build pack `TASKBOARD.md` § Commission); `docs/contracts/frontier.md`; Atlas
ADR 0072.
