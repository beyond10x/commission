---
format: aep.planning-md/3
id: decision-blocker:aep-governed-case
kind: decision-blocker
status: cleared
title: Nobody has decided which AEP record a Commission case names, or which AEP surface the adapter calls
refs:
- provider: taskboard
  reference: M-011
relations:
- blocks: epic:governor-adapter
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-04T13:22:00Z", actor: "human:timo", revision: 3}
---
## Question

When AEP is the governor, which AEP record does a Commission `CaseId` name, and through which AEP
surface does the adapter read that case’s revision and frontier inputs, submit evidence and request
a move: an existing planning artifact over the `aep-client` service wire, a new governed-change
entity built in `beyond10x/aep`, or the AEP crates in process?

## Relation

Commission `Case` -> AEP governed case (an instance of ELS `software.change/1`).

- Settled on Commission’s side: the id is opaque and issued by the governor, and the governor
  holds the case’s truth (comments on `CaseId` and `Case` in `ess/domains/responsibility.yaml`;
  Atlas ADR 0069: AEP “governs concrete engineering cases and maintains the durable engineering
  record”). These are comments and an ADR, not an ess/1 `relations:` entry.
- Not settled: the far entity. No ess/1 document declares an AEP governed case or change (AEP has
  no `ess/` domain), and AEP code has none: its store holds planning artifacts. The ELS design
  (`els/docs/design/engineering-lifecycle-specification-design.md` § 20.2, § 36) maps an ELS
  Change to a “durable governed work instance” in AEP, and that design is a proposal.
- Follows from the answer: cardinality (one `CaseId` to one AEP record), and how `Case.revision`
  maps to an AEP revision. AEP’s service contract already refuses a command whose asserted revision
  is not current (`aep/crates/plan/aep-contract/src/command.rs:133-135`, `expected_revision`;
  `error.rs:22`, `RevisionConflict`) — inferred, code only.

## What it stops

Every part of TASKBOARD M-011 (build pack `ROADMAP.md` Phase 6): the governor adapter, frontier
mapping, evidence submission, the authority bridge, stale-revision rejection and durable
suspend/resume. Each reads or writes the AEP case, so none was drafted as a story.

## Outside this store

- The ELS `software.change/1` fixture case the epic’s acceptance names does not exist yet:
  `els/crates/els/src/lib.rs` holds only the protocol id (TASKBOARD E-002).
- The AEP side of the governor role is owned by `beyond10x/aep` (Atlas `epic:ga-aep-governor`,
  Owners). No AEP planning artifact plans it yet.

## Answer (2026-10-04)

Overtaken: AEP is not the governor. The operator chose a governor of its own (Atlas ADR 0089, "The
governor is its own component", amending ADRs 0069, 0077 and 0079): beyond10x/governor implements
Commission's `Governor` and `EvidencePort` over Canon, and a Commission `CaseId` names a case that
governor opens and holds in its case store. There is no AEP adapter, so no AEP surface to choose.
