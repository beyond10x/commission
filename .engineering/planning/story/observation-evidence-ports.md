---
format: aep.planning-md/3
id: story:observation-evidence-ports
kind: story
status: proposed
title: Observation and evidence reach the governor through separate ports
summary: Separate observation and evidence ports in ports/evidence.rs; evidence names one or more observations; nothing in Commission turns an observation or trace into evidence.
refs:
- provider: taskboard
  reference: M-006
relations:
- decomposes: epic:commission-core
- depends_on: story:governor-port
- depends_on: story:authority-provider-port
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/commission-testkit/src/fake_governor.rs
- confidence: inferred
  path: crates/commission-testkit/tests/observation_evidence.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: crates/commission/src/ports/evidence.rs
- confidence: cited
  path: crates/commission/src/ports/mod.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 9
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"review_outcome":3}}}
---
## Outcome

Observations and evidence reach the governor through two separate ports. Both live in their own
module, `crates/commission/src/ports/evidence.rs`:

- the **observation port** records an observation, which is a raw report: source, subject, time and
  payload;
- the **evidence port** submits evidence, which is typed and attributable: kind, subject revision,
  producer, facts, provenance, and the observations it interprets.

Observation and evidence are distinct generated types. Nothing in Commission converts an
observation, an executor output or a trace into evidence (Atlas ADR 0074; `AGENTS.md:31-32`;
`docs/contracts/evidence.md:34-46`).

**Producer.** The producer of submitted evidence is supplied by the trusted caller beside the
payload. It is never read out of the payload or out of model output (`AGENTS.md:25-27`).

**Observations behind an evidence record.** An evidence record references one or more
observations through `observation_ids`. This is the operator's decision of 2026-10-04,
`decision-blocker:evidence-observation-link`, now cleared. It is declared at
`ess/domains/responsibility.yaml:234-235` and `:242-248`. The evidence port refuses a record that
names no observation, with a typed error.

**Evidence adapter.** With that cardinality decided, the evidence-adapter port is no longer blocked.
It is in this story as the `EvidenceAdapter` trait in the same module. An adapter interprets one or
more observations into evidence records, and each record carries the `observation_ids` it read
(`docs/contracts/evidence.md:42`). An adapter is a verifier that the trusted integration supplies.
Commission's runtime never calls one on executor output.

**Fake governor.** The fake governor (`crates/commission-testkit/src/fake_governor.rs`, from
`story:governor-port`) implements both ports. It records what each port receives.

**Delivery from the loop.** This story builds the observation port and takes an executor output at
it; it does not build the path that carries executor output there. The loop's delivery of executor
output to the observation port is wired by `story:local-runtime-loop`, later in the chain.

## Shared surface

This story is link 7 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:authority-provider-port`, and
`story:stale-revision-action-request` depends on it. The whole order is in
`story:ess-hard-gate` § Shared surface.

The same chain orders these edits:

- `crates/commission/src/lib.rs`
- `ports/mod.rs`
- `crates/commission-testkit/src/fake_governor.rs`

## ESS

Change `commission.responsibility.Observation` (`ess/domains/responsibility.yaml:205-218`) and
`commission.responsibility.Evidence` (`:220-252`) first. Add the fields
`docs/contracts/evidence.md:7-32` names that are missing: an observation's time and payload, and
evidence's facts and provenance. Then pass `ess specify validate --path ess` and regenerate with
`task generate`. Keep the `observations` relation exactly as declared. Add no relation from
Observation to anything. None of these changes is a command, so the conformance suite gains no
scenario.

## Domain relations

- Evidence -> Case, many-to-one, references, carrying `subject_revision`:
  `ess/domains/responsibility.yaml:237-241`, `commission.responsibility.Evidence` relation `case`.
- Evidence -> Observation, one evidence record to one or more observations, references, via
  `observation_ids`: `:242-248`, relation `observations`.

## Scope

- `crates/commission/src/ports/evidence.rs` (new)
- `crates/commission/src/ports/mod.rs`
- `crates/commission/src/lib.rs`
- `crates/commission-testkit/src/fake_governor.rs`
- `crates/commission-testkit/tests/observation_evidence.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

## Acceptance

The test `observation_and_evidence_stay_apart` in
`crates/commission-testkit/tests/observation_evidence.rs` passes with the fake governor and the fake
executor. It checks these expectations:

1. The observation port, called with a fake executor output that says tests passed, delivers
   exactly one `Observation` to the fake governor. The governor's evidence record stays empty.
2. Evidence whose payload claims producer `P2` is submitted with the trusted caller supplying `P1`.
   It arrives carrying `P1`.
3. A test-local `EvidenceAdapter` interprets two recorded observations into one evidence record.
   Submitted through the evidence port, the record arrives with `observation_ids` naming both
   observations.
4. An evidence record with an empty `observation_ids` is refused with the port's typed error. The
   fake governor records nothing for it.

## Notes

- Canon: no change.

## Source

TASKBOARD M-006 (build pack `TASKBOARD.md` § Commission); `docs/contracts/evidence.md`; Atlas
ADR 0074; `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 26.
