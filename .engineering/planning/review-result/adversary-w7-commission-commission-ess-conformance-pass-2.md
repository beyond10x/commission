---
format: aep.planning-md/3
id: review-result:adversary-w7-commission-commission-ess-conformance-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w7 adversary, commission story:commission-ess-conformance, pass 2
relations:
- reviews: story:commission-ess-conformance
revision: 1
---
unit: commission/commission-ess-conformance, the working tree at 1a37f64 + uncommitted phase 2 + pass-1 fixes + my 4 new untracked test files
verdict: NEEDS-CHANGE
cases: executed 188→203, red 3
origin: introduced 5 / pre-existing 0 / undecided 0
wrote-outside-worktree: 5 paths (part 6)
needs-coordinator: whether decision 3 ("a forced stale picks a revision different from expected") covers the whole `Integer` range. If it does, F1 holds the unit.

**1. `git --no-pager diff --stat`** (same 7 files and counts as when I started; my files are untracked test files only)
```
 Cargo.lock                                     |   1 +
 Taskfile.yml                                   |   6 +
 crates/commission-conformance/Cargo.toml       |   1 +
 crates/commission-conformance/src/lib.rs       | 381 +++++++++++++++++++++++--
 crates/commission-conformance/tests/conform.rs |  10 +-
 crates/commission-xtask/tests/checks.rs        |   1 +
 crates/commission/tests/generated_model.rs     |  57 +++-
 7 files changed, 432 insertions(+), 25 deletions(-)
?? crates/commission-conformance/tests/adversary2_conform_{codec,governor,isolation,text}.rs
```

**2. Cases added** (each target run alone before the suite; log is `scratch/adv2/cases.log`)

| File | Case | Now |
|---|---|---|
| adversary2_conform_codec.rs | `adversary2_uuid_refuses_text_that_is_not_a_canonical_uuid` | **red** |
| | `…suspend_run_echoes_non_integer_numbers` (1.5, 0.1, 1e-300, 1e300, 1e40, f64::MAX, 5e-324, i64::MIN, u64::MAX) | green |
| | `…suspend_run_echoes_negative_zero` | green |
| | `…codec_round_trip_is_exact_and_never_falls_back` (30k+ fixed-seed numbers; binary64 bits kept, `decimal_literal` never refuses) | green |
| adversary2_conform_governor.rs | `adversary2_forced_stale_names_two_different_revisions` (i64::MIN, -1, 0, 1, i64::MAX) | **red** |
| | `adversary2_stale_scenario_at_the_smallest_revision_passes` (synthesized stale scenario with the literal set to i64::MIN) | **red** |
| | forced not-admitted gives reasons `[BLOCKED]`; needs-authority gives `CAPABILITY`; unforced is admitted (3 cases) | green |
| adversary2_conform_isolation.rs | an armed `stale` left unconsumed does not leak into the next scenario; runs and events do not leak; two runs of one suite give identical reports. Each leak probe has a control that fails. | green |
| adversary2_conform_text.rs | escapes, controls, U+2028, U+10FFFF and empty keys through `SuspendRun` (Authority, Dependency, Evidence) and the revalidate `action` field | green |

Red output, verbatim:
```
codec::uuid decoded `not-a-uuid` as Some(Uuid("not-a-uuid")), which the generated Uuid cannot hold
thread 'adversary2_forced_stale_names_two_different_revisions' panicked at crates/commission-conformance/src/governor.rs:64:69:
attempt to subtract with overflow
thread 'adversary2_stale_scenario_at_the_smallest_revision_passes' panicked at crates/commission-conformance/src/governor.rs:64:69:
attempt to subtract with overflow
```
My first isolation probe was a setup error in my own test: `expect_no_event` needs a command before it. I fixed it with a resume of an unknown run before counting it, so it is not reported as a finding.

**3. Suite run** (after the cases existed): `cargo test --workspace --locked --offline --no-fail-fast` gave EXIT=101, 200 passed, 3 failed, 203 executed. The 3 failures are exactly the red cases above. `cargo clippy -p b10x-commission-conformance --all-targets -- -D warnings` exit 0 and `cargo fmt --check` exit 0, after I dropped one redundant `.clone()` in my governor file. I re-ran that target after the edit: same 2 red.

**4. Findings** (all on the working tree above)

| # | file:line | What was measured | What reaches it | Verdict / origin |
|---|---|---|---|---|
| F1 | crates/commission-conformance/src/governor.rs:64 | `checked_add(1).unwrap_or(expected - 1)` computes `expected - 1` eagerly, so at `i64::MIN` it panics in debug and test builds. Through `run_suite` the conform run aborts with no report. Fix: `unwrap_or_else(\|\| expected - 1)` or `wrapping_add(1)`. | Not the synthesized suite (its literal is 1.0). It is an `Integer` value the input admits, which is the range decision 3 was written for. The pass-1 fix introduced it. | NEEDS-CHANGE / introduced |
| F2 | crates/commission-conformance/src/codec.rs:26-28 | The module doc (:3-4) says decoding answers `None` when the type cannot hold the value. `uuid()` accepts any text, while the generated reader `uuid_at` (generated/rust/commission/src/json.rs:552-557) refuses non-canonical spellings. Fix: filter with `ess_primitives::facts::is_canonical_uuid`, or reword the doc. | Nothing: no synthesized scenario sends a malformed uuid. | INFEASIBLE / introduced |
| F3 | crates/commission-conformance/src/codec.rs:79-82 | The doc says `decimal_literal` refuses binary64-only magnitudes. It accepts them, because both sides of its `exact()` comparison are `None`. So the `serde_json` fallback at :86 and the `Text` fallback at :87 never fire for a spelling `to_json` writes; the green round-trip case shows this. | Readers of the doc. | CONFIRMED / introduced |
| F4 | crates/commission-conformance/src/codec.rs:83-88 | `from_json` is `pub`. The generated reader admits `1e400` (json.rs:152-198); this function turns it into `Node::Text`, and `0.30000000000000001` is rounded. A number silently becomes text. | Nothing: the only input is `to_json` output echoed by RunStore (lib.rs:321). | INFEASIBLE / introduced |
| F5 | Taskfile.yml:15 | `conform` runs before `drift`. When generated/ is stale against ess/, the scenario failures print before `drift` names the cause. `cargo test --workspace` then runs the conform test a second time. | `task check`, which CI runs with ess 0.52.0 installed (.github/workflows/check.yml:49-66). | CONFIRMED / introduced |

**5. Attacked, could not break**
- The `decimal_literal` fallback: it never fires on the codec's own path (30k+ numbers), and binary64 bits survive the round trip.
- Non-integer numbers, binary64-only magnitudes and subnormals pass the `SuspendRun` scenario.
- Negative zero passes. The sign is lost in the bytes, but ESS defines -0 and 0 as one value.
- Escaped and unicode strings and keys pass, including empty keys and empty containers.
- Forced not-admitted and needs-authority are answered for the right reason: the blocked entry and the one capability. A frontier for another case or an unlisted action is never what answers them.
- Scenario isolation holds: an armed outcome, runs and events all reset in `begin_scenario`; nothing outside `RunStore` holds global state.
- The report is deterministic across two runs.
- CI order: `ess` is installed before `task check`.
- Doc comments in lib.rs and governor.rs match behaviour, apart from F1.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w7/commission-commission-ess-conformance/scratch/adv2/suite.json`
- `…/scratch/adv2/cases.log`
- `…/scratch/adv2/cases2.log`
- `…/scratch/adv2/suite.log`
- The assigned build dir `~/.cache/b10x-target/commission-w7-commission-ess-conformance`. The tests' `CARGO_TARGET_TMPDIR` files are removed by the tests themselves.

I did not acquire a worktree session lease.

```findings
- file: crates/commission-conformance/src/governor.rs
  line: 64
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "unwrap_or(expected - 1) is evaluated eagerly, so a forced stale at expected_case_revision = i64::MIN panics with subtract overflow and aborts the whole conformance run"
- file: crates/commission-conformance/src/codec.rs
  line: 26
  category: contract-drift
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "codec::uuid decodes any text, including non-canonical spellings the generated wire reader refuses, contrary to the module doc's None-when-the-type-cannot-hold-it promise"
- file: crates/commission-conformance/src/codec.rs
  line: 79
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "number_of's doc says decimal_literal refuses binary64-only magnitudes, but it accepts them, so both fallbacks are dead on every spelling to_json writes"
- file: crates/commission-conformance/src/codec.rs
  line: 87
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "pub from_json turns a model Json number such as 1e400, which the generated reader admits, into Node::Text instead of refusing it"
- file: Taskfile.yml
  line: 15
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "check runs conform before drift, so a stale generated model shows up as scenario failures before drift names the cause, and cargo test --workspace runs conform a second time"
```
