---
format: aep.planning-md/3
id: decision-blocker:action-operation-binding
kind: decision-blocker
status: open
title: Nobody has decided who declares which Connector operations a frontier action binds to, or how many
relations:
- blocks: story:effect-invocation
revision: 1
---
## Question

For an action in a frontier, which artifact declares the Connector operation(s) the Commission
runtime invokes through, and how many may it bind to? Candidates: the action definition in the
protocol (Canon or ELS), the Commission composition, a deployment configuration, or the Connectors
catalog. Part of the same question: does the binding name the Connection, or does Connectors choose
it?

## Why it is Commission's now

Atlas ADR 0082 (operator, 2026-10-04): the Commission runtime invokes "through the action's binding",
so Commission is the reader of the binding. ADR 0082 § Open asks whether this question, filed in the
loom store as loom `decision-blocker:action-operation-binding`, is now Commission's to answer. This
record takes it over for Commission; the loom record stays open only for whether an unbound action
is still offered in the catalogue Loom projects.

## Relation

Frontier action -> Connector operation (`instance_id`, `operation_id`). Cardinality, ownership and
lifecycle coupling: UNMAPPED. No ESS document in this repository declares it.

## Evidence

- Loom `decision-blocker:action-operation-binding` § What is settled around it (the Connectors side
  is typed in connectors `ess/domains/mutations.yaml`; that a binding exists, not who owns it).
- Atlas `epic:ga-governed-effects` § Outcome: "Connectors bind operations to protocol actions".

## What it stops

`story:effect-invocation`: the binding lookup, the unbound refusal and the binding port vocabulary
in `ess/`.

## Clears when

An accepted decision names the owner of the action-to-operation binding and its cardinality, and the
owning repository declares it as a `relations:` entry in its ESS domain.
