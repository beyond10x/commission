---
format: aep.planning-md/3
id: review-result:adversary-w5-commission-run-outcomes-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w5 adversary, commission story:run-outcomes, pass 2
relations:
- reviews: story:run-outcomes
revision: 1
---
```
unit: commission/run-outcomes, working tree of commission-w5-run-outcomes (HEAD 2958bfa plus the uncommitted phase 2 and pass-1 files)
verdict: NEEDS-CHANGE
cases: executed 151→156, red 1
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 paths (part 6)
needs-coordinator: yes. generated/rust/commission/src/behaviour.rs and obligation.rs are untracked, and lib.rs declares both modules, so the phase-2 commit must add them. Finding 1's test is in crates/commission-xtask/tests/ rather than the two places you named, because only that crate's tests can reach the xtask binary.
```

**1. `git --no-pager diff --stat`**

It shows only the implementor's 11 files, all modified before I started. Everything I added is an untracked new test file (`git status`):
- `crates/commission-testkit/tests/adversary2_run_conformance.rs`
- `crates/commission-testkit/tests/adversary2_run_capability.rs`
- `crates/commission-testkit/tests/adversary2_run_frontier_rule.rs`
- `crates/commission-xtask/tests/adversary2_run_nested_traits.rs`

None of these is an implementation path.

**2. Cases added**

| Case | Asserts | Now | Red evidence |
|---|---|---|---|
| `adversary2_run_a_hand_written_sealed_trait_is_not_a_model_type` | `no-hand-model` accepts a private `mod sealed { pub trait Sealed {} }` | red | run alone: `src/lib.rs:2: \`trait Sealed\` is a hand-written model type; take it from the generated crate` |
| `adversary2_run_a_verdict_for_a_case_or_whitespace_variant_is_no_verdict` | a verdict for `PROD.DEPLOY`, `prod.deploy `, `" prod.deploy"` and similar does not count for `prod.deploy` | green | mutants M1 and M2: red at `adversary2_run_capability.rs:67`. The unit's suite stays green on both (106 passed, 0 failed). |
| `adversary2_run_an_action_listed_admissible_and_refused_is_not_admitted` | with `deploy` listed both Admissible and Blocked (or Admissible and ApprovalRequired), rule 6 does not continue | green | mutant M3: `left: Continue`. The unit's suite stays green (106/0). |
| `adversary2_run_the_synthesized_run_scenarios_pass_against_run_store` | a hand-written runner executes all 9 `ess verify conform synthesize` scenarios against `Generated<RunStore>`; a wrong-state error must carry the run's state before the command | green | see the next row |
| `adversary2_run_the_runner_fails_a_store_that_breaks_the_scenarios` | proves the runner can fail: a store that forgets suspensions fails ≥5 scenarios, one that lists nothing fails 7 | green | built red first: my guess of 9 failures for the empty store was wrong, because the 2 unknown-instance scenarios expect no view row |

**3. Suite run, after the cases existed**

- Command: `cargo test --workspace --locked --no-fail-fast`, with the brief's `CARGO_TARGET_DIR`.
- Result: exit 101, 155 passed, 1 failed (`adversary2_run_a_hand_written_sealed_trait_is_not_a_model_type`).
- All 5 new test names appear in the run output.
- `cargo fmt --check` exit 0; `cargo clippy --workspace --all-targets --locked -- -D warnings` exit 0.
- The "before" count of 151 is 156 minus my 5 cases. I did not do a separate run without them.

**4. Findings** (they cover the working tree named in the header)

| # | file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|---|
| 1 | `crates/commission-xtask/src/main.rs:534` | INFEASIBLE / introduced | `nested_traits` also goes into the generated crate's private `sealed` modules (9 of them), so it reserves `Sealed`. A hand-written sealed trait is refused as a model type, although nothing outside the generated crate can name `Sealed`. Fix: only descend into `pub` modules. I tried that on a copy: all 32 xtask tests and this case pass. | Nothing today: no `Sealed` in `crates/commission/src` or `crates/commission-testkit/src`. It hits the next person who writes a sealed trait there. |
| 2 | `crates/commission/src/outcome.rs:89` | CONFIRMED / introduced | The capability comparison is exact, which is correct. But the unit's suite still passes if it is changed to ignore case (M1) or trim whitespace (M2), so nothing protects that exactness. My capability case now does. | Every NeedsAuthority row; a verdict counts as a grant. |
| 3 | `crates/commission/src/outcome.rs:113` | CONFIRMED / introduced | The suite still passes if rule 6 reads an action's listed status instead of asking `admit` (M3). Under that mutant, an action listed twice (Admissible and Blocked) keeps the run going. My frontier-rule case now catches it. | `admission.rs:7-16` handles actions listed more than once on purpose. |
| 4 | `crates/commission/src/outcome.rs:102` | CONFIRMED / introduced (judgement) | `NoUsefulAction` on a frontier that admits an action returns `Continue`. The executor said it has nothing useful to do, and the next turn sees the same frontier. The row 1 amendment only covers a frontier that admits nothing. The story and `docs/contracts/commission-executor.md` say nothing about `NoUsefulAction`. | `story:local-runtime-loop`, which is not written yet. |

**5. What I attacked and could not break**

- `RunStore` implements `RunStorage` and `Context`, and `Generated<RunStore>` implements all 3 command behaviours and the `RunStates` query.
- All 9 synthesized scenarios pass when run by hand.
- A wrong-state result carries `RunStateConflict` with the run's current state. An unknown run gives `WrongStateUnknownInstance`, which has no state to carry.
- The `RunStates` view matches the store.
- The capability comparison is exact; a mismatch in case or whitespace fails closed.
- `Marker` being reserved is deliberate: it is a public generated name, and the unit's own test pins it.

**6. Paths written outside the worktree**

- `~/.cache/ga-wave-2026-10-04-w5/commission-run-outcomes/scratch/adv2/suite.json` (kept)
- `~/.cache/ga-wave-2026-10-04-w5/commission-run-outcomes/scratch/adv2/suite-run.log` (kept)
- `~/.cache/b10x-target/commission-w5-run-outcomes/tmp/adversary2_run_conformance/` and `.../tmp/adversary2_run_nested_traits/` (test scratch inside the brief's build dir)
- Already deleted: `~/.cache/b10x-target/commission-w5-mutants2` (576M) and `scratch/adv2/mutant-src`

**7.**

```findings
- file: crates/commission-xtask/src/main.rs
  line: 534
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "nested_traits descends into the generated crate's private sealed modules, so no-hand-model refuses an ordinary hand-written sealed trait named Sealed as a model type; descending only into pub modules fixes it"
- file: crates/commission/src/outcome.rs
  line: 89
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "the unit's suite survives a capability binding that ignores case or trims whitespace, so the byte-exact binding of an authority verdict to its capability was unguarded until adversary2_run_capability"
- file: crates/commission/src/outcome.rs
  line: 113
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "the unit's suite survives rule 6 reading an action's status instead of asking admit, which continues the run on an action listed both Admissible and Blocked; adversary2_run_frontier_rule now kills it"
- file: crates/commission/src/outcome.rs
  line: 102
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "NoUsefulAction on a frontier that admits an action derives Continue, so the runtime loop can re-ask an executor that has declared nothing useful against the same frontier, and no source decides whether it should"
```
