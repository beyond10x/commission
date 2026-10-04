---
format: aep.planning-md/3
id: review-result:adversary-w6-commission-stale-revision-action-request-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w6 adversary, commission story:stale-revision-action-request, pass 2
relations:
- reviews: story:stale-revision-action-request
revision: 1
---
unit: commission/stale-revision-action-request, working tree c235b0e + phase 2 + pass-1 files, in ~/.local/state/worktree/trees/b10x/commission/commission-w6-stale-revision-action-request
verdict: CONFIRMED (2 red cases, no blocker)
cases: executed 179→184, red 2
origin: introduced 7 / pre-existing 0 / undecided 0
wrote-outside-worktree: 7 paths in the assigned scratch dir (listed in part 6)
needs-coordinator: whether F1 is fixed in the code (move the case check first) or in the doc comment, and whether F2's command input should gain `action_request_id` in this unit

**Verdict:** two of my new cases fail. The other three pass and catch changes the existing suite let through. No finding blocks the unit.

**1. `git --no-pager diff --stat`**

The tracked diff has no lines from me; it is the 12 files the implementor and phase 1 left. My only writes are two new untracked test files:
`?? crates/commission-testkit/tests/adversary2_request_model.rs`
`?? crates/commission-testkit/tests/adversary2_request_revalidation.rs`
I wrote nothing outside test files.

**2. Cases added.** I ran these alone, before the suite: `cargo test --locked -p b10x-commission-testkit --test adversary2_request_revalidation --test adversary2_request_model`.

| case | asserts | now |
|---|---|---|
| `adversary2_request_revalidation.rs:76` revision_behind_the_request_is_stale | request at N+1, governor revision N, frontier N+1 → `Stale{21,20}`, and only `CurrentRevision` is called | green; fails if `:61` becomes `current > expected` (that variant returns Admitted) |
| `:98` frontier_behind_the_request_is_stale | revision N+1, frontier N → `Stale{21,20}` | green; fails if `:67` becomes `>` |
| `:135` other_case_frontier_at_another_revision_is_not_admitted | frontier for another case at revision 3 → NotAdmitted naming both cases (doc `action_request.rs:15-16`) | **red** |
| `adversary2_request_model.rs:165` relations_match_the_story | compiled model has `Run.requests` (owns/many/run_id), `ActionRequest.case` (references/one/case_id), the AuthorityDecision relation, and a required `action_request_id: ActionRequestId`. It also deletes `Run.requests` in a copy of `ess/` and confirms the check then fails | green; the in-test mutation shows it can fail |
| `:218` revalidation_input_is_the_request | the command input holds every field of the request, including its identity | **red** |

The red output, verbatim, from that first run:
```
thread 'adversary2_request_revalidation_input_is_the_request' panicked at crates/commission-testkit/tests/adversary2_request_model.rs:228:5:
RevalidateActionRequest's input is not the request: it lacks ["action_request_id"] (input ["run_id", "case_id", "expected_case_revision", "action", "arguments"], request ["action_request_id", "run_id", "case_id", "expected_case_revision", "action", "arguments"])
thread 'adversary2_request_other_case_frontier_at_another_revision_is_not_admitted' panicked at crates/commission-testkit/tests/adversary2_request_revalidation.rs:151:18:
a frontier for another case, at that case's revision 3, must be refused as not admitted (action_request.rs:15-16); got Stale { error: ActionRequestStale { expected_case_revision: 20, current_case_revision: 3 } }
```
After that run I applied `rustfmt` to my two files only, which moved the line numbers. `cargo fmt --check` and clippy `-D warnings` on both binaries then exited 0.

**3. Suite run.** `cargo test --workspace --locked --no-fail-fast`, with the brief's `CARGO_TARGET_DIR` and `CARGO_INCREMENTAL=0`. Full log in `scratch/adv2/suite.log`.
```
test adversary2_request_revalidation_input_is_the_request ... FAILED   (adversary2_request_model.rs:244:5)
test result: FAILED. 1 passed; 1 failed; ...
test adversary2_request_other_case_frontier_at_another_revision_is_not_admitted ... FAILED   (adversary2_request_revalidation.rs:155:18)
test result: FAILED. 2 passed; 1 failed; ...
EXIT=101    summed: passed 182 failed 2
```
I did not run a separate suite with my files deselected. The `179` is this run's 184 minus the 5 cases in my two new binaries.

**4. Findings that are judgement only**
- **J1** (INFEASIBLE, note, introduced): `request()` (`crates/commission/src/action_request.rs:34-49`) takes only a `RunId`.
  - So it cannot check the frontier against the run's `case_revision`. `ess/domains/responsibility.yaml:386` says "a proposal made on another revision is stale".
  - It also cannot check the frontier's case against the run's commission's case. `ActionRequest.case_id` and Run→Commission→case are two routes to a case, and nothing requires them to agree.
  - What reaches it: no non-test caller of `request()` exists today. The later story `local-runtime-loop` will be the first.
- **J2** (CONFIRMED, note, introduced): the generated `RevalidateActionRequestBehavior` (`generated/rust/commission/src/responsibility.rs`, obligations module) has no implementation.
  - Its 4 new scenarios are neither answered nor listed in `ess/SKIPPED.md`. `adversary2_run_conformance.rs:548` leaves them out by name prefix.
  - Nothing breaks because of it: drift compares the generated tree only, no-hand-model checks names only, and `ess_gate` counts refusals only.
  - The story leaves this to `commission-ess-conformance`.

**5. Attacked, could not break**
- **Relations vs story decisions:** the compiled `Run.requests`, `ActionRequest.case` and `AuthorityDecision.action_request` match the story exactly.
- **`AuthorityDecision.action_request_id`:**
  - It is required, not optional.
  - Nothing in `crates/` builds `AuthorityDecisionData` (grep), so no code sets it wrongly.
  - `authority.rs:11-12` says the record is written "elsewhere".
- **Unimplemented generated trait:** nothing in the code depends on it to compile.
- **Narrowing the existing conformance suite:** the base suite had 9 scenarios, all of which still match the Run prefixes, so the filter drops none of them (`scratch/adv2/base-ids.txt` vs `head-ids.txt`).
- **"Revalidation changes nothing":** both governor calls only read.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w6/commission-stale-revision-action-request/scratch/adv2/` holds `base-ess/`, `base-suite.json`, `head-suite.json`, `base-ids.txt`, `head-ids.txt`, `model.json` and `suite.log`.
- The test's mutated copy in `CARGO_TARGET_TMPDIR/adversary2_request_relations_mutant` is deleted by the test itself and is gone.
- The build used only the brief's `CARGO_TARGET_DIR`. The disk is at 5.6G free, still below the 10G floor.

**7.**
```findings
- file: crates/commission/src/action_request.rs
  line: 67
  category: contract-drift
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "A frontier for another case at a different revision is refused as Stale carrying the other case's revision as current_case_revision, while the doc at :15-16 promises NotAdmitted naming both cases; the fix is to check the case (:70-80) before the revision (:67-69) or reword the doc, and only a misbehaving governor reaches it (adversary2_request_revalidation.rs:135, red)."
- file: ess/domains/responsibility.yaml
  line: 642
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "The RevalidateActionRequest input leaves out action_request_id although :496 says revalidation takes the request as its input, so a needs-authority outcome cannot name the request an AuthorityDecision must reference (adversary2_request_model.rs:218, red)."
- file: crates/commission/src/action_request.rs
  line: 61
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Changing current != expected to current > expected passed the unit's suite because no case had the governor behind the request; adversary2_request_revalidation.rs:76 now catches it."
- file: crates/commission/src/action_request.rs
  line: 67
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Changing issued.case_revision != expected to > passed the unit's suite because no case had a frontier older than the request; adversary2_request_revalidation.rs:98 now catches it."
- file: ess/domains/responsibility.yaml
  line: 389
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Deleting the Run.requests or ActionRequest.case relation that the story decides left every gate step green; adversary2_request_model.rs:165 now checks both and shows on a mutated copy that it can fail."
- file: crates/commission/src/action_request.rs
  line: 34
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "request() takes only a RunId, so it cannot check the frontier's revision against Run.case_revision (yaml:386, where a proposal made on another revision is stale) or the frontier's case against the run's commission's case, and nothing outside the tests calls it yet."
- file: generated/rust/commission/src/responsibility.rs
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Nothing implements RevalidateActionRequestBehavior and its 4 conformance scenarios are neither answered nor in ess/SKIPPED.md; no gate depends on either, and the story leaves both to commission-ess-conformance."
```
