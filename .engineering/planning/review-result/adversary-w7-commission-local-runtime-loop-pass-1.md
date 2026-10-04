---
format: aep.planning-md/3
id: review-result:adversary-w7-commission-local-runtime-loop-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w7 adversary, commission story:local-runtime-loop, pass 1
relations:
- reviews: story:local-runtime-loop
revision: 1
---
I couldn't run anything: `/` showed 9.5G free on all four checks, below your 10G floor, so I stopped before any build. I wrote three cases that I expect to fail and five that I expect to pass, working each out by hand against the code. None has been compiled or run, so nothing here is confirmed yet.

```
unit: commission/local-runtime-loop, phase-1 7b72b12 plus uncommitted phase 2 in crates/commission/src/runtime.rs
verdict: blocked: disk (df -h / showed 9.5G free on all four checks); 3 cases expected red, worked out by hand, not run
cases: executed 185→185 (nothing run; 185 is the count the unit reported), red 0 measured
origin: introduced 4 / pre-existing 1 / undecided 0
wrote-outside-worktree: none
needs-coordinator: free disk, then run the new file alone and then the suite; decide whether one Run may span case revisions (finding 1)
```

**1. `git --no-pager diff --stat`** (the new file is untracked, so it shows only in `git status`)
```
 crates/commission/src/runtime.rs | 291 ++++++++++++++++++++++++++++++++++++++-
 1 file changed, 290 insertions(+), 1 deletion(-)
?? crates/commission-testkit/tests/adversary_loop_runtime.rs
```
I touched no implementation file: `runtime.rs` is the implementor's change. The new file passes `rustfmt --edition 2024 --check` with exit 0.

**2. Cases added** in `~/.local/state/worktree/trees/b10x/commission/commission-w7-local-runtime-loop/crates/commission-testkit/tests/adversary_loop_runtime.rs`. None has been run, so there is no red output to quote.

| Case | Asserts | Expected |
|---|---|---|
| `request_is_made_against_its_runs_case_revision` | every admitted request's `expected_case_revision` equals its Run's `case_revision` | red: `inspect` admitted at revision 8 in a Run started at 7 |
| `governor_failure_mid_iteration_leaves_no_unnamed_running_run` | after a governor failure in revalidation (the observation is already delivered), either the error names the run or the Run is not `Running` | red: neither is true |
| `bound_counts_the_frontier_the_executor_saw` | the executor is asked at most twice on frontiers of an unchanged revision 6 | red: it is asked 3 times |
| `authority_allow_admits_the_request`, `authority_deny_admits_nothing_and_is_bounded`, `authority_failure_admits_nothing`, `completion_at_the_bound_wins`, `needs_human_judgment_ends_the_run` | the implementor's untested choices | green |

To run: `cargo test --locked -p b10x-commission-testkit --test adversary_loop_runtime`, then `cargo test --workspace --locked`.

**3. Suite run:** not run (blocked on disk).

**4. Findings**
- **F1 (spec vs code), blocker, introduced.** `runtime.rs:160-163` reloads the revision on every iteration, and `:196-210` binds each request to the current frontier's revision, inside the one Run started at `:143`. The spec says the opposite in `ess/domains/responsibility.yaml:386` ("a proposal made on another revision is stale") and `:390` ("against that run's case revision"). What reaches it: any case that moves during a loop, for example another commission on the same case, which the story allows. The phase-1 bound test (`runtime_loop.rs:701`) also assumes one Run spans revisions, so either the spec or the loop has to change. That choice is yours.
- **F2 (governor failure part-way), warning, introduced.** `LoopError` (`:92-99`) carries no run id. The `?` at `:176` (observe) and `:202` (revalidate) leave the started Run `Running`, with an observation already delivered and any admitted requests lost. A retry then starts a second Run. What reaches it: `GovernorError::GovernorUnavailable`, which every real governor can return.
- **F3 (stall bound), note, introduced.** At `:247` the bound is keyed on the revision loaded at the start of the iteration, not the frontier's revision. If the case moves between the load and the frontier read, the executor runs 3 times on an unchanged frontier. The wave-5 note suggested keying on the frontier revision.
- **J1 (authority), note, introduced.** At `:216-226`, a Deny reason or a provider-failure message never reaches `LoopEnd`. The run ends `NoAdmissibleAction`, which looks the same as an empty frontier. The redaction that `check_authority` does on that message is thrown away.
- **J2 (Run lifecycle), note, pre-existing.** The spec's Run lifecycle has only `Running` and `Suspended`, with no terminal state (`responsibility.yaml:400-405`). So every loop that ends any way other than suspended leaves its Run `Running`. `ResumeRun` refuses such a run, so after needs-authority or needs-human-judgment a new Run starts instead of continuing the old one.

**5. Attacked, could not break (by reading, not run)**
- Allow, Deny and provider failure: only Allow admits; `Allow(Unit(false))` does not admit.
- The executor only ever gets the frontier from the same iteration.
- Completion is checked before the bound fires.
- The call order the test pins matches the doc comments.
- Suspend that does not suspend: `NotSuspended` handles `WrongState`, which can't happen with `Generated<RunStore>`. I wrote no case for it.

**6. Paths written outside the worktree:** none. The session lease `adversary-w7-local-runtime-loop-p1` was acquired and released through `worktree hook`.

```findings
- file: crates/commission/src/runtime.rs
  line: 196
  category: contract-drift
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The loop keeps one Run across a case revision change and admits requests at the new revision, which ess Run.case_revision calls stale; red case written, not run (disk)."
- file: crates/commission/src/runtime.rs
  line: 92
  category: concurrency
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "A governor failure after StartRun returns a LoopError that names no run and leaves that Run Running after an observation was delivered; red case written, not run (disk)."
- file: crates/commission/src/runtime.rs
  line: 247
  category: boundary
  severity: note
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The stall bound keys on the loaded revision, so a case moving between the load and the frontier read lets the executor run three times on an unchanged frontier; red case written, not run (disk)."
- file: crates/commission/src/runtime.rs
  line: 216
  category: judgement
  severity: note
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "A Deny reason or a provider-failure message never reaches LoopEnd, so the NoAdmissibleAction end cannot be told apart from an empty frontier."
- file: ess/domains/responsibility.yaml
  line: 400
  category: judgement
  severity: note
  verdict: NEEDS-CHANGE
  origin: pre-existing
  message: "The Run lifecycle has no terminal state, so every loop end other than suspended leaves its Run Running and ResumeRun cannot continue it."
```
