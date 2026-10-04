---
format: aep.planning-md/3
id: review-result:adversary-w5-commission-observation-evidence-ports-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w5 adversary, commission story:observation-evidence-ports, pass 2
relations:
- reviews: story:observation-evidence-ports
revision: 1
---
unit: commission/observation-evidence-ports, working tree commission-w5-observation-evidence-ports (HEAD e2487aa + uncommitted phase 2 + pass-1 fixes)
verdict: CONFIRMED (no blocker; two red cases, both INFEASIBLE)
cases: executed 143→149, red 2
origin: introduced 5 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 paths
needs-coordinator: should whitespace in a trusted producer be refused or trimmed? (finding 1)

**1. Diff stat** (`git --no-pager diff --stat`, tracked files; untracked listed below)
```
 crates/commission-testkit/src/fake_governor.rs     |  53 ++++++++-
 .../tests/observation_evidence.rs                  |  62 +++++++++-
 crates/commission/src/ports/evidence.rs            | 127 ++++++++++++++++++++-
```
Those three are the unit's own changes. The only files I added are untracked tests: `crates/commission-testkit/tests/adversary2_evidence_{producer,check_order,fake_log}.rs`. I touched no implementation file. I ran `rustfmt` on my two green files only (layout changes, no logic).

**2. Cases added**

| File | Asserts | Now |
|---|---|---|
| `adversary2_evidence_producer.rs` `a_padded_trusted_producer_is_refused_or_arrives_as_the_id_it_names` | `"P1\n"`, `" P1"`, `"P1 "`: either `NoProducer`, or the record carries `"P1"` | red |
| same file, `an_invisible_trusted_producer_names_nobody` | `\u{200B}`, `\u{FEFF}`, `\u{0}`, `\u{200B}\u{2060}` are refused with `NoProducer` | red |
| `adversary2_evidence_check_order.rs` (2 tests) | blank producer plus empty ids gives `NoProducer`; a local refusal never calls the port; on the fake, a local refusal hides `UnknownCase` | green, pins the order |
| `adversary2_evidence_fake_log.rs` (2 tests) | the ports take no scripted answer and add nothing to `calls()`; 3 threads × 5000 (evidence, observe, `current_revision`) lose and reorder nothing, and no record shrinks between reads | green |

Red output from running the producer file alone, before the suite (verbatim excerpt):
```
assertion `left == right` failed: the trusted producer "P1\n" passed the blank check as "P1", so whitespace is not part of the id, yet the record carries it
  left: ["P1\n"]
 right: ["P1"]
---- an_invisible_trusted_producer_names_nobody stdout ----
... the trusted producer "\u{200b}" renders as nothing and names nobody; the governor recorded [EvidenceData { ... producer: "\u{200b}", ... }]
  left: Ok(())
 right: Err(NoProducer)
test result: FAILED. 0 passed; 2 failed; ... EXIT=101
```

**3. Suite run** (after the cases existed): `cargo test --workspace --locked --no-fail-fast`, exit 101. The per-binary summary lines add up to 147 passed and 2 failed; the only failures are the two above. I got the before count of 143 by subtracting my 6 cases from that same run, not from a second run. Each of my three files ran under its own binary name in this tree's output. `cargo fmt --check` exits 0. I did not run clippy (see part 5).

**4. Findings**

| # | file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|---|
| 1 | `crates/commission/src/ports/evidence.rs:115,122` | INFEASIBLE / introduced | The blank check uses `producer.trim()`, but the record stores `producer.to_owned()` untrimmed. `"P1\n"` is admitted and never compares equal to `"P1"`. Red case above. | Only tests call `submit_evidence`; I found no production caller. Fix: trim at :122, or refuse padded input. |
| 2 | `evidence.rs:115` | INFEASIBLE / introduced | Producers that are invisible but not `White_Space` (ZWSP, BOM, NUL) are admitted, so a record can be attributed to nobody. Red case above. | Same: no caller found. |
| 3 | `evidence.rs:103-106,115-120` | CONFIRMED / introduced, mutant | The order of refusals is fixed (NoProducer, then NoObservation, then Governor) but only implied by the order of the doc list. Each existing test sets one fault with everything else valid (`observation_evidence.rs:239-249`, `:197`). Reading those inputs, swapping the two checks or calling the port first would stay green. My check-order file catches both. | Any caller that branches on the error variant. |
| 4 | `crates/commission-testkit/src/fake_governor.rs:16` | CONFIRMED / introduced, mutant | No unit test reads `calls()` or the script after a port call, so "Neither port takes a scripted answer" was unpinned. A `receive` that answered through `state.answer` would stay green. My fake-log file catches it. | `story:local-runtime-loop` scripts one answer per governor call. |
| 5 | `evidence.rs:13-16` | CONFIRMED / introduced, judgement | The module says evidence enters "only through `submit_evidence`" (push). ADR 0079 Decision 2 has the governor ask an `EvidenceProvider` for kind K / subject S / revision R (pull). ADR 0079 under Open names this story as a candidate home for that contract. The module does not mention ADR 0079 or say whether `EvidenceAdapter` is that provider. The ADR 0074 wording does match. | Readers choosing where `EvidenceProvider` lives. |

**What the downstream stories will need** (gaps only, no demands; all inferred from reading):
- **local-runtime-loop:**
  - Commission has no clock or id port. A10 needs a trusted `observed_at` and an `observation_id`, and AGENTS.md § Rules says the model never supplies trusted time.
  - Nothing defines which `ExecutorOutcome` variants become observations, or what the loop does when `observe` returns `Err`.
  - The observation log and `calls()` share no sequence, so A10's "from that step" can only be matched by payload or subject.
  - Nothing enforces "runtime never calls an `EvidenceAdapter`". A source scan like no-hand-model could.
- **adapter-conformance-suites:**
  - `ObservationPort` and `EvidencePort` have no read side; `observations()` and `evidence()` exist only on the fake. A governor kit run against a real adapter needs the factory to supply an inspection hook before it can check "arrives with the observation ids it was submitted with".
  - The fake accepts evidence at any `subject_revision` and with unknown observation ids, so the kit cannot require refusing either.

**5. Attacked and could not break**
- Concurrent `receive`, `observe` and `calls()`: one lock, nothing lost or reordered (15k operations).
- `AttributedEvidence` cannot be built outside `submit_evidence` (private field, no `Clone`, `Default` or `From`).
- The producer in the payload's `facts` and `provenance` is carried through unchanged and never read.
- Mutant builds: **not run**. Free disk on `/` was 9.4–9.5G, below the 10G floor for starting a build, so `~/.cache/b10x-target/commission-w5-mutants1` was never created. For the same reason clippy was not run. Mutant survival in findings 3 and 4 comes from reading test inputs, not from a build.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w5/commission-observation-evidence-ports/scratch/adversary2/red-producer.log`
- `~/.cache/ga-wave-2026-10-04-w5/commission-observation-evidence-ports/scratch/adversary2/order-and-log.log`
- `~/.cache/ga-wave-2026-10-04-w5/commission-observation-evidence-ports/scratch/adversary2/suite.log`
- Build output also went to the brief's `CARGO_TARGET_DIR` (test binaries only).

**7. Findings block**
```findings
- file: crates/commission/src/ports/evidence.rs
  line: 122
  category: boundary
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "the blank check trims the trusted producer but the record stores it untrimmed, so \"P1\\n\" is admitted and never equals \"P1\"; no production caller of submit_evidence exists yet"
- file: crates/commission/src/ports/evidence.rs
  line: 115
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "a producer made only of invisible non-White_Space characters (ZWSP, BOM, NUL) is admitted and attributes the record to nobody"
- file: crates/commission/src/ports/evidence.rs
  line: 103
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the precedence NoProducer > NoObservation > Governor is implied only by the doc's list order and no unit test combines two faults, so swapping the checks would survive; adversary2_evidence_check_order.rs pins it"
- file: crates/commission-testkit/src/fake_governor.rs
  line: 16
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "no unit test reads calls() or the script after a port call, so a receive that consumed a scripted answer would survive; adversary2_evidence_fake_log.rs pins it"
- file: crates/commission/src/ports/evidence.rs
  line: 13
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the module says evidence enters only through submit_evidence (push) and does not mention ADR 0079's EvidenceProvider (governor pulls by kind, subject and revision), whose home ADR 0079 leaves open with this story as a candidate"
```
