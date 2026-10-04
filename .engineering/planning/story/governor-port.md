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
- depends_on: story:frontier-admission
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: Cargo.lock
- confidence: inferred
  path: crates/commission-testkit/Cargo.toml
- confidence: cited
  path: crates/commission-testkit/src/fake_governor.rs
- confidence: inferred
  path: crates/commission-testkit/src/lib.rs
- confidence: inferred
  path: crates/commission-testkit/tests/governor_port.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: crates/commission/src/ports/governor.rs
- confidence: cited
  path: crates/commission/src/ports/mod.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":2}}}
---
## Outcome

The `Governor` port in its own module, `crates/commission/src/ports/governor.rs`. This story also
creates `crates/commission/src/ports/mod.rs`. Later port stories each add one line to it.

Given a commissioned case reference, the port returns three things:

- the case's current revision;
- the frontier issued for that revision;
- the governor's determination of whether the case is complete, and with which outcome.

Port failures are typed errors, not strings. There are at least two: an unknown case, and a
governor that cannot answer. This story replaces the bootstrap signature
`frontier(&Commission) -> Result<Frontier, String>` (`crates/commission/src/lib.rs:49-51`).
Observation and evidence methods belong to `story:observation-evidence-ports`.

**Fakes live in a new crate.** This story is the first to need a fake, so it creates
`crates/commission-testkit/` (package `b10x-commission-testkit`). The crate holds one module per
fake, and the public adapter kits go under `src/kits/` (`story:adapter-conformance-suites`). Its
first module is `crates/commission-testkit/src/fake_governor.rs`: a scripted fake governor whose
script sets each case's revision, frontier and determination per call and can make a call fail.
The testkit depends on `b10x-commission`, and `b10x-commission` does not depend on the testkit.
Tests that use a fake therefore live in `crates/commission-testkit/tests/`. The workspace's
`members = ["crates/*"]` takes the new crate in without an edit.

## Shared surface

This story is link 3 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:frontier-admission`, and
`story:agent-executor-port` depends on it. The full order is in
`story:generated-responsibility-model` § Shared surface. Two more files are edited along the same
chain: `crates/commission/src/lib.rs` and `ports/mod.rs`, and `Cargo.lock`.

## ESS

The port's error and determination vocabulary are model types. Declare both in
`ess/domains/responsibility.yaml` before any code: an error enum (unknown case, governor
unavailable) and a determination union (open, or complete carrying the governor's outcome). Then
pass `ess specify validate --path ess` and regenerate with `task generate`. Neither is a command,
so this adds no conformance scenario.

## Domain relations

- Commission -> Case, many-to-one, references: `ess/domains/responsibility.yaml:149-153`,
  `commission.responsibility.Commission` relation `case`. A case may hold many commissions
  (`:126-127`), and the port answers per case, whichever commission asks.
- Frontier -> Case, many-to-one, references, carrying `case_revision`: `:195-199`,
  `commission.responsibility.Frontier` relation `case`.

## Scope

- `crates/commission/src/ports/mod.rs` (new)
- `crates/commission/src/ports/governor.rs` (new)
- `crates/commission/src/lib.rs`
- `crates/commission-testkit/Cargo.toml` (new)
- `crates/commission-testkit/src/lib.rs` (new)
- `crates/commission-testkit/src/fake_governor.rs` (new)
- `crates/commission-testkit/tests/governor_port.rs` (new)
- `Cargo.lock`
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

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

- Canon: no change.
- Only the governor determines completion, never the executor
  (`docs/contracts/commission-executor.md:50-52`). This port is where that determination enters
  Commission.

## Source

TASKBOARD M-003 (build pack `TASKBOARD.md` § Commission); Atlas ADR 0069 (AEP is one governor
implementation); `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 21.
