---
format: aep.planning-md/3
id: story:stale-revision-action-request
kind: story
status: proposed
title: Action requests are bound to a case revision and refused when stale
summary: A proposed action becomes a request at the frontier's revision, revalidated against the current revision and frontier before use.
refs:
- provider: taskboard
  reference: M-008
relations:
- decomposes: epic:commission-core
- depends_on: story:governor-port
- depends_on: story:observation-evidence-ports
- depends_on: story:agent-executor-port
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: crates/commission-testkit/tests/action_request.rs
- confidence: inferred
  path: crates/commission/src/action_request.rs
- confidence: cited
  path: crates/commission/src/lib.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 9
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"review_outcome":3}}}
---
## Outcome

A proposed action becomes an action request bound to the case revision of the frontier it was
chosen from. The request lives in a new module, `crates/commission/src/action_request.rs`, and
carries:

- the commission;
- the case id;
- the expected case revision;
- the action;
- the arguments, as `ProposedActionArguments`.

`ProposedActionArguments` is declared by `story:agent-executor-port` and only consumed here. The
expected revision always comes from the frontier the run was given, never from executor output.

Immediately before anything is done with a request, Commission revalidates it. It reads the current
case revision from the governor, then re-checks the action against the current frontier with the
admission check from `story:frontier-admission`. A request whose expected revision is not current
is refused as stale, and the refusal names both revisions. This holds even when the action would
be admissible now. A selection made on stale state is not permission (`AGENTS.md:28-30`;
`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 36, lines 1490-1512).

The test's moving case comes from the scripted fake governor
(`crates/commission-testkit/src/fake_governor.rs`), which `story:governor-port` already lets
advance a case's revision. This story needs no change to the fake.

## Shared surface

This story is link 8 of the `epic:commission-core` chain over `ess/domains/responsibility.yaml` and
`generated/rust/commission/`. It depends on `story:observation-evidence-ports`, and
`story:run-outcomes` depends on it. The whole order is in `story:ess-hard-gate`
§ Shared surface. It also depends directly on `story:agent-executor-port`, which owns
`ProposedActionArguments`.

## ESS

The action request is a new noun. Declare it in `ess/domains/responsibility.yaml` first:

- an entity `commission.responsibility.ActionRequest`, with an identity newtype and the fields
  above;
- a command that revalidates a request, with three outcomes: admitted, refused as stale (naming the
  expected and the current revision), and refused as not admitted by the frontier.

This is the domain's first command. It is the one that gives
`ess verify conform synthesize --path ess` scenarios to synthesize; a probe on 2026-10-04 with
ess 0.52.0 synthesized 0 before it. Pass `ess specify validate --path ess`, then regenerate with
`task generate`.

`story:commission-ess-conformance` is later in the chain. It answers these scenarios through its
Rust target, or names a skipped one in `ess/SKIPPED.md`, which `story:generated-responsibility-model`
creates.

## Domain relations

- Action request -> Case, many-to-one, references, via `case_id`. `ActionRequest` carries `case_id`
  in the history design § 36 (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1497-1504`).
  Declare this as a `relations:` entry.
- Action request -> Commission, many-to-one. This is inferred, not read from any source. § 36 names
  an `actor` without a type, and the executor proposes within one commission's run
  (`crates/commission/src/lib.rs:53-55`). Whether a commission owns its requests or merely
  references them is not settled. Under the ESS hard gate (`story:ess-hard-gate`, ADR 0076) no
  `UNMAPPED:` marker may be written into `ess/`, and a `commission` field without its relation is a
  foreign key nobody checks. The question is `decision-blocker:action-request-commission`, which
  blocks this story; once decided, declare the relation as a `relations:` entry.
- Run -> case revision: `commission.responsibility.Run` field `case_revision`
  (`ess/domains/responsibility.yaml:174-176`, "a proposal made on another revision is stale").

## Scope

- `crates/commission/src/action_request.rs` (new)
- `crates/commission/src/lib.rs`
- `crates/commission-testkit/tests/action_request.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (chain surface)

## Acceptance

The test `action_request_revalidation` in `crates/commission-testkit/tests/action_request.rs` passes
with these expectations:

1. A request built at revision N is revalidated after the fake governor has moved the case from N
   to N+1. It is refused as stale, and the refusal names N and N+1, although its action is
   admissible at N+1.
2. The same request built at N+1 passes revalidation.
3. A request at the current revision whose action the current frontier does not list is refused as
   not admitted, naming the action.
4. The request's arguments are of the generated type `ProposedActionArguments`.
5. `ess/domains/responsibility.yaml` declares the action-request command. The test reads the file
   and finds the command's name.
6. The test runs `ess verify conform synthesize --path ess --out <temporary dir>/suite.json`. The
   command exits 0, and the suite holds at least 1 scenario.
7. The test runs `ess specify validate --path ess`, which exits 0, and then
   `ess specify compile --path ess --format json`. In the compiled model,
   `entities["commission.responsibility.AuthorityDecision"]` has a field `action_request_id`, and its
   `relations` hold an entry with `kind` `references`, `target`
   `commission.responsibility.ActionRequest`, `cardinality` `one` and `via` `action_request_id`.

## Notes

- Canon: no change, and no Canon type is used. The frontier's revision is the generated
  `Frontier`'s `case_revision` (`ess/domains/responsibility.yaml:192-193`); the action is matched
  by the `action` string of its `FrontierAction` (`story:ess-hard-gate`).
- Expectations 6 and 7 need `ess` on `PATH`. `task spec` already does.

## Source

TASKBOARD M-008 (build pack `TASKBOARD.md` § Commission); `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md`
§ 36; Atlas `docs/design/governed-autonomy/invariants.md` § 4.

## Authority decision relation

With the ActionRequest entity this story declares the relation the operator decided on 2026-10-04
(decision-blocker:authority-decision-owner, cleared): `AuthorityDecision` gains an `action_request_id`
field and a `references` relation to exactly one `ActionRequest`; a decision never covers a later
request. `ess specify validate --path ess` must pass with it.
