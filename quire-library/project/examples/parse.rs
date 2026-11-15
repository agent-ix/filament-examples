fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("runbooks/RB-002-rotate-keys.md")?;
    // Parsing needs no module.
    let doc = quire_rs::parse_document(&text);
    println!("type: {:?}", quire_rs::concept_type(&doc));
    for s in &doc.sections {
        println!("section {:?}, level {}, lines {}..{}", s.heading, s.level, s.start_line, s.end_line);
        for child in &s.children {
            println!("  child {:?}", child.heading);
        }
    }
    let steps = quire_rs::section(&doc, "Steps").expect("Steps section");
    println!("steps content: {:?}", steps.content);
    let items = quire_rs::parse_bullet_list(&steps.content, None);
    println!("steps: {}", items.len());
    for r in quire_rs::search(&doc, "key") {
        println!("{:?}: {} match(es), first at line {}", r.section.heading, r.matches.len(), r.matches[0].line);
    }
    Ok(())
}
