use std::path::Path;

const GOOD: &str = "---\nid: RB-010\ntitle: Example\ntype: Runbook\n---\n# RB-010: Example\n\n## Purpose\n\nStop the job.\n\n## Steps\n\n- Stop it.\n";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = quire_rs::Registry::load_module(Path::new("modules/runbook"))?;
    let runbook = registry.archetype("Runbook").expect("Runbook is registered");
    let cases = [
        ("a valid document", GOOD.to_string()),
        ("an id that breaks the schema", GOOD.replace("RB-010", "X-10")),
        ("an empty type", GOOD.replace("type: Runbook", "type: \"\"")),
        ("a description that is not text", GOOD.replace("type: Runbook", "type: Runbook\ndescription: 5")),
        ("tags that are not text", GOOD.replace("type: Runbook", "type: Runbook\ntags: [1]")),
        ("an empty Purpose", GOOD.replace("Stop the job.", "")),
        ("a placeholder Purpose", GOOD.replace("Stop the job.", "TODO")),
        ("a repeated heading", GOOD.replace("## Steps", "## Purpose\n\nTwice.\n\n## Steps")),
        ("two problems at once", GOOD.replace("RB-010", "X-10").replace("## Purpose", "## Overview")),
    ];
    for (name, text) in cases {
        let result = quire_rs::validate_document_in_registry(&registry, runbook, &text);
        println!("{name}: valid {}", result.is_valid);
        for e in &result.errors {
            println!("  {} (line {:?}): {}", e.reason.as_str(), e.line, e.message);
        }
    }
    Ok(())
}
