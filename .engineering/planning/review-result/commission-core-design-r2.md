---
format: aep.planning-md/3
id: review-result:commission-core-design-r2
kind: review-result
status: active
title: Commission core decomposition — design critic, round 2
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

- story:authority-provider-port — its body says the AuthorityDecision ownership question "is still open" and tells the implementor to "leave the AuthorityDecision entity and its marker as they are"; the blocker is cleared, the ESS marker is already gone, and `story:stale-revision-action-request` now owns the relation, so the body should say the question is decided and that the relation is declared by `story:stale-revision-action-request` (later in the chain, order already recorded by `depends_on`) — `.engineering/planning/story/authority-provider-port.md:81-85` against `.engineering/planning/story/stale-revision-action-request.md:128-132` and `ess/domains/responsibility.yaml:253-256`

**What I read.** 12 artifacts in full: the epic, the 11 stories, the `commission-core-design-r1` result, and the blockers `authority-decision-owner` and `evidence-observation-link`. I ran `aep plan artifact relations`, `list`, `graph`, `show`, `blocked` and `validate`; `validate` reports `valid`, so it adds no finding. I walked all of the graph's `depends_on` and `decomposes` edges (about 60), including the edges to `epic:governor-adapter`, `epic:vertical-slices` and the visions. The `depends_on` edges are acyclic, and `epic:governor-adapter` and `epic:vertical-slices` depend on the epic and the epic never depends back.

**Round-1 findings.** All three are closed:
- **`SuspensionReason`:** `story:agent-executor-port` now declares it and `story:run-outcomes` only consumes it (`agent-executor-port.md` § Type ownership; `run-outcomes.md` "declared by `story:agent-executor-port`").
- **`ProposedActionArguments`:** the story owns it, and `story:stale-revision-action-request` has the edge `depends_on story:agent-executor-port`.
- **Blockers on the epic:** the three blockers are cleared, and `aep plan artifact blocked` lists none on `epic:commission-core`.

**What I could not establish.**
- **Chain trade-off (not a finding).** The 11 stories now form one total order, `generated-responsibility-model` through `adapter-conformance-suites`, with a direct edge between each consecutive pair. Every body states the reason, the shared `ess/domains/responsibility.yaml` and `generated/rust/commission/` surface (`generated-responsibility-model.md` § Shared surface), so it is not a defect. The options are to keep the order, or to split the specification into per-story domain files and generated directories so the stories can run in parallel. I do not ask for an edge to be removed.
- **Out of my lane (acceptance).** `story:stale-revision-action-request` carries the AuthorityDecision relation only in a trailing section (line 128). Its Acceptance has no expectation for it beyond `ess specify validate` passing.
- **Out of my lane (acceptance or scope).** `story:observation-evidence-ports` acceptance 1 has an executor output "reach the fake governor as exactly one Observation", but no story's body names what produces that Observation. `story:local-runtime-loop` does not record observations.

```findings
- file: .engineering/planning/story/authority-provider-port.md
  line: 81
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the body says the AuthorityDecision ownership question is still open and tells the implementor to leave the entity and its marker as they are, but decision-blocker:authority-decision-owner is cleared, the ESS marker is gone, and story:stale-revision-action-request now declares the relation, so the body should say the question is decided and name that story as the owner of the relation
```
