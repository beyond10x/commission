---
title: Observation and evidence (sketch)
description: The design sketch separating raw observations from admitted evidence.
lede: An observation is a raw report. Evidence is what the protocol or governor admits. Nothing turns one into the other by itself.
source: docs/contracts/evidence.md, a design sketch
source_url: https://github.com/beyond10x/commission/blob/main/docs/contracts/evidence.md
---

:::note[Design sketch]

This page renders `docs/contracts/evidence.md`, a design sketch. The records below are
illustrative. The observation and evidence ports that carry them exist in
`b10x_commission::ports::evidence`; nothing in Commission turns an observation, an executor output
or a trace into evidence.

:::

An **observation** is a raw report from a runtime, an integration or a verifier.

```yaml title="observation, illustrative"
source: connector:github-actions
subject: implementation:R2
observed_at: 2026-10-04T00:00:00Z
payload:
  conclusion: success
```

**Evidence** is validated, typed, attributable information that the protocol or governor admits.

```yaml title="evidence, illustrative"
kind: test_result
subject:
  type: implementation
  revision: R2
producer:
  principal: service:ci
observed_at: 2026-10-04T00:00:00Z
facts:
  tests.pass: true
provenance:
  source: github-actions
  run_id: "1234"
```

## The rule

```text
Observation ≠ Evidence
```

- An evidence adapter may interpret one or more observations.
- The governor decides whether a piece of evidence applies.
- A raw model statement such as "tests passed" is not automatically evidence. Nor is a model or
  tool trace.

## In the generated model

`Observation` and `Evidence` are separate entities. An evidence record references the case it is
about and the observations it interprets. See the [domain model](../../reference/domain-model.md#evidence).
