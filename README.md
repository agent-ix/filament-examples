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
| [shop](examples/shop) | Shop | 23 | 1 |
| [architecture](examples/architecture) | Enterprise | 14 | 2 |
| [parts-saga](examples/parts-saga) | Distributed | 6 | 2 |
| [cell](examples/cell) | Safety | 12 | 0 |
| [rover](examples/rover) | Safety | 11 | 6 |

## Getting started

You need Node.js 20 or later, `make`, and the FSL command-line driver. The Makefile reads the driver path from one variable, `FSL`, set in [config.mk](config.mk). Its default is `fsl`.

The driver is built from the public [quire-driver](https://github.com/agent-ix/quire-driver) repository. Its binary is still named `quire` until the rename to `fsl`. Pass its path:

```
git clone https://github.com/agent-ix/quire-driver
cargo build --locked -p quire-cli --manifest-path quire-driver/Cargo.toml
make check staged FSL=$PWD/quire-driver/target/debug/quire
```

The npm package `@agent-ix/quire-cli` also installs a command named `quire`. It is a Markdown tool and cannot run these examples. Keep it out of `FSL`.

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

Each case prints a line such as `shop/paid-short: exit 10 ok`. The exit code is the verdict: 0 holds, 10 violated, 20 refused, 22 incomplete data, 21 not available yet.

## How staging works

A staged case is an example for a feature the language does not support yet. It is written anyway, kept in the repository and run in CI, and the docs and landing page do not show it until it passes.

Every case has an entry in its system's `example.toml`. A staged entry records the exit code it gives today, the exit code it should give when the feature exists, the ticket it waits on, and a one-line reason.

- `make staged` fails a staged case that no longer fails as recorded.
- A staged case that now gives its ready exit code makes the target fail with `promote <case>`.
- `make promote CASE=<system>/<case>` flips the manifest entry to running and records the output.
- `make record` re-records every `expected/` file from the driver. Never edit those files by hand.

Review the recorded output of a promoted case before you commit it. A failing case must fail for the reason it is meant to show.

## Folder layout

Each system folder holds the following.

| Path | Contents |
| --- | --- |
| `example.toml` | One entry per case |
| `model/` | The domain model, as Semantic IR JSON |
| `rules/` | FSL source, with each requirement as a comment |
| `data/`, `requests/` | Snapshots and one request file per case |
| `expected/` | Recorded driver output for each case |

## Models

The models are Semantic IR JSON documents, and no tool in the language can author them yet. The generator in [tools](tools/README.md) writes them from short descriptions. The generated models are committed, so you do not need the generator to run an example.

## Building and testing

The runner tests use a fake driver and need no FSL binary:

```
make test
```

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) first. Every contribution needs a signed [CLA](CLA.md).

## License

MIT. See [LICENSE](LICENSE).
