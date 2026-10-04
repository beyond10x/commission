---
format: aep.planning-md/3
id: decision-blocker:evidence-observation-link
kind: decision-blocker
status: cleared
title: Nobody has decided how many observations one evidence record interprets, or whether it references them
relations:
- blocks: epic:commission-core
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-03T23:54:23Z", actor: "human:timo", revision: 3}
---
## Question

How are evidence and observations related? Does one evidence record interpret one observation or
several? Can one observation feed several evidence records? Does an evidence record keep a
reference to the observations it came from, or only a free-form provenance?

## Relation

Evidence -> Observation. Cardinality in both directions, and whether Evidence holds a reference to
Observation or only a copy of its provenance.

## Why nothing settles it

- `ess/domains/responsibility.yaml:204-205` marks it: "UNMAPPED: which observations an evidence
  record interprets (one or many) is not settled; no relation from Evidence to Observation is
  declared."
- The two prose sources disagree. `docs/contracts/evidence.md:151` says "An evidence adapter may
  interpret one or more observations" (many observations to evidence), and the pre-Commission
  design (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 26) sketches
  `interpret(&observation) -> Vec<Evidence>` (one observation to many evidence records).
- No code has either type.

## What Is Blocked

The evidence-adapter part of TASKBOARD M-006 ("Define observation/evidence ports"): the port that
turns observations into evidence. Its signature states this cardinality.
`story:observation-evidence-ports` is drafted without it and covers only the governor-facing
observation and evidence ports.

## What Would Clear It

A recorded decision, then a `relations:` entry on `commission.responsibility.Evidence` (or a
recorded decision that none exists) in `ess/domains/responsibility.yaml` that
`ess specify validate --path ess` accepts, replacing the marker. An evidence-adapter story can then
be drafted.

## Who Can Clear It

Commission’s owner (the operator). Atlas `invariants.md` § 1 gives evidence admissibility to the
governor, so AEP’s owner may want a say.

## What We Are Doing Meanwhile

Observations and evidence reach the governor through separate ports, and nothing in Commission
converts one into the other.

## Decision (operator, 2026-10-04)

One or more: Evidence carries observation_ids and references many Observations, in ess/domains/responsibility.yaml.
