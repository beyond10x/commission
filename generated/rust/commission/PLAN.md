<!--
  generated from commission v1
  model digest 277056ebe75e6eeba0f32e739403d2898e04d9e7db35fdb5794c6e803995bca4
  contract digest b9d039400832e04b3894b8f9bc1f4e70c0a16725fbb7b214b05664222d81ea9a
  do not edit: regenerate with `ess synthesize --layout crate`
-->
# Synthesis plan — commission v1

Scope: `component-skeletons`, laid out as `crate`, planned by `ess-synth`. Regenerate with `ess synthesize --layout crate`.

69 capabilities: **69 generated**, **0 obligations**, **0 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

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
| command contract | `commission.responsibility.ResumeRun` |
| command behaviour | `commission.responsibility.ResumeRun` |
| command contract | `commission.responsibility.StartRun` |
| command behaviour | `commission.responsibility.StartRun` |
| command contract | `commission.responsibility.SuspendRun` |
| command behaviour | `commission.responsibility.SuspendRun` |
| event type | `commission.responsibility.RunResumed` |
| event type | `commission.responsibility.RunStarted` |
| event type | `commission.responsibility.RunSuspended` |
| error type | `commission.responsibility.RunStateConflict` |
| view type | `commission.responsibility.RunStates` |
| view query | `commission.responsibility.RunStates` |

## Ports — yours to provide

What the specification fully determines is generated; what it cannot determine is an obligation. A generated command behaviour or view query reads and writes through the ports below, and they are yours to provide: synthesis generates each port's contract and never an implementation of one, so where instances live stays your decision.

| port | what it answers |
| --- | --- |
| storage | one per entity a generated behaviour or query reads or writes: the instance stored under an identity; storing, replacing and removing one; and every stored instance, in the order the store keeps them |
| context | where a generated behaviour asks it: the caller's attributes, every identity and value the specification says the implementation assigns, and whether each `external:` branch is taken |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
