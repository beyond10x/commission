---
format: aep.planning-md/3
id: decision-blocker:authority-decision-owner
kind: decision-blocker
status: cleared
title: Nobody has decided whether an authority decision belongs to a commission, a case or one action request
relations:
- blocks: story:approval-suspend-resume-slice
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-04T00:07:51Z", actor: "human:timo", revision: 3}
---
## Question

Does an authority decision belong to a commission, to a case, or to one action request?

## Relation

AuthorityDecision -> Commission | Case | action request (ownership and cardinality).

## Why nothing settles it

`ess/domains/responsibility.yaml` marks it on `commission.responsibility.AuthorityDecision`:
"UNMAPPED: whether a decision belongs to a commission, a case or a single action request is not
settled by the sources." No code exists beyond the bootstrap crate.

## What it stops

`story:approval-suspend-resume-slice` (TASKBOARD I-003): a grant given after the process restart has
to be matched to the merge request made before it, and what the decision attaches to decides how
that match is made and whether one grant could authorise a later merge. `story:software-change-slice`
(I-001) is not stopped: it consults the AuthorityProvider at the call and stores no decision.

## Decision (operator, 2026-10-04)

One action request: a decision grants exactly one action request at one case revision; a later merge needs a new grant. Recorded in ess/domains/responsibility.yaml on AuthorityDecision; story:stale-revision-action-request declares the relation (AuthorityDecision references one ActionRequest) with the ActionRequest entity.
