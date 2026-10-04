---
format: aep.planning-md/3
id: story:port-skeleton
kind: story
status: implemented
title: 'Spec-first skeleton: the port vocabulary in ESS, generated once, with empty module and fake files'
summary: Declares the settled governor, executor, authority, observation/evidence and run-outcome types in ESS, regenerates once, removes the bootstrap contracts and Canon, and creates the empty port, module, fake and kit files with their mod lines.
relations:
- decomposes: epic:commission-core
- depends_on: story:ess-hard-gate
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: crates/commission-testkit/Cargo.toml
- confidence: cited
  path: crates/commission-testkit/src/fake_authority.rs
- confidence: cited
  path: crates/commission-testkit/src/fake_executor.rs
- confidence: cited
  path: crates/commission-testkit/src/fake_governor.rs
- confidence: cited
  path: crates/commission-testkit/src/kits/authority.rs
- confidence: cited
  path: crates/commission-testkit/src/kits/governor.rs
- confidence: cited
  path: crates/commission-testkit/src/kits/mod.rs
- confidence: cited
  path: crates/commission-testkit/src/lib.rs
- confidence: cited
  path: crates/commission-testkit/tests/skeleton.rs
- confidence: cited
  path: crates/commission-xtask/
- confidence: cited
  path: crates/commission/Cargo.toml
- confidence: cited
  path: crates/commission/src/action_request.rs
- confidence: cited
  path: crates/commission/src/admission.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: crates/commission/src/outcome.rs
- confidence: cited
  path: crates/commission/src/ports/authority.rs
- confidence: cited
  path: crates/commission/src/ports/evidence.rs
- confidence: cited
  path: crates/commission/src/ports/executor.rs
- confidence: cited
  path: crates/commission/src/ports/governor.rs
- confidence: cited
  path: crates/commission/src/ports/mod.rs
- confidence: cited
  path: crates/commission/src/runtime.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T02:07:09Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-04T02:18:12Z", actor: "human:timo", revision: 5}
- {from: "active", to: "implemented", at: "2026-10-04T02:45:21Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1,"review_outcome":2,"verification":1}}}
---
## Outcome

One story lands, up front, every shared-file edit the port stories of `epic:commission-core` used
to make one after another. After it, each port story owns only its own module file, its own fake,
its own test and its own fixtures, and the port stories run side by side (§ Shared surface).

The story has four parts.

- **(a) The port vocabulary is declared in ESS, once.** Every declaration in § ESS goes into
  `ess/domains/responsibility.yaml`, `task ess-gate` stays green, and `task generate` regenerates
  `generated/rust/commission/` once. Each declaration is one a later story's body already settles;
  this story only moves it earlier. A declaration nobody has settled stays with its story (§ Not
  here).
- **(b) The bootstrap contracts leave the crate root.** `crates/commission/src/lib.rs` keeps the
  `model` re-export and loses everything else the bootstrap wrote by hand: the traits `Governor`,
  `AgentExecutor` and `AuthorityProvider` and the enums `ExecutorOutcome`, `RunOutcome` and
  `AuthorityDecision` (`lib.rs:13`, `:20-70`). Each port story writes its trait in its own module
  over the generated types. With them go the `use b10x_canon` line, the `b10x-canon` dependency in
  `crates/commission/Cargo.toml`, its `Cargo.lock` entries, and the `AGENTS.md` § Work bullet about
  the Canon pin (this was `story:agent-executor-port`'s; it moves here because it edits `lib.rs`,
  `Cargo.lock` and `AGENTS.md`). The two `PENDING_REPLACEMENT` entries in
  `crates/commission-xtask/src/main.rs:39-42`, which allow the hand-written `ExecutorOutcome` and
  `AuthorityDecision` until the port stories replace them, are removed with the definitions they
  allow, and the xtask tests that name them are brought along.
- **(c) Empty module files and their `mod` lines.** `lib.rs` declares `pub mod ports;`,
  `pub mod admission;`, `pub mod action_request;`, `pub mod outcome;` and `pub mod runtime;`.
  `crates/commission/src/ports/mod.rs` declares `governor`, `executor`, `authority` and `evidence`.
  Each of those nine files exists and holds only a module doc comment naming the story that fills
  it.
- **(d) The testkit crate, empty.** `crates/commission-testkit/` (package
  `b10x-commission-testkit`, depending on `b10x-commission`; `b10x-commission` does not depend on
  it) with `src/lib.rs` declaring `pub mod fake_governor;`, `pub mod fake_executor;`,
  `pub mod fake_authority;` and `pub mod kits;`, and `src/kits/mod.rs` declaring `governor` and
  `authority`. Each fake and kit file holds only a module doc comment naming the story that fills
  it. The workspace's `members = ["crates/*"]` takes the crate in without an edit; `Cargo.lock`
  gains it.

## Shared surface

Before this story, the twelve stories of `epic:commission-core` were one chain over
`ess/domains/responsibility.yaml`, `generated/rust/commission/` and `crates/commission/src/lib.rs`
(`story:ess-hard-gate` § Shared surface). This section supersedes that chain for every story not
yet implemented.

This story depends on `story:ess-hard-gate`. After it, the waves are:

| wave | stories | why they are apart from the next |
|---|---|---|
| this story | `story:port-skeleton` | every story below fills a file it creates |
| next | `story:frontier-admission`, `story:governor-port`, `story:agent-executor-port`, `story:authority-provider-port` | no behaviour of one is used by another |
| then | `story:observation-evidence-ports`, `story:stale-revision-action-request`, `story:run-outcomes` | each uses a fake or a check from the wave before |
| last | `story:local-runtime-loop`, `story:commission-ess-conformance`, `story:adapter-conformance-suites` | each drives behaviour from the wave before |

Who still edits a shared file after this story, and why it stays safe:

- `ess/domains/responsibility.yaml` and `generated/rust/commission/`: `story:frontier-admission`
  (admission result; the contract-drift settlement) and `story:stale-revision-action-request`
  (`ActionRequest` and its command), each the only story in its wave that does;
  `story:commission-ess-conformance` only if a scenario shows a specification defect.
- `Taskfile.yml`: `story:agent-executor-port` (`deps-guard`) and `story:commission-ess-conformance`
  (`conform`), in different waves.
- `Cargo.lock`: `story:commission-ess-conformance` (the `ess-conformance` dependency).
- `crates/commission-xtask/`: `story:run-outcomes` (`no-hand-model` refuses `RunOutcome` and enums).
- `crates/commission-testkit/src/fake_governor.rs`: `story:governor-port`, then
  `story:observation-evidence-ports`, which extends that fake and depends on it.

## ESS first

Atlas ADR 0080: the first commit changes only the specification, a named test is red on it, and
later commits make it pass.

- **First commit:** `ess/domains/responsibility.yaml` only, with every declaration below; `task
  ess-gate` (validate `--strict-requires`, compile, synthesize with 0 refusals, no `UNMAPPED:`)
  passes on it.
- **Red on that commit:** `drift_passes_on_the_committed_tree`
  (`crates/commission-xtask/tests/checks.rs`, and `task drift`) fails, because the committed
  `generated/rust/commission/` no longer matches a fresh synthesis of `ess/`.
- **Then:** `task generate`, the Rust changes of parts (b) to (d), and the test
  `skeleton_lands_port_vocabulary_and_modules`.

The declarations, each naming the story whose body settles it:

- **Governor** (`story:governor-port` § ESS): an enum `commission.responsibility.GovernorError` with
  the variants `UnknownCase` and `GovernorUnavailable`; a union
  `commission.responsibility.CompletionDetermination` with `Open` and `Complete { outcome: String }`
  (the governor's outcome identifier as the governor reports it, `story:run-outcomes` § Notes).
- **Executor** (`story:agent-executor-port` § ESS, `docs/contracts/commission-executor.md:29-47`):
  - `commission.responsibility.ProposedActionArguments`, a `newtype` of `Json`;
  - `commission.responsibility.HumanDecisionRequest`, carried as `String` or `Json`;
  - `commission.responsibility.SuspensionReason`, a `union` with the variants `Authority`, `Human`,
    `Evidence`, `Time`, `Dependency`, `Budget` and `ExternalAvailability`
    (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1477-1485`). `Human` carries
    `HumanDecisionRequest`, `Dependency` carries `List<commission.responsibility.CaseId>`; a payload
    whose type Commission's sources declare nowhere is carried as `String` or `Json`, and no entity
    is invented for it;
  - `commission.responsibility.ExecutorOutcome` (`:66-69`) turns from an enum into a `union`:
    `ProposedAction { action: String, arguments: ProposedActionArguments }`,
    `NeedsHumanJudgment { request: HumanDecisionRequest }`, `Suspended { reason: SuspensionReason }`,
    `NoUsefulAction` and `CompletedLocalReasoning`. The action is a `String`, the type of a
    `FrontierAction`'s `action`; identity, authority and case revision are not part of it
    (`AGENTS.md` § Rules).
- **Authority** (`story:authority-provider-port` § ESS): a union
  `commission.responsibility.AuthorityVerdict` with `Allow`, `Deny { reason: String }` and
  `ApprovalRequired { request: String }`, the bootstrap shape (`lib.rs:65-70`) on a name distinct
  from the `AuthorityDecision` entity. The `AuthorityContext` comment (`:60-61`) states the decision
  in place of the deferral: the context is opaque to Commission, passed to the provider unchanged
  and read by nobody else.
- **Observation and evidence** (`story:observation-evidence-ports` § ESS,
  `docs/contracts/evidence.md:7-32`): `Observation` gains `observed_at: Timestamp` and
  `payload: Json`; `Evidence` gains `facts: Json` and `provenance: Json`. The `observations`
  relation stays exactly as declared, and no relation from `Observation` is added.
- **Run outcome and suspension** (`story:run-outcomes` § ESS): a union
  `commission.responsibility.RunOutcome` with `Completed { outcome: String }`,
  `Suspended { reason: SuspensionReason }`, `NeedsAuthority { request: String }`,
  `NeedsHumanJudgment { request: HumanDecisionRequest }`,
  `NeedsExternalEvidence { requirements: List<String> }` and `NoAdmissibleAction`. `Run` gains the
  state `Suspended`; neither `Running` nor `Suspended` is terminal; two transitions, suspend
  (`Running` → `Suspended`) and resume (`Suspended` → `Running`), each with its command, the suspend
  command carrying a `SuspensionReason`. The note at `:210-211` ("the Suspended state arrives with
  its command outcome in story M-007") is replaced by the state itself.

The two Run commands are the domain's first commands, so `ess verify conform synthesize` gains
scenarios. They are answered by `story:commission-ess-conformance` (through the behaviour of
`story:run-outcomes`), or named in `ess/SKIPPED.md` there.

### Coordinator decisions (wave 2026-10-04-w3)

Phase 1 found that ESS 0.52.0 cannot hold some declarations above as written (a union variant must
carry exactly one type; there is no unit type and an empty struct is refused), and that the two
`Run` commands leave synthesis with 6 refusals because nothing creates a `Run`. These decisions
replace the conflicting parts above; where they differ, this section wins.

1. **Variants without payload.** A type whose variants all lack a payload is an `enum`. In a union
   that mixes both, a payload-less variant carries `commission.responsibility.Unit`, a `newtype` of
   `Boolean` whose value is always `true`, declared once. This is a stand-in until ESS supports unit
   variants (filed on beyond10x/ess); it is removed then.
2. **Variants with named fields** carry one struct each, named `<Union><Variant>`, holding exactly
   the fields named above: `CompletionDeterminationComplete { outcome: String }`,
   `ExecutorOutcomeProposedAction { action: String, arguments: ProposedActionArguments }`,
   `ExecutorOutcomeNeedsHumanJudgment { request: HumanDecisionRequest }`,
   `ExecutorOutcomeSuspended { reason: SuspensionReason }`,
   `AuthorityVerdictDeny { reason: String }`, `AuthorityVerdictApprovalRequired { request: String }`,
   and for `RunOutcome` the six variant structs with the fields listed above.
3. **Undeclared payloads.** `HumanDecisionRequest` is a `newtype` of `Json`. In `SuspensionReason`,
   `Authority`, `Time`, `Budget` and `ExternalAvailability` carry `Json`; `Evidence` carries
   `List<String>`, the same type as `RunOutcome`'s `NeedsExternalEvidence.requirements`; `Human`
   and `Dependency` stay as declared above.
4. **No `Run` commands here.** The `Suspended` state, the suspend and resume transitions and their
   commands move to `story:run-outcomes`, which owns that behaviour and now edits `ess/` for it. This
   story declares no command, so synthesis stays at 0 scenarios and 0 refusals. `RunOutcome` itself
   stays here.

Acceptance item 1 reads "unions with exactly the variants" against these shapes. Acceptance item 2's `Run` states and transitions (`Suspended`, suspend, resume) move to `story:run-outcomes` with decision 4; this story's test does not check them. The union tag is `kind`; `Unit` carries no invariant (synthesis refuses one no view publishes, `ESS-SYNTH-013`).

### Not here

- **The admission result** stays with `story:frontier-admission`. Its refusal carries a
  `FrontierAction`'s reasons, and whether those reasons stay `List<String>` is one of the contract
  differences that story settles first (§ Contract drift to settle first there).
- **`ActionRequest`, its revalidation command and `AuthorityDecision.action_request_id`** stay with
  `story:stale-revision-action-request`. The entity's fields and relations are settled
  (`decision-blocker:action-request-commission`, `decision-blocker:authority-decision-owner`), but
  the command's input, response and outcomes and the request's lifecycle states are not written
  anywhere; that story is the only one in its wave that edits `ess/`, so keeping it there costs no
  width.
- **Typed evidence-port errors** and anything else a port story's body does not put in `ess/` stay
  Rust in that story.

## Domain relations

No relation is added or changed. `Run` keeps `commission_id` and the `runs` relation from
`Commission` (`:192-197`); a suspended run stays the same Run across resume (operator decision of
2026-10-04, `decision-blocker:suspended-run-continuity`, cleared).

## Scope

- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (regenerated once)
- `crates/commission/src/lib.rs` (bootstrap items out; `mod` lines in)
- `crates/commission/src/ports/mod.rs`, `ports/governor.rs`, `ports/executor.rs`,
  `ports/authority.rs`, `ports/evidence.rs` (new, empty)
- `crates/commission/src/admission.rs`, `action_request.rs`, `outcome.rs`, `runtime.rs` (new, empty)
- `crates/commission/Cargo.toml`, `Cargo.lock` (`b10x-canon` out; the testkit crate in)
- `AGENTS.md` (§ Work: the Canon pin bullet removed)
- `crates/commission-xtask/` (`PENDING_REPLACEMENT` emptied, its tests brought along)
- `crates/commission-testkit/Cargo.toml`, `src/lib.rs`, `src/fake_governor.rs`,
  `src/fake_executor.rs`, `src/fake_authority.rs`, `src/kits/mod.rs`, `src/kits/governor.rs`,
  `src/kits/authority.rs` (new, empty)
- `crates/commission-testkit/tests/skeleton.rs` (new)

## Acceptance

The test `skeleton_lands_port_vocabulary_and_modules` in
`crates/commission-testkit/tests/skeleton.rs` passes, `task ess-gate` passes, and `task check`
passes. The test checks these expectations:

1. In the model from `ess specify compile --path ess --format json`:
   `types["commission.responsibility.GovernorError"]` is an enum with exactly `UnknownCase` and
   `GovernorUnavailable`; `CompletionDetermination`, `ExecutorOutcome`, `SuspensionReason`,
   `AuthorityVerdict` and `RunOutcome` are unions with exactly the variants § ESS names;
   `ProposedActionArguments` is a newtype of `Json`.
2. In the same model, `entities["commission.responsibility.Run"]` has the states `Running` and
   `Suspended`, neither terminal, and the transitions suspend and resume; `Observation` has the
   fields `observed_at` and `payload`; `Evidence` has `facts` and `provenance`, and its
   `observations` relation is unchanged.
3. The test names each generated type in item 1 through `b10x-commission`'s re-export, and
   `std::any::type_name` of each begins with `commission::`.
4. The test compiles `use b10x_commission::ports::{governor, executor, authority, evidence};`,
   `use b10x_commission::{admission, action_request, outcome, runtime};` and
   `use b10x_commission_testkit::{fake_governor, fake_executor, fake_authority, kits};`.
5. The real `cargo tree -p b10x-commission -e normal --prefix none` names no `b10x-canon`.
6. `crates/commission/src/lib.rs` defines no `trait` and no `enum`.

## Notes

- **Between this story and the port stories the crate has no port traits.** The bootstrap ones are
  deleted here, not moved, because moving `Governor` and `AgentExecutor` would keep `b10x_canon`
  imports in two modules owned by two parallel stories, and removing the Canon dependency would then
  tie `story:agent-executor-port` to `story:governor-port`. Loom names
  `b10x_commission::{AgentExecutor, Commission, ExecutorOutcome}` (`loom/crates/loom/src/lib.rs:6`)
  through a `branch = "main"` dependency pinned by Loom's `Cargo.lock`; it already names a
  `Commission` that `lib.rs` no longer exports, so moving that pin is a Loom story either way.
- **Canon:** no change to Canon. Commission stops depending on it here.

## Source

`story:governor-port`, `story:agent-executor-port`, `story:authority-provider-port`,
`story:observation-evidence-ports` and `story:run-outcomes` § ESS (the declarations, moved);
`docs/contracts/commission-executor.md`, `docs/contracts/evidence.md`; Atlas ADRs 0076, 0080; re-plan for
wider waves (operator instruction of 2026-10-04).


