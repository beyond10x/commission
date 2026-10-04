---
format: aep.planning-md/3
id: story:governor-port
kind: story
status: proposed
title: Governor port returns the frontier for a case's current revision
summary: 'Typed Governor port with a scripted fake: current revision, frontier, completion determination, typed errors.'
refs:
- provider: taskboard
  reference: M-003
relations:
- decomposes: epic:commission-core
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
scope:
- confidence: cited
  path: crates/commission-testkit/src/fake_governor.rs
- confidence: inferred
  path: crates/commission-testkit/tests/governor_port.rs
- confidence: cited
  path: crates/commission/src/ports/governor.rs
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

The `Governor` port in its own module, `crates/commission/src/ports/governor.rs`, which
`story:port-skeleton` creates empty together with `ports/mod.rs` and its `mod` lines.

Given a commissioned case reference, the port returns three things:

- the case's current revision;
- the frontier issued for that revision, as the generated `commission.responsibility.Frontier`
  with its Commission-owned claims, obligations and actions (`story:ess-hard-gate`);
- the governor's determination of whether the case is complete, and with which outcome, as the
  generated `CompletionDetermination`.

Port failures are typed errors, not strings: the generated `GovernorError`, with an unknown case
and a governor that cannot answer. The bootstrap signature
`frontier(&Commission) -> Result<Frontier, String>`, whose `Frontier` was `b10x_canon::Frontier`,
is deleted by `story:port-skeleton`; this story writes the trait anew. The port names no Canon
type. Observation and evidence methods belong to `story:observation-evidence-ports`.

**The fake governor.** `crates/commission-testkit/src/fake_governor.rs`, created empty by
`story:port-skeleton` in the testkit crate (package `b10x-commission-testkit`, one module per fake,
adapter kits under `src/kits/`), becomes a scripted fake governor whose script sets each case's
revision, frontier and determination per call and can make a call fail. The testkit depends on
`b10x-commission`, and `b10x-commission` does not depend on the testkit, so tests that use a fake
live in `crates/commission-testkit/tests/`.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on `story:port-skeleton` (its module,
its fake file and the `GovernorError` and `CompletionDetermination` types). It runs beside
`story:frontier-admission`, `story:agent-executor-port` and `story:authority-provider-port`; it
uses none of their behaviour, so its old ordering edge on `story:frontier-admission` is gone.

`story:frontier-admission` regenerates the model in the same wave and may change the frontier's
item types. This story's fake and test carry `Frontier` values without building or reading claims,
obligations or actions (empty item lists), so that change cannot break it.

`story:observation-evidence-ports` extends this story's fake governor in the next wave and depends
on it; `story:stale-revision-action-request`, `story:run-outcomes` and
`story:adapter-conformance-suites` use the fake and depend on it too.

## ESS first

- **Specification change: none in this story.** The declarations it relies on are landed by
  `story:port-skeleton`: `commission.responsibility.GovernorError` (`UnknownCase`,
  `GovernorUnavailable`) and `commission.responsibility.CompletionDetermination` (`Open`,
  `Complete { outcome: String }`), with the existing `Frontier` and `CaseId`. Neither is a command,
  so there is no conformance scenario to add.
- **First commit, red.** The test `governor_port_contract` alone, in
  `crates/commission-testkit/tests/governor_port.rs`. It is red because `ports::governor` declares
  no `Governor` trait and `fake_governor` holds no fake: the test does not compile.
- **Then.** The trait and the scripted fake, which make it pass.

## Domain relations

- Commission -> Case, many-to-one, references: `ess/domains/responsibility.yaml:187-191`,
  `commission.responsibility.Commission` relation `case`. A case may hold many commissions
  (`:164-165`), and the port answers per case, whichever commission asks.
- Frontier -> Case, many-to-one, references, carrying `case_revision`: `:239-243`,
  `commission.responsibility.Frontier` relation `case`.

## Scope

- `crates/commission/src/ports/governor.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-testkit/src/fake_governor.rs` (created empty by `story:port-skeleton`; filled
  here)
- `crates/commission-testkit/tests/governor_port.rs` (new)

## Acceptance

The test `governor_port_contract` in `crates/commission-testkit/tests/governor_port.rs` passes
against the scripted fake governor, with these expectations:

1. The script advances a commissioned case from revision N to N+1. The first call returns a
   frontier carrying the case id and revision N, and the second call returns one carrying
   revision N+1.
2. A case the script marks complete with outcome `X` returns the determination complete carrying
   `X`. A case not marked complete returns open.
3. A case id the fake does not hold returns the typed unknown-case error, not a string.
4. A call the script makes fail returns the typed governor-unavailable error.

## Notes

- Canon: no change, and the port names no Canon type. The case id it answers for is the generated
  `CaseId`. `story:port-skeleton` removes the `b10x-canon` dependency.
- The error set is the two variants `story:port-skeleton` declares. A further governor error is a
  specification change and starts with its own `## ESS first`.
- Only the governor determines completion, never the executor
  (`docs/contracts/commission-executor.md:50-52`). This port is where that determination enters
  Commission.

## Source

TASKBOARD M-003 (build pack `TASKBOARD.md` § Commission); Atlas ADR 0069 (AEP is one governor
implementation); Atlas ADR 0080 (spec first, then red);
`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 21.
