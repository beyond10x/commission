---
format: aep.planning-md/3
id: review-result:adversary-w5-commission-observation-evidence-ports-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w5 adversary, commission story:observation-evidence-ports, pass 1
relations:
- reviews: story:observation-evidence-ports
revision: 1
---
unit: commission/observation-evidence-ports, working tree on e2487aa plus uncommitted phase 2 plus my 2 untracked test files
verdict: CONFIRMED (one red case, INFEASIBLE in reach; one suite gap that a new green case now covers)
cases: executed 138→141, red 1
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 6 paths (part 6)
needs-coordinator: whether the fake must refuse evidence for an unknown case or unknown observation ids (finding 3)

**1. Diff.** `git diff --stat` shows only phase 2's two implementation files (`fake_governor.rs`, `ports/evidence.rs`, 156+/3-). I edited neither. `git status` shows my two untracked files, both test files:
- `crates/commission-testkit/tests/adversary_evidence_attribution.rs`
- `crates/commission-testkit/tests/adversary_evidence_governor_refusal.rs`

**2. Cases added. Each was run alone before the suite.**

| case | what it asserts | now |
|---|---|---|
| `evidence_attributed_to_nobody_is_refused` | `submit_evidence` with producer `""` or `"   "` is refused, and nothing is recorded | **red** |
| `a_governor_refusal_comes_back_as_the_governor_error` | a port that refuses with `GovernorUnavailable` or `UnknownCase` comes back as `Err(EvidenceError::Governor(<same>))`, delivered once | green; red on mutants MA and MB |
| `an_accepted_record_is_delivered_once_and_a_refused_one_never` | delivered once with the caller's producer; empty `observation_ids` never reaches any port | green |

Red output from the solo run, exit 101:
```
panicked at crates/commission-testkit/tests/adversary_evidence_attribution.rs:42:9:
evidence attributed by the trusted caller to "" names no producer, yet it was admitted: Ok(()); the governor recorded [EvidenceData { ... producer: "", observation_ids: [ObservationId(Uuid("...0901"))], ... }]
```
After `rustfmt` and removing `.clone()` on Copy types for clippy, the panic line is :40.

**3. Suite, run after the cases existed.** `cargo test --workspace --locked --no-fail-fast` with the brief's `CARGO_TARGET_DIR` exited 101. Summary: passed 140, failed 1 (`adversary_evidence_attribution`). The 138 "before" is the sum of the implementer's `gate-test.log`. `cargo fmt --check` exited 0. `cargo clippy --workspace --all-targets --locked -- -D warnings` exited 0. `-- --list` shows all 3 new tests in this tree.

**4. Findings** (all introduced; the base had a one-line stub in `evidence.rs`)

| # | file:line | verdict | measured | what reaches it |
|---|---|---|---|---|
| 1 | `crates/commission/src/ports/evidence.rs:110` | INFEASIBLE | an empty or blank trusted producer is admitted as `AttributedEvidence`. Red case above. Fix: refuse it with a typed error before line 109 | no production caller of `submit_evidence` yet; only tests call it |
| 2 | `crates/commission/src/ports/evidence.rs:113` | CONFIRMED | the original 138 cases stayed green under MA (`let _ = port.receive(..); Ok(())`) and MB (any refusal mapped to `GovernorUnavailable`). Only my refusal case went red, because the fake never refuses | any real governor that refuses evidence |
| 3 | `crates/commission-testkit/src/fake_governor.rs:247` | CONFIRMED | the fake records evidence for a case it does not hold, and evidence naming observation ids it never received. Its `Governor` calls answer `UnknownCase` for an unknown case | the governor kit in `story:adapter-conformance-suites` says it "answers an unknown case with the typed error"; if that kit applies the check to evidence, the fake fails it. The story does not require refusal. Judgement, so no case written |
| 4 | `crates/commission-testkit/tests/observation_evidence.rs:142` | INFEASIBLE | a producer the payload claims (`"producer": "P2"`) reaches the governor verbatim in `facts` and `provenance`, and the acceptance test pins this via `..claimed` at :157 | nothing reads `provenance.producer` today. The AGENTS.md rule is that the model never supplies the evidence producer's identity |

**5. Attacked and could not break**
- **Forging `AttributedEvidence`:** a compile probe in a scratch copy failed with E0423 (private field), E0308 (no `Clone`), and E0277 for both `Default` and `From<EvidenceData>`. No serde. `#![forbid(unsafe_code)]` in all three crates. The only constructor is `evidence.rs:109`.
- **Producer overwrite:** complete for the `producer` field.
- **Duplicate `observation_ids`:** passed through unchanged. That matches the kit's "arrives with the observation ids it was submitted with".
- **Executor output or trace turning into evidence:** I found no such path in `crates/` (searched for evidence and observ). Keeping adapters off executor output is left to `story:local-runtime-loop`.
- **Concurrency:** each new log is one push under the single Mutex, with no ordering claim across logs. Nothing to break.
- **Acceptance mutants:** the implementer's M1–M4b were all killed. None of the acceptance assertions could pass if the code under test returned a default value.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w5/commission-observation-evidence-ports/scratch/adversary1/` (directory)
- …/adversary1/evidence.rs.orig
- …/adversary1/fake_governor.rs.orig
- …/adversary1/mutant-MA.log, mutant-MA2.log, mutant-MB.log
- …/adversary1/suite.log
- Deleted: …/adversary1/tree (the mutation copy) and `~/.cache/b10x-target/commission-w5-mutants1`.
- I also took and released a worktree session lease named `adversary1-observation-evidence-ports`.

**7.**
```findings
- file: crates/commission/src/ports/evidence.rs
  line: 110
  category: boundary
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "submit_evidence admits evidence attributed by the trusted caller to an empty or blank producer, so AttributedEvidence can carry a producer that names nobody."
- file: crates/commission/src/ports/evidence.rs
  line: 113
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "Swallowing or collapsing the governor's refusal leaves the acceptance suite green because the fake never refuses; adversary_evidence_governor_refusal.rs now kills both mutants."
- file: crates/commission-testkit/src/fake_governor.rs
  line: 247
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The fake records evidence for a case it does not hold and for observation ids it never received, while its Governor calls refuse an unknown case with UnknownCase."
- file: crates/commission-testkit/tests/observation_evidence.rs
  line: 142
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "A producer claimed in the payload's facts and provenance reaches the governor verbatim, and the acceptance test pins that, so a later fix that strips it would break the acceptance test."
```
