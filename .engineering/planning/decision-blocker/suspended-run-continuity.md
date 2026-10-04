---
format: aep.planning-md/3
id: decision-blocker:suspended-run-continuity
kind: decision-blocker
status: cleared
title: Nobody has decided whether resuming after a restart continues the suspended run or starts a new run of the same commission
relations:
- blocks: story:approval-suspend-resume-slice
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-03T23:54:23Z", actor: "human:timo", revision: 3}
---
## Question

When a run suspends awaiting authority and its process exits, does resuming continue that same run,
or start a new run of the same commission? Equivalently: which entity carries the suspension, the
run or the commission?

## Relation

Commission -> Run across a suspension (lifecycle coupling: may a suspended run outlive the process
that started it).

## Why nothing settles it

- `ess/domains/responsibility.yaml` declares `Commission.runs` (owns, many) but gives Run and
  Commission one lifecycle state each (`Running`, `Assigned`); no suspension state exists yet
  (TASKBOARD M-007 is undone).
- Atlas `epic:ga-vertical-slices` (I-003) says "a run that suspends at the merge approval is
  resumed", which reads as the same run continuing.
- The build pack history design, `docs/history/beyond10x-agent-sdk-design-pre-commission-name.md`
  section 9, shows "Run 2: same agent resumes after production approval" and section on authority
  says "assignment becomes durably suspended", which reads as the commission holding the suspension
  and a new run resuming.

The two sources disagree, and neither is an ess/1 relation.

## What it stops

`story:approval-suspend-resume-slice` (TASKBOARD I-003). Its acceptance is written so it holds under
either answer, but the implementation cannot be written until this is decided.

## Decision (operator, 2026-10-04)

Continue the same run: the run id survives a restart and the run resumes, so run state is durable. Noted on Run in ess/domains/responsibility.yaml; the Suspended state arrives with its command in story M-007.
