---
format: aep.planning-md/3
id: review-result:adversary-w4-commission-agent-executor-port-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w4 adversary, commission story:agent-executor-port, pass 1
relations:
- reviews: story:agent-executor-port
revision: 1
---
```
unit: commission/agent-executor-port, working tree = 27126af + uncommitted phase 2 (worktree commission-w4-agent-executor-port)
verdict: NEEDS-CHANGE
cases: executed 75→81, red 3
origin: introduced 6 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 paths kept (2 logs, 5 fixture dirs under one parent, my test binary in the brief's build dir); 2 deleted (the mutants build dir and the mutant source copy)
needs-coordinator: yes. The story prescribes `cargo tree -p b10x-commission -e normal --prefix none` word for word, so fixing finding 1 means changing the story's acceptance text, not only the code.
```

**1. Diff stat**

```
 Taskfile.yml                                     |  6 ++++
 crates/commission-testkit/src/fake_executor.rs   | 42 +++++++++++++++++++++++-
 crates/commission-testkit/tests/executor_port.rs | 15 +++++++++
 crates/commission/src/ports/executor.rs          | 24 ++++++++++++--
 4 files changed, 84 insertions(+), 3 deletions(-)
?? crates/commission-testkit/tests/adversary_executor_guard.rs
```

The four modified files are the implementor's uncommitted phase 2, as you handed it to me; I edited none of them. My only change is the new, untracked test file. No non-test path was touched by me.

**2. Cases added** in `crates/commission-testkit/tests/adversary_executor_guard.rs`

The guard's code is private to `executor_port.rs`. Each case therefore runs the guard's own compiled `executor_port` test binary, with `CARGO_MANIFEST_DIR` pointed at a small fixture workspace. The guard reads that variable at run time, so it runs its own `cargo tree` on the fixture. The harness refuses a binary older than `executor_port.rs`. All fixture crates are local path crates, so nothing is fetched.

| case | line | asserts | now |
|---|---|---|---|
| `guard_refuses_a_plain_provider_dependency` | :228 | plain normal dependency on `async-openai` is refused (control) | green |
| `guard_refuses_a_renamed_provider_dependency` | :240 | `llm-client = { package = "async-openai" }` is refused | green |
| `guard_refuses_a_transitive_provider_dependency` | :254 | `glue` → `async-openai` is refused | green |
| `guard_refuses_a_provider_behind_a_feature` | :274 | optional `async-openai` behind feature `openai` is refused | **red** |
| `guard_refuses_loom_for_another_target` | :295 | `[target.'cfg(target_arch = "wasm32")'.dependencies] b10x-loom` is refused | **red** |
| `deny_list_covers_each_major_provider_family` | :313 | deny list has a crate for each of OpenAI, Anthropic, Google, Cohere, Mistral, Bedrock, Ollama, llama.cpp | **red** |

The two red guard cases first prove the dependency is real: `cargo tree --all-features` and `cargo tree --target all` on the fixture both name the crate.

Red output, from running this file alone before the suite (excerpt of `scratch/adversary-p1-red.log`):
```
---- guard_refuses_loom_for_another_target stdout ----
panicked at crates/commission-testkit/tests/adversary_executor_guard.rs:214:5:
the dependency guard passed a b10x-commission that depends on b10x-loom for target_arch = "wasm32":
running 1 test
test executor_port_contract ... ok
---- guard_refuses_a_provider_behind_a_feature stdout ----
panicked at crates/commission-testkit/tests/adversary_executor_guard.rs:214:5:
the dependency guard passed a b10x-commission that depends on async-openai behind the non-default feature `openai`:
test executor_port_contract ... ok
---- deny_list_covers_each_major_provider_family stdout ----
panicked at crates/commission-testkit/tests/adversary_executor_guard.rs:334:5:
model-provider-deny.txt names no crate for ["Cohere"]; it names ["async-openai", ... "llama-cpp-2"]
test result: FAILED. 3 passed; 3 failed; ... exit=101
```

**Mutants.** I built them from a copy in the scratch dir into `commission-w4-mutants4`, then deleted both.

| mutant | existing `executor_port_contract` | my control cases |
|---|---|---|
| M1: guard's `cargo tree` gets `--depth 0` (executor_port.rs:80) | **green (survives)** | red, 3 of 3 |
| M2: `-e normal` becomes `-e build` (executor_port.rs:78); I checked the binary was rebuilt (newer mtime, no `--depth` string) | **green (survives)** | red |
| M3: fake uses `front().cloned()` instead of `pop_front()` (fake_executor.rs:38) | red at executor_port.rs:176 | n/a |

**3. Suite** (run after the cases existed): `cargo test --workspace --locked --no-fail-fast` with the brief's build dir, offline.
- Exit 101. Only `adversary_executor_guard` fails: 3 passed, 3 failed. All other binaries pass.
- 81 tests ran: the implementor's 75 (from `scratch/gate.log.test`) plus my 6.
- The raw summed count shows 83 because my failure messages quote the guard's own `test result` lines. I counted per binary from the `Running` lines.
- `cargo fmt --check` exit 0. `cargo clippy -p b10x-commission-testkit --all-targets -- -D warnings` exit 0.

**4. Findings** (they cover the working tree on top of 27126af)

| # | file:line | verdict | origin | measured | what reaches it |
|---|---|---|---|---|---|
| 1 | crates/commission-testkit/tests/executor_port.rs:72 | NEEDS-CHANGE | introduced | the guard checks default features on the host target only, so it misses a provider behind a feature and Loom under a wasm32 target (adversary_executor_guard.rs:274, :295) | `task check` → `deps-guard` lets any change that adds such a dependency through. A sibling crate turning the feature on hits the same gap. Nothing in the repo does this today. Fix: add `--all-features --target all` (needs a story amendment) |
| 2 | crates/commission-testkit/tests/executor_port.rs:239 | CONFIRMED | introduced | the only presence check on the real listing is the root line, so M1 and M2 leave the existing test green. My three control cases catch both | an edit to the guard's arguments. Fix: keep the controls, or also assert the listing names `commission v1.0.0` |
| 3 | model-provider-deny.txt | NEEDS-CHANGE | introduced | no Cohere crate on the list (case :313) | epic acceptance: "`cargo tree` shows no model-provider crate". I could not check exact crate names offline (for example `cohere-rust`), so I don't know which spellings exist |
| 4 | crates/commission-testkit/src/fake_executor.rs:33 | CONFIRMED | introduced | `ScriptedExecutor` ignores its inputs and keeps no call log | `story:local-runtime-loop` Acceptance 1 needs "the fakes' call logs", and that story's scope leaves out `fake_executor.rs`. It can be worked around with a logging wrapper in `runtime_loop.rs` |
| 5 | docs/contracts/commission-executor.md:16 | CONFIRMED | introduced | the doc says `Result<ExecutorOutcome, ExecutorError>` and "likely async". The port is synchronous and cannot return an error | a reader of the contract. A failing real executor's only way out is `Suspended(ExternalAvailability)` |

**5. Attacked, not broken**
- Renamed, transitive and plain dependencies: all refused (green cases).
- Underscore vs hyphen: crates.io does not allow both spellings of one name, and `cargo tree` prints the published name, so only a typo in the deny list would slip through.
- Dev-dependencies: left out by `-e normal`, as the story intends; they do not ship.
- Build-dependencies: also left out. A build-dependency on Loom would be a cycle cargo refuses, and nothing adds a provider as a build-dependency.
- Testkit: not inspected. The story scopes the guard to `b10x-commission`, and the testkit has `publish = false`.
- ADR 0082: `run` takes only shared references. `Commission<Assigned>` has only `new`, `state`, `data` and `into_data` (generated responsibility.rs:949-968). No variant marks completion or carries an effect handle.
- Panic when the script runs out: fine for run-outcomes (one call per row) and local-runtime-loop (its scripts end with a final outcome).
- M3 is caught by the existing test.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w4/commission-agent-executor-port/scratch/adversary-p1-red.log`
- `~/.cache/ga-wave-2026-10-04-w4/commission-agent-executor-port/scratch/adversary-p1-suite.log`
- `~/.cache/b10x-target/commission-w4-agent-executor-port/tmp/adversary-executor-guard/{plain,renamed,transitive,feature,target}` (276K), plus my test binary in that build dir
- deleted: `~/.cache/b10x-target/commission-w4-mutants4`
- deleted: `~/.cache/ga-wave-2026-10-04-w4/commission-agent-executor-port/scratch/adversary-p1-mutant`

Disk is at the 10G floor: `/` has 10G free after cleanup.

```findings
- file: crates/commission-testkit/tests/executor_port.rs
  line: 72
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The guard runs cargo tree without --all-features or --target all, so a model-provider crate behind a non-default feature and b10x-loom under a wasm32 target dependency both pass deps-guard (adversary_executor_guard.rs:274, :295 red)."
- file: crates/commission-testkit/tests/executor_port.rs
  line: 239
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "The real-listing check only proves the root line is present, so mutants adding --depth 0 or switching -e normal to -e build leave executor_port_contract green; the adversary control cases kill both."
- file: model-provider-deny.txt
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The deny list names no Cohere SDK crate, while every other major provider family in the brief is covered (adversary_executor_guard.rs:313 red)."
- file: crates/commission-testkit/src/fake_executor.rs
  line: 33
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "ScriptedExecutor ignores its inputs and keeps no call log, while story:local-runtime-loop Acceptance 1 needs the fakes' call logs and its scope excludes fake_executor.rs, so that story must wrap the fake in its own test file."
- file: docs/contracts/commission-executor.md
  line: 16
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The contract sketch still promises Result<ExecutorOutcome, ExecutorError> and an async production shape, while the port returns a bare ExecutorOutcome synchronously with no executor-failure channel."
```
