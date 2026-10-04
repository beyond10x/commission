---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-authority-provider-port-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:authority-provider-port, pass 1
relations:
- reviews: story:authority-provider-port
revision: 1
---
unit: commission/authority-provider-port, working tree at 20a70fe plus the uncommitted phase 2
verdict: CONFIRMED
cases: executed 75→80, red 0
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 2 paths (plus 1 build dir, now deleted)
needs-coordinator: decide whether to keep the 5 new cases, which pass against this tree and kill the 7 mutants the acceptance test misses

I found no way for an error, an unknown capability, approval-required or a panic to come out as an allow. The real result is a suite gap: 7 of 12 mutants pass `authority_port_contract`. The new file kills all 7.

**1. `git --no-pager diff --stat` (worktree)**
```
 crates/commission-testkit/src/fake_authority.rs | 91 ++++++++-
 crates/commission/src/ports/authority.rs        | 90 ++++++++-
```
Both lines are the implementer's uncommitted phase 2, not my edits. My only addition is an untracked test file, `?? crates/commission-testkit/tests/adversary_authority_fail_closed.rs`. I changed no implementation file.

**2. Cases added** in `~/.local/state/worktree/trees/b10x/commission/commission-w4-authority-provider-port/crates/commission-testkit/tests/adversary_authority_fail_closed.rs`. All 5 pass against the tree (my first run of the file alone: 5 passed, exit 0). Each one fails against a mutant:

| Test | Asserts | Fails on mutant |
|---|---|---|
| `unknown_capability_is_refused_and_recorded` | unknown capability, `""` and an empty table give `Refused` and `!allows()`, and those failed calls are recorded | M1, M2, M8–M10, M13 |
| `capability_is_passed_and_matched_exactly` | `" Payment.Send "` is allowed only when spelled exactly; near spellings are refused; each is recorded exactly | M1–M4, M7, M8, M13 |
| `later_table_entry_replaces_earlier_one` | a later `answer`/`fail` for the same capability replaces the earlier one, as the doc says | M5, M6, M8, M12 |
| `panicking_provider_yields_no_verdict` | a panic in `decide` leaves `check_authority` without a verdict | none (documents current behaviour) |
| `shared_provider_records_every_call_across_threads` | `Arc<dyn AuthorityProvider + Send + Sync>` works across 8 threads; all 8 calls recorded, including the 4 failures | M1, M2, M9, M10 |

**Mutants** (on a scratch copy, built in `commission-w4-mutants3`):

| Mutant | `authority_port` | new file |
|---|---|---|
| M1 fake: unknown capability returns `Allow` (fake_authority.rs:85) | **passes** | fails |
| M2 fake: unknown capability returns early without recording | **passes** | fails |
| M3 `check_authority` passes `capability.trim()` (authority.rs:84) | **passes** | fails |
| M4 fake records `capability.to_lowercase()` (:80) | **passes** | fails |
| M5/M6 `answer`/`fail` use `or_insert` (:47, :54) | **passes** | fails |
| M7 fake looks up a lowercased key (:82) | **passes** | fails |
| M8 provider error becomes `Deny` | fails | fails |
| M9 `allows()` = not Deny | fails | fails |
| M10 `allows()` also true for `Refused` | fails | fails |
| M12 refusal drops the provider's message | fails | fails |
| M13 fake records a `null` context | fails | fails |

Full log: `~/.cache/ga-wave-2026-10-04-w4/commission-authority-provider-port/scratch/adversary-mutants.log`

**3. Suite**, run after the cases existed: `cargo test --workspace --locked` exit 0, 80 passed. That is 75 from the implementer's `gate.log` (04:51:56, written before my file) plus my 5; the run printed `Running tests/adversary_authority_fail_closed.rs ... 5 passed` from the unit's own build dir. `cargo fmt --check` exit 0; `cargo clippy -p b10x-commission-testkit --all-targets -- -D warnings` exit 0.

**4. Findings** (they cover the working tree on top of 20a70fe)

| # | file:line | What was measured | What reaches it | Verdict / origin |
|---|---|---|---|---|
| F1 | `crates/commission-testkit/src/fake_authority.rs:85` | M1 and M2 pass the acceptance test: nothing checks that an unknown capability is refused or recorded | `story:local-runtime-loop` acceptance 9 reads this fake's call log, and `story:run-outcomes` scripts its rows through it | CONFIRMED / introduced, warning |
| F2 | `crates/commission/src/ports/authority.rs:84` | M3 (trim), plus M4 and M7 in the fake, pass: nothing checks that the capability reaches the provider exactly | every caller of `check_authority`; frontier capabilities are passed as strings | CONFIRMED / introduced, warning |
| F3 | `crates/commission-testkit/src/fake_authority.rs:47` | M5 and M6 pass: the replace behaviour the doc promises is untested | test authors who re-script one capability | CONFIRMED / introduced, note |
| F4 | `crates/commission/src/ports/authority.rs:67` | `allows()` is true for `Allow(Unit(false))`, though the spec (`ess/domains/responsibility.yaml:66`) says Unit "is always true" | nothing found: no decoder or code builds `Unit(false)`. I wrote no failing test | INFEASIBLE / introduced, note |

**5. Could not break**
- No path where an error, unknown capability, approval-required or panic gives an allow.
- The context is opaque: Commission never reads `authority_context` (grep finds no use outside the port), and passes `&CommissionData` by shared reference.
- The trait is object-safe and works through `&dyn` and `Arc<dyn … + Send + Sync>`. It has no `Send + Sync` supertraits, the same as the sibling `Governor` and `AgentExecutor` ports.
- The fake recovers from a poisoned lock and records failed calls.
- Calling `decide` directly skips the `Refused` mapping, but it returns a `Result`, so it does not fail open.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w4/commission-authority-provider-port/scratch/adversary-mutants.log`
- `~/.cache/ga-wave-2026-10-04-w4/commission-authority-provider-port/scratch/adversary-suite.log`
- Deleted: `~/.cache/b10x-target/commission-w4-mutants3`, plus the scratch `adversary-mutant-tree/`, `adversary-mutants.zsh`, `m.out` and `m2.out`.
- I also built into the brief's dir `~/.cache/b10x-target/commission-w4-authority-provider-port`. Disk free is 12G.

**7.**
```findings
- file: crates/commission-testkit/src/fake_authority.rs
  line: 85
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "authority_port_contract never asks about an unknown capability, so a fake that allows it (M1) or skips recording it (M2) stays green; adversary_authority_fail_closed.rs kills both"
- file: crates/commission/src/ports/authority.rs
  line: 84
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "nothing checks that the capability reaches the provider exactly: trim in check_authority (M3), and lowercase record or lookup in the fake (M4, M7), all pass the acceptance test"
- file: crates/commission-testkit/src/fake_authority.rs
  line: 47
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the documented replace-earlier-entry behaviour of answer and fail is untested; or_insert mutants M5 and M6 stay green"
- file: crates/commission/src/ports/authority.rs
  line: 67
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "allows() is true for Allow(Unit(false)), a value the spec says cannot occur; nothing found constructs it"
```
