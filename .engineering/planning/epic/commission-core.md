---
format: aep.planning-md/3
id: epic:commission-core
kind: epic
status: active
title: 'Commission core: responsibility contracts over fakes'
summary: ESS-led Agent, Case, Commission, Frontier, Governor, AgentExecutor, AuthorityProvider, ports and outcomes, with a fake runtime loop.
refs:
- provider: atlas
  reference: epic:ga-commission-core
relations:
- serves: vision:governed-autonomy
- serves: vision:O1
- serves: vision:O2
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 2, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 3, decided_on: {"recorded":{"review_outcome":1}}}
---
## Outcome

The responsibility model and runtime contracts, led by the ESS specification in `ess/`. Covers
TASKBOARD M-001 … M-010 and I-007 (the Commission ESS specification).

## Acceptance

With a fake governor and a fake executor, a Commission loads a case, obtains a frontier, invokes the
executor, refuses a proposed action absent from the current frontier or proposed against a stale case
revision, and represents blocked, suspended and completed outcomes; `cargo tree` shows no
model-provider crate; `task check` runs the Commission ESS conformance suite.

## Rule

Model types come from `ess generate synthesize`, not hand transcription (ess:specifying). The
bootstrap types in `crates/commission` are replaced by generated ones as the stories land.

## Source

Atlas `epic:ga-commission-core`; Atlas ADRs 0070, 0075; `docs/contracts/`.
