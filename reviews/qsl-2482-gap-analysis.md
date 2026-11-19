---
id: SR-1434
title: "gap-analysis — QSL-2482 common mistakes"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-examples; tools/fsl-examples/tests/common_mistakes.rs; QSL-2482"
review_set: subset
---

## Summary

Ticket: QSL-2482. Reviewed frozen PR filament-examples#61 at c574e804218e9c1ed70ab5c66fc022fac94abde3. Only the added native common_mistakes.rs test module is in scope. No production, dependency, workflow, schema, or compatibility changes. Original inline user inputs exercise the existing public ProcessDriver seam; helper search found no existing refusal/location helper to reuse. No applicable local spec/AssuranceProfile exists.

Accepted peer evidence: corrected 19 focused contracts passed; added missing-limit-operand contract passed; 11 strengthened location cases passed; targeted Clippy passed. Tests, builds, matrix, baseline, and CLI receipt were not repeated. The lead owns the one final 20-contract gate. The two strict owner-pending tests are excluded from runnable coverage; QSL-2477 / FSL PR3102 and QSL-2642 remain upstream dependencies. No ticket completion or repository-wide assurance is claimed.

## Verdict

**PASS** — the original medium finding is fixed at 22dc710da348666418dc454ce5fab656379ea451; upstream pending behavior remains excluded from coverage.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The strict pending invalid-format case never asserts an actionable fix, so adding only the flag name and rejected value can satisfy it. | tools/fsl-examples/tests/common_mistakes.rs:353 | correct-requirement-no-evidence |

The assigned QSL-2642 behavior includes both the offending --format/yaml context and an actionable fix. A message such as "invalid --format yaml" satisfies every current assertion while still offering no correction. Keep this test pending under the same owner and require a concrete supported format or equivalent actionable correction before it can pass.

## Coverage

Reconciliation: manual diff-scoped acceptance-to-assertion check; no matrix executed by explicit dispatch instruction. No root spec or declared root traceability model; no fabricated criterion tags.

Plan completion: not assessed

Semantic intent of the assigned mistake contracts was examined as requested; this is not a repository-wide gap audit. No stubs, exit-only assertions, copied schemas/models/binaries, or coverage claims over the two pending tests were added.

## Examined units

```yaml
{
  "scope": [
    {
      "id": "assert_refusal",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn assert_refusal(dir: &Path, args: &[&str], expected: Expected<'_>) -> TestResult {\n    let binary = std::env::var_os(\"FSL\").ok_or(\"set FSL to the current-main driver\")?;\n    let run = ProcessDriver::new(binary).run(\n        dir,\n        &args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),\n    )?;\n    assert_eq!(run.exit, expected.exit, \"{args:?}: {}\", run.stdout);\n    let outcome: Value = serde_json::from_str(&run.stdout)?;\n    assert_eq!(outcome[\"format\"], \"filament-outcome/1\", \"{outcome}\");\n    assert_eq!(outcome[\"last_stage\"].as_str(), expected.stage, \"{outcome}\");\n    let diagnosti"
    },
    {
      "id": "source_mistake",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn source_mistake(body: &str, expected: Expected<'_>, options: &[&str]) -> TestResult {\n    let dir = tempfile::tempdir()?;\n    fs::write(dir.path().join(\"mistake.fsl\"), format!(\"{HEADER}{body}\\n\"))?;\n    let mut args = vec![\"check\", \"mistake.fsl\", \"--format\", \"json\"];\n    args.extend_from_slice(options);\n    assert_refusal(dir.path(), &args, expected)\n}"
    },
    {
      "id": "source_case",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "macro_rules! source_case {\n    ($name:ident, $body:expr, $code:expr, $cause:expr, $stage:expr, $context:expr) => {\n        #[test]\n        #[ignore = \"QSL-2482 live mistake journey: requires current-main FSL\"]\n        fn $name() -> TestResult {\n            source_mistake(\n                $body,\n                Expected {\n                    exit: 20,\n                    code: $code,\n                    cause: Some($cause),\n                    stage: Some($stage),\n                    context: $context,\n                    source_locus: true,\n                    hint: false,\n                },\n "
    },
    {
      "id": "misspelled_pure_keyword",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    misspelled_pure_keyword,\n    \"function quote using v(): Integer pur { 4 }\",\n    \"invalid_syntax\",\n    \"unexpected-token\",\n    \"S1\",\n    \"pur\"\n);"
    },
    {
      "id": "unclosed_function_body",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    unclosed_function_body,\n    \"function quote using v(): Integer pure { 4\",\n    \"invalid_syntax\",\n    \"unexpected-end\",\n    \"S1\",\n    \"expected\"\n);"
    },
    {
      "id": "invalid_character_in_body",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    invalid_character_in_body,\n    \"function quote using v(): Integer pure { ` }\",\n    \"invalid_syntax\",\n    \"invalid-token\",\n    \"S1\",\n    \"`\"\n);"
    },
    {
      "id": "invalid_text_escape",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    invalid_text_escape,\n    r#\"function quote using v(): String pure { \"\\q\" }\"#,\n    \"invalid_syntax\",\n    \"invalid-escape\",\n    \"S1\",\n    \"\\\\q\"\n);"
    },
    {
      "id": "integer_returned_as_boolean",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    integer_returned_as_boolean,\n    \"function eligible using v(): Boolean pure { 4 }\",\n    \"ill_typed\",\n    \"type-mismatch\",\n    \"S3\",\n    \"4\"\n);"
    },
    {
      "id": "boolean_returned_as_integer",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    boolean_returned_as_integer,\n    \"function quantity using v(): Integer pure { false }\",\n    \"ill_typed\",\n    \"type-mismatch\",\n    \"S3\",\n    \"false\"\n);"
    },
    {
      "id": "undeclared_price_name",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    undeclared_price_name,\n    \"function quote using v(): Integer pure { missingPrice }\",\n    \"missing_declaration\",\n    \"missing-name\",\n    \"S3\",\n    \"missingPrice\"\n);"
    },
    {
      "id": "boolean_added_to_integer",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    boolean_added_to_integer,\n    \"function quote using v(): Integer pure { true + 4 }\",\n    \"ill_typed\",\n    \"type-mismatch\",\n    \"S3\",\n    \"+\"\n);"
    },
    {
      "id": "division_without_selected_law",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    division_without_selected_law,\n    \"function quote using v(): Integer pure { 9 div 2 }\",\n    \"missing_declaration\",\n    \"missing-selection\",\n    \"S3\",\n    \"div\"\n);"
    },
    {
      "id": "library_not_supplied",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "source_case!(\n    library_not_supplied,\n    \"import \\\"price-library\\\";\\nfunction quote using v(): Integer pure { 4 }\",\n    \"missing_import\",\n    \"missing-selection\",\n    \"I1\",\n    \"price-library\"\n);\n\n#[test]\n#[ignore = \"QSL-2482 live mistake journey: requires current-main FSL\"]"
    },
    {
      "id": "trailing_unsupported_declaration",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn trailing_unsupported_declaration() -> TestResult {\n    let dir = tempfile::tempdir()?;\n    fs::write(\n        dir.path().join(\"mistake.fsl\"),\n        format!(\"{HEADER}function quote using v(): Integer pure {{ 4 }}\\nmisspelled\\n\"),\n    )?;\n    assert_refusal(\n        dir.path(),\n        &[\"check\", \"mistake.fsl\", \"--format\", \"json\"],\n        Expected {\n            exit: 21,\n            code: \"unsupported_construct\",\n            cause: Some(\"declaration-form\"),\n            stage: Some(\"S1\"),\n            context: \"misspelled\",\n            source_locus: true,\n            hint: false,\n        },\n"
    },
    {
      "id": "unknown_integer_division_selection",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn unknown_integer_division_selection() -> TestResult {\n    let dir = tempfile::tempdir()?;\n    assert_refusal(\n        dir.path(),\n        &[\n            \"check\",\n            \"mistake.fsl\",\n            \"--integer-division\",\n            \"ceiling\",\n            \"--format\",\n            \"json\",\n        ],\n        Expected {\n            exit: 20,\n            code: \"unknown_profile\",\n            cause: Some(\"unsupported-selection\"),\n            stage: None,\n            context: \"ceiling\",\n            source_locus: false,\n            hint: false,\n        },\n    )\n}"
    },
    {
      "id": "argument_case",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "macro_rules! argument_case {\n    ($name:ident, [$($arg:expr),+], $code:expr, $context:expr) => {\n        #[test]\n        #[ignore = \"QSL-2482 live mistake journey: requires current-main FSL\"]\n        fn $name() -> TestResult {\n            let dir = tempfile::tempdir()?;\n            assert_refusal(dir.path(), &[$($arg),+], Expected { exit: 20, code: $code, cause: None, stage: None, context: $context, source_locus: false, hint: false })\n        }\n    };\n}"
    },
    {
      "id": "missing_check_source",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    missing_check_source,\n    [\"check\", \"missing-source.fsl\", \"--format\", \"json\"],\n    \"io-error\",\n    \"missing-source.fsl\"\n);"
    },
    {
      "id": "missing_run_request",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    missing_run_request,\n    [\"run\", \"missing-request.json\", \"--format\", \"json\"],\n    \"io-error\",\n    \"missing-request.json\"\n);"
    },
    {
      "id": "missing_parse_source",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    missing_parse_source,\n    [\n        \"parse\",\n        \"shop.example\",\n        \"prices\",\n        \"missing-parse.fsl\",\n        \"--format\",\n        \"json\"\n    ],\n    \"io-error\",\n    \"missing-parse.fsl\"\n);"
    },
    {
      "id": "misspelled_format_option",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    misspelled_format_option,\n    [\"check\", \"unused.fsl\", \"--fomat\", \"json\", \"--format\", \"json\"],\n    \"invalid-request\",\n    \"--fomat\"\n);"
    },
    {
      "id": "unknown_limit_setting",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    unknown_limit_setting,\n    [\n        \"check\",\n        \"unused.fsl\",\n        \"--limit\",\n        \"s1.typo=3\",\n        \"--format\",\n        \"json\"\n    ],\n    \"invalid-request\",\n    \"s1.typo\"\n);"
    },
    {
      "id": "limit_missing_equals",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    limit_missing_equals,\n    [\n        \"check\",\n        \"unused.fsl\",\n        \"--limit\",\n        \"s1.tokens\",\n        \"--format\",\n        \"json\"\n    ],\n    \"invalid-request\",\n    \"s1.tokens\"\n);\n\n#[test]\n#[ignore = \"QSL-2482 live mistake journey: requires current-main FSL\"]"
    },
    {
      "id": "input_exceeds_explicit_byte_budget",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn input_exceeds_explicit_byte_budget() -> TestResult {\n    source_mistake(\n        \"function quote using v(): Integer pure { 4 }\",\n        Expected {\n            exit: 22,\n            code: \"stage_limit_exceeded\",\n            cause: Some(\"input-bytes-exceeded\"),\n            stage: Some(\"S1\"),\n            context: \"s1.input_bytes\",\n            source_locus: false,\n            hint: false,\n        },\n        &[\"--limit\", \"s1.input_bytes=1\"],\n    )\n}\n\n#[test]\n#[ignore = \"QSL-2642: invalid output-format diagnostic omits --format and a fix\"]"
    },
    {
      "id": "unknown_output_format_pending_",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn unknown_output_format_pending_2642() -> TestResult {\n    let binary = std::env::var_os(\"FSL\").ok_or(\"set FSL to the current-main driver\")?;\n    let dir = tempfile::tempdir()?;\n    let output = Command::new(binary)\n        .args([\"check\", \"unused.fsl\", \"--format\", \"yaml\"])\n        .current_dir(dir.path())\n        .output()?;\n    assert_eq!(output.status.code(), Some(20), \"{output:?}\");\n    let diagnostic = String::from_utf8(output.stderr)?;\n    assert!(\n        diagnostic.contains(\"--format\") && diagnostic.contains(\"yaml\"),\n        \"{diagnostic}\"\n    );\n    assert!(\n        !diagnostic.conta"
    },
    {
      "id": "catalogued_fix_hints_pending_",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "fn catalogued_fix_hints_pending_2477() -> TestResult {\n    source_mistake(\n        \"function eligible using v(): Boolean pure { 4 }\",\n        Expected {\n            exit: 20,\n            code: \"ill_typed\",\n            cause: Some(\"type-mismatch\"),\n            stage: Some(\"S3\"),\n            context: \"4\",\n            source_locus: true,\n            hint: true,\n        },\n        &[],\n    )?;\n    source_mistake(\n        \"function quote using v(): Integer pure { 4\",\n        Expected {\n            exit: 20,\n            code: \"invalid_syntax\",\n            cause: Some(\"unexpected-end\"),\n            s"
    },
    {
      "id": "limit_flag_without_operand",
      "path": "tools/fsl-examples/tests/common_mistakes.rs",
      "role": "examined",
      "excerpt": "argument_case!(\n    limit_flag_without_operand,\n    [\"check\", \"unused.fsl\", \"--format\", \"json\", \"--limit\"],\n    \"invalid-request\",\n    \"--limit\"\n);"
    }
  ]
}
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The pending oracle additionally requires a usable --format text or --format json correction, without a full-prose golden. Pending upstream behavior remains pending; no known-bug test was rerun. |

Fix inspected at 22dc710da348666418dc454ce5fab656379ea451. No test, build, matrix, metadata, or full-review repetition.

```rust
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
```
