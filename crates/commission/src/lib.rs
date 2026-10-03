#![forbid(unsafe_code)]

//! Bootstrap Commission contracts.
//!
//! Production interfaces will almost certainly be async and split into
//! multiple crates. The purpose of this scaffold is to freeze dependency
//! direction and vocabulary.

use b10x_canon::{ActionId, CaseId, Frontier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommissionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commission {
    pub id: CommissionId,
    pub agent: AgentId,
    pub case: CaseId,
}

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
    fn frontier(&self, commission: &Commission) -> Result<Frontier, String>;
}

pub trait AgentExecutor {
    fn run(&self, commission: &Commission, frontier: &Frontier) -> Result<ExecutorOutcome, String>;
}

pub trait AuthorityProvider {
    fn authorize(
        &self,
        commission: &Commission,
        capability: &str,
    ) -> Result<AuthorityDecision, String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityDecision {
    Allow,
    Deny { reason: String },
    ApprovalRequired { request: String },
}
