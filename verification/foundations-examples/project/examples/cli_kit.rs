use ix_cli_kit::exit::Outcome;
use ix_cli_kit::json::canonical;
use ix_cli_kit::streams::ColorChoice;

fn main() {
    // The five exit statuses, and whether each one still carries a complete result.
    for outcome in [
        Outcome::Ok,
        Outcome::Partial,
        Outcome::Refused,
        Outcome::Invalid,
        Outcome::Internal,
    ] {
        println!(
            "{} = {}, carries a payload: {}",
            outcome.code(),
            outcome.as_str(),
            outcome.carries_payload()
        );
    }
    println!("exit status 5 is reserved: {:?}", Outcome::from_code(5));

    // Canonical JSON sorts the keys of every object at every depth.
    let value = serde_json::json!({"b": {"z": 1, "y": 2}, "a": [3, 1]});
    println!("canonical: {}", canonical(&value).unwrap());

    // Color is decided from two facts: whether the stream is a terminal and whether NO_COLOR is set.
    println!("auto, terminal: {}", ColorChoice::Auto.decide(true, false));
    println!("auto, piped: {}", ColorChoice::Auto.decide(false, false));
    println!("auto, terminal, NO_COLOR set: {}", ColorChoice::Auto.decide(true, true));
    println!("always, piped: {}", ColorChoice::Always.decide(false, false));
}
