---
format: aep.planning-md/3
id: review-result:adversary-w1-commission-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w1 adversary, commission unit, pass 1
relations:
- reviews: story:generated-responsibility-model
revision: 1
---
unit: commission — working tree at ~/.local/state/worktree/trees/b10x/commission/commission-w1-generated-responsibility-model, base 013e3924, uncommitted
verdict: red
cases: executed 15→21, red 6
origin: introduced 10, pre-existing 0, undecided 0
wrote-outside-worktree: ~/.cache/ga-wave-2026-10-04-w1/commission/scratch/suite.log (may have overwritten the implementer's own suite.log); ~/.cache/b10x-target/commission-w1/tmp/adversary/ (11 case dirs)
needs-coordinator: yes. My suite log may have replaced an implementer log in the shared scratch dir. Also, acceptance 2 needs a decision on the order of steps in `check`.

The unit fails: 6 new test cases are red against `no-hand-model`, and acceptance 2 is not met through `task check`.

**1. Files touched** (`git status --short -uall`; I changed no tracked files)
- I added one file: `crates/commission-xtask/tests/adversary_no_hand_model.rs` (a test file). It is untracked like the rest of `crates/commission-xtask/`, so it does not show in `git diff --stat`.
- `git --no-pager diff --stat` is the same as the implementer's: 7 files, +225 / −17. No other path is mine.

**2. Cases added** (in that file; run alone with `cargo test -p commission-xtask --test adversary_no_hand_model`: 0 passed, 6 failed)

| case | what it does | red output |
|---|---|---|
| `adversary_flags_a_hand_written_type_reexported_under_a_model_name` | `pub struct Handmade(pub String); pub use self::Handmade as AgentId;` | `…/src: no hand-written model type` |
| `adversary_flags_a_model_type_defined_through_a_macro` | a newtype macro, then `id!(AgentId); id!(CommissionId);` | `…/src: no hand-written model type` |
| `adversary_flags_a_raw_identifier_model_type` | `pub struct r#AgentId(pub String);` | `…/src: no hand-written model type` |
| `adversary_flags_every_entity_the_story_says_is_generated` | `pub struct <name> { pub id: String }` | `no-hand-model passed hand-written ["Agent", "AgentRevision", "Case", "PrincipalId", "AuthorityContext", "CommissionData"]` |
| `adversary_flags_a_path_module_outside_src` | `#[path = "../model/hand.rs"] pub mod hand;` with that file defining `AgentId` | `…/src: no hand-written model type` |
| `adversary_does_not_flag_a_string_literal` | `pub const HINT: &str = "struct Commission comes from …";` | `src/lib.rs:1: \`struct Commission\` is a hand-written model type` |

Clippy and fmt pass on the new file.

**3. Suite run** (after the cases existed)
- Command: `cargo test --workspace --locked --no-fail-fast`, exit 101.
- `generated_model` 2 ok, xtask unit tests 3 ok, `checks` 10 ok, `adversary_no_hand_model` 0/6.
- The before count is 15: the same run with my binary excluded.

**4. Findings**

| # | file:line | verdict | what was measured | what reaches it |
|---|---|---|---|---|
| F1 | Taskfile.yml:15-16 | NEEDS-CHANGE | The unit's own mutation (middle byte → `x`, offset 23828) breaks the generated crate. On a scratch copy, `cargo check` exits 101 at `responsibility.rs:662`. In `check`, clippy runs before drift, so drift never runs. Acceptance 2 says "its drift step fails", and here it does not. Fix: move `task: drift` and `task: no-hand-model` ahead of fmt and clippy. The xtask depends only on clap. | `task check`, any byte change that does not compile |
| F2 | crates/commission-xtask/src/main.rs:38 | CONFIRMED | `MODEL_TYPES` holds only the 5 names the story listed. A hand-written Agent, AgentRevision, Case, PrincipalId, AuthorityContext or CommissionData passes, although the Outcome and acceptance 4 call them generated. | any later story in the chain |
| F3 | main.rs:342-363 | CONFIRMED | A hand-written type re-exported under a model name passes. | an ordinary `pub use` |
| F4 | main.rs:342-363 | CONFIRMED | Types defined through a macro pass. | a newtype macro for ids is a common Rust pattern |
| F5 | main.rs:347 | INFEASIBLE | `r#AgentId` passes. | nothing found |
| F6 | main.rs:303-310 | INFEASIBLE | Only `.rs` files under `src` are read, so a `#[path]` or `include!` source outside `src` passes. | nothing found |
| F7 | main.rs:345 | INFEASIBLE | The scan does not understand string literals or `/* */` comments, so it flags non-definitions. | nothing found |
| F8 | Taskfile.yml:28, crates/commission/Cargo.toml:10 | CONFIRMED | With `generated/rust/commission/` absent, `cargo run --locked -p commission-xtask -- generate` exits 101 (`no matching package named commission`). The generator needs its own output to start. | deleting the tree to regenerate; or `generate` failing after `remove_dir_all` (main.rs:142), e.g. with the disk full (it is at 99%) |
| F9 | crates/commission/src/lib.rs:21,66 | CONFIRMED | Hand-written `ExecutorOutcome` and `AuthorityDecision` sit next to generated types of the same names (responsibility.rs:102,478). `b10x_commission::AuthorityDecision` is not the model's type. | callers by name; the port stories are meant to move these |
| F10 | crates/commission/Cargo.toml:10 | CONFIRMED | Nothing ties the `commission` path dependency to the directory `drift` checks. A hand-written crate at another path would pass drift, no-hand-model and both tests. | nothing found |

**5. What I attacked and could not break**
- `generate` on a scratch root with a stale file: the stale file is removed and the output is byte-identical to the committed tree.
- The narrowed `cargo fmt --check -v` reads no file under `generated/`.
- `ess` refuses a `requires` pin newer than itself (exit 1).
- `.ess-output` is skipped. A missing file, a stray file and a changed byte that still compiles are all named.
- The unit's own tests can each fail. None of them is an assertion that always passes.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w1/commission/scratch/suite.log`. This may have overwritten an implementer file of the same name; I did not list the directory first.
- `~/.cache/b10x-target/commission-w1/tmp/adversary/*` (11 case dirs).
- `scratch/acc2`, `scratch/boot` and `scratch/req` were created and then deleted.

```findings
[
{"file":"Taskfile.yml","line":15,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"clippy runs before drift in check, so a generated-source byte change that breaks compilation stops task check before the drift step runs and names the file, contrary to acceptance 2"},
{"file":"crates/commission-xtask/src/main.rs","line":38,"category":"acceptance","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"MODEL_TYPES omits Agent, AgentRevision, Case, PrincipalId, AuthorityContext and CommissionData, which the Outcome and acceptance 4 call generated, so hand-written copies pass"},
{"file":"crates/commission-xtask/src/main.rs","line":342,"category":"boundary","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"a hand-written type re-exported under a model name via pub use ... as AgentId passes no-hand-model"},
{"file":"crates/commission-xtask/src/main.rs","line":350,"category":"boundary","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"model types defined through a macro_rules newtype macro pass no-hand-model"},
{"file":"crates/commission-xtask/src/main.rs","line":347,"category":"boundary","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"pub struct r#AgentId passes because the word split leaves r and AgentId as separate words"},
{"file":"crates/commission-xtask/src/main.rs","line":310,"category":"boundary","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"only .rs files under src are scanned, so a #[path] module or include! source outside src defining AgentId passes"},
{"file":"crates/commission-xtask/src/main.rs","line":345,"category":"boundary","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"a string literal or block comment containing struct Commission is reported as a hand-written definition"},
{"file":"crates/commission/Cargo.toml","line":10,"category":"judgement","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"task generate cannot run when generated/rust/commission is absent because the workspace path dependency fails to load (exit 101), and generate removes that tree before copying"},
{"file":"crates/commission/src/lib.rs","line":66,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"hand-written ExecutorOutcome and AuthorityDecision share names with generated model types, so b10x_commission exposes two distinct types under each name"},
{"file":"crates/commission/Cargo.toml","line":10,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"nothing binds the commission path dependency to the directory drift checks, so a hand-written crate at another path passes every gate"}
]
```
