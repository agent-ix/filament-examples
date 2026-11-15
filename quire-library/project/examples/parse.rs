fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("runbooks/RB-002-rotate-keys.md")?;
    // Parsing needs no module.
    let doc = quire_rs::parse_document(&text);
    println!("type: {:?}", quire_rs::concept_type(&doc));
    for s in &doc.sections {
        println!("section {:?}, level {}, lines {}..{}", s.heading, s.level, s.start_line, s.end_line);
        for child in &s.children {
            println!("  child {:?}, id {}", child.heading, child.id);
        }
    }

    // Query a section, its list and a text search.
    let steps = quire_rs::section(&doc, "Steps").expect("Steps section");
    println!("steps content: {:?}", steps.content);
    println!("steps: {}", quire_rs::parse_bullet_list(&steps.content, None).len());
    for r in quire_rs::search(&doc, "key") {
        println!("{:?}: {} match(es), first at line {}", r.section.heading, r.matches.len(), r.matches[0].line);
    }

    // Only ATX headings start a section, a heading inside a fence does not, and a
    // {#name} attribute becomes the block id.
    let small = "Intro.\n\n# Title {#blk-title}\n\nBody.\n\n```sh\n# not a heading\n```\n\n## Back\n\nSetext\n======\n";
    let doc = quire_rs::parse_document(small);
    println!("preamble: {:?}", doc.preamble);
    let title = &doc.sections[0];
    println!("{:?} block_id {:?}, children {}", title.heading, title.block_id, title.children.len());
    println!("fence kept in content: {}", title.content.contains("# not a heading"));
    println!("setext line kept in content: {}", title.children[0].content.contains("Setext"));

    // Bad front matter does not fail the parse: there is no front matter, and its text is the preamble.
    let bad = quire_rs::parse_document("---\nkey: [unclosed\n---\n# Heading\n");
    println!("bad front matter: {:?}, preamble {:?}", bad.frontmatter, bad.preamble);
    Ok(())
}
