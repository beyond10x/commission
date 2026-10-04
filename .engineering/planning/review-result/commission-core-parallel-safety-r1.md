---
format: aep.planning-md/3
id: review-result:commission-core-parallel-safety-r1
kind: review-result
status: active
title: Commission core decomposition — parallel-safety critic, round 1
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
needs-revision

story:governor-port — this and story:agent-executor-port both depend only on story:frontier-admission, so nothing orders them, and both replace the adjacent `Governor` and `AgentExecutor` traits in `crates/commission/src/lib.rs` (cited, both bodies), edit `ess/domains/responsibility.yaml` and regenerate the same committed model; neither body names the other, so add an ordering edge recording the shared file as its reason or split the surface (one module per port) — .engineering/planning/story/governor-port.md:23 (and .engineering/planning/story/agent-executor-port.md:21)

story:authority-provider-port — it depends only on story:generated-responsibility-model, so it is unordered against story:frontier-admission, story:governor-port, story:agent-executor-port, story:observation-evidence-ports, story:stale-revision-action-request and story:commission-ess-conformance, and it moves `AuthorityProvider`/`AuthorityDecision` in `crates/commission/src/lib.rs` and adds a type to `ess/domains/responsibility.yaml` (cited) without saying either file is shared; add ordering edges naming that file, or split the surface — .engineering/planning/story/authority-provider-port.md:21

story:observation-evidence-ports — this and story:stale-revision-action-request both depend only on story:governor-port; each extends the `Governor` port and the scripted fake governor (this body: two new methods and a recording fake; the other: "the current case revision is read from the governor" against a fake whose case moves; cited, fake's location inferred) and neither says so; add an ordering edge naming the shared port and fake, or split the surface — .engineering/planning/story/observation-evidence-ports.md:24 (and .engineering/planning/story/stale-revision-action-request.md:22)

story:generated-responsibility-model — it says "Each later story regenerates it after its own `ess/` change", but it does not say that stories unordered with each other (frontier-admission/authority-provider-port, governor-port/agent-executor-port, observation-evidence-ports/stale-revision-action-request) all rewrite one committed generated tree and one `ess/domains/responsibility.yaml`; because this story decides "where it lives", its body needs a layout that lets those regenerations merge (cited, ess/ section of each body) — .engineering/planning/story/generated-responsibility-model.md:29

story:governor-port — "Commission's test support" holds the scripted fake governor here and in story:observation-evidence-ports, story:stale-revision-action-request, story:authority-provider-port, story:agent-executor-port, story:run-outcomes and story:local-runtime-loop, and no body names a file or module for it (inferred: `crates/commission/src/lib.rs` is the only source file today), so concurrent stories would edit one unnamed surface; the body that creates the fake must name where fakes live — .engineering/planning/story/governor-port.md:20

story:commission-ess-conformance — it adds a `task check` step and an `ess-conformance` git dependency (`Cargo.toml`, `Cargo.lock`, `Taskfile.yml` `check` task; inferred from "`task check` synthesizes…"), as story:agent-executor-port adds its dependency-guard step to the same task (cited); the two are unordered (that one stops at frontier-admission, this one at stale-revision-action-request) and neither admits the shared `Taskfile.yml`; add an ordering edge or split the check steps into separate tasks — .engineering/planning/story/commission-ess-conformance.md:20 (and .engineering/planning/story/agent-executor-port.md:45)

story:commission-ess-conformance — `ess/SKIPPED.md` does not exist in the tree (not in `git ls-files`), the body expects every skip in it, and seven stories say "once this has landed … named in `ess/SKIPPED.md`" with no edge to it; story:observation-evidence-ports, story:run-outcomes and story:local-runtime-loop are unordered against it, so who creates the file and whose scenarios the first run must already cover depends on landing order; state the creator and the ordering — .engineering/planning/story/commission-ess-conformance.md:22

story:adapter-conformance-suites — the body names no crate, module or path for the public test kit ("a crate other than `b10x-commission`") and no home for the fakes it runs against, so its surface is unplaced rather than safe; it is unordered against story:agent-executor-port, story:run-outcomes, story:local-runtime-loop and story:commission-ess-conformance, and a new crate would touch `Cargo.toml`/`Cargo.lock`/`Taskfile.yml`; name the surface, or keep it out of any concurrent set — .engineering/planning/story/adapter-conformance-suites.md:21

**What I read:** 11 stories decomposing `epic:commission-core` and the epic itself. I ran `aep plan artifact list`, `show` on each, `graph` and `validate`, plus `crates/commission/src/lib.rs`, `Cargo.toml`, `Taskfile.yml` and `ess/domains/responsibility.yaml`. `aep plan waves` does not exist; I derived the unordered pairs from the `depends_on` edges in `graph`. Across the 11 stories:
- **Cited:** all 11 name `ess/domains/responsibility.yaml`. For code, 6 cite `crates/commission/src/lib.rs` (generated-responsibility-model, frontier-admission, agent-executor-port, authority-provider-port, governor-port, run-outcomes).
- **Inferred:** 4 (stale-revision-action-request, observation-evidence-ports, local-runtime-loop, commission-ess-conformance).
- **Unplaceable:** 1 (adapter-conformance-suites).

**What I could not establish:**
- Where the generated crate will live and whether `lib.rs` is split into modules. Both are undecided in the bodies, so the `lib.rs` and generated-tree collisions are file-level claims.
- Line citations that do not match the tree, which are outside my lane (acceptance or design). The bodies cite `lib.rs:66-79`, but the file is 70 lines. They cite `responsibility.yaml:246-247`, but that file is 244 lines. The `UNMAPPED` markers actually sit at lines 115, 117, 165, 188 and 230.
- The three open decision-blockers on `epic:commission-core` all concern `ess/domains/responsibility.yaml`, which the stories edit. Resolving them will edit the same file, but they are outside the set I was given.

```findings
[
  {
    "file": ".engineering/planning/story/governor-port.md",
    "line": 23,
    "category": "parallel-safety",
    "severity": "blocker",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "story:governor-port and story:agent-executor-port both depend only on story:frontier-admission and both replace adjacent traits in crates/commission/src/lib.rs, edit ess/domains/responsibility.yaml and regenerate the same committed model, and neither body says so; add an ordering edge recording the shared file or split the surface (cited)"
  },
  {
    "file": ".engineering/planning/story/authority-provider-port.md",
    "line": 21,
    "category": "parallel-safety",
    "severity": "blocker",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "story:authority-provider-port depends only on story:generated-responsibility-model, so it is unordered against frontier-admission, governor-port, agent-executor-port, observation-evidence-ports, stale-revision-action-request and commission-ess-conformance, yet it changes crates/commission/src/lib.rs and ess/domains/responsibility.yaml without saying they are shared; add ordering edges or split the surface (cited)"
  },
  {
    "file": ".engineering/planning/story/observation-evidence-ports.md",
    "line": 24,
    "category": "parallel-safety",
    "severity": "blocker",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "story:observation-evidence-ports and story:stale-revision-action-request both depend only on story:governor-port and both extend the Governor port and the scripted fake governor, and neither says so; add an ordering edge naming the shared port or split the surface (port cited, the fake's location inferred)"
  },
  {
    "file": ".engineering/planning/story/generated-responsibility-model.md",
    "line": 29,
    "category": "parallel-safety",
    "severity": "blocker",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "the body says each later story regenerates the committed tree but not that stories unordered with each other all rewrite one generated tree and one responsibility.yaml; as the story that decides where the generated model lives it must state a layout that lets those regenerations merge (cited)"
  },
  {
    "file": ".engineering/planning/story/governor-port.md",
    "line": 20,
    "category": "parallel-safety",
    "severity": "warning",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "the phrase 'Commission's test support' covers the fakes for governor-port, observation-evidence-ports, stale-revision-action-request, authority-provider-port, agent-executor-port, run-outcomes and local-runtime-loop and no body names a file or module for it; the story that creates the fake must name where fakes live (inferred, lib.rs is the only source file)"
  },
  {
    "file": ".engineering/planning/story/commission-ess-conformance.md",
    "line": 20,
    "category": "parallel-safety",
    "severity": "blocker",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "story:commission-ess-conformance adds a task check step and an ess-conformance git dependency while the unordered story:agent-executor-port adds its dependency-guard step to the same task; neither admits the shared Taskfile.yml and Cargo files; add an ordering edge or split the check steps (agent-executor-port side cited, Taskfile/Cargo side inferred)"
  },
  {
    "file": ".engineering/planning/story/commission-ess-conformance.md",
    "line": 22,
    "category": "parallel-safety",
    "severity": "warning",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "ess/SKIPPED.md does not exist yet and seven stories rely on it with only a prose 'once this has landed' and no edge, so observation-evidence-ports, run-outcomes and local-runtime-loop collide with this story depending on landing order; state who creates the file and the ordering (cited, file absent from the tree)"
  },
  {
    "file": ".engineering/planning/story/adapter-conformance-suites.md",
    "line": 21,
    "category": "parallel-safety",
    "severity": "warning",
    "verdict": "needs-revision",
    "origin": "introduced",
    "message": "the body names no crate, module or path for the public test kit or for the fakes it runs against, so its surface is unplaced rather than safe, and it is unordered against agent-executor-port, run-outcomes, local-runtime-loop and commission-ess-conformance; name the surface or keep it out of any concurrent set"
  }
]
```
