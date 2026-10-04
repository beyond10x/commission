---
format: aep.planning-md/3
id: review-result:commission-core-scope-r1
kind: review-result
status: active
title: Commission core decomposition — scope critic, round 1
relations:
- reviews: epic:commission-core
- reviews: story:adapter-conformance-suites
- reviews: story:agent-executor-port
- reviews: story:authority-provider-port
- reviews: story:commission-ess-conformance
- reviews: story:frontier-admission
- reviews: story:generated-responsibility-model
- reviews: story:governor-port
- reviews: story:local-runtime-loop
- reviews: story:observation-evidence-ports
- reviews: story:run-outcomes
- reviews: story:stale-revision-action-request
revision: 1
---
approve

The decomposition covers every promise in `epic:commission-core` exactly once and adds nothing the parent did not ask for.

I extracted 11 promises from the parent and traced all 11 to a story. The parent body is at `.engineering/planning/epic/commission-core.md`; I did not cite line numbers because the `aep plan artifact show` output does not print them.

| Promise | Claimed by |
|---|---|
| M-001 `Agent`, `CaseRef`, `Commission` | `story:generated-responsibility-model` |
| M-002 `Frontier` | `story:frontier-admission` |
| M-003 `Governor` | `story:governor-port` |
| M-004 `AgentExecutor` | `story:agent-executor-port` |
| M-005 `AuthorityProvider` | `story:authority-provider-port` |
| M-006 observation and evidence ports | `story:observation-evidence-ports` |
| M-007 suspension and run outcomes | `story:run-outcomes` |
| M-008 stale-revision action request | `story:stale-revision-action-request` |
| M-009 local runtime loop with fakes | `story:local-runtime-loop` |
| M-010 adapter conformance tests | `story:adapter-conformance-suites` |
| I-007 Commission ESS specification | `story:commission-ess-conformance` |

The parent's acceptance clauses and its Rule trace as follows.
- **Fake-driven loop:** the load, frontier, executor, refuse-absent, refuse-stale, and blocked/suspended/completed clauses are in `story:local-runtime-loop`.
- **`cargo tree` clause:** it is in `story:agent-executor-port`.
- **`task check` conformance clause:** it is in `story:commission-ess-conformance`.
- **Rule (generated types):** it is in `story:generated-responsibility-model`, and the later stories each regenerate the model.

- **Omissions are named, not gaps.** The exclusions are resume and the durable suspension record, the evidence-adapter port, the AEP-governed suspension check, and an `AgentExecutor` suite. Each is tied to an open `decision-blocker` or to a stated reason in the story body. M-011 and M-012 belong to `epic:governor-adapter`, and the vertical slices belong to `epic:vertical-slices`. No story in the set claims any of them.
- **No duplicated outcome.** `story:local-runtime-loop` repeats the stale and absent-from-frontier refusals only as the epic-level integration check. The unit claims sit in `story:stale-revision-action-request` and `story:frontier-admission`, and its Notes say so.

**What I read:** 12 artifacts, all with whole bodies (the parent plus 11 stories), plus `epic:ga-commission-core`, `epic:governor-adapter`, three decision-blockers, and the TASKBOARD Commission and Integration lists. Commands run: `aep plan artifact show` on each, `aep plan artifact graph`, and `grep` on `docs/design/governed-autonomy/TASKBOARD.md` and `ess/domains/responsibility.yaml`.

**What I could not establish:**
- **Out of my lane (acceptance or design critic).** The stories cite `ess/domains/responsibility.yaml` marker lines 131-132, 133, 181-182, 204-205 and 246-247. In this worktree the `UNMAPPED:` markers are at lines 115, 117, 165, 188 and 230, and the file is 244 lines long. This may be a different revision of the file than the one the stories were drafted from. It does not set my verdict.
- **Not judged.** `story:agent-executor-port` adds a `b10x-loom` dependency guard beyond the parent's "no model-provider crate" wording. It is traceable to ADR 0075, which is in the parent's Source line, so I did not count it as reach beyond the parent.

```findings
[]
```
