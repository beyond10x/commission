---
format: aep.planning-md/3
id: review-result:commission-core-scope-r2
kind: review-result
status: active
title: Commission core decomposition — scope critic, round 2
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

story:adapter-conformance-suites — the story narrows M-010 ("Add conformance tests for adapters") to the Governor and AuthorityProvider kits and names no kit for the `EvidenceAdapter` trait that `story:observation-evidence-ports` now adds to this set; its "Not in this story" list (suspend/resume, AgentExecutor suite) leaves the evidence-adapter suite neither claimed nor excluded, although the history design § 52 lists "Evidence-adapter conformance" beside the two it covers — .engineering/planning/story/adapter-conformance-suites.md:52-58 (cf. .engineering/planning/story/observation-evidence-ports.md:56, docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1944)

**What you need to change.** Either add an evidence-adapter kit to the story, or add a third "Not in this story" bullet that states the evidence-adapter suite is dropped and why. The second option is enough to clear the finding.

**Round-1 items.** I checked these against the revised bodies and none holds any more.
- **Resume.** The Suspended state and the suspend and resume commands in `story:run-outcomes` are not reach beyond the parent. Cleared `decision-blocker:suspended-run-continuity` assigns them to M-007. Persisting a suspended run across a restart stays with `story:approval-suspend-resume-slice`.
- **Evidence adapter.** `EvidenceAdapter` in `story:observation-evidence-ports` is inside M-006. `decision-blocker:evidence-observation-link` names it as M-006's blocked part and it is now cleared.
- **AuthorityDecision relation.** The `AuthorityDecision` → `ActionRequest` relation in `story:stale-revision-action-request` is assigned to that story by cleared `decision-blocker:authority-decision-owner`. It belongs to the Commission ESS spec (I-007), which the parent covers.
- **Loop refusals.** The stale and absent-from-frontier refusals in `story:local-runtime-loop` are the epic-level integration check over the unit claims in `story:stale-revision-action-request` and `story:frontier-admission`. They are not a duplicated outcome.

**What I read:** 12 artifacts, all with whole bodies (`epic:commission-core` and the 11 stories). Commands run: `aep plan artifact show` on each and on `review-result:commission-core-scope-r1`, `aep plan artifact graph`, `aep plan artifact show` on `epic:governor-adapter`, `story:approval-suspend-resume-slice` and four decision-blockers, `epic:ga-commission-core` in the Atlas store, `grep` on the TASKBOARD and the history design § 52, and `grep -n` on the two story files cited above for line numbers.

**Promises extracted from the parent: 11 TASKBOARD items (M-001…M-010, I-007) plus 6 acceptance and Rule clauses, 17 in all.**
- **Traced to a story:** 16 of 17.
- **Traced only partly:** 1, M-010, which has the finding above.

The acceptance and Rule clauses:
- the fake-driven loop
- the refuse-absent clause
- the refuse-stale clause
- blocked, suspended and completed outcomes
- the `cargo tree` clause
- the `task check` conformance clause
- generated types, per the Rule

**What I could not establish (out of my lane, did not set the verdict):**
- **Design lane:** `story:authority-provider-port` says `AuthorityDecision` and its marker are left alone and `decision-blocker:authority-decision-owner` is "still open". That blocker is now cleared, and `story:stale-revision-action-request` makes the relation. The wording is stale (`.engineering/planning/story/authority-provider-port.md`, ESS section).
- **Acceptance or design lane:** the stories cite `UNMAPPED` marker lines in `ess/domains/responsibility.yaml`, such as `:183-184`, `:255-256` and `:204-205`. I did not re-check them against the current file.

```findings
- file: .engineering/planning/story/adapter-conformance-suites.md
  line: 52
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the story narrows M-010 ("Add conformance tests for adapters") to the Governor and AuthorityProvider kits and names no evidence-adapter kit, although story:observation-evidence-ports now adds an EvidenceAdapter trait to this set; its "Not in this story" list leaves the evidence-adapter suite neither claimed nor excluded, so the plan says less than the parent and nothing records the decision
```
