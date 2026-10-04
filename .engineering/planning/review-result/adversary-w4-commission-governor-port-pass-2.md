---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-governor-port-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:governor-port, pass 2
relations:
- reviews: story:governor-port
revision: 1
---
unit: commission/governor-port, working tree = 8f46ec4 + uncommitted phase 2 + pass-1 fixes, in `~/.local/state/worktree/trees/b10x/commission/commission-w4-governor-port`
verdict: CONFIRMED (2 notes). The one red case is INFEASIBLE: I found no caller that uses the fake from two threads.
cases: executed 83→90, red 1
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 6 paths (part 6)
needs-coordinator: the red concurrency case makes the gate red. Either fix the fake (one-line move, below) or decide what happens to the case.

**1. Diff.** `git --no-pager diff --stat` lists only the implementor's tracked files (`fake_governor.rs`, `governor_port.rs`, `ports/governor.rs`). I changed none of them. My two files are new and untracked:
- `crates/commission-testkit/tests/adversary2_governor_combinations.rs`
- `crates/commission-testkit/tests/adversary2_governor_concurrency.rs`

Both are test files. No implementation file was edited.

**2. Cases added (each run alone before the suite)**

| File / case | Asserts | Now |
|---|---|---|
| concurrency: `the_kth_logged_call_received_the_kth_answer` | two threads each make 50k calls; log position k got answer k | **red** |
| combinations: `complete_then_with_items_keeps_the_determination_and_the_items` | the determination survives `with_items` | green |
| combinations: `with_items_then_complete_keeps_the_items_and_the_determination` | the items survive `complete` | green |
| combinations: `an_unavailable_answer_stays_unavailable_under_every_combination` | both orders, all 3 methods, plus the last answer repeating | green |
| combinations: `every_issued_frontier_carries_its_own_answers_case_and_revision` | mixed script (methods, items, completion, failure, two cases): each frontier carries its own answer's `case_id` and `case_revision`; the repeated last answer gets a fresh id | green |
| combinations: `the_fake_scripts_a_case_moving_under_a_pending_request` | stale-revision acceptance 1 and 2 can be scripted | green |
| combinations: `the_fake_scripts_every_frontier_the_run_outcome_rows_read` | run-outcomes rows 1, 3, 5 and 6 can be scripted | green |

Red output on its first run, verbatim:
```
panicked at crates/commission-testkit/tests/adversary2_governor_concurrency.rs:76:5:
145 of 100000 logged calls did not receive the answer at their log position; first: Some((1636, CurrentRevision(CaseId("case-contended")), 1657))
test result: FAILED. 0 passed; 1 failed
```
It was red in 20 of 20 repeat runs (36 to 296 mismatches each).

**3. Suite run** (`cargo test --workspace --locked --no-fail-fast`, after the cases existed): exit 101. 90 tests ran and 1 failed, the concurrency case; every other target is `ok`. `cargo fmt --check` and `cargo clippy --workspace --all-targets --locked -- -D warnings` both exited 0. The 83 "before" figure is the same run with my two test targets left out of the count (90 − 7).

**4. Findings**

| file:line | Finding | Verdict | What reaches it | Origin |
|---|---|---|---|---|
| `crates/commission-testkit/src/fake_governor.rs:149-154` | `answer` logs the call under the `calls` lock, releases it, and only then takes the `scripts` lock. Under concurrent calls the log order and the answer order diverge, so `calls()` (`:139`) no longer pairs a call with its answer. A reader can also see a call logged before it is answered. That second effect only matters for concurrent readers. **Fix:** push the call while holding the `scripts` lock. Mutant M4 does exactly that, and the case went green 20 of 20. | INFEASIBLE (warning) | Nothing found. The loop in `story:local-runtime-loop` is sequential. Only pass-1's `the_fake_is_a_shareable_trait_object` shares the fake across threads. | introduced |
| `fake_governor.rs:96-105` | `GovernorCall` has no shared sequence or tick. `story:local-runtime-loop` acceptance 1 asks the fakes' call logs to show, per iteration, the case loaded and the frontier obtained before the executor is invoked. Two separate per-fake logs cannot show the order between fakes. The executor fake in `commission-w4-agent-executor-port` has no call log at all. A test-side wrapper that reads `governor.calls()` from inside the executor call would work around it. | CONFIRMED (note) | `story:local-runtime-loop` acceptance 1 | introduced |
| `fake_governor.rs:149-160` | All three methods take answers from one queue. Every loop script therefore has to count each governor call per iteration, and adding one `completion()` check shifts every later revision. The story asks for this per-call design. It is fragile for loop acceptance 3, 5 and 8. | CONFIRMED (note) | `story:local-runtime-loop` acceptance 3, 5 and 8 | introduced |

**Mutants** (built in a scratch copy with the extra target dir):

| Mutant | Existing suite | My cases |
|---|---|---|
| M1: `with_items` resets the determination to Open | **survived** | killed |
| M2: `complete` clears the items | **survived** | killed |
| M3: `with_items` revives an unavailable answer | killed (`governor_port.rs:180`) | killed |
| M4: log under the scripts lock (the fix) | green | green, which shows the red case fails for the reason it states |

**5. Attacked and could not break**
- Combining `with_items` with `complete` and `unavailable`, in either order: correct (`fake_governor.rs:63-93`).
- Every issued frontier's `case_id` and `case_revision` always match the call's own answer. The scripted items carry no case or revision field, so they cannot contradict the frontier.
- Stale-revision acceptance 1 and 2 and run-outcomes rows 1, 3, 5 and 6 can all be scripted with the fake as it is.
- The doc comments in `ports/governor.rs` and `fake_governor.rs` match the behaviour for sequential use. `ports::evidence` exists, as the doc says.
- The `UnknownCase` fallback at `:163` can never be reached, because `script` refuses an empty script.

**6. Written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w4/commission-governor-port/scratch/adv2-combinations.log`
- `~/.cache/ga-wave-2026-10-04-w4/commission-governor-port/scratch/adv2-suite.log`
- `~/.cache/ga-wave-2026-10-04-w4/commission-governor-port/scratch/adv2-suite.log.fmt`
- `~/.cache/ga-wave-2026-10-04-w4/commission-governor-port/scratch/adv2-suite.log.clippy`
- `~/.cache/ga-wave-2026-10-04-w4/commission-governor-port/scratch/adv2-mut/`: the original fake, `m1.rs` to `m4.rs` and their logs. The source copy is deleted.
- `~/.cache/b10x-target/commission-w4-mutants`: deleted. I did not touch `commission-w4-mutants2`; it belongs to another session.

Disk was at 11G free while I worked.

**7.**
```findings
- file: crates/commission-testkit/src/fake_governor.rs
  line: 149
  category: concurrency
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "answer() logs under the calls lock and pops under a separate scripts lock, so under concurrent calls the k-th logged call does not receive the k-th answer (red 20/20, 36-296 of 100000); no concurrent caller found"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 96
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "GovernorCall carries no shared sequence, so per-fake logs cannot show the cross-fake order that local-runtime-loop acceptance 1 asks the fakes' call logs to show"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 149
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "all three methods consume one answer queue, so loop tests must count every governor call per iteration and an added completion() check shifts later revisions"
```
