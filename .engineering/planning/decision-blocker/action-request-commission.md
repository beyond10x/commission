---
format: aep.planning-md/3
id: decision-blocker:action-request-commission
kind: decision-blocker
status: cleared
title: Nobody has decided whether a commission owns or references its action requests, or how many it holds
relations:
- blocks: story:stale-revision-action-request
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-04T01:04:03Z", actor: "human:timo", revision: 3}
---
## Question

An action request carries the commission whose run proposed it (`story:stale-revision-action-request`
§ Outcome). Does the commission **own** its action requests, so that a request has no meaning
without its commission, or does an action request **reference** the commission and outlive the
link? And is it exactly one commission per request?

## Relation

`commission.responsibility.ActionRequest` -> `commission.responsibility.Commission`, via a
`commission_id` field typed `commission.responsibility.CommissionId`.

- Settled: the executor proposes within one commission's run (`crates/commission/src/lib.rs:53-55`),
  and the request lists the commission among what it carries.
- Not settled: `owns` against `references`, and the cardinality. The history design § 36
  (`docs/history/beyond10x-agent-sdk-design-pre-commission-name.md:1497-1504`) names an `actor`
  without a type. No contract in `docs/contracts/` names the relation.

## Why it blocks

The story that declares `ActionRequest` would have written this as an `UNMAPPED:` marker. Under the
ESS hard gate (Atlas ADR 0076, `story:ess-hard-gate`) `ess/` carries no `UNMAPPED:` string, and a
`commission_id` field declared without its `relations:` entry is a link nothing checks. So the
question is recorded here, and the story waits for the answer.

## Answer that would be taken

`references`, cardinality one, via `commission_id`: a request is evidence of what a commission
proposed and is kept with the authority decision that answers it
(`decision-blocker:authority-decision-owner`), so it outlives a deleted commission. Not taken
without the operator.

## Decision (coordinator, 2026-10-04)

`Run` owns its `ActionRequest`s: relation `requests`, kind `owns`, cardinality many, via `run_id` on ActionRequest. A request is made inside one run against that run's case revision (staleness is judged against it), and resuming continues the same run (operator, 2026-10-04), so the request never outlives its run. The commission is reached through the run; no direct ActionRequest -> Commission relation. story:stale-revision-action-request declares it in `ess/` with the entity.
