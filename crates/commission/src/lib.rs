#![forbid(unsafe_code)]

//! Bootstrap Commission contracts.
//!
//! Production interfaces will almost certainly be async and split into
//! multiple crates. The purpose of this scaffold is to freeze dependency
//! direction and vocabulary.
//!
//! The responsibility model (Agent, AgentRevision, Case, Commission and their ids) is generated
//! from `ess/` into `generated/rust/commission/` and re-exported here as [`model`]. It is never
//! written by hand.

use b10x_canon::{ActionId, Frontier};

/// The responsibility model, synthesized from the ESS specification.
pub use commission as model;

use model::responsibility::CommissionData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutorOutcome {
    ProposedAction {
        action: ActionId,
        arguments_json: String,
    },
    NeedsHumanJudgment {
        request: String,
    },
    Suspended {
        reason: String,
    },
    NoUsefulAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    CaseCompleted { outcome: String },
    NeedsAuthority { request: String },
    NeedsHumanJudgment { request: String },
    NeedsExternalEvidence { requirements: Vec<String> },
    NoAdmissibleAction,
    Suspended { reason: String },
}

pub trait Governor {
    fn frontier(&self, commission: &CommissionData) -> Result<Frontier, String>;
}

pub trait AgentExecutor {
    fn run(
        &self,
        commission: &CommissionData,
        frontier: &Frontier,
    ) -> Result<ExecutorOutcome, String>;
}

pub trait AuthorityProvider {
    fn authorize(
        &self,
        commission: &CommissionData,
        capability: &str,
    ) -> Result<AuthorityDecision, String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityDecision {
    Allow,
    Deny { reason: String },
    ApprovalRequired { request: String },
}
