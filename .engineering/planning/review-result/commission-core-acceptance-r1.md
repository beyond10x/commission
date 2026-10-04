---
format: aep.planning-md/3
id: review-result:commission-core-acceptance-r1
kind: review-result
status: active
title: Commission core decomposition — acceptance critic, round 1
relations:
- reviews: epic:commission-core
- reviews: story:adapter-conformance-suites
- reviews: story:agent-executor-port
- reviews: story:authority-provider-port
- reviews: story:commission-ess-conformance
- reviews: story:frontier-admission
- reviews: story:generated-responsibility-model
- reviews: story:governor-port
- reviews: story:local-runtime-loop
- reviews: story:observation-evidence-ports
- reviews: story:run-outcomes
- reviews: story:stale-revision-action-request
revision: 1
---
needs-revision
story:stale-revision-action-request — the acceptance never observes the ESS action-request command that `story:commission-ess-conformance` needs for its "at least one passed scenario" gate, so this story can close with the dependent's acceptance unsatisfiable — .engineering/planning/story/stale-revision-action-request.md:49 (ESS section :29-31; .engineering/planning/story/commission-ess-conformance.md:42)
story:frontier-admission — the title and summary promise to close the Frontier contents marker, but the acceptance is a unit test over hand-built Canon frontiers that passes with the marker still in the spec — .engineering/planning/story/frontier-admission.md:51 (Closes paragraph :30)
story:run-outcomes — the clause "carrying the provider's request for an approval-required decision" attaches to no named outcome (the Outcome calls it "needs authority"), and the needs-human-judgment and needs-external-evidence outcomes have no row — .engineering/planning/story/run-outcomes.md:49
story:adapter-conformance-suites — the failing case "an authority provider whose error is read as an allow" names what Commission does with an error, not what a broken provider fake does, so two people would build different broken fakes — .engineering/planning/story/adapter-conformance-suites.md:52
story:agent-executor-port — the acceptance joins a fake-executor test and a dependency-guard failure with "and", which are independent outcomes with different mechanisms, so one can pass while the other fails — .engineering/planning/story/agent-executor-port.md:44
story:governor-port — the acceptance joins "frontier at N+1 on the second call" and "typed unknown-case error" with "and", and names no check for the completion determination the Outcome says the port returns — .engineering/planning/story/governor-port.md:43
story:observation-evidence-ports — the acceptance joins "receives the output only as an Observation and no Evidence" and "evidence arrives with the trusted caller's producer" with "and", which are independent invariants — .engineering/planning/story/observation-evidence-ports.md:48
story:authority-provider-port — the acceptance joins pass-through of allow, deny and approval-required with "returns a refusal, never an allow, when the provider returns an error", which are independent outcomes — .engineering/planning/story/authority-provider-port.md:47
story:local-runtime-loop — the one sentence lists about seven independent outcomes (load, frontier, invoke, refuse absent, refuse superseded, then completed, suspended and no-admissible-action endings), and "the outcomes its scripts lead to" is circular — .engineering/planning/story/local-runtime-loop.md:48
story:generated-responsibility-model — the acceptance joins a regeneration-drift gate with "passes on a tree where ... none of them defined by hand", and nothing in `task check` fails when a hand-written `AgentId` remains — .engineering/planning/story/generated-responsibility-model.md:52

Read: 14 artifacts (the 11 stories that decompose `epic:commission-core`, plus the 3 decision-blockers that block it, and `epic:commission-core` itself) via `aep plan artifact list`, `graph` and `show`. I also read `ess/domains/responsibility.yaml`, `crates/commission/src/lib.rs`, `Taskfile.yml`, `docs/contracts/commission-executor.md` and the Canon pin `cf29c4b`. `ess verify conform synthesize` and `ess generate synthesize --target rust` exist in ess 0.52.0. `aep plan artifact validate` reports valid.

Could not establish:
- **Stale line citations.** Citations to `ess/domains/responsibility.yaml` in story and blocker bodies are 16 lines off: the file has 244 lines, and the cited `:246-247` is past its end. The UNMAPPED markers are actually at lines 115, 117, 165, 188 and 230. This is outside my lane (bodies, not acceptance) and did not affect my verdict.
- **Overlap.** `story:local-runtime-loop`, `story:stale-revision-action-request` and `story:run-outcomes` each claim the stale-refusal or derived-outcome result, which belongs to the scope and parallel-safety critics.
- **ESS-first change.** Most acceptances do not observe the `ess/` change their own ESS section requires. I judged it only where a dependent story or the title makes it load-bearing, as in the first two findings.
- **Decision-blockers.** I found no acceptance defect in the three: "What Would Clear It" is observable through `ess specify validate --path ess`.

```findings
[
  {"file": ".engineering/planning/story/stale-revision-action-request.md", "line": 49, "category": "acceptance", "severity": "blocker", "verdict": "needs-revision", "origin": "introduced", "message": "the acceptance never observes the ESS action-request command that story:commission-ess-conformance needs for its 'at least one passed scenario' gate, so this story can close with the dependent's acceptance unsatisfiable"},
  {"file": ".engineering/planning/story/frontier-admission.md", "line": 51, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the title and summary promise to close the Frontier contents marker, but the acceptance is a unit test over hand-built Canon frontiers that passes with the marker still in the spec"},
  {"file": ".engineering/planning/story/run-outcomes.md", "line": 49, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the clause 'carrying the provider's request for an approval-required decision' attaches to no named outcome (the Outcome calls it 'needs authority'), and the needs-human-judgment and needs-external-evidence outcomes have no row"},
  {"file": ".engineering/planning/story/adapter-conformance-suites.md", "line": 52, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the failing case 'an authority provider whose error is read as an allow' names what Commission does with an error, not what a broken provider fake does, so two people would build different broken fakes"},
  {"file": ".engineering/planning/story/agent-executor-port.md", "line": 44, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the acceptance joins a fake-executor test and a dependency-guard failure with 'and', which are independent outcomes with different mechanisms, so one can pass while the other fails"},
  {"file": ".engineering/planning/story/governor-port.md", "line": 43, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the acceptance joins 'frontier at N+1 on the second call' and 'typed unknown-case error' with 'and', and names no check for the completion determination the Outcome says the port returns"},
  {"file": ".engineering/planning/story/observation-evidence-ports.md", "line": 48, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the acceptance joins 'receives the output only as an Observation and no Evidence' and 'evidence arrives with the trusted caller's producer' with 'and', which are independent invariants"},
  {"file": ".engineering/planning/story/authority-provider-port.md", "line": 47, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the acceptance joins pass-through of allow, deny and approval-required with 'returns a refusal, never an allow, when the provider returns an error', which are independent outcomes"},
  {"file": ".engineering/planning/story/local-runtime-loop.md", "line": 48, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the one sentence lists about seven independent outcomes (load, frontier, invoke, refuse absent, refuse superseded, then completed, suspended and no-admissible-action endings), and 'the outcomes its scripts lead to' is circular"},
  {"file": ".engineering/planning/story/generated-responsibility-model.md", "line": 52, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the acceptance joins a regeneration-drift gate with 'passes on a tree where ... none of them defined by hand', and nothing in task check fails when a hand-written AgentId remains"}
]
```
