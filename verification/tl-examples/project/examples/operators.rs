use tl_mltl::{
    analyze_horizon, evaluate_closed, evaluate_past, evaluate_prefix, ClockBinding,
    EvaluationLimits, PastEvaluationLimits, PastEvaluationRelationInput, PositionHistoryDocument,
    PositionObservation, TruthValue,
};
use tl_parse::tl_syntax::{FormulaDocument, PropositionId, SemanticProfile};
use tl_parse::{parse, parse_clean_ascii_v2, parse_clean_ascii_v3, ParseLimits};

// A trace is written as instants separated by ';'. An instant lists the true propositions
// separated by ',' and may be empty.
fn trace(text: &str) -> Vec<Vec<PropositionId>> {
    if text.is_empty() {
        return vec![];
    }
    text.split(';')
        .map(|instant| {
            let mut ids: Vec<PropositionId> = instant
                .split(',')
                .filter(|p| !p.is_empty())
                .map(|p| PropositionId(p[1..].parse().unwrap()))
                .collect();
            ids.sort();
            ids
        })
        .collect()
}

fn word(v: TruthValue) -> &'static str {
    match v {
        TruthValue::True => "true",
        TruthValue::False => "false",
        TruthValue::Pending => "pending",
    }
}

fn future(source: &str, derived: bool, profile: SemanticProfile) -> FormulaDocument {
    let limits = ParseLimits::default();
    let report = if derived {
        parse_clean_ascii_v2(source, profile, limits).document
    } else {
        parse(source, profile, limits).document
    };
    report.expect("the source parses")
}

fn main() {
    println!("-- closed and open prefix");
    let rows = [
        ("p0", "p0", false), ("p0", "p1", false), ("p0", "", false), ("!p0", "p1", false),
        ("p0&p1", "p0,p1", false), ("p0|p1", "p1", false), ("p0->p1", "p0", false),
        ("p0<->p1", "p0,p1", false), ("true", "", false), ("false", "p0", false),
        ("F[0,2] p0", "p1;p1;p0", false), ("F[0,2] p0", "p1;p1;p1", false),
        ("F[0,2] p0", "p1;p1", false), ("G[0,2] p0", "p0;p0;p0", false),
        ("G[0,2] p0", "p0;p1;p0", false), ("G[0,2] p0", "p0;p0", false),
        ("F[2,4] p0", "p0;p0;p0;p0;p0", false), ("F[2,4] p0", "p0;p0", false),
        ("G[0,0] p0", "p0", false), ("F[0,1] G[0,1] p0", "p1;p0;p0", false),
        ("p0 U[0,3] p1", "p0;p0;p1", false), ("p0 U[1,2] p1", "p1;p0;p1", false),
        ("p0 U[1,2] p1", "p1;p1;p1", false), ("p0 R[0,2] p1", "p1;p1;p1", false),
        ("p0 R[0,2] p1", "p1;p1,p0;p1", false), ("p0 R[0,2] p1", "p1;p0", false),
        ("p0 R[0,2] p1", "p1;p1", false), ("G[0,5](p0 -> F[1,3] p1)", "p0;;p1;p0;;", false),
        ("G[0,3](p0 -> F[1,2] p1)", "p0;p1;p0;", false),
        ("p0 W[0,2] p1", "p0;p0;p0", true), ("p0 W[0,2] p1", "p0;p1", true),
        ("p0 W[0,2] p1", "p1;p0", true), ("p0 W[0,2] p1", "p0;;p1", true),
        ("p0 M[0,2] p1", "p1;p1,p0;p1", true), ("p0 M[0,2] p1", "p1;p1;p1", true),
        ("p0 M[0,2] p1", "p1,p0;p1", true),
    ];
    for (text, tr, derived) in rows {
        let t = trace(tr);
        let closed = future(text, derived, SemanticProfile::ClosedTraceV1);
        let c = evaluate_closed(closed.validate().unwrap(), "f", &t, "t", EvaluationLimits::default()).unwrap();
        let open = future(text, derived, SemanticProfile::OnlinePrefixV1);
        let o = evaluate_prefix(open.validate().unwrap(), "f", &t, "t", false, EvaluationLimits::default()).unwrap();
        println!("| `{}` | `{}` | {} | {} |", text.replace('|', "\\|"), tr, word(c.verdict), word(o.verdict));
    }

    println!("-- past");
    let rows = [
        ("O[0,2] p0", "p0;p1;p1;p1", 3), ("O[0,2] p0", "p0;p1;p1;p1", 2),
        ("O[0,2] p0", "p0;p1;p1;p1", 1), ("H[0,1] p0", "p0;p0;p1", 1),
        ("H[0,1] p0", "p0;p0;p1", 2), ("Y p0", "p0;p1", 1), ("Y p0", "p0;p1", 0),
        ("Y p0", "p1;p1", 1), ("O[1,1] p0", "p0;p1", 1), ("p0 S[0,3] p1", "p1;p0;p0;p0", 3),
        ("p0 S[0,3] p1", "p1;p1;p0;p0", 3), ("p0 S[1,3] p1", "p1;p0;p0;p0", 3),
        ("p0 S[0,2] p1", "p1;p0;p0;p0", 3), ("p0 T[0,2] p1", "p1;p1;p1;p1", 3),
        ("p0 T[0,2] p1", "p1;p1;p0;p1", 3), ("Y !p0", "p0;p0", 0), ("H[0,2] p0", "p0;p0", 1),
    ];
    for (text, tr, anchor) in rows {
        let doc = parse_clean_ascii_v3(text, ParseLimits::default()).document.expect("parses");
        let t = trace(tr);
        let observations = t
            .iter()
            .enumerate()
            .map(|(i, props)| PositionObservation::new(i as u64, props.clone(), None))
            .collect();
        let history = PositionHistoryDocument::new(
            "h", 1, 0, t.len() as u64 - 1, Some(ClockBinding::EventPosition), observations,
        )
        .unwrap();
        let r = evaluate_past(
            doc.validate().unwrap(), "f", &history, anchor, "m", 1,
            PastEvaluationRelationInput::Original, PastEvaluationLimits::default(),
        )
        .unwrap();
        println!("| `{}` | `{}` | {} | {} |", text, tr, anchor, r.verdict);
    }

    println!("-- horizon");
    for (text, derived) in [
        ("p0&p1", false), ("F[0,2] p0", false), ("F[2,4] p0", false), ("p0 U[0,3] p1", false),
        ("p0 R[0,2] p1", false), ("G[0,3](p0 -> F[1,2] p1)", false), ("G[0,5](p0 -> F[1,3] p1)", false),
        ("p0 W[0,2] p1", true),
    ] {
        let doc = future(text, derived, SemanticProfile::ClosedTraceV1);
        let h = analyze_horizon(doc.validate().unwrap(), "f").unwrap();
        println!("| `{}` | {} | {} | {} |", text, h.lookahead, h.propagation_delay, h.required_buffer);
    }
}
