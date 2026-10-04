---
format: aep.planning-md/3
id: review-result:adversary-w6-commission-stale-revision-action-request-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w6 adversary, commission story:stale-revision-action-request, pass 1
relations:
- reviews: story:stale-revision-action-request
revision: 1
---
unit: commission/stale-revision-action-request, working tree at c235b0e plus the uncommitted phase 2
verdict: CONFIRMED (warning): no red case. One suite gap lets a governor failure become `Admitted` without any test failing.
cases: executed 170→179, red 0
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 paths (part 6)
needs-coordinator: mutants were not run. Disk had 7.2G free on `/`, below the 10G build floor in the global rules, so I did not create the mutant build dir. I can run them once there is room.

**1. Diff.** `git --no-pager diff --stat` shows only the implementor's 12 tracked files (530+/30-), unchanged by me. My two files are untracked, so they do not appear in it. Both are test files: `crates/commission-testkit/tests/adversary_request_governor_errors.rs` and `crates/commission-testkit/tests/adversary_request_listing.rs`. I changed no implementation, `ess/` or `generated/` file.

**2. Cases added.** All are green now. Each one targets a mutant the existing suite cannot see. Run alone: `cargo test --locked -p b10x-commission-testkit --test adversary_request_governor_errors --test adversary_request_listing` gave `4 passed`, `5 passed`, EXIT=0.

| case | asserts | mutant it kills (by inspection, not run) |
|---|---|---|
| `unknown_case_fails_with_unknown_case` | `Err(UnknownCase)`; the governor log shows only `CurrentRevision` | `map_err(\|_\| GovernorUnavailable)` at `src/action_request.rs:60` |
| `unavailable_revision_never_admits` | script: build, then unavailable, then admissible at N; result is `Err(GovernorUnavailable)` and no frontier is read | `.unwrap_or(expected)` at :60, which returns `Ok(Admitted)` (fails open) |
| `unavailable_frontier_fails_with_unavailable` | `Err(GovernorUnavailable)`; log shows `CurrentRevision` then `Frontier` | swallowing the error at :65 |
| `unknown_case_at_frontier_keeps_its_variant` | a frontier `UnknownCase` stays `UnknownCase` | `map_err` at :65 |
| `admissible_and_blocked` / `admissible_and_approval_required` (both orders) | the result is NotAdmitted with the reasons, or NeedsAuthority, never Admitted | revalidate checking only the first entry, or any `Admissible` entry, instead of calling `admit()` |
| `approval_without_one_capability` | a blank capability, or two capabilities, gives NotAdmitted with literal reasons | the same |
| `near_miss_name` | `"Merge"`, `"merge "` and `""` are each NotAdmitted with no reasons | a fuzzy name match |
| `arguments_are_carried_byte_for_byte` | duplicate keys, member order and the spellings `10.50`, `1e3`, `-0` match a hand-built `Value`, and render back to the exact input text | normalising the arguments in `request()` |

**3. Suite, run after the cases existed.** `cargo test --workspace --locked` with the brief's `CARGO_TARGET_DIR`: EXIT=0, 179 cases. 170 before is the same run with my two binaries (9 cases) left out of the count.

**4. Findings** (they cover the working tree on c235b0e)
- **F1 (warning, mutant).** `crates/commission/src/action_request.rs:60,65`. What was measured: `tests/action_request.rs` never scripts `Answer::unavailable` or an unknown case (grep finds no match), and it is the only existing caller of `revalidate`. So nothing tests the governor-failure paths. A mutant reading a failed revision as the expected one would turn an unavailable governor into `Admitted` and stay green. What reaches it: the `revalidate` step in `story:local-runtime-loop` and in `story:effect-invocation`, with any governor that can fail. My cases close this gap if they are kept.
- **F2 (note, contract drift).** `ess/domains/responsibility.yaml:656` says the stale outcome means the governor's current revision is not the expected one. The code (`action_request.rs:67-68`) also answers stale when the frontier was issued for another revision, and then puts the frontier's revision in `current_case_revision`. A future adapter or runner working from the ESS text would not implement that guard. What reaches it: a governor whose revision and frontier calls disagree, which is the race this unit tests.
- **F3 (note, INFEASIBLE).** `action_request.rs:57`: `revalidate` never reads `run_id`. A request whose run is suspended still comes back Admitted, although the spec says a request never outlives its run. What reaches it: nothing found. The loop revalidates inside a running run, and your decision allows no outcome other than the four.

**5. Attacked and could not break**
- **Fail-open:** an unlisted action, Blocked, ApprovalRequired and duplicate listings all go through `admit()`, so none of them reaches Admitted.
- **Revision race:** a frontier issued for N+1 after reading revision N comes back stale.
- **Order:** stale is decided before any frontier is fetched.
- **Arguments:** carried byte for byte.
- **Case identity:** a frontier for another case is refused as not admitted. At a different revision it is refused as stale, which matches the module's own order.
- **Callers:** `story:local-runtime-loop` (acceptance 2, 3, 4, 9) and `story:effect-invocation` (steps 1-2) can call `request` and `revalidate` as they are. Both must treat NeedsAuthority plus an authority allow as the go-ahead, because by your decision `revalidate` takes no authority decision as input.

**6. Paths written outside the worktree** (the mutant dir `~/.cache/b10x-target/commission-w6-mutants1` was never created)
- `~/.cache/ga-wave-2026-10-04-w6/commission-stale-revision-action-request/scratch/adv1/` (`suite.json`, `cases-alone.log`, `suite.log`)
- New test binaries in the brief's `~/.cache/b10x-target/commission-w6-stale-revision-action-request`, from the case run and the normal suite run.

My session lease `adversary1-w6-action-request` is acquired and released.

```findings
- file: crates/commission/src/action_request.rs
  line: 60
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "No existing revalidation case drives a failing governor, so a mutant that reads a failed revision as the expected one turns GovernorUnavailable into Admitted and the suite stays green."
- file: ess/domains/responsibility.yaml
  line: 656
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The stale outcome's external condition covers only a current-revision mismatch, while the code also answers stale for a frontier issued at another revision and puts that frontier's revision in current_case_revision."
- file: crates/commission/src/action_request.rs
  line: 57
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "revalidate never reads run_id, so a request whose run is suspended still comes back Admitted; no caller was found that revalidates outside a running run."
```
