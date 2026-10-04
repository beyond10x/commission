---
format: aep.planning-md/3
id: review-result:commission-core-acceptance-r2
kind: review-result
status: active
title: Commission core decomposition — acceptance critic, round 2
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
story:observation-evidence-ports — expectation 1, "A fake executor output that says tests passed reaches the fake governor as exactly one `Observation`", names a delivery path from executor output to the observation port that no scope file or Outcome paragraph builds (the loop is link 9, after this story), so the test can only pass by wiring it in test code. The expectation should be restated at the observation port, or the delivering function should be named in the Outcome and scope — .engineering/planning/story/observation-evidence-ports.md:109 (scope :94-101)
story:local-runtime-loop — the Outcome's step 6, "It consults authority where the frontier requires it", has no expectation among the eight, so a loop that never calls the provider passes. An expectation is needed that an `ApprovalRequired` action reaches the fake provider and ends the run as needs-authority carrying `Q` — .engineering/planning/story/local-runtime-loop.md:40 (acceptance :90-104)
story:run-outcomes — the summary opens "Generated RunOutcome" and the ESS section requires a six-variant `union`, but no expectation observes that `RunOutcome` is declared in `ess/` or comes from the generated crate. The rows would pass with a hand-written enum, which is the defect round 1 found on `story:frontier-admission` (title promises the marker closes, test passes with it open) — .engineering/planning/story/run-outcomes.md:7 (acceptance :108-130)
story:stale-revision-action-request — the closing section "Authority decision relation" adds a new field, `AuthorityDecision.action_request_id`, with a `references` relation to `ActionRequest` (operator decision of 2026-10-04). Expectations 1-6 never read for it, so the field is absent before the story and nothing shows it present after — .engineering/planning/story/stale-revision-action-request.md:128-133 (acceptance :100-115)
story:generated-responsibility-model — the Outcome says the story creates `ess/SKIPPED.md` and that "no story depends on landing order to find it", but none of the four expectations observes the file. `story:commission-ess-conformance` expectation 3 would then pass vacuously on a missing file — .engineering/planning/story/generated-responsibility-model.md:72 (acceptance :130-139)

Read: 11 of 11 stories that decompose `epic:commission-core`, plus `epic:commission-core` and `review-result:commission-core-acceptance-r1`. Commands: `aep plan artifact list`, `aep plan artifact show` (each id), `aep plan artifact kinds`, `aep plan artifact lifecycle story` and `aep plan artifact validate` (valid, 33 artifacts). In the tree I read `ess/domains/responsibility.yaml`, `Taskfile.yml` and `Cargo.toml`. I ran `ess generate synthesize` and `ess verify conform synthesize` into a scratch directory, since deleted; the generated crate has `src/responsibility.rs`, and the suite has 0 scenarios today, which matches expectations 5 and 6 of `story:stale-revision-action-request`.

Round-1 outcomes. All ten round-1 findings landed:
- The multi-outcome sentences are now one named test or `task check` with numbered, individually observable expectations.
- `story:stale-revision-action-request` now observes the command and the 0-to-at-least-1 scenario transition.
- `story:frontier-admission` now checks that the marker is gone.
- `story:adapter-conformance-suites` now defines the broken fakes concretely.

Could not establish:
- Out of my lane: `story:authority-provider-port` (:73-77) still says `decision-blocker:authority-decision-owner` is "still open", but it is cleared and `ess/domains/responsibility.yaml:254-257` records the decision. This is a body inconsistency for the design or scope critic.
- Out of my lane: line citations to `ess/domains/responsibility.yaml` match the current uncommitted file (e.g. `:183-184`, `:165-180`), but I did not recheck every one.
- Unease, not findings: `story:local-runtime-loop` expectation 4 ("no effect is executed") asserts an absence for which no effect port exists. `story:agent-executor-port` expectation 1 checks that a fake returns what its script names. I treated both as easy but observable and did not flag them.

```findings
[
  {"file": ".engineering/planning/story/observation-evidence-ports.md", "line": 109, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "expectation 1 names an executor-output-to-observation delivery that no scope file or Outcome paragraph builds (the loop is a later link), so the test can only pass by wiring it in test code; restate it at the observation port or name the delivering function"},
  {"file": ".engineering/planning/story/local-runtime-loop.md", "line": 40, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the Outcome's step 6 'It consults authority where the frontier requires it' has no expectation among the eight, so a loop that never calls the authority provider passes"},
  {"file": ".engineering/planning/story/run-outcomes.md", "line": 7, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the summary promises a generated RunOutcome and the ESS section requires a six-variant union, but no expectation observes that RunOutcome is declared in ess/ or comes from the generated crate, so the rows pass with a hand-written enum"},
  {"file": ".engineering/planning/story/stale-revision-action-request.md", "line": 128, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the closing section adds AuthorityDecision.action_request_id and a references relation to ActionRequest, but expectations 1-6 never observe it, so nothing distinguishes the world before the story from the world after"},
  {"file": ".engineering/planning/story/generated-responsibility-model.md", "line": 72, "category": "acceptance", "severity": "warning", "verdict": "needs-revision", "origin": "introduced", "message": "the Outcome says the story creates ess/SKIPPED.md and that no story depends on landing order to find it, but none of the four expectations observes the file, so story:commission-ess-conformance expectation 3 can pass vacuously on a missing file"}
]
```
