---
format: aep.planning-md/3
id: decision-blocker:commission-principal-type
kind: decision-blocker
status: cleared
title: Nobody has decided what a commission's principal and authority context are, or who issues them
relations:
- blocks: epic:commission-core
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-03T23:54:23Z", actor: "human:timo", revision: 3}
---
## Question

What is a commission’s principal, and what is its authority context: Commission types (and if so,
entities or opaque identifiers like `CaseId`), or references to records another component owns
(Mandate, Identity)? Who issues each one, and how many of each does one commission hold?

## Relation

Commission -> Principal and Commission -> authority context. Cardinality, ownership and which side
may exist without the other.

## Why nothing settles it

- `ess/domains/responsibility.yaml:133` marks it: "UNMAPPED: principal and authority context are
  named in commission-design.md without a type."
- `docs/design/commission-design.md:16-25` lists "Principal" and "Authority context" among what a
  commission binds, with no type. Atlas ADR 0070 says "with authority context", also untyped.
- No code has either: the bootstrap `Commission` (`crates/commission/src/lib.rs:26-31`) carries
  id, agent and case only, and `AuthorityProvider::authorize` (`lib.rs:66-72`) takes the whole
  commission plus a capability string.
- Atlas `docs/design/governed-autonomy/invariants.md` § 1 gives authority and delegation to
  Mandate, which points to another component owning these records. That is an ownership hint in
  prose. It is not a relation.

## What Is Blocked

The part of TASKBOARD M-001 ("Stabilize ... `Commission`") that adds a principal and an authority
context to the Commission entity. `story:generated-responsibility-model` is drafted without them.
`story:authority-provider-port` is drafted so that the provider receives the commission and does its
own principal resolution. It does not depend on this answer.

## What Would Clear It

A recorded decision, then the fields and `relations:` entries on
`commission.responsibility.Commission` in `ess/domains/responsibility.yaml` that
`ess specify validate --path ess` accepts, replacing the marker.

## Who Can Clear It

Commission’s owner (the operator), with Mandate’s owner if the answer puts the record in Mandate.

## What We Are Doing Meanwhile

The authority port is keyed on the commission and a capability, as the bootstrap does. No story
gives the model or the executor any way to supply a principal.

## Decision (operator, 2026-10-04)

Commission-local types: PrincipalId (newtype of String) and AuthorityContext (newtype of Json), fields on Commission in ess/domains/responsibility.yaml. Binding to Mandate is phase-7 work.
