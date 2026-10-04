//! The Rust ESS conformance target for `b10x-commission`.
//!
//! [`CommissionTarget`] is the `ess_conformance::ConformanceTarget` the suite synthesized from
//! `ess/` runs against, and [`run_suite`] runs one suite against it and returns the
//! `ess-conformance-report/2` document a caller reads the verdict from.
//!
//! The target answers no command yet: every command reaches no declared outcome, every view is
//! empty and no event is observed.

use ess_conformance::target::{
    ConformanceTarget, EventObservationRequest, ExternalOutcomeControl, ImplementationIdentity,
    ObservedEvent, RedeliveryRequest, ScenarioContext, SemanticCommandRequest,
    SemanticCommandResult, SemanticViewRequest, SemanticViewResult, TargetError,
};
use ess_conformance::{AdmittedSuite, CountReport, Runner};

/// The name this implementation is reported under.
pub const IMPLEMENTATION: &str = "b10x-commission";

/// The conformance target over `b10x-commission`.
#[derive(Debug, Default)]
pub struct CommissionTarget;

impl ConformanceTarget for CommissionTarget {
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new(
            IMPLEMENTATION,
            env!("CARGO_PKG_VERSION"),
        ))
    }

    fn begin_scenario(&self, _scenario: &ScenarioContext) -> Result<(), TargetError> {
        Ok(())
    }

    fn execute_command(
        &self,
        _request: SemanticCommandRequest,
    ) -> Result<SemanticCommandResult, TargetError> {
        Ok(SemanticCommandResult::undeclared())
    }

    fn query_view(&self, _request: SemanticViewRequest) -> Result<SemanticViewResult, TargetError> {
        Ok(SemanticViewResult::of([]))
    }

    fn observe_events(
        &self,
        _request: EventObservationRequest,
    ) -> Result<Vec<ObservedEvent>, TargetError> {
        Ok(Vec::new())
    }

    fn configure_external_outcome(
        &self,
        request: ExternalOutcomeControl,
    ) -> Result<(), TargetError> {
        Err(TargetError::unsupported(
            format!("forcing `{}`", request.force),
            "this target forces no external outcome",
        ))
    }

    fn redeliver_event(&self, request: RedeliveryRequest) -> Result<(), TargetError> {
        Err(TargetError::unsupported(
            format!("redelivering `{}`", request.event),
            "this target redelivers no event",
        ))
    }

    fn end_scenario(&self, _scenario: &ScenarioContext) -> Result<(), TargetError> {
        Ok(())
    }
}

/// One run of a suite against [`CommissionTarget`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executed {
    /// `ess-conformance-report/2`, canonical: the document the verdict is read from.
    pub report: String,
    /// The runner's per-scenario results with their diagnostics, for a reader of a red run.
    pub diagnostics: String,
}

/// Admit `suite` (the JSON `ess verify conform synthesize` writes), run it against
/// [`CommissionTarget`] and return the report.
///
/// # Errors
///
/// Returns the admission failure when the bytes are not an admissible suite, and the report
/// failure when the run and the suite disagree.
pub fn run_suite(suite: &str) -> Result<Executed, String> {
    let admitted = AdmittedSuite::from_json(suite).map_err(|error| error.to_string())?;
    let executed = Runner::for_suite(admitted.suite()).run_admitted(&admitted, &CommissionTarget);
    let report = CountReport::from_run(&executed, &admitted)
        .and_then(|report| report.to_canonical_json())
        .map_err(|error| error.to_string())?;
    let diagnostics =
        serde_json::to_string_pretty(&*executed).map_err(|error| error.to_string())?;
    Ok(Executed {
        report,
        diagnostics,
    })
}
