---
format: aep.planning-md/3
id: decision-blocker:case-commission-cardinality
kind: decision-blocker
status: cleared
title: Nobody has decided how many commissions one case may hold at once
relations:
- blocks: epic:commission-core
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-03T23:54:23Z", actor: "human:timo", revision: 3}
---
## Question

How many commissions may one case hold at once: exactly one, or several in parallel (for example
one per role)? And if several, may two of them hold the same agent revision?

## Relation

Case -> Commission. Cardinality (one or many, at once) and whether the far side may be zero. The
other direction is settled: a commission references exactly one case
(`ess/domains/responsibility.yaml`, `commission.responsibility.Commission` relation `case`,
cardinality one, lines 149-153).

## Why nothing settles it

- `ess/domains/responsibility.yaml:131-132` marks it: "UNMAPPED: how many commissions a case may
  hold at once (one, or many in parallel) is not settled by the sources; no relation from Case to
  Commission is declared until a story decides it."
- No code declares it: the bootstrap `Commission` (`crates/commission/src/lib.rs:26-31`) holds one
  case and says nothing about siblings.
- The pre-Commission design (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md` § 9
  and § 30) draws one case with several assignments (architecture, implementation, verification,
  release). That is prose in a historical design, not an `ess/1` relation, and it does not say
  whether they are held at once.

## What Is Blocked

The part of TASKBOARD M-001 ("Stabilize `Agent`, `CaseRef`, `Commission`") that writes a
`relations:` entry from Case to Commission. `story:generated-responsibility-model` is drafted
without it and leaves the marker in place. One-to-one and one-to-many give different schemas and
lifecycles, so no story writes the relation until this is answered.

## What Would Clear It

A recorded decision on the cardinality, then a `relations:` entry on
`commission.responsibility.Case` in `ess/domains/responsibility.yaml` that
`ess specify validate --path ess` accepts, replacing the marker.

## Who Can Clear It

Commission’s owner (the operator).

## What We Are Doing Meanwhile

Every M-001 to M-010 story is drafted so that its acceptance holds under either answer: none of
them reads or limits the number of commissions on a case.

## Decision (operator, 2026-10-04)

Many in parallel: a case may hold several commissions at once. Recorded in ess/domains/responsibility.yaml (Commission references one case; no limit per case).
