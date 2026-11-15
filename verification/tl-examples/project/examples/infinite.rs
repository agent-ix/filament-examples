use tl_mltl::infinite::{
    evaluate_lasso, Disposition, EvaluationLimit, EvidenceClosure, LassoRequest, ObservationValue,
};
use tl_syntax::{
    InfiniteClock, InfiniteFormulaDocument, InfiniteNode, InfiniteNodeKind as K, Interval,
    LassoTraceDocument, NodeId, PartialValuation, PropositionId, SemanticProfile, TemporalInterval,
    TraceObservation, ValuationEntry,
};

// A lasso of two observations in which p0 is true, followed by a loop of one more instant in
// which p0 is true, so p0 is true at every position.
fn trace() -> LassoTraceDocument {
    let propositions = [PropositionId(0)];
    let observation = |position: u32| TraceObservation {
        position,
        valuation: PartialValuation::new(
            "map".to_owned(),
            &propositions,
            vec![ValuationEntry { proposition: propositions[0], value: ObservationValue::True }],
        )
        .unwrap(),
    };
    LassoTraceDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        "map".to_owned(),
        propositions.to_vec(),
        vec![observation(0), observation(1)],
        vec![observation(2)],
    )
    .unwrap()
}

fn run(nodes: Vec<InfiniteNode>, root: u32, position: u64) -> Disposition {
    let graph = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(root),
        nodes,
    )
    .unwrap();
    let lasso = trace();
    let graph_id = graph.content_identity().unwrap();
    let trace_id = lasso.content_identity().unwrap();
    evaluate_lasso(&LassoRequest {
        formula: &graph,
        trace: &lasso,
        fairness: None,
        evidence_closure: EvidenceClosure::Closed,
        graph_id: &graph_id,
        trace_id: &trace_id,
        selected_position: position,
        limit: EvaluationLimit::default(),
    })
    .unwrap()
    .disposition
}

fn main() {
    let p0 = || InfiniteNode::new(K::Proposition { proposition: PropositionId(0) });
    let not_p0 = || InfiniteNode::new(K::Not { operand: NodeId(0) });

    // Y !p0 at position 0. The origin-history profile reads position -1 as false, so it is true.
    let y = vec![p0(), not_p0(), InfiniteNode::new(K::StrongPrevious { operand: NodeId(1) })];
    println!("Y !p0 at position 0: {:?}", run(y, 2, 0));

    // H[0,2] p0 at position 1. The origin-history profile reads position -1 as false, so it is false.
    let interval = TemporalInterval::Closed(Interval::new(0, 2).unwrap());
    let h = vec![p0(), InfiniteNode::new(K::Historically { interval, operand: NodeId(0) })];
    println!("H[0,2] p0 at position 1: {:?}", run(h, 1, 1));
}
