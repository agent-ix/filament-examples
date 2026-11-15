use std::path::Path;

fn main() {
    // Walk a folder, parse its documents and build the corpus.
    let spec = quire_rs::Spec::from_repo(quire_rs::load_repo(Path::new("runbooks")));
    println!("documents: {}", spec.len());
    println!("RB-002 found: {}", spec.by_id("RB-002").is_some());

    // Problems found while loading: a document without a uuid, a link to a missing document.
    for d in spec.diagnostics() {
        println!("diagnostic: {d:?}");
    }

    // The links of one document.
    for e in spec.outgoing("RB-002") {
        println!("{} --{}--> {} ({:?})", e.source, e.edge_type, e.target, e.resolution);
    }
    // Links whose target is not in the corpus.
    for e in spec.dangling() {
        println!("dangling: {} --{}--> {}", e.source, e.edge_type, e.target);
    }
    // Documents that link to one document.
    for e in spec.referencing("RB-001") {
        println!("RB-001 is referenced by {}", e.source);
    }
    // Identifiers written in text without a link.
    for r in quire_rs::unlinked_references(&spec) {
        println!("unlinked: {} in {} ({:?})", r.token, r.path.display(), r.fix);
    }
}
