# Commission

**Build agents you can give responsibility to.**

Commission is the Rust SDK and runtime model for governed autonomous workers. It binds a reusable
agent revision to a durable case, under a governor that decides what the agent may do next:

- an **Agent** is reusable configuration; a **Case** is durable work; a **Commission** binds one to
  the other with a principal and authority context; a **Run** is one bounded period of execution;
- the governor returns a **Frontier**: what is known and unknown, which obligations are open, which
  actions are admissible and which need authority;
- an **AgentExecutor** proposes an action from that frontier; Commission revalidates it against the
  current case revision, frontier and authority before anything happens.

The executor never marks a case complete. Completion is the governor's determination.
[Loom](https://github.com/beyond10x/loom) is the native executor; other harnesses, workflow
executors, humans and test fakes can implement the same contract.

## Status

Bootstrap. Design: [`docs/design/commission-design.md`](docs/design/commission-design.md); contracts:
[`docs/contracts/`](docs/contracts/); the earlier SDK design:
[`docs/history/`](docs/history/).

## Build

```console
task check
```

## Licence

Apache-2.0.
