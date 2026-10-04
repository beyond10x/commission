---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-frontier-admission-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:frontier-admission, pass 1
relations:
- reviews: story:frontier-admission
revision: 1
---
```
unit: commission/frontier-admission, working tree on 0d34b55 + uncommitted phase 2 (worktree commission-w4-frontier-admission)
verdict: NEEDS-CHANGE
cases: executed 75→82, red 2
origin: introduced 6 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path (scratch/adversary1/, listed in part 6)
needs-coordinator: the duplicate-entries rule (F1): refuse on conflict, or add a uniqueness rule to the spec and contract
```

The phase-2 code passes its own gate, but two of my new cases fail on it. The frontier contract leaves out the `frontier_id` field. A frontier that lists one action twice, `Admissible` first and `Blocked` second, lets the action through. The existing acceptance test also misses 8 of 9 mutants.

**1. Diff stat.** `git --no-pager diff --stat` shows the same 9 implementor files with the same numbers as when I started. I didn't touch them. My only changes are 2 new test files, which git lists as untracked (`??`):
- `crates/commission/tests/adversary_admission_semantics.rs`
- `crates/commission/tests/adversary_admission_contract.rs`

No implementation, `ess/` or `generated/` file was changed. That rule held.

**2. Cases added.** Each one was run on its own before the suite.

| Case | Asserts | Now |
|---|---|---|
| `conflicting_duplicate_entries_never_admit` (semantics.rs:157) | the same action listed twice with different statuses is never admitted, in either order | **red** |
| `frontier_contract_shape_is_the_specified_fields` (contract.rs:75) | the keys in the contract's shape block equal the fields the spec gives `Frontier` (identity included) and its 3 item types | **red** |
| `admissible_carries_unit_true` | result is exactly `Admissible(Unit(true))` | green, catches mutant m1 |
| `approval_required_without_capability_carries_its_reasons` | result is exactly `Refused{action, reasons}` | green, catches m2 |
| `unlisted_action_is_refused_with_no_reasons` | an unlisted action, and an empty frontier, give `Refused{action, []}` | green, catches m3 |
| `blocked_with_a_capability_is_still_refused` | `Blocked` plus `Some(cap)` is refused, not sent for authority | green, catches m8 |
| `action_names_match_exactly` | different case, padding, a prefix, an extension or `""` are each refused with no reasons | green, catches m5, m6, m7 |

Red output, verbatim:
```
Admissible then Blocked was sorted Admissible(Unit(true)); Blocked then Admissible was sorted Refused(AdmissionRefused { action: "repository.merge", reasons: ["tests.pass is Unknown"] })
docs/contracts/frontier.md's shape disagrees with ess/domains/responsibility.yaml: specified but not shown ["frontier_id"]; shown but not specified []
```
In a scratch copy, the contract case passed once a `frontier_id:` line was added to the doc, and failed with `["priority"]` once an extra `priority:` key was added. So it fails both ways.

**3. Suite and gate.** My first clippy run failed on my own test file (`collapsible_if`). I fixed the test file, not the code under test.

| Step | Exit status |
|---|---|
| `cargo fmt --check` | 0 |
| `cargo clippy ... -D warnings` | 0 |
| `cargo test --workspace --locked --no-fail-fast` | **101**: only my 2 red cases fail; every other binary passes (82 executed) |
| `ess specify validate --strict-requires` | 0 |
| `ess verify conform synthesize` | 0 (0 scenarios, 0 refusals) |
| `xtask drift` | 0 ("no drift from ess/") |
| `xtask no-hand-model` | 0 |

The before count of 75 is 82 minus my 7 cases, taken from the per-binary summary lines. It does not come from a separate run.

**4. Findings** (they cover the working tree on top of 0d34b55)

| # | file:line | Finding | Verdict | What reaches it |
|---|---|---|---|---|
| F1 | `crates/commission/src/admission.rs:19` | "First entry wins" makes the result depend on list order. `[Admissible X, Blocked X]` admits X and `[Admissible X, ApprovalRequired X]` skips the approval. This conflicts with the AGENTS.md rule to fail toward less authority. | INFEASIBLE, warning | Nothing found in this tree. The spec and contract don't forbid duplicates, and the frontier producer (the governor port) is in a sibling unit. |
| F2 | `docs/contracts/frontier.md:20` | The shape says it follows the spec but leaves out the `frontier_id` identity. The differences table (`:73`) doesn't mention it. The other 13 fields match. | NEEDS-CHANGE, warning | Anyone reading the contract |
| F3 | `crates/commission/tests/frontier_admission.rs:94` | Case-insensitive (m5), trimmed (m6) and prefix (m7) name matching all pass the acceptance test, and each one widens what gets admitted. | CONFIRMED, warning | `admit` is called only from tests so far |
| F4 | `crates/commission/tests/frontier_admission.rs:49` | Checking the capability before the status (m8) passes the acceptance test and sends a `Blocked` action for authority. | CONFIRMED, warning | same |
| F5 | `crates/commission/tests/frontier_admission.rs:89` | Tests 4 and 5 check only the action name. Dropping reasons (m2) or adding reasons to an unlisted action (m3) still passes. | CONFIRMED, note | same |
| F6 | `crates/commission/tests/frontier_admission.rs:64` | `matches!(Admissible(_))` lets `Unit(false)` (m1) pass, though the spec says the value is always true. | CONFIRMED, note | same |

Suggested fixes (not applied):
- **F1:** refuse when an action's entries disagree, or add a uniqueness rule to the spec and contract.
- **F2:** add one `frontier_id:` line to the shape and a row to the table.

**Mutants tried** (on a scratch copy of the tree):

| Mutant | Acceptance test | My cases |
|---|---|---|
| m1 `Unit(false)` | survived | caught |
| m2 ApprovalRequired without capability drops reasons | survived | caught |
| m3 unlisted action gets a reason | survived | caught |
| m4 last entry wins (`.rev()`) | survived | only the red F1 case notices; nothing pins first-wins |
| m5 case-insensitive name match | survived | caught |
| m6 trimmed name match | survived | caught |
| m7 prefix name match | survived | caught |
| m8 capability checked before `Blocked` | survived | caught |
| m9 `Blocked` drops reasons | caught | caught |

**5. Attacked and could not break:**
- A `Blocked` action is never admissible.
- An unlisted, padded or differently-cased name is refused.
- `ApprovalRequired` is never sorted admissible.
- The result's type name starts with `commission::`.
- There is no generated drift and the ESS gate passes.
- The 8 rows of the differences table are each true against the spec.
- `Frontier` has only the `Issued` state, so there is no stale-state case.
- Rechecking authority and case revision belongs to `story:stale-revision-action-request`, not here.

**6. Paths written outside the worktree:**
- `~/.cache/ga-wave-2026-10-04-w4/commission-frontier-admission/scratch/adversary1/`: the mutant sources `mutants/m0..m9`, 20 mutant logs, and `fmt.log`, `clippy.log`, `suite.log`. The scratch tree copy is deleted.
- `~/.cache/b10x-target/commission-w4-mutants2`: deleted.
- The brief's build directory was reused for the gate, which rewrote its `suite.json`.

```findings
- file: crates/commission/src/admission.rs
  line: 19
  category: property
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "first-wins lookup admits an action whose duplicate entry is Blocked or ApprovalRequired when the Admissible entry comes first, so the result depends on list order instead of failing toward less authority"
- file: docs/contracts/frontier.md
  line: 20
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "the rewritten contract shape claims to follow the specification but omits the Frontier identity field frontier_id, and the differences table does not name the omission"
- file: crates/commission/tests/frontier_admission.rs
  line: 94
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "case-insensitive, trimmed and prefix action-name matching mutants all pass the acceptance test, each widening what is admitted"
- file: crates/commission/tests/frontier_admission.rs
  line: 49
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "matching on capability before status sends a Blocked action that names a capability for authority, and the acceptance test stays green"
- file: crates/commission/tests/frontier_admission.rs
  line: 89
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "cases 4 and 5 assert only the refused action name, so dropping ApprovalRequired reasons or inventing reasons for an unlisted action survives"
- file: crates/commission/tests/frontier_admission.rs
  line: 64
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "case 1 matches Admissible(_) so returning Unit(false), which the specification says is always true, survives"
```
