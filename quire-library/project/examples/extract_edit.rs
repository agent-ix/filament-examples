use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = quire_rs::Registry::load_module(Path::new("modules/runbook"))?;
    let runbook = registry.archetype("Runbook").expect("Runbook is registered");
    let text = std::fs::read_to_string("runbooks/RB-002-rotate-keys.md")?;
    let doc = quire_rs::parse_document(&text);

    // Extract records with the rules of the type.
    let dsl = runbook.body_extraction().expect("Runbook declares body_extraction");
    let extraction = quire_rs::extract(&doc, dsl)?;
    println!("records: {}", extraction.records.len());
    println!("{}", serde_json::to_string_pretty(&extraction.records[0])?);

    // Replace the body of one section.
    let edited = quire_rs::update_section(&doc, "Purpose", "\nRotate the keys every quarter.\n\n")?;
    let changed: Vec<&str> = edited.lines().zip(text.lines()).filter(|(a, b)| a != b).map(|(a, _)| a).collect();
    println!("changed lines: {changed:?}");

    // Replace a whole block, heading line included, by its {#name} attribute.
    let small = quire_rs::parse_document("# Note\n\n## Purpose {#blk-purpose}\n\nOld.\n");
    let renamed = quire_rs::update_block(&small, "blk-purpose", "## Goal {#blk-purpose}\n\nNew.\n")?;
    println!("{renamed:?}");
    // A heading that is not in the document is an error, and nothing changes.
    println!("{}", quire_rs::update_section(&doc, "Missing", "x").unwrap_err());
    Ok(())
}
