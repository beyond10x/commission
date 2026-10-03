# Contract Sketch — Frontier

`Frontier` is the bridge between governance and execution.

It is not merely a tool list.

A frontier should contain enough structured information to answer:

- what is currently known;
- what is unknown or contradicted;
- what obligations remain;
- what actions are admissible;
- which actions require authority;
- what conclusions/outcomes are currently blocked and why.

Illustrative shape:

```yaml
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

## Invariant

An executor may not invoke an action absent from the current frontier/admissible set.
