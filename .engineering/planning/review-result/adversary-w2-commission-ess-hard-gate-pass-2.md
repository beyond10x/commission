---
format: aep.planning-md/3
id: review-result:adversary-w2-commission-ess-hard-gate-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w2 adversary, commission story:ess-hard-gate, pass 2
relations:
- reviews: story:ess-hard-gate
revision: 1
---
unit: commission/ess-hard-gate, working tree on base 82d380a (uncommitted unit changes plus my one untracked test file)
verdict: NEEDS-CHANGE
cases: executed 48→59, red 1
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 paths (part 6)
needs-coordinator: none

**1. Diff stat.** `git --no-pager diff --stat` lists the same 10 implementor paths as before I started. That command does not show untracked files, so here is `git status --short`: the only new path is `?? crates/commission/tests/adversary2_gate.rs`, which is a test file. I changed no implementation file, nothing in `ess/` and nothing in `generated/`.

**2. Cases added** in `crates/commission/tests/adversary2_gate.rs`. The gate's functions are private, so this file loads `ess_gate.rs` as a module with `#[path]`. It then reruns its own binary to execute only `ess_gate_src::ess_gate`, with `CARGO_MANIFEST_DIR` pointing at a scratch repository that holds a changed copy of `ess/`. This runs the tree's real gate code from start to finish, without patching it. A side effect: the 9 tests in `ess_gate.rs` also run a second time inside this binary.

| case | asserts | now |
|---|---|---|
| `adversary2_gate_reads_the_summary_count_not_a_quoted_one` | First checks ess directly: on the changed copy, `synthesize` exits 0 and prints `1 refusal(s)`. Its earlier `refused:` detail line repeats an invariant whose text contains `see 0 refusal(s) here`. The case then requires the gate to fail at step 3. | **red** |
| `adversary2_gate_refuses_an_older_required_ess` | Sets `ess-inputs.yaml` to `requires: ess 0.51.0` and requires the gate to fail at step 1 with `--strict-requires refuses a newer release`. Without the flag, every step passes (I checked). | green; it catches a mutant the existing suite misses |

Red output from running that case alone (`cargo test --locked -p b10x-commission --test adversary2_gate -- --exact adversary2_gate_reads_the_summary_count_not_a_quoted_one`, saved to `scratch/adv2/red-A.txt`):
```
panicked at crates/commission/tests/adversary2_gate.rs:168:5:
the gate passed a specification ess refused one scenario of:
status: exit status: 0
stdout:
running 1 test
test ess_gate_src::ess_gate ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out
EXIT=101
```

**3. Suite run**, after the cases existed:
- `cargo fmt --check`: exit 0, after I reformatted one assert in my own file.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0.
- `cargo test --workspace --locked --no-fail-fast`: exit 101. The only failure is `adversary2_gate: 10 passed; 1 failed`. The other test binaries all pass: ess_gate 9, generated_model 2, xtask 5, adversary_no_hand_model 6, adversary_pass2 3, checks 23, lib 0.
- `executed 48`: the same run with the 11 cases of `adversary2_gate` taken out.
- Also exit 0: `validate --strict-requires`, `drift` and `no-hand-model`.

**4. Findings** (they cover the working tree on 82d380a)

| # | file:line | what was measured | what reaches it | verdict / origin |
|---|---|---|---|---|
| F1 | `crates/commission/tests/ess_gate.rs:185` | `refusals()` uses `find_map`, so it returns the **first** `<n> refusal(s)` pair anywhere in stdout. ess 0.52.0 prints the `refused:` detail lines **before** the summary, and those lines quote the author's own invariant text word for word. As a result the gate passed a spec that ess refused 1 scenario of (adversary2_gate.rs:168). Fix: `words.windows(2).rev().find_map(` (`scratch/adv2/fix-last-count.patch`). With it applied in a scratch copy, all 11+9 cases pass. A sturdier fix would match only the summary line (`… scenario(s) (… authored), N refusal(s), written to`). | Any edit to `ess/` whose invariant contains a text literal with `<n> refusal(s)`, which is a valid spec (validate exit 0). No such spec exists today. Someone would have to write it. | NEEDS-CHANGE / introduced |
| F2 | `crates/commission/tests/ess_gate.rs:110` | Dropping `--strict-requires` (mutant M1) left all 9 `ess_gate.rs` cases green. My second case now catches it. | Any bump of `ess-inputs.yaml` that falls behind the installed ess, which is the normal state between pin moves | CONFIRMED / introduced |
| F3 | `docs/contracts/frontier.md:26,31,45-46,10` | The new types cite this contract (`ess/domains/responsibility.yaml:76,81,90,99`), but they do not match it. The contract keys items by `id`; the spec uses `claim`/`obligation`/`action`. The contract has `status: open` plus `priority`; the spec has `open: Boolean`. The contract's `reasons` are structured `{claim, required, actual}`; the spec's are `List<String>`. The contract's statuses have no approval-required value. The contract says "unknown or contradicted", but `Truth` cannot say "contradicted". The spec follows the story's own field list, so the gap is in the document. | Anyone who reads the contract to build or check a frontier | CONFIRMED / introduced |
| F4 | `AGENTS.md:52` | ADR 0076 says "No story in these repositories is implemented while its gate is red". AGENTS.md narrows this to "A story that edits `ess/` passes `ess-gate`". It also leaves out the ADR's rule that the step-4 test is removed once the ess release that refuses open questions is pinned. | Agents who read AGENTS.md in place of the ADR | CONFIRMED / introduced |

**5. Attacked and could not break**
- **Generated tree vs spec:** the type names, field order and the variants Admissible/ApprovalRequired/Blocked match, and `drift` reports no drift. The generated crate has no serde, so there are no serde names to compare.
- **Re-export (expectation 7):** the test builds through `b10x_commission::model` (`pub use commission as model`, unchanged from base).
- **Frontier→Case:** the `references`/`via: case_id` relation is unchanged at `responsibility.yaml:238-243`. The story's line citation `:195-199` is now out of date because the file grew.
- **ess exit codes:** validate, compile and synthesize all exit 1 on an invalid spec, so steps 1 and 2 cannot pass one.
- **CI:** `check.yml` installs ess 0.52.0 by checksum and runs `task check`, which now includes `ess-gate`. No CI change is needed.
- **Mutants:** removing `--format json` (M6) is caught by 4 cases. Turning off the `suite.is_file()` guard (M2, `ess_gate.rs:172`) **survives**. I found no real ess run that exits 0 without writing a suite, so I wrote no case for it.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w2/commission-ess-hard-gate/scratch/adv2/` holds: `p1/` (probe spec), `red-A.txt`, `suite.txt`, `s.json`, `fix-last-count.patch`, and `mutrepo/` (a scratch crate `adv2mut` with copies of `ess/`, `Taskfile.yml`, `AGENTS.md`, `Cargo.lock` and both test files).
- `~/.cache/b10x-target/commission-w2-ess-hard-gate/debug/deps/adv2mut-*`, `adversary2_gate-03e6e16921b9e8a6`, `ess_gate-794bf3c403e8549e`: build outputs of the scratch crate, in the brief's own build directory.
- Free space on `/` is 11G, so it stayed above the 10G minimum.

**7. Findings block**
```findings
- file: crates/commission/tests/ess_gate.rs
  line: 185
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: refusals() takes the first "<n> refusal(s)" in stdout, so a refused detail line quoting an invariant literal "0 refusal(s)" makes the gate pass a synthesis ess refused one scenario of
- file: crates/commission/tests/ess_gate.rs
  line: 110
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: dropping --strict-requires from step 1 left every ess_gate.rs case green; adversary2_gate_refuses_an_older_required_ess now catches it
- file: docs/contracts/frontier.md
  line: 26
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the new value types cite frontier.md, but their keys, obligation status, structured reasons, approval status and "contradicted" claim value do not match it
- file: AGENTS.md
  line: 52
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: AGENTS.md § ESS narrows ADR 0076's "no story is implemented while its gate is red" to stories that edit ess/, and omits that the marker test is removed once ESS refuses open questions
```
