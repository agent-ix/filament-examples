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
| [rover](examples/rover) | Safety | 31 | 8 |

## Getting started

You can run the examples in the toolchain image, with the prebuilt driver, or with a driver you build yourself.

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

### With the prebuilt driver

The [install steps](install-test/steps.md) install Node.js, download the prebuilt driver for macOS or Linux from the [driver releases](https://github.com/agent-ix/quire-driver/releases), and run every case. The install test runs them on a clean Ubuntu 22.04 machine, and a manual workflow runs the test.

### From source

The install steps also build the driver. You need Node.js 20 or later, `make`, a Rust toolchain and the FSL command-line driver. The Makefile reads the driver path from one variable, `FSL`, set in [config.mk](config.mk). Its default is `fsl`.

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
- `make readme` rewrites the recorded output shown in each system README from `expected/`. `make index` rewrites [index.json](index.json). `make check` fails when either is stale.

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

## Index

[index.json](index.json) has one entry per case. An entry gives the system, case id, status, rule file, data files, request file and recorded output file. It also gives the verdict, the exit code, the ticket a staged case waits on and a one-line statement of the rule. The docs site and landing page read it. All paths are relative to the repository root.

## Models

The models are Semantic IR JSON documents, and no tool in the language can author them yet. The generator in [tools](tools/README.md) writes them from short descriptions. The generated models are committed, so you do not need the generator to run an example.

## Building and testing

The runner tests use a fake driver and need no FSL binary:

```
make test
```

The install test runs the commands of [install-test/steps.md](install-test/steps.md) on a bare `ubuntu:22.04` container. It needs Docker and a few minutes:

```
make install-test
```

## Contributing

Open a pull request. [CONTENT_RIGHTS.md](CONTENT_RIGHTS.md) lists what may be committed.

## License

MIT. See [LICENSE](LICENSE).
