---
format: aep.planning-md/3
id: story:authority-provider-port
kind: story
status: proposed
title: AuthorityProvider port decides outside the model and fails toward less authority
summary: Allow, deny or approval-required at the call; a provider error is a refusal, never an allow.
refs:
- provider: taskboard
  reference: M-005
relations:
- decomposes: epic:commission-core
- depends_on: story:generated-responsibility-model
- depends_on: story:agent-executor-port
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/commission-testkit/src/fake_authority.rs
- confidence: inferred
  path: crates/commission-testkit/src/lib.rs
- confidence: inferred
  path: crates/commission-testkit/tests/authority_port.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: crates/commission/src/ports/authority.rs
- confidence: cited
  path: crates/commission/src/ports/mod.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":3}}}
---
## Outcome

The `AuthorityProvider` port gets its own module, `crates/commission/src/ports/authority.rs`.

Authority stays outside the model. Commission asks the provider at the moment of the call, passing
the commission and the capability. The commission now carries its principal and authority context
(`ess/domains/responsibility.yaml:139-142`). The provider answers with one of three decisions:

- **allow**;
- **deny**, with the provider's reason;
- **approval required**, with the provider's request.

The shape comes from the bootstrap trait and enum (`crates/commission/src/lib.rs:57-70`), moved
onto generated types.

If the provider fails, the answer is a refusal. A failure never becomes an allow (`AGENTS.md:33-34`,
"fail toward less authority").

A static fake provider in `crates/commission-testkit/src/fake_authority.rs` implements the port.
Its table maps each capability to a decision or to a failure, and it records every principal and
authority context it is asked about.

**Authority context stays opaque.** `AuthorityContext` is a Commission-local `newtype` of `Json`.
The spec says its structure "is decided with the AuthorityProvider port (M-005)"
(`ess/domains/responsibility.yaml:59-63`). This story decides that the structure stays opaque to
Commission. Commission passes the value to the provider unchanged and never reads it. The provider
is the only reader. Record this in that comment, in place of the deferral.

## Shared surface

This story is link 6 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:agent-executor-port`, and
`story:observation-evidence-ports` depends on it. The full order is in
`story:ess-hard-gate` § Shared surface. The same chain also orders the edits to
`crates/commission/src/lib.rs`, `ports/mod.rs` and `crates/commission-testkit/src/lib.rs`.

## ESS

Make these changes in `ess/domains/responsibility.yaml` first:

- Declare the decision the port returns as a model type: a `union` of allow, deny with reason, and
  approval required with request. Give it a name distinct from the `AuthorityDecision` entity.
- Update the `AuthorityContext` comment as above.

Then pass `ess specify validate --path ess` and regenerate with `task generate`. Neither change is a
command, so no conformance scenario is added.

**Who owns a stored decision is decided.** A stored `AuthorityDecision` belongs to exactly one
action request at one case revision, and a later action needs a new decision (operator decision of
2026-10-04, `decision-blocker:authority-decision-owner`, cleared; recorded at
`ess/domains/responsibility.yaml:254-257`). The relation that says so, `AuthorityDecision.action_request_id`
referencing exactly one `ActionRequest`, is declared by `story:stale-revision-action-request`,
later in the chain, because that story declares `ActionRequest`. This story does not edit the
`AuthorityDecision` entity (`:258-270`). It returns the decision at the call and stores none.

## Domain relations

- **Commission -> principal and authority context.** These are fields of Commission-local types,
  not relations (`ess/domains/responsibility.yaml:139-142`; `PrincipalId` `:54-57`,
  `AuthorityContext` `:59-63`; operator decision of 2026-10-04,
  `decision-blocker:commission-principal-type`, cleared). The provider decides for the commission's
  principal under its authority context.
- **Binding to Mandate.** This is phase-7 work (`:128-129`). Mandate is not a dependency here.

## Scope

- `crates/commission/src/ports/authority.rs` (new)
- `crates/commission/src/ports/mod.rs`
- `crates/commission/src/lib.rs`
- `crates/commission-testkit/src/fake_authority.rs` (new)
- `crates/commission-testkit/src/lib.rs`
- `crates/commission-testkit/tests/authority_port.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

## Acceptance

The test `authority_port_contract` in `crates/commission-testkit/tests/authority_port.rs` passes. It
runs Commission's authority check against the static fake provider and checks these expectations:

1. A capability the fake allows returns allow.
2. A capability the fake denies with reason `R` returns deny carrying `R`.
3. A capability the fake answers with approval required and request `Q` returns approval required
   carrying `Q`.
4. A capability for which the fake returns an error returns a refusal, not an allow.
5. For each call, the fake records the commission's `principal` and `authority_context` exactly as
   the commission holds them.

## Notes

- **Canon:** no change, and no Canon type is used. The capability comes from the frontier: the
  `capability` of a `FrontierAction` whose `status` is `ApprovalRequired` (`story:ess-hard-gate`),
  which the admission check of `story:frontier-admission` reports as needs-authority.
- **Mandate:** Mandate is the intended managed implementation. It is not a dependency here.

## Source

TASKBOARD M-005 (build pack `TASKBOARD.md` § Commission); Atlas `docs/design/governed-autonomy/invariants.md`
§§ 3, 7; `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 22.
