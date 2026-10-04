---
format: aep.planning-md/3
id: epic:vertical-slices
kind: epic
status: proposed
title: 'Vertical slices: software change, incident, suspend/resume'
summary: Integration tests I-001 to I-003 through Commission and Loom on the real AEP governor.
refs:
- provider: atlas
  reference: epic:ga-vertical-slices
relations:
- depends_on: epic:governor-adapter
- serves: vision:governed-autonomy
- serves: vision:O1
- serves: vision:O2
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:55Z", actor: "human:timo", revision: 2}
---
## Outcome

The end-to-end demonstrators, as integration tests under `tests/`. Covers TASKBOARD I-001 … I-003.

## Acceptance

As Atlas `epic:ga-vertical-slices` states it, per slice: I-001 stale R1 evidence, blocked merge,
projected actions changing after R2 tests, authority outside the model, stale-proposal refusal, no
unprojected action executed; I-002 incident leaves emergency mode with cause `UNKNOWN`; I-003
suspend at merge approval and resume after a process restart at the same case revision.

## Source

Atlas `epic:ga-vertical-slices`; build pack `START-HERE.md`.
