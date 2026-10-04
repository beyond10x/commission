---
format: aep.planning-md/3
id: review-result:adversary-w3-commission-port-skeleton-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w3 adversary, commission story:port-skeleton, pass 2
relations:
- reviews: story:port-skeleton
revision: 1
---
```
unit: commission/port-skeleton, working tree on c6f7f83 + uncommitted phase 2 + pass-1 fixes (worktree commission-w3-port-skeleton)
verdict: CONFIRMED
cases: executed 66→68, red 2
origin: introduced 2 / pre-existing 1 / undecided 0
wrote-outside-worktree: 3 paths
needs-coordinator: none
```

Both new cases are red: `no-hand-model` misses a model type that a macro expands to, and one defined in a module file loaded by `#[path]` from outside the scanned directory. Both escapes compile with rustc. I found no realistic caller that does either, so they are notes, not blockers.

**1. Diff stat.** `git --no-pager diff --stat` is unchanged from the handed tree: 15 files, +431/−147, all from the implementor. My only addition is one untracked test file: `?? crates/commission-xtask/tests/adversary2_scanner_reach.rs`. I did not touch any implementation, `ess/` or `generated/` file.

**2. Cases added.** File: `~/.local/state/worktree/trees/b10x/commission/commission-w3-port-skeleton/crates/commission-xtask/tests/adversary2_scanner_reach.rs`. Both are red now. Captured run of this file alone (`cargo test -p commission-xtask --test adversary2_scanner_reach`, EXIT=101):

| Case | Asserts | Red output (verbatim) |
|---|---|---|
| `adversary2_no_hand_model_refuses_a_model_type_a_macro_expands_to` | `macro_rules! stamp { () => { pub enum GovernorError {…} } } stamp!();` is refused | `no-hand-model passed a hand-written `GovernorError`: status: exit status: 0 … no hand-written model type (88 generated type names checked)` |
| `adversary2_no_hand_model_follows_a_path_attribute_out_of_the_scanned_directory` | `#[path = "../elsewhere/hand.rs"] pub mod hand;`, where `hand.rs` holds `pub struct RunOutcome;`, is refused | `no-hand-model passed a hand-written `RunOutcome`: status: exit status: 0 …` |

I compiled both scenarios with `rustc --crate-type lib` in scratch. Both built, so each one really puts a hand-written `GovernorError` or `RunOutcome` into a crate.

**3. Suite run** (after the cases existed): `cargo test --workspace --locked --no-fail-fast` gives 66 passed, 2 failed (only `adversary2_scanner_reach`), EXIT=101. The earlier count of 66 is 68 minus my 2 cases. `cargo fmt --check` exits 0, and clippy on `-p commission-xtask --all-targets -D warnings` exits 0.

**4. Findings**

| file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|
| `crates/commission-xtask/src/main.rs:603-683` (`hand_model_items`) | INFEASIBLE / introduced | Item-level `macro_rules!` expansions and `include!` are never read. Suggested fix: scan macro token streams for `struct\|enum\|type\|union\|trait <reserved>`. | Nothing found. None of the four wave-4 stories (governor, executor, authority, frontier-admission) mentions macros. |
| `crates/commission-xtask/src/main.rs:552-563` (`rust_sources` walk) | INFEASIBLE / introduced | A `#[path]` module outside `--src` is not scanned. Suggested fix: follow `#[path]` relative to the file, or refuse `#[path]`. | Nothing found. |
| `AGENTS.md:17` | CONFIRMED / pre-existing (also at base 491c2ae) | § Boundary names `CaseRef`, but the generated model has `CaseId`. The text has no Canon remnant left; the removed bullet was the only one. | A reader of AGENTS.md. |

**5. Attacked and could not break**
- **Forms the scanner does catch** (probed with the real binary): nested `mod`, `pub(crate)`, `#[cfg(any())]` / `#[cfg(test)]` items, `type X = …`, `pub use generated::Thing as SuspensionReason`, grouped renames, `{self as X}`, structs in closures. A missing `--src` directory exits 1.
- **Forms that pass but are not definitions** (they don't define a type here): associated types (`type RunOutcome = u8;` in an impl), `extern crate … as RunOutcome`, a plain `pub use b10x_canon::Frontier;` (allowed by the scanner's own test), and nested state markers like `Running` or `Registered`. The help text says only top-level names are reserved.
- **Wiring for wave 4:** each of the four stories' typed scope holds only its own module, fake and test, plus `Taskfile.yml` for agent-executor-port and `ess/` + `generated/` for frontier-admission. None needs `lib.rs`, `ports/mod.rs`, testkit `lib.rs`, `kits/mod.rs`, or a Cargo.toml / Cargo.lock edit.
- **Unit through the JSON codec:** this tree generates no codec module (`generated/.../lib.rs` has only `json`, `primitives`, `responsibility`), so there is nothing to round-trip yet.
  - Read from ESS source 2f554561b, not run against 0.52.0: the union is adjacently tagged (`ess-synth/src/rust/wire.rs:155-170`), so a field named `kind` cannot collide with the tag.
  - The `Unit(pub bool)` stand-in (`generated/.../responsibility.rs:368`) accepts `Unit(false)`, and two values of a variant that carries no data then compare unequal. Decision 1 already records this as a stand-in without an invariant, so I added no case.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w3/commission-port-skeleton/scratch/adversary2-red.log`
- `~/.cache/ga-wave-2026-10-04-w3/commission-port-skeleton/scratch/adversary2-suite.log` and `…/scratch/adversary2-suite-nff.log`
- Builds and test temp files in `~/.cache/b10x-target/commission-w3-port-skeleton`, the brief's build directory, under `tmp/adversary2_scanner_reach/`.

The probe directory `scratch/adv2-probe` is deleted. I created no separate mutant build directories.

**7. Findings block**
```findings
- file: crates/commission-xtask/src/main.rs
  line: 603
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "no-hand-model does not see a model type produced by a macro_rules! expansion (or include!), so `stamp!()` expanding to `pub enum GovernorError` passes with exit 0"
- file: crates/commission-xtask/src/main.rs
  line: 552
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "no-hand-model scans only files under --src, so a `#[path = \"../elsewhere/hand.rs\"] mod hand;` defining `pub struct RunOutcome` compiles into the crate unrefused"
- file: AGENTS.md
  line: 17
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: "AGENTS.md Boundary names `CaseRef`, which neither ess/ nor the generated model declares; the model type is `CaseId`"
```
