---
format: aep.planning-md/3
id: review-result:adversary-w7-commission-adapter-conformance-suites-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w7 adversary, commission story:adapter-conformance-suites, pass 2
relations:
- reviews: story:adapter-conformance-suites
revision: 1
---
unit: commission/adapter-conformance-suites, working tree on 26da484 + uncommitted phase 2 + pass-1 adversary files
verdict: NEEDS-CHANGE
cases: executed 199→206, red 7
origin: introduced 7 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 paths (part 6)
needs-coordinator: whether findings 5, 6 and 7 (kit coverage beyond the four acceptance items) hold this unit or go to their own story

**1. `git --no-pager diff --stat`**
```
 crates/commission-testkit/src/kits/authority.rs | 277 ++++++++++++++-
 crates/commission-testkit/src/kits/governor.rs  | 433 +++++++++++++++++++++++-
```
Both are phase-2 implementation and predate this pass. My files are untracked test files only: `tests/adversary2_kits_governor.rs` and `tests/adversary2_kits_authority.rs`. I edited no implementation file.

**2. Cases added, each run alone before the suite, all red now** (logs: `scratch/adv2-red.log`, `scratch/adv2-red-governor.log`)

| # | Case | Asserts | Red output |
|---|---|---|---|
| 1 | `adversary2_kits_observation_not_evidence_catches_promotion_at_completion` | a governor that turns observations into evidence when asked for `completion` fails observation-not-evidence | all 5 checks `pass` |
| 2 | `…_catches_promotion_at_frontier` | the same, with the promotion in `frontier` | all 5 `pass` |
| 3 | `adversary2_kits_superseded_revision_catches_a_revision_that_repeats` | a governor whose revision goes 1, 2, 1 fails superseded-revision. The case first checks that the defect is real: after two changes, `revalidate` admits a request made at revision 1 | all 5 `pass` (fails at :241, after the check passed) |
| 4 | `adversary2_kits_catch_a_completion_that_never_holds_the_case` | a governor whose `completion` answers `UnknownCase` for the held case fails some check | all 5 `pass` |
| 5 | `adversary2_kits_current_frontier_holds_a_fixture_to_the_documented_advance_contract` | a governor with no defect, whose `advance` moves to a frontier where the action is `ApprovalRequired` with a capability, passes | `FAIL current-frontier: after the case moved, the frontier for the held case at revision 2 admits no action` |
| 6 | `adversary2_kits_authority_catches_a_cache_that_ignores_the_capability` | a provider that answers every capability with the first verdict it decided fails some check | all 4 `pass` |
| 7 | `adversary2_kits_backing_failure_catches_a_stale_allow_served_while_the_backing_is_down` | a provider that returns the last allow when its backing call fails fails backing-failure | all 4 `pass` |

After the red runs, `cargo fmt` flagged one line wrapping in case 7. I joined the line; the logic is unchanged.

**3. Suite run, after the cases existed**
- `cargo test --workspace --locked --no-fail-fast` → EXIT=101, passed 199, failed 7. The 7 failures are exactly the cases above (`scratch/adv2-suite.log`).
- `cargo fmt --check` → 0.
- `cargo clippy -p b10x-commission-testkit --test adversary2_kits_governor --test adversary2_kits_authority -- -D warnings` → 0.

**4. Findings** (all cover the working tree above; origin is `introduced` because both kit files were empty at base 174bf07)

| # | file:line | Verdict | What reaches it |
|---|---|---|---|
| 1 | governor.rs:370-372 | NEEDS-CHANGE | A governor that interprets observations while deciding completion or building a frontier. That breaks the story's "never as evidence" rule and passes the kit, because the extra call is only `current_revision`. Fix: call `frontier` and `completion` too before reading evidence. |
| 2 | governor.rs:275-295 | NEEDS-CHANGE | An adapter whose revision comes from the case's state, such as a hash, so an earlier revision can come back. The kit moves the case once, so it cannot see a repeat. Fix: move it twice and check that a request made at the first revision is still stale. |
| 3 | governor.rs:41-43, 299-325 | NEEDS-CHANGE | `hold` promises an open case, but no check asks `completion` about it. The unknown-case check only tests that the error appears for an unknown case. The runtime-loop unit in this wave is the first caller of `completion`. Fix: require `completion(held) == Ok(Open)`. |
| 4 | governor.rs:45 vs 219-221 | CONFIRMED | `advance` only promises "a later revision", but current-frontier fails if the new frontier admits no action outright. This came in with pass-1 fix 1. An adapter whose natural case change brings in an approval gets a false failure. Fix: either drop the admit requirement after the move, or document it on `advance`. |
| 5 | authority.rs:10-12, 240-241 | NEEDS-CHANGE | The decoy capabilities are stored but never asked about, so a cache whose key leaves out the capability (a common adapter bug) passes. Fix: in each as-given check, also ask about one decoy and require its own verdict back. |
| 6 | authority.rs:48-53, 252-258 | NEEDS-CHANGE | The backing is fixed when the provider is built, so the kit can never make a call fail after one succeeded. A provider that returns the last allow while its backing is down passes. Fix: a backing the kit can switch after the provider is built, plus a check that gets an allow first and then a failure. |

**5. Attacked, could not break**
- **catch_unwind side effects:** each check builds a fresh governor and its own backing inside `unwound` (governor.rs:150,178; authority.rs:186,240). The kit holds no lock and no state. Only a fixture's own shared state can be poisoned, and that is outside the kit.
- **Panic hook:** the default hook still prints to stderr. The test harness captures it, and a caught panic still fails its check.
- **Report Display:** the name always comes from the same entry as the outcome (governor.rs:134-137), and all names are unique.
- **Fixture hold/advance lying:** an `advance` that does nothing fails superseded-revision, and a `hold` that holds the unknown case fails unknown-case.
- **Decoy name collisions:** `kit.*` names are unlikely to match an adapter's real capability, and the byte order 0 < c < z holds.
- **Use from an outside crate:** `tests/kits.rs` uses only the public API. There is no compiled doc example, only an inline `run(&MyFixture).assert_passed()`.
- **Doc comments against behaviour:** they match after the pass-1 fixes, except finding 4.

**6. Paths written outside the worktree**
- `~/.cache/b10x-target/commission-w7-adapter-adv` (build directory)
- `~/.cache/ga-wave-2026-10-04-w7/commission-adapter-conformance-suites/scratch/adv2-red.log`
- `~/.cache/ga-wave-2026-10-04-w7/commission-adapter-conformance-suites/scratch/adv2-red-governor.log`
- `~/.cache/ga-wave-2026-10-04-w7/commission-adapter-conformance-suites/scratch/adv2-suite.log`

**7.**
```findings
- file: crates/commission-testkit/src/kits/governor.rs
  line: 370
  category: acceptance
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "observation-not-evidence makes only one more current_revision call, so a governor that turns observations into evidence in completion or frontier passes every check (adversary2_kits_governor.rs cases 1 and 2)"
- file: crates/commission-testkit/src/kits/governor.rs
  line: 275
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "superseded-revision moves the case once, so a governor whose revision repeats (1, 2, 1) and admits a request made two changes ago passes"
- file: crates/commission-testkit/src/kits/governor.rs
  line: 299
  category: mutant
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "no check asks completion about the held case, so a governor answering UnknownCase to completion for every case passes all five checks"
- file: crates/commission-testkit/src/kits/governor.rs
  line: 45
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "advance promises only a later revision, but current-frontier fails a governor with no defect when the frontier after the move admits no action outright"
- file: crates/commission-testkit/src/kits/authority.rs
  line: 240
  category: mutant
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "the decoy capabilities are never asked about, so a provider whose cache key leaves out the capability passes, despite the module doc saying a provider must answer the capability it was asked"
- file: crates/commission-testkit/src/kits/authority.rs
  line: 252
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "the backing cannot be made to fail after it has answered, so a provider that returns its last allow while the backing is down passes backing-failure"
```
