---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-authority-provider-port-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:authority-provider-port, pass 2
relations:
- reviews: story:authority-provider-port
revision: 1
---
unit: commission/authority-provider-port, working tree at 20a70fe plus uncommitted phase 2 and the pass-1 fixes
verdict: INFEASIBLE (worst is a warning): a provider failure passes the provider's raw text into Commission's refusal
cases: executed 81→84, red 3
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 paths (1 kept, 2 created and deleted)
needs-coordinator: decide whether the 3 red cases stay. Only the context case rests on a written rule. The 4096-byte limit and the one-line rule are mine.

**1. Diff stat**

`git --no-pager diff --stat` lists only the implementor's uncommitted files: `fake_authority.rs` (+91) and `ports/authority.rs` (+124). I did not change either. Both are byte-identical to the copies I took before mutating (checked with `cmp`). My only file is new and untracked, so the stat does not list it: `crates/commission-testkit/tests/adversary2_authority_error.rs`.

**2. Cases added (all red, run alone before the suite)**

| case | asserts | red output |
|---|---|---|
| `refusal_does_not_carry_the_authority_context` | a provider whose error quotes its request must not put the authority context into the refusal's Display or Debug | `Display \`authority provider failed: backend rejected ledger.write for request {"delegation":{"token":"delegation-token-9f41c2",...}}\`` |
| `refusal_display_is_one_line` | a `\n` in the provider's text must not start a second, forged log line | `left: 2 right: 1 … "authority provider failed: timeout\nauthority allowed payment.send for principal-c"` |
| `refusal_display_is_bounded` | Display is at most 4096 bytes | `got 1048603 bytes from a 1048576-byte provider message` |

**3. Suite**

- Command: `cargo test --workspace --locked --no-fail-fast` with the brief's build dir. Exit 101.
- Only `adversary2_authority_error` failed (`0 passed; 3 failed`). Every other binary is ok, including `authority_port` 1, `adversary_authority_fail_closed` 5 and `checks` 30.
- 84 tests ran. 81 is that total minus my file's 3.
- Log: `~/.cache/ga-wave-2026-10-04-w4/commission-authority-provider-port/scratch/adversary2-suite.log`.
- `rustfmt --check` and `cargo clippy -p b10x-commission-testkit --all-targets -D warnings` are clean on the new file.

**4. Findings (cover the working tree above)**

| # | file:line | verdict / severity | what was measured | what reaches it |
|---|---|---|---|---|
| F1 | `crates/commission/src/ports/authority.rs:89` | INFEASIBLE / warning | The refusal passes the provider's text through unchanged. A provider that echoes its request leaks the context, against the spec's "opaque to Commission … read by nobody else" (`ess/domains/responsibility.yaml:60-61`). The sibling governor port uses a typed `GovernorError` declared in ESS, with no free text. Fix: a typed error declared in ESS, or Commission drops the provider's text. | Nothing found. No caller of `check_authority` in any w4 tree, and the only provider is the fake, which does not echo. |
| F2 | `authority.rs:49` | INFEASIBLE / note | Display has no length limit and does not escape the text. A cheap fix is to format the message with `{:?}`: it passes the one-line case and the current suite stays green. | Nothing logs a refusal today. |
| F3 | `authority.rs:78-86` | INFEASIBLE / note | `check_authority` accepts a commission in any lifecycle state. Today only `Assigned` exists (`ess/…:344-346`, `generated/rust/commission/src/responsibility.rs:916-935`). Once a terminal state is added, a closed commission can still be checked without a compile error. | Cannot be reached today: there is only one state. |
| F4 | `authority.rs:24-28` | INFEASIBLE / note | Raised as a note, not a demand. The port's capability is a plain `&str` with no scope. ADR 0083 (decisions 1 and 3) has scoped grants such as `metrics.read{cluster: prod-eu, namespace: foo}`, and has Commission narrow each call by the grant. The port can only express a scope if it is written into the string. | No caller. Source: Atlas `0083-capabilities-and-runtime-binding.md` (worktree `atlas/atlas`, b3d4eb87; not yet on origin/main). |

**Mutants** (on a scratch copy, own build dir)

| mutant | result |
|---|---|
| M1: Display formats the message with `{:?}` | survives the existing suite |
| M2: `check_authority` appends the commission's context to the refusal | survives the acceptance test. Only pass 1's equality check (`adversary_authority_fail_closed.rs:119-123`) catches it. |
| M3: Display drops the message | caught by `authority_port_contract` |

**5. Attacked, could not break**

- **Debug of `AuthorityCheck`:** it holds only the verdict or the provider's error, so it shows the context only if the provider's text does (that is F1).
- **Debug of `AuthorityQuery`:** it prints the context, but it is the test fake and records the context on purpose (acceptance item 5).
- **Changing the fake's answers concurrently:** not possible. `answer()` and `fail()` take `mut self` by value, there is no `Clone`, and nothing mutates through `&self`.
- **Calling `check_authority` with a non-`Assigned` commission:** no such commission can be built.

**6. Paths written outside the worktree**

- `~/.cache/ga-wave-2026-10-04-w4/commission-authority-provider-port/scratch/adversary2-suite.log` (kept)
- `~/.cache/ga-wave-2026-10-04-w4/commission-authority-provider-port/scratch/adversary2-copy` (deleted)
- `~/.cache/b10x-target/commission-w4-mutants3` (deleted)
- Session lease taken and released.

**7. Findings block**

```findings
- file: crates/commission/src/ports/authority.rs
  line: 89
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "check_authority hands the provider's free-text error into the refusal unchanged, so a provider that echoes its request leaks the authority context the spec says nobody but the provider reads"
- file: crates/commission/src/ports/authority.rs
  line: 49
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "AuthorityProviderError Display is unbounded (1 MiB in, 1048603 bytes out) and unescaped, so a provider newline forges a second log line"
- file: crates/commission/src/ports/authority.rs
  line: 78
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "check_authority is generic over every commission state, so once the lifecycle gains a terminal state a closed commission can still be asked for authority without a compile error"
- file: crates/commission/src/ports/authority.rs
  line: 24
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "the port's capability is an unscoped string, while ADR 0083 grants capabilities per scope and has Commission narrow each invocation by the grant"
```
