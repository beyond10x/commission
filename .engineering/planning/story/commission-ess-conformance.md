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
- depends_on: story:local-runtime-loop
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
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
revision: 5
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

By the time this story starts, the chain has put every command into `ess/`:

- the action-request command, from `story:stale-revision-action-request`;
- the suspend and resume commands for a Run, from `story:run-outcomes`.

This story answers the scenarios of all of them. The only later link,
`story:adapter-conformance-suites`, adds no noun and no command.

ESS ships Go and TypeScript conformance packages. They are not used here, because anything committed
in this repository that runs is Rust (`AGENTS.md:35`). Two Rust targets over `ess-conformance`
already exist as precedents: Mandate `crates/mandate-conformance` and Entity Runtime
`checks/ess-conformance`.

## Shared surface

This story is link 10 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:local-runtime-loop`, and
`story:adapter-conformance-suites` depends on it. The whole order is in
`story:generated-responsibility-model` § Shared surface. The same chain orders this story's edits to
`Taskfile.yml`, `Cargo.lock` and `ess/SKIPPED.md`.

## ESS

The suite covers the whole of `ess/`: `ess-inputs.yaml`, `system.yaml` and
`domains/responsibility.yaml`. This story changes the specification only when a scenario shows the
specification is wrong. If it changes it, `ess specify validate --path ess` passes and
`task generate` regenerates.

## Domain relations

None assumed.

## Scope

- `crates/commission-conformance/` (new): `Cargo.toml`, `src/lib.rs`, `tests/conform.rs`
- `Cargo.lock`
- `Taskfile.yml` (the task `conform` and one line in `check`)
- `ess/SKIPPED.md`
- `ess/domains/responsibility.yaml` and `generated/rust/commission/` (chain surface; changed only if
  a scenario shows a specification defect)

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
`AGENTS.md` § ESS.
