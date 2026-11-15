use tl_parse::tl_syntax::SemanticProfile;
use tl_parse::{
    format_document, parse, parse_clean_ascii_v2, parse_clean_ascii_v3, FormatLimits, ParseLimits,
};

fn main() {
    let report = parse(
        "G[0,2]( p0 -> F[1,1] p1 )",
        SemanticProfile::ClosedTraceV1,
        ParseLimits::default(),
    );
    let document = report.document.expect("the source parses");
    let text = format_document(&document, FormatLimits::default()).text.unwrap();
    println!("canonical: {text}");

    let refused = parse("p0 U[3,1] p1", SemanticProfile::ClosedTraceV1, ParseLimits::default());
    println!("document: {}", refused.document.is_some());
    println!("{}", refused.diagnostics[0]);

    // Dialect v2 adds W and M, which become primitive nodes.
    let v2 = parse_clean_ascii_v2("p0 W[0,4] p1", SemanticProfile::ClosedTraceV1, ParseLimits::default());
    let w = v2.document.unwrap();
    println!("v2: {}", format_document(&w, FormatLimits::default()).text.unwrap());

    // Dialect v3 reads the past operators.
    let v3 = parse_clean_ascii_v3("O[0,2]p0 S[1,3] Yp1", ParseLimits::default());
    println!("v3 diagnostics: {}", v3.diagnostics.len());
}
