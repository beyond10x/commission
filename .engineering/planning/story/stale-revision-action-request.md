---
format: aep.planning-md/3
id: story:stale-revision-action-request
kind: story
status: implemented
title: Action requests are bound to a case revision and refused when stale
summary: A proposed action becomes a request at the frontier's revision, revalidated against the current revision and frontier before use.
refs:
- provider: taskboard
  reference: M-008
relations:
- decomposes: epic:commission-core
- depends_on: story:governor-port
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
- depends_on: story:frontier-admission
scope:
- confidence: inferred
  path: crates/commission-testkit/tests/action_request.rs
- confidence: cited
  path: crates/commission/src/action_request.rs
- confidence: cited
  path: ess/domains/responsibility.yaml
- confidence: cited
  path: generated/rust/commission/
revision: 17
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "proposed", to: "active", at: "2026-10-04T03:47:39Z", actor: "human:timo", revision: 14, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "active", to: "implemented", at: "2026-10-04T04:23:37Z", actor: "human:timo", revision: 17, decided_on: {"recorded":{"test_result":1,"review_outcome":8,"verification":1}}}
---
## Outcome

A proposed action becomes an action request bound to the case revision of the frontier it was
chosen from. The request lives in the module `crates/commission/src/action_request.rs`, which
`story:port-skeleton` creates empty with its `mod` line, and carries:

- the run that proposed it, through which the commission is reached
  (`decision-blocker:action-request-commission`, cleared: `Run` owns its requests);
- the case id;
- the expected case revision;
- the action;
- the arguments, as `ProposedActionArguments`.

`ProposedActionArguments` is declared by `story:port-skeleton` and only consumed here. The
expected revision always comes from the frontier the run was given, never from executor output.

Immediately before anything is done with a request, Commission revalidates it. It reads the current
case revision from the governor, then re-checks the action against the current frontier with the
admission check from `story:frontier-admission`. A request whose expected revision is not current
is refused as stale, and the refusal names both revisions. This holds even when the action would
be admissible now. A selection made on stale state is not permission (`AGENTS.md` § Rules;
`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 36, lines 1490-1512).

The test's moving case comes from the scripted fake governor
(`crates/commission-testkit/src/fake_governor.rs`), which `story:governor-port` lets advance a
case's revision. This story needs no change to the fake.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on:

- `story:port-skeleton`, for its module file and `ProposedActionArguments`;
- `story:governor-port`, a real dependency: the `Governor` port gives the current revision, and the
  scripted fake governor moves the case;
- `story:frontier-admission`, a real dependency: revalidation reuses its admission check.

Its old edges on `story:observation-evidence-ports` (ordering only) and on
`story:agent-executor-port` (which owned `ProposedActionArguments` before `story:port-skeleton`
took the declaration) are gone.

It still edits `ess/domains/responsibility.yaml` and regenerates `generated/rust/commission/`, and
it is the only story in its wave that does: `story:observation-evidence-ports` and
`story:run-outcomes` beside it touch neither. `story:local-runtime-loop`,
`story:commission-ess-conformance` and `story:adapter-conformance-suites` depend on it.

## ESS first

Atlas ADR 0080: the first commit changes only the specification, a named test is red on it, and
later commits make it pass. `ActionRequest` stays with this story rather than with
`story:port-skeleton` because its command's input, response and outcomes and the request's
lifecycle states are written nowhere yet; this story settles them.

- **Specification change (first commit, `ess/domains/responsibility.yaml` only).**
  - an entity `commission.responsibility.ActionRequest`, with an identity newtype and the fields
    above;
  - `Run` gains the relation `requests`, kind `owns`, cardinality many, via `run_id` on
    `ActionRequest` (`decision-blocker:action-request-commission`);
  - `ActionRequest` gains the relation `case`, kind `references`, cardinality one, via `case_id`;
  - `AuthorityDecision` gains `action_request_id` and a `references` relation to exactly one
    `ActionRequest` (§ Authority decision relation);
  - a command that revalidates a request, with three outcomes: admitted, refused as stale (naming
    the expected and the current revision), and refused as not admitted by the frontier.
- **Red on that commit.** `drift_passes_on_the_committed_tree`
  (`crates/commission-xtask/tests/checks.rs`, and `task drift`) fails: the committed
  `generated/rust/commission/` no longer matches a fresh synthesis of `ess/`.
- **Then.** `task generate`, the request and its revalidation, and the test
  `action_request_revalidation`; `task ess-gate` passes throughout.

The revalidation command joins the Run's suspend and resume commands that `story:port-skeleton`
declares. `story:commission-ess-conformance` answers its scenarios through its Rust target, or names
a skipped one in `ess/SKIPPED.md`.

## Domain relations

- Action request -> Case, many-to-one, references, via `case_id`. `ActionRequest` carries `case_id`
  in the history design § 36 (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1497-1504`).
  Declared as a `relations:` entry.
- Run -> Action request, one-to-many, owns, via `run_id` (coordinator decision of 2026-10-04,
  `decision-blocker:action-request-commission`, cleared). A request is made inside one run against
  that run's case revision and never outlives its run. The commission is reached through the run;
  there is no direct `ActionRequest` -> `Commission` relation.
- Run -> case revision: `commission.responsibility.Run` field `case_revision`
  (`ess/domains/responsibility.yaml:212-214`, "a proposal made on another revision is stale").

## Scope

- `crates/commission/src/action_request.rs` (created empty by `story:port-skeleton`; filled here)
- `crates/commission-testkit/tests/action_request.rs` (new)
- `ess/domains/responsibility.yaml`, `generated/rust/commission/` (the action request and its
  command; the only story in its wave that edits them)

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
  `Frontier`'s `case_revision`; the action is matched by the `action` string of its
  `FrontierAction` (`story:ess-hard-gate`).
- Expectations 6 and 7 need `ess` on `PATH`. `task spec` already does.

## Source

TASKBOARD M-008 (build pack `TASKBOARD.md` § Commission); `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md`
§ 36; Atlas `docs/design/governed-autonomy/invariants.md` § 4; Atlas ADR 0080.

## Authority decision relation

With the ActionRequest entity this story declares the relation the operator decided on 2026-10-04
(decision-blocker:authority-decision-owner, cleared): `AuthorityDecision` gains an `action_request_id`
field and a `references` relation to exactly one `ActionRequest`; a decision never covers a later
request. `ess specify validate --path ess` must pass with it.


## From wave 2026-10-04-w4 (frontier-admission, adversary pass 2, F5)

`admission::admit` has four results: admissible, needs authority, refused, and (through the
refusal reasons) blocked. The revalidation command above lists three outcomes. It gains a
needs-authority outcome: an `ApprovalRequired` action is never revalidated as admitted without an
authority decision. Settle the outcome's shape in this story's ESS change.


## Coordinator decisions (wave 2026-10-04-w6)

- Option A: `RevalidateActionRequest` takes the whole request as input (`run_id`, `case_id`,
  `expected_case_revision`, `action`, `arguments`); its `admitted` outcome changes nothing. No
  creating command, event or view is added (synthesis: 13 scenarios, 0 refusals).
- `ActionRequestId` of `Uuid`; one state `Requested`; relations `Run.requests` (owns, many, via
  `run_id`), `ActionRequest.case` (references, one), `AuthorityDecision.action_request` (references,
  one, via `action_request_id`).
- Outcomes checked in order: stale (`ActionRequestStale`), not admitted (`ActionNotAdmitted` with
  reasons), needs authority (`ActionNeedsAuthority`), admitted. No unknown-request or
  governor-unavailable outcome.

### Adversary decisions (wave 2026-10-04-w6)

- Pass 1: the failing-governor mutant is caught by `adversary_request_governor_errors.rs` (fixed).
  Stale for a frontier issued at another revision, and `run_id` not being read, stay notes for later
  stories (no-op).
- Pass 2 F1: the case is checked before the frontier revision; a frontier for another case is
  `ActionNotAdmitted` at any revision (fixed).
- Pass 2 F2: the review found the specification wrong. `RevalidateActionRequest` input gains
  `action_request_id` (`ess/domains/responsibility.yaml:644-645`), and the refusal reason names it
  (fixed, ADR 0080 spec fix).
- Pass 2 mutant notes on `>` vs `!=` and the dropped relations are caught by the adversary2 tests
  (fixed).
- `request()` taking only a `RunId`, and `RevalidateActionRequestBehavior` left unimplemented, stay
  with story:commission-ess-conformance and later stories (no-op).
