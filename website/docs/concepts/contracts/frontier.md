---
title: Frontier (sketch)
description: The design sketch of the frontier, the bridge between governance and execution.
---

# Frontier

:::note[Design sketch]

This page renders `docs/contracts/frontier.md`, a design sketch. The shape below is illustrative,
not a wire format. The typed form that exists today is in the generated
[domain model](../../reference/domain-model.md#frontier) and [value types](../../reference/types.md#frontieraction).

:::

A frontier is the bridge between governance and execution. It is not just a list of tools. It
carries enough structure to answer six questions about a case at one revision:

- what is currently known;
- what is unknown or contradicted;
- which obligations remain;
- which actions are admissible;
- which actions require authority;
- which conclusions are blocked, and why.

```yaml title="frontier/1, illustrative"
format: frontier/1

case:
  id: CHG-1842
  revision: 17

claims:
  - id: tests.pass
    value: unknown

obligations:
  - id: verify.tests
    status: open
    priority: normal

actions:
  - id: repository.inspect
    status: admissible
    input_schema: {...}

  - id: tests.run
    status: admissible
    input_schema: {...}

  - id: repository.merge
    status: blocked
    reasons:
      - claim: tests.pass
        required: true
        actual: unknown

conclusions:
  accepted:
    status: blocked
```

Here the merge is blocked because the claim it depends on, `tests.pass`, is still unknown. Running
the tests is admissible, so that is the useful next step.

## Invariant

An executor may not invoke an action absent from the current frontier's admissible set.

## In the generated model

Claims carry a three-valued truth (`True`, `False`, `Unknown`). Each action has a status of
`Admissible`, `ApprovalRequired` or `Blocked`, an optional capability an approval needs, and the
reasons a block names. Obligations are open or discharged. A frontier references the case it was
issued for.
