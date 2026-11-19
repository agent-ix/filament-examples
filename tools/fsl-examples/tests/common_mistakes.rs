//! Real user mistakes through the public CLI (QSL-2482).
//!
//! Run the explicit live lane with FSL set to the current-main driver. Sources
//! are original temporary user files; no sibling source or schema is embedded.

use fsl_examples::runner::{Driver, ProcessDriver};
use serde_json::Value;
use std::fs;
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const HEADER: &str =
    "language \"ix:native\" edition \"1-draft\";\nprofile v = \"filament.value.complete/v1\";\n";

struct Expected<'a> {
    exit: i32,
    code: &'a str,
    cause: Option<&'a str>,
    stage: Option<&'a str>,
    context: &'a str,
    source_locus: bool,
    hint: bool,
}

fn assert_refusal(dir: &Path, args: &[&str], expected: Expected<'_>) -> TestResult {
    let binary = std::env::var_os("FSL").ok_or("set FSL to the current-main driver")?;
    let run = ProcessDriver::new(binary).run(
        dir,
        &args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
    )?;
    assert_eq!(run.exit, expected.exit, "{args:?}: {}", run.stdout);
    let outcome: Value = serde_json::from_str(&run.stdout)?;
    assert_eq!(outcome["format"], "filament-outcome/1", "{outcome}");
    assert_eq!(outcome["last_stage"].as_str(), expected.stage, "{outcome}");
    let diagnostics = outcome["diagnostics"]
        .as_array()
        .ok_or("missing diagnostics array")?;
    let diagnostic = diagnostics
        .iter()
        .find(|d| d["code"] == expected.code && d["cause"].as_str() == expected.cause)
        .ok_or_else(|| format!("missing {}/ {:?}: {outcome}", expected.code, expected.cause))?;
    assert!(
        diagnostic.get("locus").is_some(),
        "missing locus: {outcome}"
    );
    if expected.source_locus {
        assert!(
            !diagnostic["locus"].is_null(),
            "source error has no location: {outcome}"
        );
        assert!(
            diagnostic["locus"].to_string().contains("mistake.fsl"),
            "wrong source: {outcome}"
        );
    }
    let context = diagnostic.to_string();
    assert!(
        context.contains(expected.context),
        "missing offending context {:?}: {outcome}",
        expected.context
    );
    if expected.hint {
        let hint = diagnostic["hint"]
            .as_str()
            .ok_or_else(|| format!("missing fix hint: {outcome}"))?;
        assert!(
            !hint.trim().is_empty() && hint.ends_with('.') && hint.len() <= 200,
            "invalid fix hint: {outcome}"
        );
    }
    Ok(())
}

fn source_mistake(body: &str, expected: Expected<'_>, options: &[&str]) -> TestResult {
    let dir = tempfile::tempdir()?;
    fs::write(dir.path().join("mistake.fsl"), format!("{HEADER}{body}\n"))?;
    let mut args = vec!["check", "mistake.fsl", "--format", "json"];
    args.extend_from_slice(options);
    assert_refusal(dir.path(), &args, expected)
}

macro_rules! source_case {
    ($name:ident, $body:expr, $code:expr, $cause:expr, $stage:expr, $context:expr) => {
        #[test]
        #[ignore = "QSL-2482 live mistake journey: requires current-main FSL"]
        fn $name() -> TestResult {
            source_mistake(
                $body,
                Expected {
                    exit: 20,
                    code: $code,
                    cause: Some($cause),
                    stage: Some($stage),
                    context: $context,
                    source_locus: true,
                    hint: true,
                },
                &[],
            )
        }
    };
}

source_case!(
    misspelled_function_keyword,
    "functon quote using v(): Integer pure { 4 }",
    "invalid_syntax",
    "unexpected-token",
    "S1",
    "functon"
);
source_case!(
    unclosed_function_body,
    "function quote using v(): Integer pure { 4",
    "invalid_syntax",
    "unexpected-end",
    "S1",
    "expected"
);
source_case!(
    invalid_character_in_body,
    "function quote using v(): Integer pure { ` }",
    "invalid_syntax",
    "invalid-token",
    "S1",
    "token"
);
source_case!(
    invalid_text_escape,
    r#"function quote using v(): String pure { "\q" }"#,
    "invalid_syntax",
    "invalid-escape",
    "S1",
    "escape"
);
source_case!(
    integer_returned_as_boolean,
    "function eligible using v(): Boolean pure { 4 }",
    "ill_typed",
    "type-mismatch",
    "S3",
    "Boolean"
);
source_case!(
    boolean_returned_as_integer,
    "function quantity using v(): Integer pure { false }",
    "ill_typed",
    "type-mismatch",
    "S3",
    "Integer"
);
source_case!(
    undeclared_price_name,
    "function quote using v(): Integer pure { missingPrice }",
    "missing_declaration",
    "missing-name",
    "S3",
    "missingPrice"
);
source_case!(
    boolean_added_to_integer,
    "function quote using v(): Integer pure { true + 4 }",
    "ill_typed",
    "operator-ineligible",
    "S3",
    "+"
);
source_case!(
    division_without_selected_law,
    "function quote using v(): Integer pure { 9 div 2 }",
    "missing_declaration",
    "missing-selection",
    "S3",
    "division"
);
source_case!(
    library_not_supplied,
    "import \"price-library\";\nfunction quote using v(): Integer pure { 4 }",
    "missing_import",
    "missing-selection",
    "S3",
    "price-library"
);

#[test]
#[ignore = "QSL-2482 live mistake journey: requires current-main FSL"]
fn trailing_unsupported_declaration() -> TestResult {
    let dir = tempfile::tempdir()?;
    fs::write(
        dir.path().join("mistake.fsl"),
        format!("{HEADER}function quote using v(): Integer pure {{ 4 }}\nmisspelled\n"),
    )?;
    assert_refusal(
        dir.path(),
        &["check", "mistake.fsl", "--format", "json"],
        Expected {
            exit: 21,
            code: "unsupported_construct",
            cause: Some("declaration-form"),
            stage: Some("S2"),
            context: "misspelled",
            source_locus: true,
            hint: true,
        },
    )
}

#[test]
#[ignore = "QSL-2482 live mistake journey: requires current-main FSL"]
fn unknown_integer_division_selection() -> TestResult {
    let dir = tempfile::tempdir()?;
    assert_refusal(
        dir.path(),
        &[
            "check",
            "mistake.fsl",
            "--integer-division",
            "ceiling",
            "--format",
            "json",
        ],
        Expected {
            exit: 20,
            code: "unknown_profile",
            cause: Some("unsupported-selection"),
            stage: None,
            context: "ceiling",
            source_locus: false,
            hint: true,
        },
    )
}

macro_rules! argument_case {
    ($name:ident, [$($arg:expr),+], $code:expr, $context:expr, $hint:expr) => {
        #[test]
        #[ignore = "QSL-2482 live mistake journey: requires current-main FSL"]
        fn $name() -> TestResult {
            let dir = tempfile::tempdir()?;
            assert_refusal(dir.path(), &[$($arg),+], Expected { exit: 20, code: $code, cause: None, stage: None, context: $context, source_locus: false, hint: $hint })
        }
    };
}

argument_case!(
    missing_check_source,
    ["check", "missing-source.fsl", "--format", "json"],
    "io-error",
    "missing-source.fsl",
    true
);
argument_case!(
    missing_run_request,
    ["run", "missing-request.json", "--format", "json"],
    "io-error",
    "missing-request.json",
    true
);
argument_case!(
    missing_parse_source,
    [
        "parse",
        "shop.example",
        "prices",
        "missing-parse.fsl",
        "--format",
        "json"
    ],
    "io-error",
    "missing-parse.fsl",
    true
);
argument_case!(
    misspelled_format_option,
    ["check", "unused.fsl", "--fomat", "json", "--format", "json"],
    "invalid-request",
    "--fomat",
    false
);
argument_case!(
    unknown_output_format,
    ["check", "unused.fsl", "--format", "yaml"],
    "invalid-request",
    "yaml",
    false
);
argument_case!(
    unknown_limit_setting,
    [
        "check",
        "unused.fsl",
        "--limit",
        "s1.typo=3",
        "--format",
        "json"
    ],
    "invalid-request",
    "s1.typo",
    false
);
argument_case!(
    limit_missing_equals,
    [
        "check",
        "unused.fsl",
        "--limit",
        "s1.tokens",
        "--format",
        "json"
    ],
    "invalid-request",
    "s1.tokens",
    false
);

#[test]
#[ignore = "QSL-2482 live mistake journey: requires current-main FSL"]
fn input_exceeds_explicit_byte_budget() -> TestResult {
    source_mistake(
        "function quote using v(): Integer pure { 4 }",
        Expected {
            exit: 22,
            code: "stage_limit_exceeded",
            cause: Some("input-bytes-exceeded"),
            stage: Some("S1"),
            context: "s1.input_bytes",
            source_locus: true,
            hint: true,
        },
        &["--limit", "s1.input_bytes=1"],
    )
}
