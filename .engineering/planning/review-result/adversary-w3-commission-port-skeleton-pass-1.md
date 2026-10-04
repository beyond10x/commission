---
format: aep.planning-md/3
id: review-result:adversary-w3-commission-port-skeleton-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w3 adversary, commission story:port-skeleton, pass 1
relations:
- reviews: story:port-skeleton
revision: 1
---
unit: commission/port-skeleton, the uncommitted phase-2 working tree on c6f7f83 (base 491c2ae)
verdict: CONFIRMED (warning): one mutant the existing suite misses; nothing red on the tree
cases: executed 64→65, red 0 on the tree (1 red against the mutant)
origin: introduced 1 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 paths (2 deleted)
needs-coordinator: whether to keep my case as the test that pins the `PENDING_REPLACEMENT` removal

The unit's phase-2 work holds up: I could not break any acceptance item. The one gap is that no test checks the removed allowance. If it is put back, the existing 64 cases all still pass. My new case catches that.

**1. Diff stat.** `git --no-pager diff --stat` lists 13 modified files. All of them are the implementor's uncommitted phase 2, none are mine. My only addition is one untracked test file: `crates/commission/tests/adversary_skeleton_no_hand_model.rs`. I changed no implementation file, nothing under `ess/` and nothing under `generated/`.

**2. The case I added.** `adversary_skeleton_no_hand_model_refuses_hand_written_bootstrap_outcomes_in_port_modules`
- **What it does:** writes `src/ports/executor.rs` holding `pub enum ExecutorOutcome {…}` and `src/ports/authority.rs` holding `pub enum AuthorityDecision {…}`. It runs this tree's `cargo run -p commission-xtask -- no-hand-model` against the real generated crate. It expects a failure that names both files at line 1.
- **On the tree:** green (case run alone, exit 0).
- **On the mutant:** red. The mutant is a scratch copy of the whole repository with the base version of `crates/commission-xtask/src/main.rs`, which still has the allowance. Run alone there it failed with exit 101:
  ```
  panicked at crates/commission/tests/adversary_skeleton_no_hand_model.rs:73:5:
  no-hand-model passed hand-written ExecutorOutcome and AuthorityDecision:
  stdout:
  note: …/src/ports/authority.rs:1: `enum AuthorityDecision` shares a name with a generated type; allowed until story:authority-provider-port replaces it
  note: …/src/ports/executor.rs:1: `enum ExecutorOutcome` shares a name with a generated type; allowed until story:agent-executor-port replaces it
  …/src: no hand-written model type (88 generated type names checked)
  ```
- **The rest of the suite misses the mutant:** `cargo test --workspace --locked --no-fail-fast -- --skip adversary_skeleton_no_hand_model` on the mutant exited 0, with 64 passed and 0 failed. That run is also where the `<before>` count of 64 comes from.

**3. Full gate on the unit tree, run after the case existed**

| Step | Exit | Result |
|---|---|---|
| `cargo fmt --check` | 0 | |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 | |
| `cargo test --workspace --locked --no-fail-fast` | 0 | 65 passed: adversary2_gate 13, adversary_skeleton 1, ess_gate 11, generated_model 2, skeleton 1, xtask unit 5, adversary_no_hand_model 6, adversary_pass2 3, checks 23 |
| `ess` validate + synthesize | 0 | "0 scenario(s) (0 authored), 0 refusal(s)" |
| `drift` | 0 | "no drift from ess/" |
| `no-hand-model` | 0 | "88 generated type names checked" |

`cargo test -- --list` confirms both `skeleton_lands_port_vocabulary_and_modules` and my case exist in this tree.

**4. Findings**

| file:line | verdict | origin | finding | what reaches it |
|---|---|---|---|---|
| `crates/commission-xtask/src/main.rs:542` | CONFIRMED (warning, mutant) | introduced | Removing `PENDING_REPLACEMENT` (story § Outcome (b)) is not pinned by any test. No test defines `ExecutorOutcome` or `AuthorityDecision`; `MODEL_NAMES` in `checks.rs:7` still lists only the 5 id/Commission names. | Next wave: `story:agent-executor-port` writes `ports/executor.rs` and `story:authority-provider-port` writes `ports/authority.rs`. Skeleton item 6 checks only `lib.rs`. **Fix:** keep my case, or add both names to an xtask definition test. |
| `crates/commission-xtask/src/main.rs:28` | CONFIRMED (note, judgement) | introduced | `no-hand-model` scans only `crates/commission/src`. The new `commission-testkit` crate, whose fakes three parallel stories fill, is not scanned. The xtask doc and the Taskfile do declare that scope, so this is not a contract break. | Nothing yet: the fakes are empty today. |

**5. What I attacked and could not break**
- **Spec vs story and coordinator decisions:** every declaration, variant set, struct field and order matches. That covers `Json` vs `String`, `Evidence: List<String>`, `Unit` as a newtype of `Boolean` with no invariant, the `kind` tag, no `Run` commands, the `AuthorityContext` comment, and the `Observation`/`Evidence` fields.
- **Skeleton test:** checks are exact, not presence-only. Unions are compared as sorted (variant, payload) pairs; structs in field order; enums exactly; tag `kind`; the `Evidence.observations` relation as its full key/value shape. Item 3 uses `type_name` with a `commission::` prefix, item 5 parses real `cargo tree` output, item 6 uses a `syn` visitor. Reading the code, the mutants I considered (a wrong payload, an extra variant, a different tag, a wrong field type, a new `Observation` relation, an enum or trait nested in `lib.rs`, `b10x-canon` back in the manifest) are all caught.
- **Generated tree:** `drift` is clean; the changes are the digest headers plus 19 new types.
- **Testkit wiring:** picked up by `members = ["crates/*"]`, `license.workspace` (Apache-2.0), `publish = false`. `syn` is a dev-dependency only, `Cargo.lock` gains the crate, and `b10x-commission` does not depend on it.
- **`AGENTS.md`:** no Canon reference is left in tracked files outside `docs/history`. CI runs `task check`, so the testkit is covered by the workspace test run.
- **Loom (not a defect here):** `loom/crates/loom/src/lib.rs:6,112` imports `b10x_commission::{AgentExecutor, Commission, ExecutorOutcome, AgentId, CommissionId}` from the crate root. Its pin `652537e` still has `b10x-canon` (Loom `Cargo.lock:11-15`). A pin bump will need:
  - the trait from `b10x_commission::ports::executor`, which comes after `story:agent-executor-port`;
  - the model types from `b10x_commission::model::responsibility`;
  - `ExecutorOutcome::ProposedAction(ExecutorOutcomeProposedAction { action: String, arguments: ProposedActionArguments(Json) })` in place of `arguments_json: String`;
  - `Suspended(ExecutorOutcomeSuspended { reason: SuspensionReason::Authority(Json) })` in place of `reason: String`;
  - `AgentExecutor::run` to take `CommissionData` instead of `Commission`;
  - a decision on Canon `Frontier` vs Commission's own `Frontier` entity.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w3/commission-port-skeleton/scratch/adv1-gate.log` (kept)
- `~/.cache/ga-wave-2026-10-04-w3/commission-port-skeleton/scratch/adv1-mutant-pending/` (deleted)
- `~/.cache/ga-wave-2026-10-04-w3/commission-port-skeleton/scratch/adv1-mutant-target/` (357M, deleted)
- My case's own scratch under the brief's `CARGO_TARGET_TMPDIR` is removed by the test itself.
- I also took and released my own worktree session lease (`adversary-port-skeleton-p1`).

```findings
- file: crates/commission-xtask/src/main.rs
  line: 542
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "Restoring the removed PENDING_REPLACEMENT allowance leaves all 64 existing cases green; only crates/commission/tests/adversary_skeleton_no_hand_model.rs catches it."
- file: crates/commission-xtask/src/main.rs
  line: 28
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "no-hand-model scans only crates/commission/src, so the new commission-testkit crate whose fakes three parallel stories fill is outside the hand-model check."
```
