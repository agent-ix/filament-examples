use tl_mltl::{analyze_horizon, evaluate_closed, evaluate_prefix, EvaluationLimits, TruthValue};
use tl_syntax::{Formula, Interval, Node, NodeId, NodeKind, PropositionId, SemanticProfile};

fn main() {
    // F[1,2] p0
    let nodes = [
        Node::new(NodeKind::Proposition { proposition: PropositionId(0) }),
        Node::new(NodeKind::Future {
            interval: Interval::new(1, 2).unwrap(),
            operand: NodeId(0),
        }),
    ];
    let limits = EvaluationLimits::default();

    // A closed trace has an end. Each instant lists the propositions that are true.
    let closed = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();
    let holds = vec![vec![], vec![PropositionId(0)]];
    let report = evaluate_closed(closed, "request-ready", &holds, "run-1", limits).unwrap();
    println!("p0 at instant 1: {:?}", report.verdict);
    let fails = vec![vec![], vec![], vec![]];
    let report = evaluate_closed(closed, "request-ready", &fails, "run-2", limits).unwrap();
    println!("p0 never true: {:?}", report.verdict);

    // An open prefix with one instant does not decide the formula yet.
    let online = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(1), &nodes).unwrap();
    let seen = vec![vec![]];
    let open = evaluate_prefix(online, "request-ready", &seen, "run-3", false, limits).unwrap();
    println!("one instant seen: {:?}", open.verdict);
    assert_eq!(open.verdict, TruthValue::Pending);

    let horizon = analyze_horizon(closed, "request-ready").unwrap();
    println!("lookahead {}, buffer {}", horizon.lookahead, horizon.required_buffer);
}
