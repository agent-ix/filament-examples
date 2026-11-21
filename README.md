# Filament examples

filament-examples is a set of example systems written in the Filament Specification Language (FSL). Each system has a domain model, rules, data, and the recorded result of every check.

You can read a rule, run it against a passing and a failing case, and compare your result with the recorded one. Copy a system folder as the starting point for a model of your own.

## The three families

- **Shop (first steps).** Values, rules about data, and rules about operations, in an online shop.
- **Enterprise and distributed systems.** Rules over a service architecture, and a protocol for a parts-processing saga.
- **Safety systems.** A press guard interlock, robot speed and separation, and a delivery rover with failsafe modes and sensor voting.

## Systems

The table lists each system with the number of cases that run today and the number that are staged.

| System | Family | Running | Staged |
| --- | --- | --- | --- |
| [grammar](examples/grammar) | Language | 200 | 1 |
| [types-checking](examples/types-checking) | Language | 203 | 1 |
| [operations-arithmetic](examples/operations-arithmetic) | Language | 306 | 6 |
| [declarations-models](examples/declarations-models) | Language | 111 | 0 |
| [claims](examples/claims) | Language | 156 | 3 |
| [docs-captures](examples/docs-captures) | Language | 7 | 0 |
| [docs-values](examples/docs-values) | Language | 15 | 0 |
| [language-values](examples/language-values) | Language | 44 | 0 |
| [engines](examples/engines) | Language | 39 | 12 |
| [shop](examples/shop) | Shop | 153 | 3 |
| [architecture](examples/architecture) | Enterprise | 31 | 12 |
| [parts-saga](examples/parts-saga) | Distributed | 32 | 0 |
| [cell](examples/cell) | Safety | 13 | 0 |
| [rover](examples/rover) | Safety | 67 | 6 |
| [lock](examples/lock) | Safety | 25 | 1 |
| [library](examples/library) | Shop | 6 | 0 |

## Beyond FSL

[quoin-first-spec](quoin-first-spec) is a small Python project with a user story, a functional requirement and tests, written with Quoin and validated with Quire. The Quoin documentation builds its "Get started" tutorial from it.

[quire-first-document](quire-first-document/README.md) is a small Node.js project with a user story, a requirement, code and tests, checked with `quire`. The Quire documentation builds its "Get started" tutorial and how-tos from it.

[assurance](assurance/README.md) is a small shop service whose refund rule is written as requirements, tested with `node --test`, and backed by test runs, a change record and a measurement recorded with Quoin. The Assurance documentation builds its guides from it.

[verification/tl-examples](verification/tl-examples/steps.md) is a small Rust project that uses the four temporal logic crates (tl-syntax, tl-parse, tl-rewrite and tl-mltl): it builds a formula, parses text, rewrites a formula, evaluates it on a trace and runs the whole chain. The temporal logic pages of the Verification documentation draw their programs and output from it.

[quire-library](quire-library/steps.md) is a small Rust project that uses the `quire-rs` library and the `@agent-ix/quire-wasm` package: it loads a module, validates a document, reads and edits it, queries a corpus and runs the same checks from Node.js and Python. The Quire documentation builds its library pages from it.

## Getting started

You can run the examples in the toolchain image or with a driver you build yourself.

### In the image

You need Docker. The image has the driver, Node.js, `make` and a copy of the examples. Run every running and staged case with one command:

```
docker run --rm ghcr.io/agent-ix/filament-toolchain make check staged
```

To run a clone of this repository, or your own changes, mount the folder:

```
docker run --rm --volume "$PWD:/work" --workdir /work ghcr.io/agent-ix/filament-toolchain make check staged
```

The [image folder](image/README.md) describes the image. 

### With a release binary

You need `make`, `curl` and Rust 1.98.1 or later: the example runner in [tools/fsl-examples](tools/README.md) is a Rust program that `make check` builds on its first run. Each [filament-driver release](https://github.com/agent-ix/filament-driver/releases) has an archive for Linux and macOS on x86_64 and arm64, and a `SHA256SUMS.txt` file. The [install steps](install-test/steps.md) list every command for a clean Ubuntu 22.04 machine: they download the archive for your platform, check its checksum and put `fsl` on your path. A manual workflow runs them in bash and zsh. The Makefile reads the driver path from one variable, `FSL`, set in [config.mk](config.mk). Its default is `fsl`.

```
make check staged
```

### From source

You need a Rust toolchain and `make`. Build the driver from the public [filament-driver](https://github.com/agent-ix/filament-driver) repository. Pass the path of the program it builds:

```
git clone https://github.com/agent-ix/filament-driver
cargo build --locked -p fsl-cli --manifest-path filament-driver/Cargo.toml
make check staged FSL=$PWD/filament-driver/target/debug/fsl
```

The npm package `@agent-ix/quire-cli` installs a Markdown tool named `quire`. It cannot run these examples.

## Usage

Run every running case and compare it with its recorded result:

```
make check FSL=/path/to/driver
```

Run every staged case and confirm it still fails the way its manifest says:

```
make staged FSL=/path/to/driver
```

Run one system with `make -C examples/shop check FSL=/path/to/driver`.

The runner can also check selected cases directly. Use a case id, a `<system>/<id>` name, or `*` and `?` wildcards; repeat `--case` to select more than one:

```
cargo run --quiet --manifest-path tools/fsl-examples/Cargo.toml -- check shop --case paid-short
cargo run --quiet --manifest-path tools/fsl-examples/Cargo.toml -- check --case 'shop/paid-*' --case 'rover/safe-stop-late'
```

`regenerate` refreshes the selected cases' expected records. `record` remains an alias. Add `--dry-run` to print the proposed file changes without writing them:

```
cargo run --quiet --manifest-path tools/fsl-examples/Cargo.toml -- regenerate shop --case 'paid-*' --dry-run
```

Each case prints a line such as `shop/paid-short: exit 10 ok`. The exit code is the verdict: 0 holds, 10 violated, 20 refused, 22 incomplete data, 21 not available yet.

A case passes when its exit code and the structure of its outcome match the record: the category, the result value, the last stage, the code and cause of the first diagnostic, and the items with their witness fields. The message text of the diagnostics is kept apart and is not compared, so a reworded message does not fail a case.

## How staging works

A staged case is an example for a feature the language does not support yet. It is written anyway, kept in the repository and run in CI, and the docs and landing page do not show it until it passes.

Every case has an entry in its system's `example.toml`. A staged entry records the exit code it gives today and the exit code it should give when the feature exists. It also records the ticket it waits on, a short phrase for what it needs, and a one-line reason.

- `make staged` fails a staged case that no longer fails as recorded.
- A staged case that now gives its ready exit code makes the target fail with `promote <case>`.
- `make promote CASE=<system>/<case>` flips the manifest entry to running and records the output.
- `make record` re-records every `expected/` file from the driver. Never edit those files by hand.
- `make readme` rewrites the recorded output shown in each system README from `expected/` and updates the Systems table counts from the manifests. `make index` rewrites [index.json](index.json). `make check` fails when any generated file is stale.

Review the recorded output of a promoted case before you commit it. A failing case must fail for the reason it is meant to show.

## Folder layout

Each system folder holds the following.

| Path | Contents |
| --- | --- |
| `example.toml` | One entry per case |
| `model/` | The domain model, as Semantic IR JSON |
| `rules/` | FSL source, with each requirement as a comment |
| `data/`, `requests/` | Snapshots and one request file per case |
| `expected/` | For each case, `<id>.json` with the recorded outcome structure and, when the outcome has diagnostics, `<id>.messages.txt` with their message text |

## Index

[index.json](index.json) has one entry per case. An entry gives the system, case id, status, rule file, data files, request file and recorded output file. It also gives the verdict, the exit code and a one-line statement of the rule. A staged case adds the ticket it waits on and a plain-language phrase for what it needs. The docs site and landing page read it. All paths are relative to the repository root.

## Models

The models are Semantic IR JSON documents, and no tool in the language can author them yet. The generator in [tools](tools/README.md) writes them from short descriptions. The generated models are committed, so you do not need the generator to run an example.

## Building and testing

The runner tests use a stand-in driver and need no FSL binary:

```
make runner-test
```

`make test` runs the tests of the Node tools and of the tutorial projects.

The install test runs the commands of [install-test/steps.md](install-test/steps.md) on a bare `ubuntu:22.04` container. It needs Docker and a few minutes:

```
make install-test
```

The assurance test runs the commands of [assurance/steps.md](assurance/steps.md) against the [assurance example](assurance/README.md), a small spec repository checked with `quire` and `quoin`. It needs Node.js, npm and git, and installs the tools into a temporary folder:

```
make assurance-test
```

The temporal logic test builds and runs the Rust programs of [verification/tl-examples/steps.md](verification/tl-examples/steps.md). It needs cargo and network access to github.com and crates.io, and is skipped when cargo is missing:

```
make tl-examples-test
```

The Quire library test builds and runs the programs of [quire-library/steps.md](quire-library/steps.md). It needs cargo, Node.js and npm, and network access to github.com, crates.io and npmjs.org, and is skipped when cargo is missing:

```
make quire-library-test
```

The Python page of the same documentation has its own test, which builds the `quire` wheel from the `v0.60.0` tag of quire-rs (a few minutes, and it needs Python, Rust and git as well):

```
make quire-library-python-test
```

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before you open a pull request: every contribution needs a signed Contributor License Agreement (CLA). [CONTENT_RIGHTS.md](CONTENT_RIGHTS.md) lists what may be committed.

## License

MIT. See [LICENSE](LICENSE).
