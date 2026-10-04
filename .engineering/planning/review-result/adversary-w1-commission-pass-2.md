---
format: aep.planning-md/3
id: review-result:adversary-w1-commission-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w1 adversary, commission unit, pass 2
relations:
- reviews: story:generated-responsibility-model
revision: 1
---
unit: commission, the uncommitted working tree at `~/.local/state/worktree/trees/b10x/commission/commission-w1-generated-responsibility-model` (base 013e3924)
verdict: NEEDS-CHANGE
cases: executed 36→39, red 1
origin: introduced 5 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 paths (part 6)
needs-coordinator: yes. The unit changed `ess/system.yaml` (from `format: ess/15` to `format: ess/20`), a file the brief marks "not yours"; it needs to be reverted or approved. Loom also needs a follow-up story (F2).

The pass-1 corrections hold. The syn scan, the model names taken from the generated crate, the bootstrap plus the rename swap, the dependency binding, and drift running before clippy all held under attack. What I found: one out-of-scope spec edit, one broken consumer (Loom), one red edge case, and two gaps in the suite's test coverage.

**1. `git --no-pager diff --stat`**: 7 files, +353/−17, the same as the implementor's. My only file is the untracked test file `crates/commission-xtask/tests/adversary_pass2.rs`. I changed no implementation file.

**2. Cases added** (in `adversary_pass2.rs`). First run alone: `cargo test --locked -p commission-xtask --test adversary_pass2`, 2 passed, 1 failed, exit 101. Output in `scratch/adv2-red.log`.

| case | asserts | now |
|---|---|---|
| `adversary2_drift_step_names_a_changed_byte_in_the_generated_manifest` | changing generated `Cargo.toml` from `1.0.0` to `1.0.1` makes the drift step (`cargo run -q --locked -p commission-xtask -- drift`) report `Cargo.toml: differs from synthesis` | **red**: `error: cannot update the lock file …/root/Cargo.lock because --locked was passed to prevent this`. Drift never runs, so the file is never named |
| `adversary2_drift_refuses_model_reexported_from_another_crate` | drift refuses `pub use b10x_canon as model;` | green. Catches mutant M1 |
| `adversary2_no_hand_model_refuses_a_rename_to_a_pending_replacement_name` | no-hand-model refuses `pub use hand::Handmade as ExecutorOutcome;` | green. Catches mutant M2 |

**Mutant probe** (on a copy in scratch, now deleted). M1 removes `&& rename.ident.unraw() == GENERATED_PACKAGE` from main.rs:332-333. M2 removes `found.is_definition &&` from main.rs:551. With both applied, the existing suite stays green: 33 tests, `task_generate` skipped, exit 0. My two cases go red, exit 101 (`adv2-mut-existing.log`, `adv2-mut-mine.log`).

**3. Suite run, after the cases existed**: `cargo test --workspace --locked --no-fail-fast`, exit 101. Results: generated_model 2 ok, xtask unit tests 5 ok, adversary_no_hand_model 6 ok, checks 23 ok, adversary_pass2 2 ok and 1 failed. clippy `-D warnings` exits 0; `cargo fmt -p b10x-commission -p commission-xtask --check` exits 0. The before count of 36 is the implementor's own gate run (`scratch/impl-pass2-gate.log`).

**4. Findings**

| # | file:line | verdict | measured | what reaches it |
|---|---|---|---|---|
| F1 | `ess/system.yaml:1` | NEEDS-CHANGE | The format line went from `ess/15` to `ess/20` (file changed 00:28:54Z, after pass 1). On a copy with `ess/15`, `ess specify validate` exits 0, and synthesis produces a tree byte-identical to the committed one (`diff -r` shows no difference). The edit has no effect, and the brief fixes this file for the unit | every commit of this unit |
| F2 | Loom `crates/loom/src/lib.rs:6,112` | CONFIRMED | A copy of Loom built against this unit's `b10x-commission` (`cargo check`, patched) fails with exit 101: E0432 for `b10x_commission::Commission`, `AgentId` and `CommissionId`. Its `impl AgentExecutor` also expects `&Commission`, but the trait now takes `&CommissionData`. The story requires deleting these types, but no story names Loom | Loom's next move of its pin (its `Cargo.lock` pins commission at 652537e) |
| F3 | `crates/commission/Cargo.toml:10` / `Taskfile.yml:41` | INFEASIBLE | the red case above. A byte change in the generated manifest's version stops cargo's lock check before drift runs, so the drift step never names the file. The story says drift "names the file" on any difference | nothing found; the version comes from the spec's `version: v1` |
| F4 | `crates/commission-xtask/src/main.rs:333` | CONFIRMED | mutant M1 survives the existing suite | a regression in the pass-1 F10 fix |
| F5 | `crates/commission-xtask/src/main.rs:551` | CONFIRMED | mutant M2 survives the existing suite | a regression in the pending-replacement allowance |

**5. Attacked and could not break**
- Rename into a pending name, re-export of a non-generated crate, and dev-dependency or target-specific copies of the `commission` package: all refused.
- `r#` raw identifiers, string literals and block comments are now handled correctly.
- `task check` ordering: drift and no-hand-model run before fmt and clippy.
- go-task 3.53 lets an exported `CARGO_TARGET_DIR` override the Taskfile's own value, so the nested `task generate` test does not write to the shared build directory.
- CI pins `ess 0.52.0`, which matches local, and synthesis output contains no absolute paths.
- Two tests (macros and `#[path]` modules, pass-1 F4/F6) now assert that these cases pass, as the coordinator decided. I did not raise them again.

**6. Paths written outside the worktree**
- Kept, as evidence: `~/.cache/ga-wave-2026-10-04-w1/commission/scratch/adv2-red.log`, `adv2-suite.log`, `adv2-mut-existing.log` and `adv2-mut-mine.log`.
- Deleted: `scratch/adv2-taskenv`, `scratch/adv2-ess15`, `scratch/adv2-loom` (Loom source copy plus its build directory), `scratch/adv2-mut` (mutant copy plus its build directory), and `~/.cache/b10x-target/commission-w1/tmp/adversary2`. My tests recreate that last one on every run.

```findings
[
{"file":"ess/system.yaml","line":1,"category":"judgement","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"format bumped ess/15 to ess/20 in a file the brief fixes for this unit; ess/15 validates and synthesizes a byte-identical tree, so the edit is unneeded and out of scope"},
{"file":"~/beyond10x/loom/crates/loom/src/lib.rs","line":6,"category":"contract-drift","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"Loom imports b10x_commission::{Commission, AgentId, CommissionId} and implements AgentExecutor over &Commission; against this unit cargo check fails E0432 (exit 101) and no story names the Loom follow-up"},
{"file":"crates/commission/Cargo.toml","line":10,"category":"contract-drift","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"a one-byte change in the generated crate's manifest version stops the --locked drift step at cargo's lock check, so drift never names Cargo.toml as the Outcome promises"},
{"file":"crates/commission-xtask/src/main.rs","line":333,"category":"mutant","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"dropping the check that the model re-export names the commission crate leaves the existing suite green; adversary2_drift_refuses_model_reexported_from_another_crate now catches it"},
{"file":"crates/commission-xtask/src/main.rs","line":551,"category":"mutant","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"dropping is_definition from the pending-replacement match leaves the existing suite green; adversary2_no_hand_model_refuses_a_rename_to_a_pending_replacement_name now catches it"}
]
```
