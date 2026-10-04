---
format: aep.planning-md/3
id: epic:governor-adapter
kind: epic
status: proposed
title: AEP governor adapter
summary: Governor adapter for AEP, held to Commission's governor conformance tests (M-011); M-012 recorded as deferred.
refs:
- provider: atlas
  reference: epic:ga-aep-governor
relations:
- depends_on: epic:commission-core
- serves: vision:governed-autonomy
- serves: vision:O1
- serves: vision:O2
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:55Z", actor: "human:timo", revision: 3}
---
## Outcome

The AEP adapter behind Commission's Governor port. Covers TASKBOARD M-011; M-012 (managed
composition) is recorded here as deferred and unscheduled.

## Acceptance

Against the Governor port with a scripted executor, the AEP adapter passes Commission's governor
conformance tests on the ELS `software.change/1` fixture case, including a stale-revision rejection
and a suspend/resume across a process restart.

## Source

Atlas `epic:ga-aep-governor`; Atlas ADR 0069.

## Also owns

Evidence submission to AEP and the evidence-adapter conformance kit (history design § 52,
`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1944`) belong here: they were
deferred out of `story:adapter-conformance-suites` on 2026-10-04 because the only evidence adapter
before phase 6 is the fake. Stories for them are drafted when this epic is decomposed again after
`decision-blocker:aep-governed-case` is answered.
