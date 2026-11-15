use tl_syntax::{
    Formula, Interval, Node, NodeId, NodeKind, PropositionBinding, PropositionId,
    SemanticProfile, SignalCatalog, SignalDeclaration, SignalDomain, SignalId,
};

fn main() {
    // F[1,3] p7: the proposition p7 holds at some instant from 1 to 3.
    let nodes = [
        Node::new(NodeKind::Proposition { proposition: PropositionId(7) }),
        Node::new(NodeKind::Future {
            interval: Interval::new(1, 3).unwrap(),
            operand: NodeId(0),
        }),
    ];
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();

    let signals = [SignalDeclaration::new(SignalId(1), "request_ready", SignalDomain::Boolean)];
    let bindings = [PropositionBinding::new(PropositionId(7), SignalId(1))];
    let mut scratch = [0_u32; 1];
    let catalog = SignalCatalog::new(&signals, &bindings, &mut scratch).unwrap();
    let bound = catalog.bind_formula(formula).unwrap();
    let signal = bound.signal_for_proposition(PropositionId(7)).unwrap();
    println!("p7 -> {}", signal.name());

    // The same catalog without a binding for p7 refuses the formula.
    let mut scratch = [0_u32; 1];
    let empty = SignalCatalog::new(&signals, &[], &mut scratch).unwrap();
    println!("{}", empty.bind_formula(formula).unwrap_err());

    // A node may not point at a later node.
    let bad = [Node::new(NodeKind::Not { operand: NodeId(1) }), Node::new(NodeKind::True)];
    println!("{:?}", Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &bad).unwrap_err());
}
