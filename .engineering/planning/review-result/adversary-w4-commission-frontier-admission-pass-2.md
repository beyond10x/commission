---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-frontier-admission-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:frontier-admission, pass 2
relations:
- reviews: story:frontier-admission
revision: 1
---
unit: commission/frontier-admission, working tree on 0d34b55 (phase 2 and the pass-1 fixes uncommitted)
verdict: NEEDS-CHANGE
cases: executed 83→90, red 2
origin: introduced 4 / pre-existing 1 / undecided 0
wrote-outside-worktree: 2 paths (1 still there, 1 deleted)
needs-coordinator: whether an empty-string capability counts as "no capability" (story § Outcome only covers `None`)

**1. Diff stat.** The tracked diff (10 files) is phase 2 plus the pass-1 fixes, as handed to me. I changed none of those files. My only additions are two untracked test files:
`crates/commission/tests/adversary2_admission_boundary.rs` and `crates/commission/tests/adversary2_admission_precedence.rs`.

**2. Cases added** (I ran each file alone first; the output below is from that run)

| Case | Asserts | Now |
|---|---|---|
| `approval_required_with_an_empty_capability_is_refused` | `ApprovalRequired` with capability `Some("")` is refused | red |
| `conflict_refusals_for_different_capabilities_differ` | capability sets {"a, b","c"} and {"a","b, c"} give different refusals | red |
| `blocked_outranks_an_approval_with_no_capability` | rule 1 decides before rule 2 (shown by which reasons are carried) | green; catches mutant M1 |
| `blocked_outranks_a_capability_conflict` | rule 1 decides before rule 3 | green; catches M3 |
| `approval_with_no_capability_outranks_a_capability_conflict` | rule 2 decides before rule 3 | green; catches M2 and M3 |
| `every_approval_with_no_capability_gives_its_reasons` | rule 2 carries the reasons of every such entry | green; catches M4 |
| `identical_reason_lists_are_taken_once` | identical reason lists are deduplicated, as the module doc says | green; catches M5 |

```
ApprovalRequired with capability "" was sorted NeedsAuthority(AdmissionNeedsAuthority { capability: "" }): there is nothing to ask for
left:  Refused(AdmissionRefused { action: "x", reasons: ["conflicting capabilities for an ApprovalRequired action: a, b, c"] })
right: Refused(AdmissionRefused { action: "x", reasons: ["conflicting capabilities for an ApprovalRequired action: a, b, c"] })
```

**Mutants tried.** Each was built from a copy in scratch with its own build dir, `commission-w4-mutants2`, which I have deleted. All five passed the acceptance test and both pass-1 files. Each one failed only my precedence case(s):
- M1: rule 2 checked before rule 1
- M2: rule 3 checked before rule 2
- M3: rule 3 checked before rule 1
- M4: rule 2 carries only the reasons of the first entry, ordered by reason list
- M5: reason lists sorted but identical lists not deduplicated

**3. Suite run** (after the cases existed, using the brief's build dir)
- `cargo test --workspace --locked --no-fail-fast`: exit 101, 88 passed, 2 failed. The 2 are my red cases.
- "Before" count: 83 is the same run minus my 7 cases. I had no earlier count.
- `cargo fmt --check`: exit 0. My first version of the precedence file was not formatted; I ran rustfmt on my two files only.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0.
- `ess specify validate --strict-requires`: exit 0.
- `ess verify conform synthesize`: exit 0.
- `commission-xtask drift`: exit 0, "no drift from ess/".
- `commission-xtask no-hand-model`: exit 0.
- `-- --list` shows all 7 new tests in this tree.

**4. Findings** (they cover the working tree on 0d34b55)

| # | file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|---|
| F1 | `crates/commission/src/admission.rs:59-69` | NEEDS-CHANGE / introduced | `Some("")` is sorted `NeedsAuthority{capability:""}` | Nothing in this tree produces frontiers yet (the governor port is a sibling unit). The next governor that sends an empty capability reaches it. |
| F2 | `crates/commission/src/admission.rs:40-77` | CONFIRMED / introduced | The order of the refusal rules and how their reasons are merged was not tested: M1 to M5 all passed the existing suite | My precedence file fixes this; nothing needs to change in the implementation |
| F3 | `crates/commission/src/admission.rs:72-75` | INFEASIBLE / introduced | Joining with ", " makes two different conflicts read the same | Only reached if capability names contain ", "; none were found |
| F4 | `docs/contracts/frontier.md:62-65` | CONFIRMED / introduced | The contract does not say how reason lists are ordered, that identical lists are taken once, or that the conflict refusal drops the entries' own reasons. All three are only in the module doc at `admission.rs:17-18`. | A consumer of the contract cannot predict a refusal's `reasons` |
| F5 | `.engineering/planning/story/stale-revision-action-request.md:93` | CONFIRMED / pre-existing | The revalidation command has 3 outcomes (admitted, stale, not admitted), but `admit()` has a 4th, `NeedsAuthority`. The text is the same at 450da32 (`git grep`). | The next story. If it maps "not Refused" to admitted, an `ApprovalRequired` action is admitted without authority. |

Fixes I would suggest:
- F1: treat a blank capability like `None`.
- F3: quote each capability name, or return one reason per capability.
- F4: add the merge rule to § Invariant.
- F5: that story should add a needs-authority outcome.

**5. Attacked and could not break**
- **Reason merging:** only identical *lists* are deduplicated. A reason repeated across different lists is kept twice, e.g. ["a","b"] and ["b"] give a,b,b. This matches the module doc; I read it in the code and did not test it.
- **Empty reason strings:** passed through unchanged. Nothing forbids them.
- **Can reason order affect the decision?** No. Which variant comes back never depends on the reasons; they only change the order of the text.
- **Very large frontiers:** linear. In a debug build, admitting against 10k / 100k / 400k entries for one action took 4.8 / 48 / 187 ms.
- **Conflict text order:** stable. Capabilities are sorted (a BTreeSet), and the permutation test pins the order.
- **Contract against code:** rules 1 to 4 are stated in the contract and match the code. Rule 5 (admissible) is stated only by the single-entry bullet.
- **Generated drift:** none.
- **Reuse by stale-revision:** `admit(&Frontier<S>, &str)` can be reused unchanged. Checking the case id and the revision stays in that story.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w4/commission-frontier-admission/scratch/adversary2/`: the mutant copy of the tree (`src/`, 1.3M, including `src/crates/commission/src/admission.rs` and `src/crates/commission/tests/probe_large_frontier.rs`) and `suite.log`. Still there.
- `~/.cache/b10x-target/commission-w4-mutants2`: about 102M, deleted.

```findings
- file: crates/commission/src/admission.rs
  line: 59
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "An ApprovalRequired entry with capability Some(\"\") is sorted NeedsAuthority with an empty capability instead of refused, although nothing can be asked for."
- file: crates/commission/src/admission.rs
  line: 40
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "Reordering the three refusal rules, carrying only the first uncapable entry's reasons, or dropping identical-list dedup all passed the existing suite; adversary2_admission_precedence.rs now catches each."
- file: crates/commission/src/admission.rs
  line: 72
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "The conflict refusal joins capability names with \", \", so different capability sets such as {\"a, b\",\"c\"} and {\"a\",\"b, c\"} produce identical refusals."
- file: docs/contracts/frontier.md
  line: 62
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The contract omits the reason-merge rule (lists ordered lexicographically, identical lists once) and that a capability conflict drops the entries' own reasons; only the module doc states them."
- file: .engineering/planning/story/stale-revision-action-request.md
  line: 93
  category: judgement
  severity: warning
  verdict: CONFIRMED
  origin: pre-existing
  message: "The revalidation command lists three outcomes with no slot for admit()'s NeedsAuthority, inviting a mapping that admits ApprovalRequired actions without authority."
```
