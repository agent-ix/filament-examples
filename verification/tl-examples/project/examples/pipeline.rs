use tl_mltl::{analyze_horizon, evaluate_closed, EvaluationLimits};
use tl_parse::tl_syntax::{PropositionId, SemanticProfile};
use tl_parse::{parse, ParseLimits};
use tl_rewrite::{rewrite, RewriteOptions};

fn main() {
    // Text in, verdict out: parse, rewrite to a normal form, evaluate on a trace.
    let source = "G[0,2](true & p0)";
    let report = parse(source, SemanticProfile::ClosedTraceV1, ParseLimits::default());
    let document = report.document.expect("the source parses");

    let rewritten = rewrite(&document, "deadline", RewriteOptions::default(), "rev-1");
    let normal = rewritten.output.as_ref().unwrap_or(&document);
    println!("rules applied: {}", rewritten.steps.len());

    let formula = normal.validate().unwrap();
    // Three instants; p0 is true at the first two and false at the third.
    let trace = vec![vec![PropositionId(0)], vec![PropositionId(0)], vec![]];
    let result = evaluate_closed(formula, "deadline", &trace, "run-1", EvaluationLimits::default()).unwrap();
    println!("verdict: {:?}", result.verdict);

    let horizon = analyze_horizon(formula, "deadline").unwrap();
    println!("lookahead {}, buffer {}", horizon.lookahead, horizon.required_buffer);
}
