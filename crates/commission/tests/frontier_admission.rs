//! story:frontier-admission: the admission check sorts a proposed action against the generated
//! `Frontier` into the generated `Admission` union.

use b10x_commission::admission::admit;
use b10x_commission::model::primitives::Uuid;
use b10x_commission::model::responsibility::{
    ActionStatus, Admission, AdmissionNeedsAuthority, AdmissionRefused, CaseId, Frontier,
    FrontierAction, FrontierData, FrontierId, frontier_state,
};

fn action(
    name: &str,
    status: ActionStatus,
    capability: Option<&str>,
    reasons: &[&str],
) -> FrontierAction {
    FrontierAction {
        action: name.into(),
        status,
        capability: capability.map(Into::into),
        reasons: reasons.iter().map(|r| (*r).into()).collect(),
    }
}

fn frontier(actions: Vec<FrontierAction>) -> Frontier<frontier_state::Issued> {
    Frontier::new(FrontierData {
        frontier_id: FrontierId(Uuid("3d0b9a7e-2c41-4f6a-9e3b-5a8c1d2e4f60".into())),
        case_id: CaseId("case-1".into()),
        case_revision: 17,
        claims: Vec::new(),
        obligations: Vec::new(),
        actions,
    })
}

#[test]
fn admission_sorts_proposed_actions() {
    let blocked_reasons = ["tests.pass is Unknown", "review.approved is False"];
    let issued = frontier(vec![
        action("repository.inspect", ActionStatus::Admissible, None, &[]),
        action(
            "repository.push",
            ActionStatus::ApprovalRequired,
            Some("repository.write"),
            &[],
        ),
        action(
            "repository.merge",
            ActionStatus::Blocked,
            None,
            &blocked_reasons,
        ),
        action(
            "deploy.production",
            ActionStatus::ApprovalRequired,
            None,
            &[],
        ),
    ]);

    // 1. Listed Admissible: admissible.
    let admissible = admit(&issued, "repository.inspect");
    assert!(
        matches!(admissible, Admission::Admissible(_)),
        "an Admissible action was sorted {admissible:?}"
    );

    // 2. Listed ApprovalRequired with a capability: needs authority, naming the capability.
    assert_eq!(
        admit(&issued, "repository.push"),
        Admission::NeedsAuthority(AdmissionNeedsAuthority {
            capability: "repository.write".into(),
        }),
        "an ApprovalRequired action with a capability must name it"
    );

    // 3. Listed Blocked with reasons R: refused, naming the action and carrying R.
    assert_eq!(
        admit(&issued, "repository.merge"),
        Admission::Refused(AdmissionRefused {
            action: "repository.merge".into(),
            reasons: blocked_reasons.iter().map(|r| (*r).into()).collect(),
        }),
        "a Blocked action must be refused with its reasons"
    );

    // 4. Listed ApprovalRequired with no capability: refused, naming the action.
    match admit(&issued, "deploy.production") {
        Admission::Refused(refused) => assert_eq!(refused.action, "deploy.production"),
        other => panic!("an ApprovalRequired action with no capability was sorted {other:?}"),
    }

    // 5. Not listed: refused, naming the action.
    match admit(&issued, "repository.delete") {
        Admission::Refused(refused) => assert_eq!(refused.action, "repository.delete"),
        other => panic!("an action the frontier does not list was sorted {other:?}"),
    }

    // 6. The result is the generated union, reached through b10x-commission's re-export.
    let name = std::any::type_name_of_val(&admit(&issued, "repository.inspect"));
    assert!(
        name.starts_with("commission::"),
        "the admission result is {name}, not the generated crate's type"
    );
    assert!(
        std::any::type_name::<Admission>().starts_with("commission::"),
        "Admission is {}, not the generated crate's type",
        std::any::type_name::<Admission>()
    );
}
