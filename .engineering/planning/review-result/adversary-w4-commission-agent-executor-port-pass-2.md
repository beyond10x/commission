---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-agent-executor-port-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:agent-executor-port, pass 2
relations:
- reviews: story:agent-executor-port
revision: 1
---
unit: commission/agent-executor-port, working tree on 27126af (phase 2 + pass-1 fixes, uncommitted) at ~/.local/state/worktree/trees/b10x/commission/commission-w4-agent-executor-port
verdict: NEEDS-CHANGE
cases: executed 82→92, red 3
origin: introduced 4 / pre-existing 2 / undecided 0
wrote-outside-worktree: 1 path remains, 2 more created and deleted (part 6)
needs-coordinator: no

**1. Diff**

`git --no-pager diff --stat` (tracked files, all the implementor's, none mine):
```
 Taskfile.yml                                     |  6 +++
 crates/commission-testkit/src/fake_executor.rs   | 68 +++++++++++++++++++++++-
 crates/commission-testkit/tests/executor_port.rs | 64 ++++++++++++++++++++--
 crates/commission/src/ports/executor.rs          | 27 +++++++++-
 docs/contracts/commission-executor.md            | 11 ++--
 model-provider-deny.txt                          |  4 ++
```
I added three untracked test files, all under `crates/commission-testkit/tests/`. I touched no implementation file and not the deny list. `cargo fmt --check` and testkit clippy with `-D warnings` both exit 0.

**2. Cases added**

| File | Cases | Asserts | Now |
|---|---|---|---|
| `adversary2_executor_contract_doc.rs` | 3 | the doc's `enum ExecutorOutcome` matches the generated union; every type the doc names exists in the model; the doc's prose never writes a tuple variant with `{ }` | **red** |
| `adversary2_executor_fake.rs` | 3 | a call past the script panics with "script is used up"; an empty script panics on the first call; the call that hits the empty script is logged, and `calls()` still works after the panic | green; red under mutant M1 |
| `adversary2_executor_guard.rs` | 4 | the guard fails with "`cargo tree` failed" when the lockfile is missing an entry, and when there is no lockfile; it refuses a denied crate printed as `(proc-macro) (*)`; `check` lists `deps-guard`, which runs exactly the story's command | green; Taskfile case red under mutant M2 |

Red output, first run of the doc file on its own, verbatim:
```
docs/contracts/commission-executor.md:30 `enum ExecutorOutcome` is not the generated union the port returns (generated/rust/commission/src/responsibility.rs):
  ProposedAction: doc carries { action: ActionId, arguments_json: String }, generated carries { action: String, arguments: ProposedActionArguments }
  ProposedAction: doc writes `{ action: ActionId, arguments_json: String }`, which neither builds nor matches the generated tuple variant
  NeedsHumanJudgment: doc writes `{ request: HumanDecisionRequest }`, which neither builds nor matches the generated tuple variant
  Suspended: doc writes `{ reason: SuspensionReason }`, which neither builds nor matches the generated tuple variant
  NoUsefulAction: doc carries unit, generated carries tuple(Unit)
  ...
  block at docs/contracts/commission-executor.md:30: ExecutorOutcome::ProposedAction names `ActionId`
  docs/contracts/commission-executor.md:26: `Suspended { reason: SuspensionReason::ExternalAvailability(..) }`.
test result: FAILED. 0 passed; 3 failed
```

**3. Suite run**

Ran after my cases existed: `cargo test --workspace --locked --offline --no-fail-fast` in the brief's build dir. Exit 101. 92 cases ran and the only failures are my 3 doc cases. Leaving out my three binaries (10 cases) from that same run gives 82, which is the before figure.

**4. Findings**

Fix for F1–F3: rewrite lines 28–49 to the generated shape (`ProposedAction(ExecutorOutcomeProposedAction { action: String, arguments: ProposedActionArguments })`, …, `NoUsefulAction(Unit)`, `CompletedLocalReasoning(Unit)`). Rewrite line 26 to `Suspended(ExecutorOutcomeSuspended { reason: SuspensionReason::ExternalAvailability(..) })`.

| # | file:line | What was measured | What reaches it | Verdict / origin |
|---|---|---|---|---|
| F1 | `docs/contracts/commission-executor.md:30` | outcome block test red (`adversary2_executor_contract_doc.rs:195`): `arguments_json` instead of `arguments`, all 5 variants in a shape that does not compile | line 13 now calls the doc "the port as built"; the story lists the doc as a source; executor authors (Loom) read it | NEEDS-CHANGE / pre-existing (`git show 450da32` has the same block, then labelled "Conceptual Rust") |
| F2 | `docs/contracts/commission-executor.md:33` | type test red (`:244`): `ActionId` is not declared anywhere in the generated model | same readers as F1 | NEEDS-CHANGE / pre-existing |
| F3 | `docs/contracts/commission-executor.md:26` | prose test red (`:284`): the failure rule is written as `Suspended { reason: .. }`, which the generated tuple variant rejects | the trait's only stated failure rule | NEEDS-CHANGE / introduced |
| F4 | `crates/commission-testkit/src/fake_executor.rs:65` | mutant M1 (return `NoUsefulAction` instead of panicking when the script is empty): all 82 existing cases stay green; `adversary2_executor_fake` goes red | `story:run-outcomes` and `story:local-runtime-loop` drive this fake | CONFIRMED / introduced |
| F5 | `Taskfile.yml:17` | mutant M2 (delete `- task: deps-guard` from `check`): all 82 existing cases stay green; the Taskfile case goes red | `story:commission-ess-conformance` edits this file two waves later | CONFIRMED / introduced |
| F6 | `docs/contracts/commission-executor.md:13` | the doc is not in the story's typed `scope` (5 paths) | your message calls the edit a pass-1 fix; whether it was sanctioned is yours to record | CONFIRMED / introduced |

**5. Attacked and held**
- **`cargo tree` failure:** a lockfile missing an entry, or no lockfile at all, makes the guard fail at `executor_port.rs:89` with the cargo error. Empty output would also fail the "names b10x-commission" check at `:252`. I did not build that as a mutant because free disk was below 10G.
- **`(*)` and `(proc-macro)` suffixes:** the real listing printed `async-openai v0.1.0 (proc-macro) (...) (*)` and the guard refused it, because it matches on the first word of each line.
- **Calls after the script runs out:** each call is logged before the script is read (`fake_executor.rs:59-65`), so the panicking call appears in `calls()`. A recovered lock means a second call past the script still panics with the script message.
- **Taskfile wiring:** `check` runs `deps-guard`, and `deps-guard` runs exactly the story's command. CI runs `task check` (`.github/workflows/check.yml:66`).

**6. Paths written outside the worktree**
- `~/.cache/b10x-target/commission-w4-agent-executor-port/tmp/adversary2-executor-guard/`: fixture workspaces my guard cases create in the build dir's temp area. They are still there, and each test run recreates them.
- `~/.cache/b10x-target/commission-w4-mutants4`: deleted.
- `~/.cache/ga-wave-2026-10-04-w4/commission-agent-executor-port/scratch/adv2-mutant-tree`: deleted.
- I took no worktree lease, because the brief forbids worktree commands.

**7. Findings block**
```findings
- file: docs/contracts/commission-executor.md
  line: 30
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: pre-existing
  message: "The outcome block in a doc that now calls itself 'the port as built' still shows the old sketch (arguments_json, braced and unit variants), not the generated tuple union the port returns."
- file: docs/contracts/commission-executor.md
  line: 33
  category: contract-drift
  severity: note
  verdict: NEEDS-CHANGE
  origin: pre-existing
  message: "The doc's ProposedAction names an ActionId type that the generated model does not declare; the model's field is action: String."
- file: docs/contracts/commission-executor.md
  line: 26
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The new failure rule writes Suspended { reason: .. }, which neither builds nor matches the generated Suspended(ExecutorOutcomeSuspended { .. }) tuple variant."
- file: crates/commission-testkit/src/fake_executor.rs
  line: 65
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "Replacing the used-up-script panic with a default NoUsefulAction leaves all 82 existing cases green; adversary2_executor_fake catches it."
- file: Taskfile.yml
  line: 17
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Deleting '- task: deps-guard' from check leaves all 82 existing cases green; adversary2_taskfile_check_runs_deps_guard_with_the_story_command catches it."
- file: docs/contracts/commission-executor.md
  line: 13
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The unit edits a contract doc that is not in the story's typed scope."
```
