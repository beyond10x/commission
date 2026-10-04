---
format: aep.planning-md/3
id: review-result:commission-core-parallel-safety-r2
kind: review-result
status: active
title: Commission core decomposition — parallel-safety critic, round 2
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

**What I read:** 11 stories decomposing `epic:commission-core`, the epic, and `review-result:commission-core-parallel-safety-r1`. I ran `aep plan artifact list`, `show` on all 11 stories, `graph` and `waves --kind story`. `aep plan validate` is not a subcommand here, so I did not run it.

**The edges give a single chain.** `waves` returns 11 waves of one story each:

`generated-responsibility-model` → `frontier-admission` → `governor-port` → `agent-executor-port` → `authority-provider-port` → `observation-evidence-ports` → `stale-revision-action-request` → `run-outcomes` → `local-runtime-loop` → `commission-ess-conformance` → `adapter-conformance-suites`

Every adjacent pair has a `depends_on` edge in `graph`. No two of the 11 stories can run at once, so none collides in time. The 146 `collision:` lines `waves` prints are all between stories already ordered by that chain.

**All eight round-1 findings are resolved.**

| Round-1 finding | What changed in the revised bodies |
|---|---|
| governor-port vs agent-executor-port unordered | `story:agent-executor-port` now depends on `story:governor-port`. Each body states its link number and the shared files. |
| authority-provider-port unordered against six stories | It now depends on `story:agent-executor-port`. |
| observation-evidence-ports vs stale-revision-action-request unordered | `story:stale-revision-action-request` now depends on `story:observation-evidence-ports`. |
| generated-responsibility-model gave no merge layout | The body now has a "Shared surface" section naming the whole chain, and a "Layout" bullet setting `generated/rust/commission/` and `.ess-output/` git-ignored. |
| No home for fakes | `story:governor-port` creates `crates/commission-testkit/`, one module per fake. |
| `Taskfile.yml` shared by agent-executor-port and commission-ess-conformance | Both are now on the chain. Each body lists `Taskfile.yml` under "Shared surface" or scope. |
| `ess/SKIPPED.md` had no creator | `story:generated-responsibility-model` creates it. `story:commission-ess-conformance` only appends to it. |
| adapter-conformance-suites surface unplaced | It now names `crates/commission-testkit/src/kits/{mod,governor,authority}.rs` and `src/lib.rs` as cited scope. |

**Surface counts, after the revision:**
- Placed by a cited path: 11 of 11.
- Placed only by inference: 0 of 11. Some individual files inside the scopes are still marked inferred, such as new test files and `admission.rs`. Those are files each story creates, and none sits on another story's surface.
- Not placed at all: 0.

**What I could not establish:**
- Outside my lane: `epic:governor-adapter` and `story:managed-composition` depend on the epic and share these surfaces, and I was told not to judge them.
- Outside my lane: the three open decision-blockers (`aep-governed-case`, `managed-composition-host`, `suspension-durable-record`) all concern `ess/domains/responsibility.yaml`, which the stories edit. They attach to other epics and stories, not to the 11.
- Outside my lane: round 1 noted that cited line numbers in some bodies do not match the tree. I did not recheck them.

```findings
[]
```
