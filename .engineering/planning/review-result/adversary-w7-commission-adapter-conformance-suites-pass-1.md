---
format: aep.planning-md/3
id: review-result:adversary-w7-commission-adapter-conformance-suites-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w7 adversary, commission story:adapter-conformance-suites, pass 1
relations:
- reviews: story:adapter-conformance-suites
revision: 1
---
unit: commission/adapter-conformance-suites, worktree commission-w7-adapter-conformance-suites at 26da484 plus the uncommitted phase 2 in kits/{governor,authority}.rs
verdict: NEEDS-CHANGE
cases: executed 185→199, red 7
origin: introduced 7 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path (the build dir, which holds the suite log)
needs-coordinator: clippy did not run on the new test files. Free disk dropped to 9.6G after the suite, under the 10G floor, so I stopped building.

**1. Diff stat** (the 2 new files are untracked, so they don't show here)
```
 crates/commission-testkit/src/kits/authority.rs | 252 ++++++++++++++-
 crates/commission-testkit/src/kits/governor.rs  | 387 +++++++++++++++++++++++-
?? crates/commission-testkit/tests/adversary_kits_authority.rs
?? crates/commission-testkit/tests/adversary_kits_governor.rs
```
The two `M` files are the implementor's phase 2. I added only the two test files and changed no implementation file.

**2. Cases added.** Each file was run alone first.

| Case | Status now |
|---|---|
| governor.rs:239 `frontier_ahead` (proves current-frontier) | green |
| governor.rs:248 `unknown_case…unavailable` (proves unknown-case, fails only that check) | green |
| governor.rs:254 `eager_promotion` (proves observation-not-evidence) | green |
| governor.rs:263 `dropped_id` (proves evidence-observation-ids, fails only that check) | green |
| authority.rs:92/98/104 (prove allow-, deny- and approval-required-as-given, each fails only its own check) | green |
| governor.rs:279 `frontier_stuck_after_the_case_moves` | **red**: all 5 checks pass |
| governor.rs:291 `promotion_on_the_next_call` | **red**: all 5 checks pass |
| governor.rs:302 `promotion_from_the_second_observation` | **red**: `FAIL evidence-observation-ids`, `pass observation-not-evidence` |
| governor.rs:314 `ids_replaced_by_all_held` | **red**: all 5 checks pass |
| governor.rs:326 `panicking_check` | **red**: `case \`kit-case-unknown\` is not held` / `governor::run panicked … no report named unknown-case` |
| authority.rs:119 `ignores_the_capability` | **red**: all 4 checks pass |
| authority.rs:131 `panicking_check` | **red**: `backing call failed: backing service unavailable` / `authority::run panicked … no report named backing-failure` |

So all 7 unproven checks now have a broken fixture that they catch.

**3. Suite** (run after the cases existed): `cargo test --workspace --locked --no-fail-fast`, EXIT=101. Totals: 192 passed, 7 failed, 199 executed; the acceptance test `kits_hold_fakes_and_catch_broken_ones` still passes. After running rustfmt on my two files, `cargo fmt --check` exits 0.

**4. Findings.** None of these is an `Err` path in the kit. The trigger is always adapter behaviour the kit's own doc says it rejects.

| # | Where | Finding | Verdict | What reaches it |
|---|---|---|---|---|
| 1 | kits/governor.rs:194 | current-frontier only reads the frontier before the case moves, and superseded-revision (:237–245) never reads it after. A governor that reports N+1 but keeps issuing the frontier for N passes everything. This goes against the doc at :7 and the `Governor::frontier` contract. | NEEDS-CHANGE | an adapter that caches frontiers |
| 2 | kits/governor.rs:332 | Evidence is read straight after `observe`, before any other call. A governor that turns observations into evidence on its next call passes. | NEEDS-CHANGE | a governor that batches its inbox |
| 3 | kits/governor.rs:349 | The evidence names exactly every observation received, so a governor that replaces the ids with every observation it holds passes. Fix: observe 3 and name 2. | NEEDS-CHANGE | wrong attribution in an adapter |
| 4 | kits/governor.rs:141, kits/authority.rs:172 | `run` does not catch a panic, so a panicking adapter returns no report and no check name. The module doc (:5) says the kit reports each failed check by name. Fix: wrap each check in `catch_unwind` and record the panic as that check's failure. | NEEDS-CHANGE | `.expect()` in adapter code |
| 5 | kits/authority.rs:215 | Each backing holds one entry for the one capability asked, so a provider that ignores the capability passes. Fix: give the backing a second capability with a different answer. | NEEDS-CHANGE | an adapter that caches verdicts per commission |
| 6 | kits/governor.rs:323 | Only one observation is sent, so a governor that turns the second one into evidence is reported under the wrong check name. | CONFIRMED (note) | as #2 |

**5. Attacked and not broken**
- `ran()` and `failed()` cannot lie: the outcomes are private and `run` is the only constructor.
- No check passes vacuously: an empty frontier fails current-frontier, observations are compared for exact equality, and an empty evidence list fails.
- The kit code itself has no panic path.
- backing-failure calls `decide` a second time, which catches a provider that fails open on its second call.

**6. Written outside the worktree:** `~/.cache/b10x-target/commission-w7-adapter-adv/` (713M, holds `adv-suite.log`).

```findings
- file: crates/commission-testkit/src/kits/governor.rs
  line: 194
  category: acceptance
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "A governor that reports revision N+1 but keeps issuing the frontier for N passes every governor check, because no check reads the frontier after advance."
- file: crates/commission-testkit/src/kits/governor.rs
  line: 332
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "observation-not-evidence reads evidence only before any further governor call, so promoting observations to evidence on the next call passes."
- file: crates/commission-testkit/src/kits/governor.rs
  line: 349
  category: mutant
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "evidence-observation-ids names exactly the set of received observations, so a governor that overwrites ids with every held observation passes."
- file: crates/commission-testkit/src/kits/governor.rs
  line: 141
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "governor::run and authority::run (authority.rs:172) propagate an adapter panic, so no report is returned and no failing check is named, contrary to the module docs."
- file: crates/commission-testkit/src/kits/authority.rs
  line: 215
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "Each backing holds a single capability, so a provider that ignores the capability asked and answers with its first verdict passes every authority check."
- file: crates/commission-testkit/src/kits/governor.rs
  line: 323
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "observation-not-evidence sends one observation, so a governor that promotes from the second observation on is reported as evidence-observation-ids instead."
```
