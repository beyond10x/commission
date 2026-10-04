---
format: aep.planning-md/3
id: review-result:adversary-w7-commission-local-runtime-loop-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w7 adversary, commission story:local-runtime-loop, pass 2
relations:
- reviews: story:local-runtime-loop
revision: 1
---
unit: commission/local-runtime-loop, working tree at 7b72b12 plus the uncommitted phase 2 and pass-1 fixes in ~/.local/state/worktree/trees/b10x/commission/commission-w7-local-runtime-loop
verdict: NEEDS-CHANGE
cases: executed 193→200, red 3
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 2 paths
needs-coordinator: F-B. The module doc admits the loop has no bound when the same request is admitted again and again, but the story's w5 note says it must have one. Decide which one stands.

**1. Diff stat**

```
 crates/commission-testkit/tests/runtime_loop.rs |  25 +-
 crates/commission/src/runtime.rs                | 402 +++++++++++++++++++++++-
```

Both of these paths are the implementor's uncommitted phase 2, which was already there when I started. The only file I added is the untracked test file `crates/commission-testkit/tests/adversary2_loop_runtime.rs`. I changed no implementation file. The one formatting run I did was `rustfmt` on my own new file.

**2. Cases added** (`~/.local/state/worktree/trees/b10x/commission/commission-w7-local-runtime-loop/crates/commission-testkit/tests/adversary2_loop_runtime.rs`)

These were run alone with `cargo test --locked -p b10x-commission-testkit --test adversary2_loop_runtime` before the suite ran. The output is saved in `scratch/adversary2-red.log`. Result: `4 passed; 3 failed`. The binary was built from this tree: "Compiling b10x-commission-testkit (…commission-w7-local-runtime-loop/…)".

| Test | Line | Now | Red output (verbatim excerpt) |
|---|---|---|---|
| `stale_after_an_idle_iteration_ends_completed` | :201 | RED | `left: NoAdmissibleAction(Unit(true))` / `right: Completed(RunOutcomeCompleted { outcome: "Done" })`; 7 governor calls, no load after the stale revalidation |
| `stale_after_an_admitted_iteration_ends_completed` | :236 | green | control: the same move after an admitted iteration ends `Completed("Done")` |
| `admitted_proposal_on_an_unchanged_frontier_is_bounded` | :290 | RED | `it asked 25 times on revisions [5, 5, …, 5], admitted 24 requests, and ended only through the test's guard` |
| `frontier_of_another_revision_never_reaches_the_executor` | :338 | green | guard for the check at runtime.rs:292 |
| `observation_failure_suspends_and_names_the_run` | :362 | green | the Run ends `Suspended`, one `SuspendRun` with `ExternalAvailability`, the error names the Run |
| `refused_suspension_is_carried_beside_the_failure` | :400 | green | `suspension: Some(Obligation(..))`; the message contains the run id, `GovernorUnavailable` and `probe` |
| `executor_suspension_reason_survives_an_observation_failure` | :453 | RED | `the executor suspended for Dependency([CaseId("case-upstream")]); SuspendRun received [ExternalAvailability(Text("governor failed: GovernorUnavailable"))] and the loop returned Err(LoopError { … failure: Governor(GovernorUnavailable), suspension: None })` |

**3. Suite run** (after the cases existed)

`cargo test --workspace --locked --no-fail-fast` with the assigned build dir exited 101. Summing the summary lines gives `passed 197 failed 3`. The three failures are the red cases above and nothing else failed. Log: `scratch/adversary2-suite.log`. `cargo fmt --check` exits 0. `cargo clippy -p b10x-commission-testkit --test adversary2_loop_runtime -- -D warnings` exits 0.

**4. Findings** (they cover the working tree named in the header)

| ID | file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|---|
| F-A | crates/commission/src/runtime.rs:359 | NEEDS-CHANGE / introduced | Doc lines :25-26 and decision F1 say a stale proposal leads to a load of the moved case and a `Completed` end "unless the case is complete". After one idle iteration, the stall bound at :359 ends the run inside the stale iteration, so a completed case ends `NoAdmissibleAction`. After an admitted iteration it ends `Completed` (the green control). Fix: a stale answer should not count toward the bound, for example `continue` from the `Stale` arm at :328, since the next load ends the run either way. | A second commission on the same case completes it between this loop's frontier read and the revalidation (responsibility.yaml:164-165 allows several commissions per case). Nothing calls `run_until_blocked` outside tests. |
| F-B | crates/commission/src/runtime.rs:323-327, doc :39-41 | CONFIRMED / introduced | An admitted request clears `idle`. No effect runs, so the case never moves. An executor that always proposes the same admitted action was asked 25 times on revision 5 and was stopped only by the test's guard. | Any executor that gives the same proposal for the same frontier. The doc admits the gap; the story's w5 note forbids it. Nothing calls it outside tests. |
| F-C | crates/commission/src/runtime.rs:296 then :303 | CONFIRMED / introduced | The observation is sent before the executor's `Suspended` is acted on. If sending it fails, the Run is suspended with `ExternalAvailability` instead, and the executor's reason is in neither `SuspendRun` nor `LoopError`. The observation payload at :388 doesn't carry the reason either. | The observation port fails on the same step the executor suspends. The fake's `observe` never fails. Nothing calls it outside tests. |
| J-1 | crates/commission/src/runtime.rs:222, :179 | CONFIRMED / introduced | The `ExternalAvailability` reason is built from `{error:?}` and then dropped: `suspend` throws away `RunSuspended`, and `LoopError` has no field for the reason. The spec says the reason "is reported, not stored" (ess/domains/responsibility.yaml:600-601), and here nothing reports it. | Every governor failure after `StartRun`. Judgement only; I wrote no case for it. |

**5. Attacked, could not break**
- Suspending a Run that is already `Suspended` can't happen. The executor-`Suspended` path returns right after suspending (:303-307), and `runs: &mut R` keeps any other writer out while the loop runs.
- Completion is checked before the revision. `CompletionDetermination` carries no revision, and decision F1 says a complete case wins, so the code matches the decision.
- Check placement for F1 (where the run ends when the case revision moves), both directions. The loaded revision is checked at :287 (existing test) and the frontier revision at :292 (my guard is green). From reading, I think no earlier test would fail if the check at :292 were removed. I did not build that mutant, because a scratch copy in the shared build dir could overwrite this tree's build output.
- The bound after an admitted request: admitted, then idle, then idle ends the run on the second idle iteration, as the doc says.
- When the run ends at completion or on a moved case, there is no executor call and no observation. On a derived end, the observation is sent before the run ends.
- Doc comments: :35-38 and :42-45 match the code. Lines :25-26 do not (F-A).
- I did not take a worktree session lease. I had no lease command from the coordinator, and I ran no worktree or store commands.

**6. Paths written outside the worktree**
- ~/.cache/ga-wave-2026-10-04-w7/commission-local-runtime-loop/scratch/adversary2-red.log
- ~/.cache/ga-wave-2026-10-04-w7/commission-local-runtime-loop/scratch/adversary2-suite.log
- Build output went into the assigned ~/.cache/b10x-target/commission-w7-local-runtime-loop.

```findings
- file: crates/commission/src/runtime.rs
  line: 359
  category: contract-drift
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "After one idle iteration a stale proposal trips the stall bound in the same iteration, so a case completed by the move ends NoAdmissibleAction, against the module doc (:25-26) and decision F1."
- file: crates/commission/src/runtime.rs
  line: 325
  category: acceptance
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "An admitted request resets the stall count and no effect moves the case, so an executor proposing the same admitted action is asked on an unchanged frontier without bound, against the story's w5 note."
- file: crates/commission/src/runtime.rs
  line: 296
  category: concurrency
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "An observation failure on an executor Suspended step suspends the Run with ExternalAvailability, and the executor's own reason is lost from SuspendRun, the error and the observation."
- file: crates/commission/src/runtime.rs
  line: 222
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The ExternalAvailability reason is built and then dropped, because suspend() discards RunSuspended and LoopError has no field for it, so the reason the spec says is reported is reported nowhere."
```
