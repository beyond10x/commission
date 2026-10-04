---
format: aep.planning-md/3
id: story:adapter-conformance-suites
kind: story
status: implemented
title: Governor and AuthorityProvider conformance suites an adapter crate can run
summary: Public test kits for the Governor and AuthorityProvider ports, green on Commission's fakes and red on broken ones.
refs:
- provider: taskboard
  reference: M-010
relations:
- decomposes: epic:commission-core
- depends_on: story:authority-provider-port
- depends_on: story:observation-evidence-ports
- depends_on: story:stale-revision-action-request
- serves: vision:O1
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:port-skeleton
- depends_on: story:governor-port
scope:
- confidence: cited
  path: crates/commission-testkit/src/kits/authority.rs
- confidence: cited
  path: crates/commission-testkit/src/kits/governor.rs
- confidence: inferred
  path: crates/commission-testkit/tests/kits.rs
revision: 12
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:13:54Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "proposed", to: "active", at: "2026-10-04T04:26:36Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "active", to: "implemented", at: "2026-10-04T05:42:47Z", actor: "human:timo", revision: 12, decided_on: {"recorded":{"test_result":1,"review_outcome":5,"verification":1}}}
---
## Outcome

Conformance suites that an adapter crate outside Commission runs against its own implementation of
a port. Each suite is a public test kit: the adapter supplies a factory for its implementation and
the kit runs every check, naming the check that fails. The kits live in the testkit crate under
`crates/commission-testkit/src/kits/`; `story:port-skeleton` creates `kits/mod.rs`, `governor.rs`
and `authority.rs` empty with their `mod` lines, and this story fills `governor.rs` and
`authority.rs`. An adapter takes them as a dev-dependency on `b10x-commission-testkit`. Two
suites:

- **Governor** (`kits/governor.rs`). It returns the frontier for the case's current revision. It
  reports a revision change so that Commission's revalidation refuses a request made at the
  superseded revision. It answers an unknown case with the typed error. It receives an executor's
  output as an observation, never as evidence, and evidence arrives with the observation ids it was
  submitted with.
- **AuthorityProvider** (`kits/authority.rs`). Allow, deny and approval-required come back as
  given. The kit hands the factory a backing service it can make fail; when that backing call fails
  the provider must answer deny or an error, which Commission reads as a refusal, never allow.

`epic:governor-adapter` (TASKBOARD M-011) holds the AEP adapter to the governor suite. Both suites
run against Commission's own fakes (`fake_governor.rs`, `fake_authority.rs`) in `task check`
through `cargo test --workspace`.

Not in this story:

- A suspend/resume-across-restart check. What resuming means is now decided (the same run
  continues; `decision-blocker:suspended-run-continuity`, cleared), and run state is Commission's,
  not the governor's. Where an AEP-governed suspension is kept across a restart is still open
  (`decision-blocker:suspension-durable-record`), and `epic:governor-adapter` owns that check.
- An AgentExecutor suite. No source names a property an executor must hold that Commission does
  not already enforce at revalidation.
- An evidence-adapter conformance kit for the `EvidenceAdapter` trait from
  `story:observation-evidence-ports` (history design § 52,
  `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1944`). It is deferred: before
  phase 6 the only evidence adapter is the fake, so a kit would have nothing but the fake to hold.
  It is taken up with the AEP evidence submission in `epic:governor-adapter`.

## Shared surface

The wave plan is in `story:port-skeleton` § Shared surface, which supersedes the chain in
`story:ess-hard-gate` § Shared surface. This story depends on these, each a real dependency:

- `story:port-skeleton`, for the empty kit files and their `mod` lines;
- `story:governor-port` and `story:authority-provider-port`, whose fakes the kits must pass;
- `story:observation-evidence-ports`, whose observation and evidence ports the governor kit checks;
- `story:stale-revision-action-request`, whose revalidation the superseded-revision check drives.

Its old edge on `story:commission-ess-conformance` was ordering only and is gone; it runs beside
that story and `story:local-runtime-loop`. It no longer edits `crates/commission-testkit/src/lib.rs`
or `kits/mod.rs`.

## ESS first

- **Specification change: none in this story.** No new noun and no command. The kits exercise the
  ports through types `story:port-skeleton` declares (`GovernorError`, `CompletionDetermination`,
  `AuthorityVerdict`, `Observation`, `Evidence`).
- **First commit, red.** The test `kits_hold_fakes_and_catch_broken_ones` alone, in
  `crates/commission-testkit/tests/kits.rs`. It is red because `kits::governor` and
  `kits::authority` hold no suite: the test does not compile.
- **Then.** The two suites, which make it pass.

## Domain relations

None beyond those its dependencies cite.

## Scope

- `crates/commission-testkit/src/kits/governor.rs` (created empty by `story:port-skeleton`; filled
  here)
- `crates/commission-testkit/src/kits/authority.rs` (created empty by `story:port-skeleton`; filled
  here)
- `crates/commission-testkit/tests/kits.rs` (new)

## Acceptance

The test `kits_hold_fakes_and_catch_broken_ones` in `crates/commission-testkit/tests/kits.rs`, a
crate other than `b10x-commission`, passes with these expectations:

1. The governor kit passes against `fake_governor.rs`.
2. The authority kit passes against `fake_authority.rs`.
3. The governor kit fails, naming its superseded-revision check, against a broken governor defined
   in the test: it wraps the scripted fake and, after the script moves the case from N to N+1,
   keeps returning revision N and the frontier for N.
4. The authority kit fails, naming its backing-failure check, against a broken provider defined in
   the test: when its backing call returns an error it returns `Allow`.

## Notes

- Canon: no change.

## Source

TASKBOARD M-010 (build pack `TASKBOARD.md` § Commission); `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md`
§ 52; `epic:governor-adapter` Acceptance; Atlas ADR 0080.

### Adversary decisions (wave 2026-10-04-w7)

- Pass 1, all fixed: current-frontier reads the frontier before and after the case moves;
  observation-not-evidence sends 2 observations and makes a further call before reading evidence;
  evidence-observation-ids observes 3 and names 2; each check runs under `catch_unwind`, so a
  panicking adapter fails only that check; each backing carries decoy capabilities.
- Pass 2, all fixed: observation-not-evidence also calls `frontier` and `completion` first;
  superseded-revision moves the case twice; a new check `held-case-open` requires `completion` to
  answer `Open` for the held case; after the move current-frontier requires only the new revision;
  each as-given check also asks a decoy; backing-failure switches a shared backing to failing after
  an allow.
- Pass 2 finding 6 made the phase-1 fake provider wrong (it kept a fixed table). Coordinator
  option A: `FakeAuthorityFixture` reads the live backing at each decision; no other phase-1 line
  changed.
