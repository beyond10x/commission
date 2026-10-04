//! The responsibility model reaches `b10x-commission` from the generated crate.

use b10x_commission::model::json::Value;
use b10x_commission::model::primitives::Uuid;
use b10x_commission::model::responsibility::{
    AgentRevisionId, AuthorityContext, CaseId, Commission, CommissionData, CommissionId,
    CommissionState, PrincipalId,
};
use std::path::Path;

/// The header `ess/SKIPPED.md` opens with; each later line names one skipped scenario.
const SKIPPED_HEADER: &str = "# Skipped conformance scenarios\n\
\n\
Each line below names one conformance scenario this repository skips, and why:\n\
`- <scenario id>: <reason>`. The list is empty when nothing is skipped.\n";

#[test]
fn generated_model_reexport() {
    let data = CommissionData {
        commission_id: CommissionId(Uuid("6f1c2a52-0d7e-4a4b-9a57-3c1f1d2b8e01".into())),
        agent_revision_id: AgentRevisionId(Uuid("0b6c4f1e-5d9a-4c2e-8f3b-7a1d2e3f4a5b".into())),
        case_id: CaseId("case-1".into()),
        principal: PrincipalId("principal-1".into()),
        authority_context: AuthorityContext(Value::Object(vec![(
            "scope".into(),
            Value::Text("read".into()),
        )])),
    };

    let commission = Commission::new(data.clone());

    assert_eq!(commission.state(), CommissionState::Assigned);
    assert_eq!(commission.data(), &data);
    assert_eq!(commission.data().case_id, CaseId("case-1".into()));
    assert_eq!(
        commission.data().principal,
        PrincipalId("principal-1".into())
    );
}

#[test]
fn skipped_file_starts_empty() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../ess/SKIPPED.md");
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} must exist: {error}", path.display()));

    assert!(
        body.starts_with(SKIPPED_HEADER),
        "{} does not open with the header:\n{body}",
        path.display()
    );
    let listed: Vec<&str> = body[SKIPPED_HEADER.len()..]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert!(listed.is_empty(), "skipped scenarios listed: {listed:?}");
}
