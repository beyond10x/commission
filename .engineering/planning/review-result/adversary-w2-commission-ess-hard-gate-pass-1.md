---
format: aep.planning-md/3
id: review-result:adversary-w2-commission-ess-hard-gate-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w2 adversary, commission story:ess-hard-gate, pass 1
relations:
- reviews: story:ess-hard-gate
revision: 1
---
unit: commission/ess-hard-gate, working tree of impl/ess-hard-gate at base 82d380a plus uncommitted changes (`~/.local/state/worktree/trees/b10x/commission/commission-w2-ess-hard-gate`)
verdict: CONFIRMED (warning): the gate holds; the suite does not pin the scan or the refusal count
cases: executed 6→8 (ess_gate binary, scratch copy; the workspace ran 45 in the worktree), red 2 (against mutants)
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path (scratch dir, below)
needs-coordinator: the two catching cases can't go under `crates/commission/tests/adversary_*.rs`. `markers`, `refusals` and `scan` are private to the `ess_gate` test binary, and `include!` refuses the file because it starts with `//!` (E0753, tested). The cases are a patch to `ess_gate.rs`, which is the implementor's file to apply.

**1. `git --no-pager diff --stat` (worktree)**
Only the implementor's 10 files plus the untracked `ess_gate.rs`. I touched no path in the worktree.

**2. Cases added**
They are in `scratch/adv/adversary_cases.rs` and as a patch in `scratch/adv/adversary-cases.patch`. Both run inside a copy of `ess_gate.rs`.

| case | asserts | red against | green on the pristine gate |
|---|---|---|---|
| `adversary_scan_finds_nested_hidden_non_yaml_and_mid_line_markers` | `markers()` reports `.open:1`, `domains/deep/er/notes.md:2` (mid-line) and `domains/x.yml:1` (trailing), exactly 3, and `scan` fails at step 4 | C1, C2, C3 | yes |
| `adversary_refusal_count_is_read_from_the_report` | `refusals()` returns Some(0), Some(2) and None on ess 0.52.0 report lines | C5 | yes |

Red output from the first run, case alone, against C1:
```
panicked at crates/commission/tests/ess_gate.rs:480:9:
scan missed `.../ess/.open:1: UNMAPPED: hidden`; found:
test result: FAILED. 0 passed; 1 failed; ... 7 filtered out
```
Against C5:
```
panicked at crates/commission/tests/ess_gate.rs:499:5:
  left: Some(0)
 right: Some(2)
```

**3. Suite run** (after the cases existed)
- Scratch copy, pristine gate plus cases: `ok. 8 passed; 0 failed`.
- Worktree, `cargo test --workspace --locked` with the brief's build dir: exit 0. Per binary: 0, 6, 2, 5, 6, 3, 23 and 0 tests.
- `drift`: exit 0. `no-hand-model`: exit 0. `cargo fmt --check`: exit 0.

**4. Findings** (all in the working tree above)

| # | file:line | finding | verdict / origin | what reaches it |
|---|---|---|---|---|
| F1 | `crates/commission/tests/ess_gate.rs:188` and `:196` | Three scan mutants keep all 6 cases green: C1 matches only lines starting `# UNMAPPED:`, C2 scans only `.yaml` files, C3 skips hidden files. The only probe (`:275`) appends one whole-line `# UNMAPPED:` to a top-level `.yaml`. The marker the base actually had was mid-line ("is marked UNMAPPED: and stays"). | CONFIRMED / introduced | Any later edit to the scan. The scan itself is correct today: D1 and D2 below go red. |
| F2 | `crates/commission/tests/ess_gate.rs:143`, `:168` | Nothing tests a nonzero refusal count. C4 (`Some(_) => {}`) and C5 (`refusals` always returns Some(0)) both keep the suite green. ess 0.52.0 exits 0 when it refuses: `main.rs:4091-4113` prints `refused:` lines and then the count. So this parse is the only thing that enforces expectation 3. My case catches C5 but not C4. Catching C4 needs a spec that ess refuses, which I did not build. | CONFIRMED / introduced | Any later edit to the parse. It is correct today. |
| F3 | `crates/commission/tests/ess_gate.rs:433`, `Taskfile.yml:29` | Test 8 checks the `ess-gate` command by substring. With `-- --skip ess_gate --skip gate_names --skip compiled` added (T1), test 8 still passes. `task check` stays a hard gate anyway, because `cargo test --workspace --locked` (`Taskfile.yml:19`) runs every unfiltered case. | CONFIRMED / introduced, note | nothing found |

**5. Attacked and held**
- Data mutants against the pristine gate:
  - D1: mid-line marker in a nested `.md` file. Red, step 4 names `notes.md:1`.
  - D2: marker in hidden `ess/.open`. Red, file and line named.
  - D3: a fourth `ActionStatus` variant. Red: `left: [... "Withdrawn"]`.
  - D4: `claims` as `Optional<…>`. Red: `"optional"` vs `"list"`.
  - D5: `actions` as `List<Optional<…>>`. Red: `"optional"` vs `"declared"`.
  - D6: `reasons` as `Optional<String>`. The ess_gate suite stays green, but `drift` exits 1.
- `ess` missing from PATH: 3 cases fail with "cannot run `ess` … needs ess 0.52.0 on PATH". Nothing skips.
- A non-zero exit fails each of steps 1–3. When no count is printed, `refusals` returns None and the gate fails.
- Expectation 6 is read from the parsed JSON structure, not by substring.
- The base had 2 markers (`:3`, `:183`). The tree now has none under `ess/`.
- `drift` and `no-hand-model` pass with no drift. CI runs `task check` with ess 0.52.0.
- The scan does not follow symlinked directories, so one makes it fail with a read error rather than pass. I did not see any symlinks under `ess/`.

**6. Written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w2/commission-ess-hard-gate/scratch/adv/` contains:
  - `copy/` (worktree copy with all mutants reverted)
  - `target/` (my own build dir, 140M)
  - `adversary_cases.rs` and `adversary-cases.patch`
  - `ess_gate.pristine.rs`, `responsibility.pristine.yaml`, `Taskfile.pristine.yml`
  - `C1-case-red.txt`, `compiled.json`, `suite.json`
- `/` has 11G free.

**7. Findings block**
```findings
- file: crates/commission/tests/ess_gate.rs
  line: 188
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: three scan mutants (prefix-only match, yaml-only files, skip hidden files) leave all six ess_gate cases green; the only marker probe is a whole-line comment in a top-level yaml file
- file: crates/commission/tests/ess_gate.rs
  line: 143
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: no case gives the refusal check a nonzero count, so accepting any count or always parsing 0 stays green although ess 0.52.0 exits 0 when it refuses
- file: crates/commission/tests/ess_gate.rs
  line: 433
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the ess-gate Taskfile check is a substring match that accepts a command skipping the gate's cases; check still enforces the gate through cargo test --workspace
```
