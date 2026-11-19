//! Real user mistakes through the public CLI (QSL-2482).
//!
//! Run the explicit live lane with FSL set to the current-main driver. Sources
//! are original temporary user files; no sibling source or schema is embedded.
//! The 20 live contracts cover stage, refusal identity and offending context.
//! Fix-hint acceptance remains partial: select the live lane with `--ignored
//! --skip pending`; the two strict pending tests retain their owner IDs.

use fsl_examples::runner::{Driver, ProcessDriver};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

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
    if expected.source_locus {
        let start = diagnostic["locus"]["start"]
            .as_u64()
            .ok_or("missing region start")?;
        let end = diagnostic["locus"]["end"]
            .as_u64()
            .ok_or("missing region end")?;
        let source = fs::read_to_string(dir.join("mistake.fsl"))?;
        let region = source
            .get(usize::try_from(start)?..usize::try_from(end)?)
            .ok_or("invalid source region")?;
        if expected.code == "invalid_syntax" && expected.cause == Some("unexpected-end") {
            assert!(
                region.is_empty(),
                "unfinished declaration must locate EOF: {outcome}"
            );
            assert_eq!(
                usize::try_from(end)?,
                source.len(),
                "EOF refusal must point to EOF: {outcome}"
            );
        } else {
            assert_eq!(
                region, expected.context,
                "wrong offending source span: {outcome}"
            );
            assert_eq!(
                usize::try_from(start)?,
                source
                    .find(expected.context)
                    .ok_or("expected offending source absent from fixture")?,
                "wrong offending source offset: {outcome}"
            );
        }
    }
    assert!(
        expected.source_locus || context.contains(expected.context),
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
                    hint: false,
                },
                &[],
            )
        }
    };
}

source_case!(
    misspelled_pure_keyword,
    "function quote using v(): Integer pur { 4 }",
    "invalid_syntax",
    "unexpected-token",
    "S1",
    "pur"
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
    "`"
);
source_case!(
    invalid_text_escape,
    r#"function quote using v(): String pure { "\q" }"#,
    "invalid_syntax",
    "invalid-escape",
    "S1",
    "\"\\q\""
);
source_case!(
    integer_returned_as_boolean,
    "function eligible using v(): Boolean pure { 4 }",
    "ill_typed",
    "type-mismatch",
    "S3",
    "4"
);
source_case!(
    boolean_returned_as_integer,
    "function quantity using v(): Integer pure { false }",
    "ill_typed",
    "type-mismatch",
    "S3",
    "false"
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
    "type-mismatch",
    "S3",
    "true + 4"
);
source_case!(
    division_without_selected_law,
    "function quote using v(): Integer pure { 9 div 2 }",
    "missing_declaration",
    "missing-selection",
    "S3",
    "9 div 2"
);
source_case!(
    library_not_supplied,
    "import \"price-library\";\nfunction quote using v(): Integer pure { 4 }",
    "missing_import",
    "missing-selection",
    "I1",
    "\"price-library\""
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
            stage: Some("S1"),
            context: "misspelled",
            source_locus: true,
            hint: false,
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
            hint: false,
        },
    )
}

macro_rules! argument_case {
    ($name:ident, [$($arg:expr),+], $code:expr, $context:expr) => {
        #[test]
        #[ignore = "QSL-2482 live mistake journey: requires current-main FSL"]
        fn $name() -> TestResult {
            let dir = tempfile::tempdir()?;
            assert_refusal(dir.path(), &[$($arg),+], Expected { exit: 20, code: $code, cause: None, stage: None, context: $context, source_locus: false, hint: false })
        }
    };
}

argument_case!(
    missing_check_source,
    ["check", "missing-source.fsl", "--format", "json"],
    "io-error",
    "missing-source.fsl"
);
argument_case!(
    missing_run_request,
    ["run", "missing-request.json", "--format", "json"],
    "io-error",
    "missing-request.json"
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
    "missing-parse.fsl"
);
argument_case!(
    misspelled_format_option,
    ["check", "unused.fsl", "--fomat", "json", "--format", "json"],
    "invalid-request",
    "--fomat"
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
    "s1.typo"
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
    "s1.tokens"
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
            source_locus: false,
            hint: false,
        },
        &["--limit", "s1.input_bytes=1"],
    )
}

#[test]
#[ignore = "QSL-2642: invalid output-format diagnostic omits --format and a fix"]
fn unknown_output_format_pending_2642() -> TestResult {
    let binary = std::env::var_os("FSL").ok_or("set FSL to the current-main driver")?;
    let dir = tempfile::tempdir()?;
    let output = Command::new(binary)
        .args(["check", "unused.fsl", "--format", "yaml"])
        .current_dir(dir.path())
        .output()?;
    assert_eq!(output.status.code(), Some(20), "{output:?}");
    let diagnostic = String::from_utf8(output.stderr)?;
    assert!(
        diagnostic.contains("--format") && diagnostic.contains("yaml"),
        "{diagnostic}"
    );
    let operands: Vec<_> = diagnostic
        .split_whitespace()
        .map(|token| token.trim_matches(['`', '\"', '\'', '.', ',', ';', ':']))
        .collect();
    assert!(
        operands
            .windows(2)
            .any(|pair| pair[0] == "--format" && matches!(pair[1], "text" | "json")),
        "diagnostic must supply a usable correction such as --format json: {diagnostic}"
    );
    assert!(
        !diagnostic.contains("No such file"),
        "options must be checked before reading unused.fsl: {diagnostic}"
    );
    Ok(())
}

// QSL-2477 / FSL PR3102 owns the missing catalogued hints in the accepted
// current-main validation build. This desired oracle stays separate from the
// runnable stage/location contracts above; absence is never accepted as success.
#[test]
#[ignore = "QSL-2477: catalogued fix hints absent in current-main validation CLI"]
fn catalogued_fix_hints_pending_2477() -> TestResult {
    source_mistake(
        "function eligible using v(): Boolean pure { 4 }",
        Expected {
            exit: 20,
            code: "ill_typed",
            cause: Some("type-mismatch"),
            stage: Some("S3"),
            context: "4",
            source_locus: true,
            hint: true,
        },
        &[],
    )?;
    source_mistake(
        "function quote using v(): Integer pure { 4",
        Expected {
            exit: 20,
            code: "invalid_syntax",
            cause: Some("unexpected-end"),
            stage: Some("S1"),
            context: "expected",
            source_locus: true,
            hint: true,
        },
        &[],
    )?;
    let dir = tempfile::tempdir()?;
    assert_refusal(
        dir.path(),
        &["run", "missing-request.json", "--format", "json"],
        Expected {
            exit: 20,
            code: "io-error",
            cause: None,
            stage: None,
            context: "missing-request.json",
            source_locus: false,
            hint: true,
        },
    )
}

argument_case!(
    limit_flag_without_operand,
    ["check", "unused.fsl", "--format", "json", "--limit"],
    "invalid-request",
    "--limit"
);
