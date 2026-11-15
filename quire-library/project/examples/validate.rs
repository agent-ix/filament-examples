use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load one module directory into an immutable registry.
    let registry = quire_rs::Registry::load_module(Path::new("modules/runbook"))?;
    println!("types: {:?}", registry.archetype_names().collect::<Vec<_>>());
    let runbook = registry.archetype("Runbook").expect("Runbook is registered");

    // Validate a document.
    let text = std::fs::read_to_string("runbooks/RB-001-restart-the-queue.md")?;
    let result = quire_rs::validate_document_in_registry(&registry, runbook, &text);
    println!("valid: {}", result.is_valid);

    // Validate a copy with the Purpose section renamed.
    let broken = text.replace("## Purpose", "## Overview");
    for e in &quire_rs::validate_document_in_registry(&registry, runbook, &broken).errors {
        println!("{:?}: {}", e.reason, e.message);
    }
    Ok(())
}
