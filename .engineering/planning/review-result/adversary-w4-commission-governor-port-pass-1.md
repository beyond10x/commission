---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-governor-port-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:governor-port, pass 1
relations:
- reviews: story:governor-port
revision: 1
---
unit: commission/governor-port. The findings cover phase-1 commit 8f46ec4 plus the uncommitted phase 2 in `~/.local/state/worktree/trees/b10x/commission/commission-w4-governor-port`
verdict: NEEDS-CHANGE (one planning-level judgement finding). No case is red on the real tree; 6 mutants survive the unit's own suite.
cases: executed 75→81, red 0 on the tree (each new case is red against its mutant, part 2)
origin: introduced 8 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path (the scratch dir, part 6); the mutant target dir is created and already deleted
needs-coordinator: yes. Nothing owns giving the fake governor scriptable frontier contents (finding J1)

**1. Diff stat**
```
 crates/commission-testkit/src/fake_governor.rs | 134 ++++++++++++++++++++++++-
 crates/commission/src/ports/governor.rs        |  26 ++++-
?? crates/commission-testkit/tests/adversary_governor_fake.rs
```
- The two `M` paths are the implementer's uncommitted phase 2, the code under attack. I did not edit them. All mutation was done on a copy in scratch.
- My only change is the untracked test file. `--stat` does not list untracked files, so it comes from `git status`.
- No charter violation.

**2. Cases added**

All are in `crates/commission-testkit/tests/adversary_governor_fake.rs`. All 6 are green on the real tree. Each one is red against the mutant it targets, and `governor_port_contract` stays green on m1–m6.

| case | mutant (copy only) | red output, verbatim |
|---|---|---|
| `every_issued_frontier_has_its_own_id` | m1: drop `*issued += 1` (fake_governor.rs:107) | `frontier ids repeat: [..."00000000-0000-4000-8000-000000000000" ×5] left: 1 right: 5` (:46) |
| `scripting_a_case_again_replaces_its_script` | m2: `.insert` → `.entry().or_insert` (:86) | `left: Ok(2) right: Ok(9)` (:59) |
| `a_complete_answer_keeps_its_revision` | m3: `complete()` sets revision 0 (:48) | `left: Ok(0) right: Ok(3)` (:70) |
| `complete_does_not_revive_an_unavailable_answer` | m4: `complete()` turns `Err` into `Ok` (:48) | `left: Ok(Complete(... "X" })) right: Err(GovernorUnavailable)` (:93) |
| `an_empty_script_is_refused` (should_panic) | m5: assert → `true` (:79) | `test did not panic as expected at ...:102:4` |
| `the_fake_is_a_shareable_trait_object` | m6: `fn probe<T>(&self) {}` added to the trait (governor.rs:14) | `error[E0038]: the trait 'Governor' is not dyn compatible` (:18) |

Control m7 (`len() > 1` → `> 0`) is already caught by `governor_port.rs:67`.

**3. Suite run, after the cases existed** (log in `scratch/adv1-suite.log`)
- `cargo fmt --check` exited 0.
- `cargo clippy --locked -p b10x-commission-testkit --all-targets -- -D warnings` exited 0.
- `cargo test --workspace --locked` exited 0, with 81 passed and 0 failed. The binary `.../commission-w4-governor-port/debug/deps/adversary_governor_fake-1ed29ef00eb71c33` ran 6.

**4. Findings**

| id | file:line | verdict | origin | what was measured / what reaches it |
|---|---|---|---|---|
| M1–M6 | fake_governor.rs:107, :86, :48 (×2), :79; governor.rs:14 | CONFIRMED | introduced | `governor_port_contract` stays green under each mutant (part 2). Reached by every later story that uses the fake, and by the documented `# Panics` and "replacing any script" promises. |
| J1 | fake_governor.rs:123-125 | NEEDS-CHANGE | introduced | See below. |
| J2 | fake_governor.rs:61 | CONFIRMED | introduced | See below. |

**J1: the fake cannot script what a frontier contains.**
- What was measured: `frontier()` always issues empty claims, obligations and actions, and no `Answer` API can set them.
- The story contradicts itself. Its Outcome (`governor-port.md:51`) says the script "sets each case's revision, frontier and determination per call". Its Shared surface (`:64`) asks for empty item lists.
- What reaches it, from the integration store:
  - `stale-revision-action-request.md:131-132` needs an action that is admissible at N+1 and a request that passes revalidation. With an empty frontier, nothing is ever admitted.
  - Yet that story says "needs no change to the fake" (`:57`), and its scope does not include `fake_governor.rs`.
  - `run-outcomes.md:92` scripts every row "through their fakes" (rows 3, 5 and 6 need actions or obligations), and `local-runtime-loop` acceptance 2, 4, 7 and 9 need them too.
- No story owns the change. A fake that holds a test-supplied `FrontierData` would build no items itself, which keeps the `frontier-admission` isolation intact.

**J2: no call log** (severity: note).
- `local-runtime-loop.md:102` needs "the fakes' call logs" to show the case loaded and the frontier obtained.
- The fake records no calls. `observation-evidence-ports` adds only an observation log and an evidence log, so no story owns this.

**5. Attacked and could not break**
- **Canon-free:** `governor.rs` imports only `crate::model::responsibility`.
- **Object safety:** the trait works as `Arc<dyn Governor + Send + Sync>` across a thread. The trait does not require `Send`/`Sync` itself. The local loop does not need that.
- **Revision type:** `i64` matches the spec's `Integer` (`generated/.../primitives.rs:9`; `case_revision: i64`, `responsibility.rs:1185`).
- **Frontier ids:** unique within one fake. They are not derived from the case and revision, and two fakes start from the same id. The spec says only that a frontier is "for one case revision", so neither is a defect.
- **Empty script panics:** this is documented misuse of test setup. The fallback at `fake_governor.rs:101` can never be reached, so changing it changes nothing.
- **Acceptance 1–4:** `governor_port_contract` checks each against literal values.
- **Moving case:** every call uses up one answer, so `current_revision` followed by `frontier` can see N and then N+1. That is consistent with a case that moves, and the revalidation scripts in `stale-revision` still work, because the last answer repeats.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w4/commission-governor-port/scratch/` with these `adv1-*` files:
  - `adv1-ws/`, a 1.3M copy of the worktree
  - `adv1-mutants.sh`
  - `adv1-fake.orig` and `adv1-port.orig`
  - `adv1-mutant-m1…m7.log`
  - `adv1-suite.log`
- `~/.cache/b10x-target/commission-w4-mutants`: 34M at its peak, already deleted.

**7.**
```findings
- file: crates/commission-testkit/src/fake_governor.rs
  line: 107
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "governor_port_contract stays green when every frontier carries the same frontier_id; killed by every_issued_frontier_has_its_own_id"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 86
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the documented replace-on-rescript is unobserved; or_insert survives the suite; killed by scripting_a_case_again_replaces_its_script"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 48
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "Answer::complete may drop its revision (test scripts at(3).complete and never reads 3); killed by a_complete_answer_keeps_its_revision"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 48
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Answer::complete may revive an unavailable answer, contrary to its doc; killed by complete_does_not_revive_an_unavailable_answer"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 79
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the documented empty-script panic can be removed with the suite green; killed by an_empty_script_is_refused"
- file: crates/commission/src/ports/governor.rs
  line: 14
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "nothing pins Governor as dyn compatible or the fake as Send+Sync; a generic trait method survives the suite; killed by the_fake_is_a_shareable_trait_object"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 123
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "the fake cannot script frontier contents although the story Outcome says the script sets the frontier, and stale-revision-action-request acceptance 1-2 (which claims no fake change) plus run-outcomes and local-runtime-loop need listed actions and obligations, with no story owning the change"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 61
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the fake records no call log, which local-runtime-loop acceptance 1 reads, and no story in the store names adding one"
```
