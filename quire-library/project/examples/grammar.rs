use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = quire_rs::Registry::load_module(Path::new("modules/requirement"))?;
    let fr = registry.archetype("FR").expect("FR is registered");
    let text = std::fs::read_to_string("requirements-draft.md")?;
    let result = quire_rs::validate_document_in_registry(&registry, fr, &text);
    // The grammar findings are warnings, so the document is still valid.
    println!("valid: {}, errors: {}, warnings: {}", result.is_valid, result.errors.len(), result.warnings.len());
    for w in &result.warnings {
        println!("{} (line {:?}): {}", w.reason.as_str(), w.line, w.message);
    }
    Ok(())
}
