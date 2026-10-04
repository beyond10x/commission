---
format: aep.planning-md/3
id: story:commission-ess-conformance
kind: story
status: proposed
title: task check runs Commission's synthesized ESS conformance suite
summary: Commission's ESS specification held to its synthesized suite through a Rust target on ess-conformance, skips named in ess/SKIPPED.md.
refs:
- provider: taskboard
  reference: I-007
relations:
- decomposes: epic:commission-core
- depends_on: story:stale-revision-action-request
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:run-outcomes
scope:
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Taskfile.yml
- confidence: inferred
  path: crates/commission-conformance/
- confidence: cited
  path: ess/SKIPPED.md
- confidence: inferred
  path: ess/domains/responsibility.yaml
- confidence: inferred
  path: generated/rust/commission/
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":1}}}
---
## Outcome

`task check` holds Commission's ESS specification (`ess/`) to its synthesized conformance suite.

- **Where the suite comes from.** `ess verify conform synthesize --path ess` writes it.
- **What it runs against.** `b10x-commission`, through a Rust target in a new crate,
  `crates/commission-conformance/` (package `b10x-commission-conformance`). That crate is built on
  the `ess-conformance` crate, a git dependency pinned by `Cargo.lock`.
- **How the run is started.** A new task, `conform`, runs the crate's test
  (`cargo test -p b10x-commission-conformance --test conform`). `check` lists it as its own step.
- **How the verdict is read.** The test reads the verdict from the report document, not from the
  runner's exit code. A passing report shows that the suite ran, that it asserted something, that
  nothing failed, and that every skip is named in `ess/SKIPPED.md`.

`story:generated-responsibility-model` creates `ess/SKIPPED.md`. This story adds a line to it only
for a scenario it cannot answer.

By the time this story starts, every command of `epic:commission-core` is in `ess/`:

- the suspend and resume commands for a Run, declared by `story:port-skeleton`, with the behaviour
  behind them from `story:run-outcomes`;
- the action-request revalidation command, declared and implemented by
  `story:stale-revision-action-request`.

This story answers the scenarios of all of them. `story:local-runtime-loop` and
`story:adapter-conformance-suites`, which run beside it, add no noun and no command.

ESS ships Go and TypeScript conformance packages. They are not used here, because anything committed
in this repository that runs is Rust (`AGENTS.md` § Rules). Two Rust targets over `ess-conformance`
already exist as precedents: Mandate `crates/mandate-conformance` and Entity Runtime
`checks/ess-conformance`.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on these, each a real dependency:

- `story:stale-revision-action-request`, whose revalidation answers the action-request scenarios;
- `story:run-outcomes`, whose suspend and resume behaviour answers the Run scenarios.

Its old edge on `story:local-runtime-loop` was ordering only (the loop no longer edits `ess/`) and
is gone, as is `story:adapter-conformance-suites`'s edge onto this story. It runs beside both.
`Taskfile.yml` is shared with `story:agent-executor-port` and `Cargo.lock` with
`story:port-skeleton`, both two or more waves earlier; `ess/SKIPPED.md` is this story's alone.

## ESS first

The suite covers the whole of `ess/`: `ess-inputs.yaml`, `system.yaml` and
`domains/responsibility.yaml`.

- **Specification change: none planned.** The commands this story answers are declared by
  `story:port-skeleton` (suspend, resume) and `story:stale-revision-action-request` (revalidation).
  If a scenario shows the specification is wrong, that change is made first in a commit of its
  own, `drift_passes_on_the_committed_tree` (`crates/commission-xtask/tests/checks.rs`) is red on
  it, and `task generate` follows (Atlas ADR 0080).
- **First commit, red.** The crate `crates/commission-conformance/` with the test
  `ess_conformance_report` and a Rust target that answers no command. The test is red because the
  report records the synthesized scenarios as failed, not passed (expectations 1 and 2).
- **Then.** The target answers each scenario through `b10x-commission`, and any scenario it cannot
  answer is named in `ess/SKIPPED.md`.

## Domain relations

None assumed.

## Scope

- `crates/commission-conformance/` (new): `Cargo.toml`, `src/lib.rs`, `tests/conform.rs`
- `Cargo.lock`
- `Taskfile.yml` (the task `conform` and one line in `check`)
- `ess/SKIPPED.md`
- `ess/domains/responsibility.yaml` and `generated/rust/commission/` (changed only if a scenario
  shows a specification defect; no other story in its wave edits them)

## Acceptance

The test `ess_conformance_report` in `crates/commission-conformance/tests/conform.rs` passes. It
synthesizes the suite from `ess/`, runs it against `b10x-commission` through the Rust target, and
checks these expectations against the report document:

1. The report records at least 1 passed scenario.
2. The report records no failed scenario.
3. Every skipped scenario in the report is named in `ess/SKIPPED.md`.
4. The report check is applied to a copy of the report that adds one skipped scenario `S`, which
   `ess/SKIPPED.md` does not name. The check fails and names `S`.

## Notes

- `ess-inputs.yaml` requires ess 0.52.0. Pin `ess-conformance` to the same release.
- Recording the report as AEP evidence on an `executable-system-specification` artifact is not part
  of this story. The store holds no such artifact.

## Source

TASKBOARD I-007 (build pack `TASKBOARD.md` § Integration, "ESS spec for Commission");
`epic:commission-core` Acceptance; Atlas `epic:ga-commission-core` Acceptance (`ess/SKIPPED.md`);
`AGENTS.md` § ESS; Atlas ADR 0080.
