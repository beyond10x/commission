---
format: aep.planning-md/3
id: story:authority-provider-port
kind: story
status: implemented
title: AuthorityProvider port decides outside the model and fails toward less authority
summary: Allow, deny or approval-required at the call; a provider error is a refusal, never an allow.
refs:
- provider: taskboard
  reference: M-005
relations:
- decomposes: epic:commission-core
- depends_on: story:generated-responsibility-model
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
scope:
- confidence: cited
  path: crates/commission-testkit/src/fake_authority.rs
- confidence: inferred
  path: crates/commission-testkit/tests/authority_port.rs
- confidence: cited
  path: crates/commission/src/ports/authority.rs
revision: 11
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "proposed", to: "active", at: "2026-10-04T02:47:36Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "active", to: "implemented", at: "2026-10-04T03:13:41Z", actor: "human:timo", revision: 11, decided_on: {"recorded":{"test_result":1,"review_outcome":6,"verification":1}}}
---
## Outcome

The `AuthorityProvider` port gets its own module, `crates/commission/src/ports/authority.rs`, which
`story:port-skeleton` creates empty with its `mod` line.

Authority stays outside the model. Commission asks the provider at the moment of the call, passing
the commission and the capability. The commission carries its principal and authority context
(`ess/domains/responsibility.yaml:177-180`). The provider answers with one of three decisions, the
generated `AuthorityVerdict`:

- **allow**;
- **deny**, with the provider's reason;
- **approval required**, with the provider's request.

The shape comes from the bootstrap trait and enum, moved onto generated types. `story:port-skeleton`
deletes the bootstrap ones from `crates/commission/src/lib.rs` and declares `AuthorityVerdict`;
this story writes the trait anew.

If the provider fails, the answer is a refusal. A failure never becomes an allow (`AGENTS.md`
§ Rules, "fail toward less authority").

A static fake provider in `crates/commission-testkit/src/fake_authority.rs` (created empty by
`story:port-skeleton`) implements the port. Its table maps each capability to a decision or to a
failure, and it records every principal and authority context it is asked about.

**Authority context stays opaque.** `AuthorityContext` is a Commission-local `newtype` of `Json`.
This story's decision, that the structure stays opaque to Commission, that Commission passes the
value to the provider unchanged and never reads it, and that the provider is the only reader, is
written into the `AuthorityContext` comment in `ess/` by `story:port-skeleton`, in place of the
deferral. This story holds the code to it.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on `story:port-skeleton` (its module,
its fake file and `AuthorityVerdict`). It runs beside `story:frontier-admission`,
`story:governor-port` and `story:agent-executor-port`. Its old edge on `story:agent-executor-port`
was ordering only (the shared `ess/`, `lib.rs`, `ports/mod.rs` and testkit `lib.rs`) and is gone.

`story:run-outcomes` and `story:adapter-conformance-suites` depend on this story for the static fake
provider.

## ESS first

- **Specification change: none in this story.** It relies on the declarations
  `story:port-skeleton` lands from this story's former § ESS: the union
  `commission.responsibility.AuthorityVerdict` (`Allow`, `Deny { reason: String }`,
  `ApprovalRequired { request: String }`) and the reworded `AuthorityContext` comment. Neither is a
  command, so there is no conformance scenario to add.
- **First commit, red.** The test `authority_port_contract` alone, in
  `crates/commission-testkit/tests/authority_port.rs`. It is red because `ports::authority` declares
  no `AuthorityProvider` trait and `fake_authority` holds no fake: the test does not compile.
- **Then.** The trait, Commission's authority check and the static fake, which make it pass.

**Who owns a stored decision is decided.** A stored `AuthorityDecision` belongs to exactly one
action request at one case revision, and a later action needs a new decision (operator decision of
2026-10-04, `decision-blocker:authority-decision-owner`, cleared; recorded at
`ess/domains/responsibility.yaml:298-301`). The relation that says so,
`AuthorityDecision.action_request_id` referencing exactly one `ActionRequest`, is declared by
`story:stale-revision-action-request`, because that story declares `ActionRequest`. This story does
not edit the `AuthorityDecision` entity. It returns the verdict at the call and stores none.

## Domain relations

- **Commission -> principal and authority context.** These are fields of Commission-local types,
  not relations (`ess/domains/responsibility.yaml:177-180`; `PrincipalId` `:55-58`,
  `AuthorityContext` `:60-64`; operator decision of 2026-10-04,
  `decision-blocker:commission-principal-type`, cleared). The provider decides for the commission's
  principal under its authority context.
- **Binding to Mandate.** This is phase-7 work (`:166-167`). Mandate is not a dependency here.

## Scope

- `crates/commission/src/ports/authority.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-testkit/src/fake_authority.rs` (created empty by `story:port-skeleton`; filled
  here)
- `crates/commission-testkit/tests/authority_port.rs` (new)

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
  which the admission check of `story:frontier-admission` reports as needs-authority. This story's
  test passes capabilities as strings and does not need that check.
- **Mandate:** Mandate is the intended managed implementation. It is not a dependency here.

## Source

TASKBOARD M-005 (build pack `TASKBOARD.md` § Commission); Atlas `docs/design/governed-autonomy/invariants.md`
§§ 3, 7; Atlas ADR 0080; `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 22.
