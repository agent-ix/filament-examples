use filament_verification_contracts::{
    contract_version, jcs_equal, jcs_sha256, parse_bounded_json, validate_contract,
    VerificationErrorCode, MAX_JSON_BYTES,
};
use serde_json::json;

fn main() {
    println!("schema version: {}", contract_version());

    // The same fields in a different order are the same record.
    let left = json!({"b": 1, "a": 2});
    let right = json!({"a": 2, "b": 1});
    println!("same record: {}", jcs_equal(&left, &right).unwrap());
    println!("digest: {}", jcs_sha256(&json!({"kind": "example"})).unwrap());

    // An unknown contract kind is refused with a stable code.
    let error = validate_contract(json!({"kind": "NotAContract"})).unwrap_err();
    println!("unknown kind: {}", error.code().as_str());
    println!("code is schema_violation: {}", error.code() == VerificationErrorCode::SchemaViolation);

    // A document over the byte limit is refused before it is processed.
    let oversized = vec![b'a'; MAX_JSON_BYTES + 1];
    let error = parse_bounded_json(&oversized).unwrap_err();
    println!("oversized: {}", error.code().as_str());
}
