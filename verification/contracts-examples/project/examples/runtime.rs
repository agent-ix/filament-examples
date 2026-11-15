use filament_contract_runtime::{
    CampaignReport, ClauseId, ClauseKind, ClauseOutcome, ContractIdentity, ExecutionPoint,
    FailureDetail, FailureKind, Observation, RequirementId, RevisionId, Verdict, VerdictContext,
};

static OBSERVATIONS: [Observation<'static>; 1] = [Observation::new(
    ClauseId::new("post-1"),
    ClauseKind::Postcondition,
    ClauseOutcome::Failed,
    Some(FailureDetail::new(
        ClauseId::new("post-1"),
        FailureKind::Postcondition,
        17,
        Some("expected output"),
    )),
)];

fn main() {
    let identity = ContractIdentity::new(RequirementId::new("FR-001"), RevisionId::new("rev-7"));
    let context = VerdictContext::new(identity, ExecutionPoint::new("after-handler"), &OBSERVATIONS);
    let detail = OBSERVATIONS[0].detail.unwrap();

    let mut report = CampaignReport::new(identity);
    report.record_verdict(&Verdict::passed(context)).unwrap();
    report.record_verdict(&Verdict::failed_postcondition(context, detail)).unwrap();
    report.record_verdict(&Verdict::rejected_precondition(context, detail)).unwrap();
    report.record_discard();

    let counts = report.counts();
    println!("accepted {}", counts.accepted());
    println!("rejected {}", counts.rejected());
    println!("failed {}", counts.failed());
    println!("discarded {}", counts.discarded());
    println!("total {}", counts.total());
    println!("{report}");

    // A verdict for another contract identity is refused.
    let other = ContractIdentity::new(RequirementId::new("FR-002"), RevisionId::new("rev-7"));
    let other_context = VerdictContext::new(other, ExecutionPoint::new("after-handler"), &OBSERVATIONS);
    println!("other identity: {:?}", report.record_verdict(&Verdict::passed(other_context)).is_err());
}
