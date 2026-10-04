---
title: Rules
description: What a model may and may not supply, and how Commission fails.
---

# Rules

These hold for every executor, whatever runs inside it.

## The model is not trusted context

A model may propose an action, arguments, a hypothesis or a plan. It never supplies identity,
authority, case revision, trusted time, approval results, evidence producer identity, protocol
version or tenant context.

## Only what the frontier admits

An executor may not invoke an action absent from the current frontier's admissible set. See
[the frontier sketch](./contracts/frontier.md).

## Revalidate before every mutating action

Immediately before execution, a mutating action is checked again against the current case
revision, frontier, authority and environment capability. A selection made on stale state is not
permission.

## A trace is not evidence

An observation is a raw report. Evidence is typed, attributable information the governor admits. A
model saying "tests passed" is not evidence, and neither is a model or tool trace. See
[observation and evidence](./contracts/evidence.md).

## Fail toward less authority

When a selector, integration, verifier or authority provider fails, the result is less authority,
less effect and more explicit uncertainty, never a silently broader capability.
