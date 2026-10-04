---
format: aep.planning-md/3
id: review-result:adversary-w7-commission-commission-ess-conformance-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w7 adversary, commission story:commission-ess-conformance, pass 1
relations:
- reviews: story:commission-ess-conformance
revision: 1
---
unit: commission/commission-ess-conformance, working tree on top of 1a37f64 (phase 1 plus uncommitted phase 2)
verdict: blocked: disk — I wrote 3 red cases but ran none of them. `df -BM /` showed 9650M free twice, which is under the 10240M floor
cases: executed 185→not run, red 0 measured (3 written)
origin: introduced 3 (read from the code, not run) / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path (below)
needs-coordinator: free disk to ≥10240M, then run `cargo test --locked -p b10x-commission-conformance --test adversary_conform_codec`

**1. Diff stat.** `git --no-pager diff --stat` shows only the implementor's uncommitted phase 2 (Cargo.lock, Taskfile.yml, conformance Cargo.toml, lib.rs, checks.rs, generated_model.rs). My only addition is a new untracked test file, `crates/commission-conformance/tests/adversary_conform_codec.rs`. I changed no implementation file.

**2. Cases written (none run).** Each takes a scenario from the freshly synthesized suite, changes only its literals, and asserts that the scenario still passes through `run_suite`.

| case | asserts | what I expect it to show (from reading the code) |
|---|---|---|
| `adversary_start_run_echoes_an_integer_above_2_pow_53` | StartRun/outcome/started with `case_revision` = 2^53+1 still passes | `codec.rs:88` `value as f64` rounds the `RunStarted` payload and the `RunStates` row to 2^53. ESS `Number` equality is exact (`facts.rs` `Ord::cmp`), so the scenario fails |
| `adversary_suspend_run_echoes_a_json_reason_exactly` | SuspendRun/outcome/suspended with reason `{"kind":"Authority","value":{"limit":2^53+1}}` still passes | `to_json` keeps the value exact. On the way back, `codec.rs:69` `from_json` parses it as `f64`, so `RunSuspended.reason` ≠ `input.reason` |
| `adversary_forced_stale_at_the_largest_revision_answers_stale` | a forced `stale` at `expected_case_revision = i64::MAX` returns `Stale` | `governor.rs:62` `saturating_add(1)` gives back `i64::MAX`, so the governor agrees with the request and `revalidate` returns `Admitted` |

Fixes I'd suggest (I did not apply them):
- `codec::number`: use `Number::from(value)`.
- `from_json` and `to_json` (`:50`): use `Number::exact_text` / `decimal_literal` instead of going through `f64`.
- `ScenarioGovernor`: use `expected.checked_add(1).unwrap_or(expected - 1)`.

**3. Suite run.** None. Every build was stopped by the disk floor.

**4. Judgement findings** (they cover the working tree):
- The 3 cases above are `INFEASIBLE`: the synthesized suite uses only the literals 1 and `"reason"`, and the repo has 0 authored scenarios. Their effect is that the codec loses ESS `Integer` and `Json` values above 2^53, which ESS admits.
- A SKIPPED.md entry for a scenario that actually passes is accepted by both checks (`conform.rs` `skipped_md_names`, `generated_model.rs` new test). This is allowed by acceptance 3. Note.
- No minimum scenario count is pinned. `total == scenarios` compares two values from the same run, so if `ess/` shrinks the test stays green. Acceptance 1 only asks for ≥1 passed. Note.

**5. Attacked and not broken** (by reading the code):
- **Hard-coded answers:** none found. Every outcome, event, error field and view row comes from `Generated<RunStore>` or `revalidate`. A decode failure gives `undeclared`, which fails the scenario rather than passing it.
- **Revalidate inputs:** all 6 fields reach `revalidate` decoded from the input. Only `expected_case_revision` and `action` show up in what the suite checks; `case_id`, `run_id`, `action_request_id` and `arguments` are never compared (the spec doesn't declare them in any outcome).
- **Report categories:** `violations` checks all 5 (`passed`, `failed`, `error`, `unsupported`, `skipped`) against `counts.rs:112`.
- **Malformed SKIPPED.md lines:** rejected by the `generated_model` test. Strict format: `- id: reason`, non-empty reason, the id must be in the suite.
- **Taskfile vs CI:** CI runs `task check`, which includes `conform`. `cargo test --workspace` runs the conform test a second time; harmless.
- **`ess` missing from PATH:** both tests panic with "`ess` must be on PATH". New: `cargo test -p b10x-commission` now needs `ess` too.

**6. Paths written outside the worktree:**
- `~/.cache/ga-wave-2026-10-04-w7/commission-commission-ess-conformance/scratch/adv/suite.json`

```findings
- file: crates/commission-conformance/src/codec.rs
  line: 88
  category: boundary
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "codec::number encodes an i64 through f64, so a case_revision above 2^53 comes back rounded in RunStarted and RunStates; case adversary_start_run_echoes_an_integer_above_2_pow_53 is written but not run (disk floor), and no synthesized scenario uses such a value"
- file: crates/commission-conformance/src/codec.rs
  line: 69
  category: boundary
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "from_json reparses a model Json number as f64, so a SuspensionReason holding an integer above 2^53 is not echoed exactly in RunSuspended.reason; case adversary_suspend_run_echoes_a_json_reason_exactly is written but not run (disk floor)"
- file: crates/commission-conformance/src/governor.rs
  line: 62
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "a forced stale at expected_case_revision i64::MAX saturates to the same revision, so the governor agrees and revalidate answers Admitted instead of Stale; case adversary_forced_stale_at_the_largest_revision_answers_stale is written but not run (disk floor)"
- file: crates/commission-conformance/tests/conform.rs
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "neither SKIPPED.md check refuses an entry for a scenario the report records as passed, so stale skips accumulate unnoticed"
- file: crates/commission-conformance/tests/conform.rs
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the report count is compared with the suite from the same run and no scenario floor is pinned, so removing commands from ess/ leaves the test green"
```
