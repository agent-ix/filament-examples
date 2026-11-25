use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process::{Command, exit};

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let config_path = match env::current_exe() {
        Ok(path) => path.with_extension("config"),
        Err(_) => exit(21),
    };
    let config = match fs::read_to_string(config_path) {
        Ok(config) => config,
        Err(_) => exit(21),
    };
    let mut fields = config.split('\t');
    match fields.next() {
        Some("predicate") => run_predicate(&arguments, fields.collect()),
        Some("full-grade") => run_full_grade(&arguments, fields.collect()),
        Some("unsupported") => {
            if !emit("{\"category\":\"unsupported\",\"items\":[]}\n") {
                exit(21);
            }
            exit(10);
        }
        Some("exit-127") => exit(127),
        Some("terminate") => {
            let pid = std::process::id().to_string();
            let _ = Command::new("kill").args(["-TERM", &pid]).status();
            std::thread::park();
        }
        _ => exit(21),
    }
}

fn run_predicate(arguments: &[String], fields: Vec<&str>) -> ! {
    if !matches_args(arguments, &["run", "run.json", "--format", "json"]) {
        exit(21);
    }
    let [calls_path, captured_path] = fields.as_slice() else {
        exit(21);
    };
    let source = match fs::read_to_string("predicate.fsl") {
        Ok(source) => source,
        Err(_) => exit(21),
    };
    let request = match fs::read_to_string("run.json") {
        Ok(request) => request,
        Err(_) => exit(21),
    };
    let Some(value) = request_value(&request) else {
        exit(21);
    };
    if !source.contains("probeValue = probeValue")
        && !source.contains("not ( probeValue < 0 )")
    {
        if !emit("{\"category\":\"refusal\",\"diagnostics\":[{\"code\":\"invalid_syntax\"}]}\n") {
            exit(21);
        }
        exit(20);
    }
    let mut calls = match OpenOptions::new().create(true).append(true).open(calls_path) {
        Ok(calls) => calls,
        Err(_) => exit(21),
    };
    if writeln!(calls, "{value}").is_err() {
        exit(21);
    }
    match fs::read_to_string(captured_path) {
        Ok(captured) if emit(&captured) => {}
        Err(_) => exit(21),
        _ => exit(21),
    }
    exit(0);
}

fn run_full_grade(arguments: &[String], fields: Vec<&str>) -> ! {
    if arguments.first().is_some_and(|arg| arg == "run") {
        if !matches_args(arguments, &["run", "run.json", "--format", "json"]) {
            exit(21);
        }
        let source = match fs::read_to_string("predicate.fsl") {
            Ok(source) => source,
            Err(_) => exit(21),
        };
        let request = match fs::read_to_string("run.json") {
            Ok(request) => request,
            Err(_) => exit(21),
        };
        let Some(value) = request_value(&request) else {
            exit(21);
        };
        let result = if source.contains("probeValue = 1") {
            value == 1
        } else if source.contains("probeValue <= 1") {
            value <= 1
        } else {
            exit(21);
        };
        let output = format!(
            "{{\"category\":\"success\",\"result\":{{\"kind\":\"completed\",\"value\":{{\"type\":\"boolean\",\"value\":{result}}}}}}}\n"
        );
        if !emit(&output) {
            exit(21);
        }
        exit(0);
    }
    if !matches_args(
        arguments,
        &["analyze", "deadlock-request.json", "--format", "json"],
    ) && !matches_args(
        arguments,
        &["analyze", "repaired-request.json", "--format", "json"],
    )
    {
        exit(21);
    }
    let [deadlock_path, repaired_path] = fields.as_slice() else {
        exit(21);
    };
    let (path, status) = if arguments[1] == "deadlock-request.json" {
        (deadlock_path, 10)
    } else {
        (repaired_path, 0)
    };
    match fs::read_to_string(path) {
        Ok(outcome) if emit(&outcome) => {}
        Err(_) => exit(21),
        _ => exit(21),
    }
    exit(status);
}

fn emit(output: &str) -> bool {
    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes()).is_ok() && stdout.flush().is_ok()
}

fn matches_args(arguments: &[String], expected: &[&str]) -> bool {
    arguments.len() == expected.len()
        && arguments
            .iter()
            .zip(expected)
            .all(|(actual, expected)| actual.as_bytes() == expected.as_bytes())
}

fn request_value(request: &str) -> Option<u8> {
    let marker = r#""value":""#;
    let start = request.find(marker)? + marker.len();
    request.as_bytes().get(start)?.is_ascii_digit().then(|| {
        request.as_bytes()[start] - b'0'
    })
}
