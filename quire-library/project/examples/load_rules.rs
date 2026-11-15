use std::path::Path;

fn main() {
    // One module folder at a time, with the tolerant constructor.
    for name in ["missing-schema", "unknown-key", "retired", "null-key", "no-keys", "per-match-inside"] {
        let path = format!("broken/{name}");
        match quire_rs::Registry::load_module(Path::new(&path)) {
            Err(e) => println!("{name}: Err {e}"),
            Ok(registry) => {
                println!("{name}: Ok, types {:?}", registry.archetype_names().collect::<Vec<_>>());
                for f in registry.failures() {
                    println!("  failure {}/{}: {}", f.module, f.archetype, f.reason);
                }
            }
        }
    }

    // A search root that does not exist is a diagnostic, even with the strict constructor.
    let registry = quire_rs::Registry::load_strict(&[Path::new("nope")]).expect("strict load of a missing root");
    println!("strict, missing root: {:?}", registry.diagnostics());

    // from_env reads both variables and always adds the default root after them.
    std::env::set_var("IX_FILAMENT_MODULES_PATH", "modules");
    std::env::set_var("IX_HOME", "ix-home");
    let registry = quire_rs::Registry::from_env().expect("from_env");
    println!("from_env modules: {:?}", registry.module_names().collect::<Vec<_>>());
}
