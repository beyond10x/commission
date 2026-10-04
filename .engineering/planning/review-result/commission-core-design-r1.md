---
format: aep.planning-md/3
id: review-result:commission-core-design-r1
kind: review-result
status: active
title: Commission core decomposition — design critic, round 1
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

- `story:agent-executor-port` — its outcome must give `ExecutorOutcome::Suspended` a payload of type `SuspensionReason`, but `story:run-outcomes` (which `depends_on` the executor story) is the one that declares `SuspensionReason`; the body should either declare `SuspensionReason` itself, or `story:run-outcomes` should only consume it — `.engineering/planning/story/agent-executor-port.md:29` (payloads from `docs/contracts/commission-executor.md:40-42`), `.engineering/planning/story/run-outcomes.md:35`
- `story:stale-revision-action-request` — the request carries "the model-generated arguments" that `story:agent-executor-port` defines as the `ProposedAction` payload, and no edge or owner records which story declares that type; add `depends_on story:agent-executor-port` or state that the request owns its own arguments type — `.engineering/planning/story/stale-revision-action-request.md:20`, `.engineering/planning/story/agent-executor-port.md:31`
- `epic:commission-core` — three `blocks` edges (case-commission-cardinality, commission-principal-type, evidence-observation-link) sit on the epic, yet each blocker's body says no drafted story needs the answer and the blocked work (the Case-to-Commission relation, principal fields, the evidence-adapter port) belongs to no story. The epic therefore stays blocked after all 11 stories land. Either draft a deferred story per blocked part and point the edge at it, as `story:managed-composition` does, or move the edges off the epic — `aep plan artifact blocked`, `.engineering/planning/decision-blocker/case-commission-cardinality.md:35-39`

What I read: 15 artifacts in full (epic, the 11 stories, and the 3 core blockers) plus `authority-decision-owner`. I ran `aep plan artifact relations|list|graph|blocked|waves|validate` (validate reported `valid`), ADR 0075, `crates/commission/src/lib.rs` and `crates/commission/Cargo.toml`. I walked all 49 edges in the full graph, including `epic:governor-adapter`, `epic:vertical-slices` and the vision edges. The `depends_on` graph is acyclic, and the first three bullets are the only seams I found. It is not a serialising chain: `story:generated-responsibility-model` is the only root, and `story:authority-provider-port`, `story:frontier-admission` and `story:agent-executor-port` can start in parallel with different parents. The ports are cut vertically, each with its fake in the same story. ADR 0075 holds: the Cargo manifest depends only on `b10x-canon`, and `story:agent-executor-port` guards against `b10x-loom`.

What I could not establish:
- Out of my lane (parallel safety): every story edits `ess/domains/responsibility.yaml` and the generated tree, and no story records a scope (`aep plan artifact waves` reports 15 unassessed).
- Out of my lane (acceptance or scope): the "Once `story:commission-ess-conformance` has landed…" sentences are conditional, so stories that land earlier add no scenarios and nothing back-fills them. Today ess 0.52.0 synthesizes 0 scenarios until the action-request command exists.
- Unease, no nameable fix: the new port-decision type in `story:authority-provider-port` sits beside the untouched `AuthorityDecision` entity at `ess/domains/responsibility.yaml:232`, and the body does not name the new type.

```findings
- file: .engineering/planning/story/agent-executor-port.md
  line: 29
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: its outcome must give ExecutorOutcome::Suspended a SuspensionReason payload, but story:run-outcomes, which depends_on this story, is the one that declares SuspensionReason, so each story holds half of one type; the body should either declare SuspensionReason itself, or story:run-outcomes should only consume it
- file: .engineering/planning/story/stale-revision-action-request.md
  line: 20
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the request carries the model-generated arguments that story:agent-executor-port defines as the ProposedAction payload, and no edge or owner records which story declares that type; add depends_on story:agent-executor-port or state that the request owns its own arguments type
- file: aep plan artifact blocked
  line:
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the three decision-blocker blocks edges sit on epic:commission-core although each blocker's body says no drafted story needs the answer and the blocked work belongs to no story, so the epic stays blocked after all 11 stories land; either draft a deferred story per blocked part and point the edge at it, or move the edges off the epic
```
