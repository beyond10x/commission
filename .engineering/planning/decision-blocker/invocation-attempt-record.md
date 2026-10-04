---
format: aep.planning-md/3
id: decision-blocker:invocation-attempt-record
kind: decision-blocker
status: open
title: Nobody has decided whether an action request references the Connector attempt its invocation produced, or how many
relations:
- blocks: story:effect-invocation
revision: 1
---
## Question

When the Commission runtime invokes an action request's effect (Atlas ADR 0082), does Commission
record the Connector attempt it produced, and how many attempts may one action request have (exactly
one, or retries)?

## Relation

`commission.responsibility.ActionRequest` -> `connectors.mutations.AttemptRecord`. Direction,
cardinality, ownership and lifecycle coupling: UNMAPPED. The Connectors side is typed (an
`AttemptRecord` references one Connection and one service configuration, connectors
`ess/domains/mutations.yaml`, per loom `story:connector-action-binding` § Domain relations at its
revision 1); nothing on the Commission side refers to it.

## What it stops

`story:effect-invocation`: the outcome its command returns after an invocation, and whether evidence
or completion can later cite the attempt.

## Clears when

An accepted decision states whether the request references its attempt(s) and the cardinality, and
the Commission ESS domain declares it as a `relations:` entry.
