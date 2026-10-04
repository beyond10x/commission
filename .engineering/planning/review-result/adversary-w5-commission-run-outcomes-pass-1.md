---
format: aep.planning-md/3
id: review-result:adversary-w5-commission-run-outcomes-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w5 adversary, commission story:run-outcomes, pass 1
relations:
- reviews: story:run-outcomes
revision: 1
---
```
unit: commission/run-outcomes, working tree of commission-w5-run-outcomes (phase-1 commit 2958bfa + uncommitted phase 2)
verdict: CONFIRMED (1 red case: acceptance row 1 read literally); otherwise nothing broken, 7 mutants the suite misses now covered
cases: executed 139→149, red 1
origin: introduced 2 / pre-existing 1 / undecided 0
wrote-outside-worktree: 2 paths (scratch/adv1/suite.json, scratch/adv1/suite.log)
needs-coordinator: yes. Story wording for acceptance row 1 against rows 5 and 6 (see F1)
```

**1. Diff stat.** `git diff --stat` shows only the implementor's 9 tracked files. I added two new untracked files, both test files:
- `crates/commission-testkit/tests/adversary_run_outcomes.rs`
- `crates/commission-testkit/tests/adversary_run_acceptance_literal.rs`

No implementation, `ess/` or `generated/` file was touched. I ran `rustfmt` and fixed one clippy lint, in my own files only.

**2. Cases added** (each file run alone first, before the suite)

| File / case | Asserts | Now |
|---|---|---|
| `adversary_run_acceptance_literal.rs` | CompletedLocalReasoning on an open case derives Continue when the frontier admits nothing | **red** |
| `adversary_run_outcomes.rs` (9 cases) | Deny → NoAdmissibleAction; provider failure → NoAdmissibleAction; Allow → Continue; an ApprovalRequired action is not counted as admissible; a refused proposal does not continue; governor completion beats executor Suspended; two runs are kept apart; an unknown run id creates nothing; a repeated run id does not replace the stored run | green, 9 passed |

Red output, verbatim:
```
panicked at crates/commission-testkit/tests/adversary_run_acceptance_literal.rs:53:5:
assertion `left == right` failed: acceptance row 1: CompletedLocalReasoning on an open case derives continue
  left: Ended(NeedsExternalEvidence(RunOutcomeNeedsExternalEvidence { requirements: ["tests pass"] }))
 right: Continue
test result: FAILED. 0 passed; 1 failed
```

**3. Suite** (`cargo test --workspace --locked --no-fail-fast`, brief's build dir)
- Exit 101: 148 passed, 1 failed, 149 executed. The only failure is the literal case.
- 139 = 149 minus my 10 cases.
- After the lint fix: `cargo fmt --check` exits 0 and `cargo clippy --workspace --all-targets --locked -- -D warnings` exits 0.

**4. Findings**

| # | file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|---|
| F1 | `crates/commission/src/outcome.rs:87` | CONFIRMED / introduced | The literal case is red. The acceptance says CompletedLocalReasoning on an open case "derives continue" and names no frontier. `derive` sends it to the frontier rule, which ends the run when nothing is admissible. `run_outcomes.rs:185` uses a frontier with an admissible action, so that row cannot tell the two readings apart. | Every run whose executor finishes local reasoning on a frontier with nothing admissible. Answering Continue there could loop forever, so the right fix may be the story's wording rather than the code. That is the coordinator's call. |
| F2 | `crates/commission/src/outcome.rs:54` | CONFIRMED / introduced | `derive` takes `Option<&AuthorityVerdict>`, which is not tied to the capability `admit` returned. An Allow obtained for another capability gives Continue. `Derived::Continue` also looks the same for "admitted proposal" and "denied proposal while some other action is admissible". | No caller yet besides tests; story:local-runtime-loop will be the first. ADR 0082's recheck right before invoking limits the risk. Judgement, note. |
| F3 | `crates/commission-xtask/src/main.rs:521` | CONFIRMED / pre-existing | `no-hand-model` reserves only top-level item names. A hand-written `pub trait StartRunBehavior {}` and `pub trait RunStatesQuery {}` in a scratch copy of `commission/src` passed (exit 0, "106 generated type names checked"). A control, `pub struct RunStates;`, was refused (exit 1). The new `responsibility::obligations` traits fall through. | xtask source is byte-identical to c8a25fa, where the nested `*_state::Marker` traits already fall through the same gap. Note. |

**5. Attacked and not broken**
- **Authority fail-open:** Deny, provider failure and refused proposals never derive Continue on that action. ApprovalRequired actions are not counted as admissible.
- **Completion before executor Suspended:** matches `derive`'s rule 1 and the story's "only the governor completes". Neither the story nor ADR 0082 orders the two otherwise.
- **Obligations filter:** keeps only open obligations, in order. Acceptance rows 5 and 6 already kill a dropped filter.
- **RunStore id collision:** it panics rather than returning a typed error. A typed error cannot be expressed: the generated `Context` port returns a bare `RunId` and StartRun has no error outcome. My case confirms the panic leaves the stored run unchanged.
- **Generated `behaviour.rs`:** PLAN.md lists 69 capabilities, all generated, 0 obligations, so no contract is left unimplemented.
- **ESS conformance:** 9 scenarios synthesize with 0 refusals, but nothing runs them yet. That is planned for story:commission-ess-conformance (status proposed), so it is not a defect of this unit.
- **Mutants were reasoned, not built.** Disk was 9.4G free throughout, below the 10G floor, so the mutants2 build dir was never created. By reading the 7 acceptance rows, these mutants survive the current suite and each is killed by one of my green cases:
  - every non-ApprovalRequired verdict answered with Continue
  - the Allow arm dropped
  - NeedsAuthority counted as admissible in the frontier rule
  - a refused proposal answered with Continue
  - executor checked before completion
  - every run stored under one key
  - the duplicate-id assert dropped

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w5/commission-run-outcomes/scratch/adv1/suite.json`
- `~/.cache/ga-wave-2026-10-04-w5/commission-run-outcomes/scratch/adv1/suite.log`
- My scratch copies (`adv1/tree`, `adv1/nhm`) were deleted. Session lease acquired and released.

```findings
- file: crates/commission/src/outcome.rs
  line: 87
  category: acceptance
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "CompletedLocalReasoning on an open case derives Continue only when the frontier admits an action, not unconditionally as acceptance row 1 states; the acceptance row's admissible frontier hides this, and story or code must settle it"
- file: crates/commission/src/outcome.rs
  line: 54
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "derive takes an authority verdict not bound to the proposal's capability, and its Continue does not tell an admitted proposal from a denied one, leaving the invoke decision to the future runtime loop's recheck"
- file: crates/commission-xtask/src/main.rs
  line: 521
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: "no-hand-model reserves only top-level generated names, so hand-written StartRunBehavior, SuspendRunBehavior, ResumeRunBehavior and RunStatesQuery traits pass the check"
```
