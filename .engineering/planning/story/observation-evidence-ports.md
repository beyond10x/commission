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
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
- depends_on: story:agent-executor-port
scope:
- confidence: cited
  path: crates/commission-testkit/src/fake_governor.rs
- confidence: inferred
  path: crates/commission-testkit/tests/observation_evidence.rs
- confidence: cited
  path: crates/commission/src/ports/evidence.rs
revision: 11
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"review_outcome":3}}}
---
## Outcome

Observations and evidence reach the governor through two separate ports. Both live in their own
module, `crates/commission/src/ports/evidence.rs`, which `story:port-skeleton` creates empty with
its `mod` line:

- the **observation port** records an observation, which is a raw report: source, subject, time and
  payload;
- the **evidence port** submits evidence, which is typed and attributable: kind, subject revision,
  producer, facts, provenance, and the observations it interprets.

Observation and evidence are distinct generated types. Nothing in Commission converts an
observation, an executor output or a trace into evidence (Atlas ADR 0074; `AGENTS.md` § Rules;
`docs/contracts/evidence.md:34-46`).

**Producer.** The producer of submitted evidence is supplied by the trusted caller beside the
payload. It is never read out of the payload or out of model output (`AGENTS.md` § Rules).

**Observations behind an evidence record.** An evidence record references one or more
observations through `observation_ids`. This is the operator's decision of 2026-10-04,
`decision-blocker:evidence-observation-link`, now cleared. It is declared at
`ess/domains/responsibility.yaml:278-279` and `:286-292`. The evidence port refuses a record that
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
output to the observation port is wired by `story:local-runtime-loop`.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on:

- `story:port-skeleton`, for its module file and the `Observation` and `Evidence` fields;
- `story:governor-port`, a real dependency: this story extends that story's fake governor in
  `crates/commission-testkit/src/fake_governor.rs`, so it needs the fake's type and edits its file;
- `story:agent-executor-port`, a real dependency: the acceptance runs with that story's scripted
  fake executor.

Its old edge on `story:authority-provider-port` was ordering only and is gone. It runs beside
`story:stale-revision-action-request` and `story:run-outcomes`, which do not touch
`fake_governor.rs`. `story:local-runtime-loop` and `story:adapter-conformance-suites` depend on it.

## ESS first

- **Specification change: none in this story.** It relies on the declarations
  `story:port-skeleton` lands from this story's former § ESS (`docs/contracts/evidence.md:7-32`):
  `Observation` gains `observed_at: Timestamp` and `payload: Json`; `Evidence` gains `facts: Json`
  and `provenance: Json`; the `observations` relation is unchanged and no relation from
  `Observation` is added. None is a command, so there is no conformance scenario to add.
- **First commit, red.** The test `observation_and_evidence_stay_apart` alone, in
  `crates/commission-testkit/tests/observation_evidence.rs`. It is red because `ports::evidence`
  declares no observation port, evidence port or `EvidenceAdapter`, and the fake governor
  implements neither port: the test does not compile.
- **Then.** The two ports, the adapter trait, the typed refusal and the fake governor's two
  implementations, which make it pass.

## Domain relations

- Evidence -> Case, many-to-one, references, carrying `subject_revision`:
  `ess/domains/responsibility.yaml:281-285`, `commission.responsibility.Evidence` relation `case`.
- Evidence -> Observation, one evidence record to one or more observations, references, via
  `observation_ids`: `:286-292`, relation `observations`.

## Scope

- `crates/commission/src/ports/evidence.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-testkit/src/fake_governor.rs` (from `story:governor-port`; extended here)
- `crates/commission-testkit/tests/observation_evidence.rs` (new)

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
ADRs 0074, 0080; `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 26.
