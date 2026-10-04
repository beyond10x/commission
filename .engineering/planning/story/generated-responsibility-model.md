---
format: aep.planning-md/3
id: story:generated-responsibility-model
kind: story
status: implemented
title: Agent, AgentRevision, CaseRef and Commission come from the generated ESS model
summary: Model committed under generated/rust/commission/ from ess generate synthesize, re-exported by b10x-commission, held by task drift and task no-hand-model.
refs:
- provider: taskboard
  reference: M-001
relations:
- decomposes: epic:commission-core
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: .gitignore
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/commission-xtask/
- confidence: cited
  path: crates/commission/Cargo.toml
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: inferred
  path: crates/commission/tests/generated_model.rs
- confidence: cited
  path: ess/SKIPPED.md
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 11
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "proposed", to: "active", at: "2026-10-04T00:23:52Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "active", to: "implemented", at: "2026-10-04T00:59:23Z", actor: "human:timo", revision: 11, decided_on: {"recorded":{"test_result":1,"review_outcome":18,"verification":1}}}
---
## Outcome

The model types `b10x-commission` uses for Agent, AgentRevision, the case reference (`CaseRef`, the
`Case` entity) and Commission come from `ess generate synthesize --target rust` over `ess/`, not
from hand transcription. This is the first story of the chain and it decides where the generated
model lives:

- **Layout.** The synthesized model is committed under `generated/rust/commission/`, written by
  `ess generate synthesize --path ess --target rust --layout crate --out <dir>`. The crate is named
  `commission` and declares its own `[workspace]` (probe 2026-10-04, ess 0.52.0), so it is not a
  member of `members = ["crates/*"]`. `crates/commission/Cargo.toml` takes it as a path dependency
  and `b10x-commission` re-exports it, so every later story imports model types through
  `b10x-commission`.
- **Regeneration.** A new task `generate` writes a fresh tree into a temporary directory and
  replaces `generated/rust/commission/` with it. Synthesizing into an existing tree that has no
  `.ess-output/` is refused (`unowned output destination: Cargo.toml`, probe 2026-10-04).
  `.ess-output/state.json` carries a per-run anchor id, inode and absolute root, so two runs differ
  in it and in nothing else. `.ess-output/` is therefore git-ignored and left out of every
  comparison.
- **Checks are Rust.** A new crate `crates/commission-xtask/` (clap derive; the organization rule
  that every committed gate is Rust) provides the two checks below; the Taskfile tasks only call
  `cargo run -p commission-xtask -- <check>`.
- **Drift.** `cargo run -p commission-xtask -- drift` regenerates into a temporary directory and
  compares it, file by file and ignoring `.ess-output/`, with `generated/rust/commission/`. It fails
  on any difference and names the file.
- **No hand-written model.** `cargo run -p commission-xtask -- no-hand-model` fails when a source
  file under `crates/commission/src` defines a struct named `AgentId`, `AgentRevisionId`, `CaseId`,
  `CommissionId` or `Commission`, and names the file and line. The hand-written `AgentId`, `CommissionId` and `Commission` in
  `crates/commission/src/lib.rs:11-22` are deleted. Canon's `CaseId` import (`lib.rs:9`) is not a
  definition and stays where the ports still need it.
- **Formatting.** The generated sources are not rustfmt-clean (probe 2026-10-04: `rustfmt --check`
  reports diffs in `src/json.rs`). `cargo fmt --all` also formats local path dependencies, so the
  `check` fmt step is narrowed to the workspace's own packages. Before the story closes, check that
  the narrowed step does not read `generated/`.
- **Taskfile.** `check` lists `drift` and `no-hand-model` as their own steps. Later stories add
  their own named tasks and one line each to `check`. No story edits another story's task body.
- **Skips file.** This story creates `ess/SKIPPED.md`: a header that says each line names one
  skipped conformance scenario and why, and an empty list. `story:commission-ess-conformance`
  reads it, and no story depends on landing order to find it.

The ports (`Governor`, `AgentExecutor`, `AuthorityProvider`, `lib.rs:49-63`) keep their bootstrap
shape here. Their stories move them onto generated types.

## Shared surface

`ess/domains/responsibility.yaml` and `generated/rust/commission/` are one surface for the eleven
stories of `epic:commission-core`. Every story that edits `ess/` regenerates that one tree, so the
eleven stories run as one chain, each depending on the previous one:
`story:generated-responsibility-model` → `story:frontier-admission` → `story:governor-port` →
`story:agent-executor-port` → `story:authority-provider-port` →
`story:observation-evidence-ports` → `story:stale-revision-action-request` →
`story:run-outcomes` → `story:local-runtime-loop` → `story:commission-ess-conformance` →
`story:adapter-conformance-suites`. This story is the first link. `Taskfile.yml`, `Cargo.toml`,
`Cargo.lock` and `crates/commission/src/lib.rs` are edited along the same chain.

## ESS

The entities this story generates are already in `ess/domains/responsibility.yaml`: `Agent`
(`:77-94`), `AgentRevision` (`:96-108`), `Case` (`:111-123`) and `Commission` (`:125-163`). They
include the Commission-local `PrincipalId` (`:54-57`) and `AuthorityContext` (`:59-63`) and the
Commission fields `principal` and `authority_context` (`:139-142`), as the operator decided on
2026-10-04. `ess specify validate --path ess` passes before any code change. Any change this
story needs is made there first. The generated tree is regenerated from `ess/` and never edited.

No marker in this story's entities stays open. The Case→Commission cardinality and the
principal/authority-context types were decided on 2026-10-04
(`decision-blocker:case-commission-cardinality` and `decision-blocker:commission-principal-type`,
both cleared).

## Domain relations

- Agent -> AgentRevision, one-to-many, the agent owns its revisions:
  `ess/domains/responsibility.yaml:86-90`, `commission.responsibility.Agent` relation `revisions`.
- Commission -> AgentRevision, many-to-one, references: `:144-148`, relation `agent_revision`.
- Commission -> Case, many-to-one, references: `:149-153`, relation `case`. A case may hold many
  commissions at once, in parallel (`:126-127`, operator 2026-10-04). The ESS file declares no
  limit per case and no Case-side relation.
- Commission -> Run, one-to-many, the commission owns its runs: `:155-159`, relation `runs`.
- Commission -> principal and authority context: fields of Commission-local types (`:139-142`),
  not relations. Binding them to Mandate is phase-7 work (`:128-129`).

## Scope

- `generated/rust/commission/` (new, generated)
- `.gitignore` (`generated/rust/commission/.ess-output/`)
- `crates/commission/Cargo.toml`, `crates/commission/src/lib.rs`
- `crates/commission/tests/generated_model.rs` (new)
- `Taskfile.yml` (tasks `generate`, `drift`, `no-hand-model`; the `check` list; the fmt step)
- `Cargo.lock`
- `ess/SKIPPED.md` (new)
- `ess/domains/responsibility.yaml` (only if the model needs a change; chain surface)

## Acceptance

`task check` meets each of these expectations:

1. It passes on the story's tree.
2. With one byte changed in `generated/rust/commission/src/responsibility.rs`, its `drift` step
   fails and names that file.
3. With `pub struct AgentId(pub String);` added back to `crates/commission/src/lib.rs`, its
   `no-hand-model` step fails.
4. Its test `generated_model_reexport` (`crates/commission/tests/generated_model.rs`) builds a
   `Commission` through `b10x-commission`'s re-export of the generated crate. The commission
   carries an `AgentRevisionId`, a `CaseId`, a `PrincipalId` and an `AuthorityContext`.
5. Its test `skipped_file_starts_empty` (`crates/commission/tests/generated_model.rs`) reads
   `ess/SKIPPED.md`: the file exists, opens with the header the Outcome describes, and lists no
   skipped scenario.

## Notes

- Canon: no change. The generated `CaseId` (`ess/domains/responsibility.yaml:26-28`) and Canon's
  `CaseId` (canon `crates/canon/src/lib.rs:19`, pin `cf29c4b` in `Cargo.lock`) are distinct
  types. Where Commission hands a case id to a Canon value, it converts at that boundary.
- If `synthesize` refuses a construct, file it on beyond10x/ess with the refusal lines. Until a
  fix ships, hand-written code is allowed only with the compile-comparison test the
  `ess:specifying` skill prescribes.

## Source

TASKBOARD M-001 (build pack `TASKBOARD.md` § Commission); `epic:commission-core` § Rule; Atlas
ADR 0070; `docs/design/commission-design.md`; operator decisions of 2026-10-04 (the cleared
decision-blockers above).
