---
format: aep.planning-md/3
id: story:ess-hard-gate
kind: story
status: implemented
title: Commission's ESS specification is a hard gate in task check, and the frontier holds Commission-owned value types
summary: 'Validate, compile and synthesize with 0 refusals and no UNMAPPED: under ess/, as test ess_gate wired as task ess-gate; frontier claims, obligations and actions become Commission value types.'
refs:
- provider: atlas
  reference: adr:0076
relations:
- decomposes: epic:commission-core
- depends_on: story:generated-responsibility-model
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/commission/tests/ess_gate.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T01:17:40Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-04T01:17:40Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-04T01:40:54Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1,"review_outcome":4,"verification":1}}}
---
## Outcome

Commission's ESS specification is a hard gate in `task check` (operator rule of 2026-10-04, Atlas
ADR 0076). The specification under `ess/` must pass four steps, and any failure fails `check`:

1. `ess specify validate --path ess --strict-requires` exits 0;
2. `ess specify compile --path ess --format json` exits 0;
3. `ess verify conform synthesize --path ess --out <temporary dir>/suite.json` exits 0 and reports
   0 refusals;
4. no file under `ess/` contains the string `UNMAPPED:`, so the specification carries no open
   question.

The story has four parts.

- **(a) The header comment stops carrying the marker.** The header of
  `ess/domains/responsibility.yaml` (`:1-4`) says "Every relation the sources do not settle is
  marked UNMAPPED: and stays out of the model". Reword it so the string `UNMAPPED:` no longer
  appears. Under the gate an unsettled relation stays out of `ess/` and its question is recorded in
  the planning store as a `decision-blocker`, not as a marker.
- **(b) The frontier's contents are Commission-owned value types.** Today the Frontier entity
  carries only `case_id` and `case_revision`, under the marker at `:183-184` ("claims, obligations
  and actions inside a frontier are Canon values; … decided in story M-002"). This story settles it:
  they are value types Commission declares itself, following `docs/contracts/frontier.md:7-53`
  (claims with a three-valued value, obligations that are open or not, actions with a status, the
  capability an approval needs and the reasons a block names) and TASKBOARD M-002. The marker is
  deleted and the Frontier comment says the contents are Commission's value types. This replaces
  the Canon-valued frontier the plan assumed before: Canon's wave-1 change deletes b10x-canon's
  bootstrap `Frontier`, `ActionId`, `ActionStatus` and `ActionCandidate`, so no Commission story
  builds on them.
- **(c) The gate is a Rust integration test.** A new test file,
  `crates/commission/tests/ess_gate.rs`, runs the four steps by invoking `ess` and fails on the
  first that does not hold, naming the step and quoting its output. The `UNMAPPED:` scan names
  every file and line it finds. The scan is a function over a directory, so the test can point it
  at a temporary copy of `ess/`. A new task, `ess-gate`, runs it
  (`cargo test -p b10x-commission --test ess_gate`), and `check` lists it as its own step. CI
  already installs ess 0.52.0 (`.github/workflows/check.yml:49-63`), which `ess/ess-inputs.yaml`
  requires, so CI needs no change.
- **(d) AGENTS.md states the gate.** `AGENTS.md` § ESS states that the specification is a hard
  gate in `task check` (ADR 0076), names the four steps and the `ess-gate` task, and says that an
  open question goes to the planning store as a `decision-blocker`, never into `ess/` as a marker.

## Shared surface

`ess/domains/responsibility.yaml` and `generated/rust/commission/` are one surface for the twelve
stories of `epic:commission-core`. Every story that edits `ess/` regenerates that one tree, so the
twelve run as one chain, each depending on the previous one:
`story:generated-responsibility-model` → `story:ess-hard-gate` → `story:frontier-admission` →
`story:governor-port` → `story:agent-executor-port` → `story:authority-provider-port` →
`story:observation-evidence-ports` → `story:stale-revision-action-request` →
`story:run-outcomes` → `story:local-runtime-loop` → `story:commission-ess-conformance` →
`story:adapter-conformance-suites`.

This story is link 2. It depends on `story:generated-responsibility-model`, and
`story:frontier-admission` depends on it. This list supersedes the eleven-link list in
`story:generated-responsibility-model` § Shared surface, which was written before this story and
is not edited while that story is in flight.

From this link on, every later link keeps the gate green: a story that edits `ess/` passes
`task ess-gate` on its own tree, and a question it cannot settle becomes a `decision-blocker`.
`Taskfile.yml` (one task and one line in `check`) and `AGENTS.md` are edited along the same chain.

## ESS

Change `ess/domains/responsibility.yaml` before any code, then pass the four steps and regenerate
with `task generate`:

- Declare an enum `commission.responsibility.ActionStatus` with the variants `Admissible`,
  `ApprovalRequired` and `Blocked`.
- Declare three structs:
  - `commission.responsibility.FrontierClaim { claim: String, value: commission.responsibility.Truth }`;
  - `commission.responsibility.FrontierObligation { obligation: String, open: Boolean }`;
  - `commission.responsibility.FrontierAction { action: String, status: commission.responsibility.ActionStatus, capability: Optional<String>, reasons: List<String> }`.
- Give `commission.responsibility.Frontier` (`:185-203`) three fields: `claims`
  (`List<commission.responsibility.FrontierClaim>`), `obligations`
  (`List<commission.responsibility.FrontierObligation>`) and `actions`
  (`List<commission.responsibility.FrontierAction>`).
- Delete the marker at `:183-184`, and reword the header at `:1-4` as in (a).

`FrontierClaim.value` reads the domain's own `Truth` enum (`:70-73`), so `Truth` stays.

A probe on 2026-10-04 with ess 0.52.0 applied exactly these declarations to a copy of the wave-1
specification (`format: ess/20`) and to the current one (`format: ess/15`). On both,
`validate --strict-requires` was valid, `compile` exited 0, `ess verify conform synthesize`
reported 0 scenarios and 0 refusals, and `ess generate synthesize --target rust --layout crate`
generated the four types (35 capabilities generated, 0 refused). These are types and fields, not
commands, so the change adds no conformance scenario.

## Domain relations

- Frontier -> Case, many-to-one, references, carrying `case_revision`:
  `ess/domains/responsibility.yaml:195-199`, `commission.responsibility.Frontier` relation `case`.
  Unchanged.
- Frontier -> claims, obligations and actions: fields of Commission-owned value types (`List` of
  `FrontierClaim`, `FrontierObligation`, `FrontierAction`), not entities and not relations. A claim,
  an obligation or an action has no identity or lifecycle of its own; it exists only inside the
  frontier issued for one case revision.

## Scope

- `ess/domains/responsibility.yaml` (chain surface)
- `generated/rust/commission/` (regenerated; chain surface)
- `crates/commission/tests/ess_gate.rs` (new)
- `Taskfile.yml` (task `ess-gate`; one line in `check`)
- `AGENTS.md` (§ ESS)

## Acceptance

The test `ess_gate` in `crates/commission/tests/ess_gate.rs` passes, and `task check` runs it
through the task `ess-gate`. It checks these expectations:

1. `ess specify validate --path ess --strict-requires` exits 0.
2. `ess specify compile --path ess --format json` exits 0, and its output parses as JSON.
3. `ess verify conform synthesize --path ess --out <temporary dir>/suite.json` exits 0 and reports
   0 refusals.
4. No file under `ess/` contains `UNMAPPED:`, the header comment of
   `ess/domains/responsibility.yaml` included.
5. The scan is run over a temporary copy of `ess/` to which one line, `# UNMAPPED: probe`, has
   been added in `domains/responsibility.yaml`. The gate fails and names that file and line.
6. In the compiled model from expectation 2, `entities["commission.responsibility.Frontier"]` has
   the fields `claims`, `obligations` and `actions`, each a list of
   `commission.responsibility.FrontierClaim`, `commission.responsibility.FrontierObligation` and
   `commission.responsibility.FrontierAction`. `types["commission.responsibility.ActionStatus"]`
   is an enum with exactly the variants `Admissible`, `ApprovalRequired` and `Blocked`.
7. A `Frontier` is built through `b10x-commission`'s re-export of the generated crate, holding one
   `FrontierClaim`, one `FrontierObligation` and one `FrontierAction` whose `status` is
   `ActionStatus::ApprovalRequired` and whose `capability` is `Some`.
8. The test reads `Taskfile.yml`: the `check` task lists `ess-gate` as a step. It reads `AGENTS.md`:
   § ESS names ADR 0076.

## Notes

- **Canon:** no change, and no Canon type is used by this story. The Cargo.lock pin `cf29c4b` is not
  moved. The bootstrap signatures that still name `b10x_canon::Frontier` and `ActionId`
  (`crates/commission/src/lib.rs`) are replaced by `story:governor-port` and
  `story:agent-executor-port`; the latter removes the `b10x-canon` dependency.
- **The gate needs `ess` on `PATH`.** `task spec` already does. The test fails, naming the missing
  binary, when `ess` cannot be run; it does not skip.
- **The gate does not replace `task spec`.** `spec` stays as it is; the gate is the stricter check.
- **Conformance is a later link.** Running the synthesized suite against `b10x-commission` is
  `story:commission-ess-conformance`. This story only requires that synthesis succeeds with no
  refusal.

## Source

Atlas ADR 0076 (operator rule of 2026-10-04: the ESS specification is a hard gate);
`docs/contracts/frontier.md`; TASKBOARD M-002 (build pack `TASKBOARD.md` § Commission);
`epic:commission-core` § Rule.
