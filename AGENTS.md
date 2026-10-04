# AGENTS.md — commission

What Commission is and how to build it is in [README.md](README.md); this file is what an agent
changing it must know. The cross-repository architecture is Atlas ADRs 0066–0075 and Atlas
`docs/design/governed-autonomy/`.

## Serves

- **O1 — governed reach.** An agent acts only on an action the current frontier admits, with
  authority checked at the call and refused by name.
- **O2 — decisions as data, with evidence.** Completion and admissibility come from the governor,
  never from the executor.

## Boundary

- Commission owns the responsibility model and runtime contracts (Atlas ADR 0070): Agent,
  AgentRevision, CaseRef, Commission, Frontier, Governor, AgentExecutor, AuthorityProvider,
  observation and evidence ports, RunOutcome, suspension.
- Commission is domain-neutral. AEP is one governor implementation (Atlas ADR 0069).
- Commission does not become an LLM harness. Shared execution contracts live here; Loom depends on
  them and implements them. No Cargo dependency from Commission core to Loom (Atlas ADR 0075).

## Rules

- The model is not trusted context. It may propose an action, arguments, a hypothesis or a plan. It
  never supplies identity, authority, case revision, trusted time, approval results, evidence
  producer identity, protocol version or tenant context.
- Revalidate every mutating action immediately before execution against the current case revision,
  frontier, authority and environment capability. A selection made on stale state is not
  permission.
- A trace is not evidence (Atlas ADR 0074). Never turn a model or tool trace into evidence
  automatically.
- When a selector, integration, verifier or authority provider fails, fail toward less authority,
  less effect and more explicit uncertainty. Never silently broaden capability.
- Anything that runs is Rust; command lines use clap derive.

## ESS

Commission is specified in ESS under `ess/`. The domain is drafted and validated before any story
introduces a noun, and `task check` runs its conformance suite once synthesized. Change the
specification first.

The specification is a hard gate in `task check` (Atlas ADR 0076). The task `ess-gate` runs
`crates/commission/tests/ess_gate.rs`, which fails `check` unless all four steps hold:

1. `ess specify validate --path ess --strict-requires` exits 0;
2. `ess specify compile --path ess --format json` exits 0;
3. `ess verify conform synthesize --path ess --out <scratch>/suite.json` exits 0 with 0 refusals;
4. no file under `ess/` contains `UNMAPPED:`.

No story is implemented while the gate is red, whether or not it edits `ess/`; a story that edits
`ess/` passes `ess-gate` on its own tree.

Spec first, then red, then implement (Atlas ADR 0080). A unit's first commit changes only `ess/`;
on it a named test fails (usually `drift`, or a conformance scenario) and the red run is recorded;
later commits make it pass without changing `ess/`. Each story's `## ESS first` names the change
and the red test. Only a change with no behaviour change is exempt, and its story says so.

A question the sources do not settle stays out of `ess/` and goes to the planning store as a
`decision-blocker`, never into `ess/` as a marker. Step 4 stands in until ESS can see open
questions: the `UNMAPPED:` scan is removed when the ESS release that refuses open entries
(beyond10x/ess `epic:typed-open-questions`) is pinned.

## Work

- Planned in the AEP store under `.engineering/`, written only through `aep plan artifact`. Body
  drafts go in `.engineering/drafts/` (ignored).
- Build with `CARGO_TARGET_DIR=$HOME/.cache/b10x-target/commission` (the Taskfile sets it).
- `b10x-canon` is a git dependency on canon `main`, pinned by `Cargo.lock`. Move the pin only in a
  story that names the Canon change it takes.
- Every commit and push is `b10x-bot[bot]`'s through `b10x-gates bot`; every GitHub write goes
  through `b10x-gates api`.
- Use a managed worktree (`worktree create --repo commission --purpose …`) for changes.
