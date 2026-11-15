use tl_rewrite::{
    check_equivalence, replay, rewrite, ConformanceOptions, ConformanceStatus, ReplayStatus,
    RewriteOptions, RewriteStatus,
};
use tl_syntax::{FormulaDocument, Node, NodeId, NodeKind, PropositionId, SemanticProfile};

fn p(id: u32) -> Node {
    Node::new(NodeKind::Proposition { proposition: PropositionId(id) })
}

fn main() {
    // not (not (true and p3))
    let nodes = vec![
        Node::new(NodeKind::True),
        p(3),
        Node::new(NodeKind::And { left: NodeId(0), right: NodeId(1) }),
        Node::new(NodeKind::Not { operand: NodeId(2) }),
        Node::new(NodeKind::Not { operand: NodeId(3) }),
    ];
    let input = FormulaDocument::new(SemanticProfile::ClosedTraceV1, NodeId(4), nodes).unwrap();

    let report = rewrite(&input, "demo", RewriteOptions::default(), "rev-1");
    assert_eq!(report.status, RewriteStatus::Normalized);
    for step in &report.steps {
        println!("applied {}", step.rule_id);
    }
    let output = report.output.as_ref().unwrap();
    println!("normal form: {} node", output.nodes().len());

    println!("replay verified: {}", replay(&input, &report).status == ReplayStatus::Verified);
    let same = check_equivalence(&input, output, "demo", ConformanceOptions::default());
    println!("equivalent: {}", same.status == ConformanceStatus::Equivalent);

    // Replacing p0 with not p0 does not preserve meaning; the check finds a trace.
    let original = FormulaDocument::new(SemanticProfile::ClosedTraceV1, NodeId(0), vec![p(0)]).unwrap();
    let wrong = FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(1),
        vec![p(0), Node::new(NodeKind::Not { operand: NodeId(0) })],
    )
    .unwrap();
    let check = check_equivalence(&original, &wrong, "negated", ConformanceOptions::default());
    println!("equivalent: {}", check.status == ConformanceStatus::Equivalent);
    println!("counterexample trace: {:?}", check.counterexample);
}
