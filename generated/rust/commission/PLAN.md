<!--
  generated from commission v1
  model digest a3b6896a96a91b19581506d7622aa6bba27b271020f12d78673c828a61542674
  contract digest 376f5be8445e9cc0e3df87c1f9b7e389768aa141b5e21a3d0cb87fac737a44cd
  do not edit: regenerate with `ess synthesize --layout crate`
-->
# Synthesis plan — commission v1

Scope: `component-skeletons`, laid out as `crate`, planned by `ess-synth`. Regenerate with `ess synthesize --layout crate`.

57 capabilities: **57 generated**, **0 obligations**, **0 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

## Generated

| capability | source |
| --- | --- |
| domain type | `commission.responsibility.ActionStatus` |
| domain type | `commission.responsibility.Admission` |
| domain type | `commission.responsibility.AdmissionNeedsAuthority` |
| domain type | `commission.responsibility.AdmissionRefused` |
| domain type | `commission.responsibility.Agent.State` |
| domain type | `commission.responsibility.AgentId` |
| domain type | `commission.responsibility.AgentRevision.State` |
| domain type | `commission.responsibility.AgentRevisionId` |
| domain type | `commission.responsibility.AuthorityContext` |
| domain type | `commission.responsibility.AuthorityDecision.State` |
| domain type | `commission.responsibility.AuthorityDecisionId` |
| domain type | `commission.responsibility.AuthorityVerdict` |
| domain type | `commission.responsibility.AuthorityVerdictApprovalRequired` |
| domain type | `commission.responsibility.AuthorityVerdictDeny` |
| domain type | `commission.responsibility.Case.State` |
| domain type | `commission.responsibility.CaseId` |
| domain type | `commission.responsibility.Commission.State` |
| domain type | `commission.responsibility.CommissionId` |
| domain type | `commission.responsibility.CompletionDetermination` |
| domain type | `commission.responsibility.CompletionDeterminationComplete` |
| domain type | `commission.responsibility.Evidence.State` |
| domain type | `commission.responsibility.EvidenceId` |
| domain type | `commission.responsibility.ExecutorOutcome` |
| domain type | `commission.responsibility.ExecutorOutcomeNeedsHumanJudgment` |
| domain type | `commission.responsibility.ExecutorOutcomeProposedAction` |
| domain type | `commission.responsibility.ExecutorOutcomeSuspended` |
| domain type | `commission.responsibility.Frontier.State` |
| domain type | `commission.responsibility.FrontierAction` |
| domain type | `commission.responsibility.FrontierClaim` |
| domain type | `commission.responsibility.FrontierId` |
| domain type | `commission.responsibility.FrontierObligation` |
| domain type | `commission.responsibility.GovernorError` |
| domain type | `commission.responsibility.HumanDecisionRequest` |
| domain type | `commission.responsibility.Observation.State` |
| domain type | `commission.responsibility.ObservationId` |
| domain type | `commission.responsibility.PrincipalId` |
| domain type | `commission.responsibility.ProposedActionArguments` |
| domain type | `commission.responsibility.Run.State` |
| domain type | `commission.responsibility.RunId` |
| domain type | `commission.responsibility.RunOutcome` |
| domain type | `commission.responsibility.RunOutcomeCompleted` |
| domain type | `commission.responsibility.RunOutcomeNeedsAuthority` |
| domain type | `commission.responsibility.RunOutcomeNeedsExternalEvidence` |
| domain type | `commission.responsibility.RunOutcomeNeedsHumanJudgment` |
| domain type | `commission.responsibility.RunOutcomeSuspended` |
| domain type | `commission.responsibility.SuspensionReason` |
| domain type | `commission.responsibility.Truth` |
| domain type | `commission.responsibility.Unit` |
| entity lifecycle | `commission.responsibility.Agent` |
| entity lifecycle | `commission.responsibility.AgentRevision` |
| entity lifecycle | `commission.responsibility.AuthorityDecision` |
| entity lifecycle | `commission.responsibility.Case` |
| entity lifecycle | `commission.responsibility.Commission` |
| entity lifecycle | `commission.responsibility.Evidence` |
| entity lifecycle | `commission.responsibility.Frontier` |
| entity lifecycle | `commission.responsibility.Observation` |
| entity lifecycle | `commission.responsibility.Run` |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
