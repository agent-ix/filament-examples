use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = quire_rs::Registry::load_module(Path::new("modules/steps-list"))?;
    for f in registry.failures() {
        println!("failure {}/{}: {}", f.module, f.archetype, f.reason);
    }
    let runbook = registry.archetype("Runbook").expect("Runbook is registered");
    let doc = quire_rs::parse_document(&std::fs::read_to_string("runbooks/RB-002-rotate-keys.md")?);
    let extraction = quire_rs::extract(&doc, runbook.body_extraction().expect("body_extraction"))?;
    println!("records: {}", extraction.records.len());
    for r in &extraction.records {
        println!("{}", serde_json::to_string(r)?);
    }
    println!("edges: {:?}", extraction.edges);
    println!("diagnostics: {:?}", extraction.diagnostics);
    Ok(())
}
