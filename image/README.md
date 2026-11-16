# Filament toolchain image

The Filament toolchain image is a Docker image for running the Filament examples and your own FSL projects. It holds the FSL driver, the Markdown tool, quoin, jq, make, git and Node.js, and is published as `ghcr.io/agent-ix/filament-toolchain` for linux/amd64 and linux/arm64.

You run the checks of the examples in the image without installing a toolchain, and you run the checks of your own project by mounting its folder.

## Getting started

You need Docker. Run the examples that are baked into the image:

```
docker run --rm ghcr.io/agent-ix/filament-toolchain make check staged
```

## Usage

Run the checks of a project in your current folder:

```
docker run --rm --volume "$PWD:/work" --workdir /work ghcr.io/agent-ix/filament-toolchain make check
```

## Commands

The image puts four commands on the path.

| Command | Program |
| --- | --- |
| `fsl` | The FSL driver |
| `fsl-examples` | The example runner from [tools/fsl-examples](../tools/README.md), which `make check` and `make staged` call |
| `quire` | The Markdown tool from the npm package `@agent-ix/quire-cli` |
| `quoin` | The quoin command-line tool |

The driver binary is `fsl`. The Markdown tool keeps its own name, `quire`, and no file inside either package is renamed.

## Building and testing

Build from the repository root and run the smoke test:

```
docker build -f image/Dockerfile -t filament-toolchain .
docker run --rm filament-toolchain smoke.sh
```

The build downloads the driver release archive for its platform and checks it against the `SHA256SUMS.txt` file of the same release. `FSL_VERSION` picks the release, and its default is the newest one:

```
docker build -f image/Dockerfile --build-arg FSL_VERSION=0.1.1 -t filament-toolchain .
```

To compile the driver from the public [filament-driver](https://github.com/agent-ix/filament-driver) repository instead, set `FSL_SOURCE=source`. The compile uses link-time optimization off, and takes several minutes:

```
docker build -f image/Dockerfile --build-arg FSL_SOURCE=source -t filament-toolchain .
```

The manual workflow [image.yml](../.github/workflows/image.yml) builds each platform on a native runner, runs `smoke.sh` on each build, and publishes one multi-platform tag for each tag in its `tags` input. The inputs `fsl_version` and `fsl_source` select the driver.

## License

The programs in the image, `fsl`, `quire` and `quoin`, are licensed under the GNU Affero General Public License, version 3 or any later version (`AGPL-3.0-or-later`). The Filament examples in `/opt/filament-examples` are licensed under the MIT license. This repository is MIT; see the [license file](../LICENSE).

The image holds the other components under their own licenses: Node.js, the Rust crates built into the three programs, and the Ubuntu packages. Its label `org.opencontainers.image.licenses` names the licenses of the first-party parts, `AGPL-3.0-or-later AND MIT`.

The license texts, a notice for every bundled component, and the places where the source code of each program is published are in `/usr/share/licenses/filament-toolchain`:

```
docker run --rm ghcr.io/agent-ix/filament-toolchain cat /usr/share/licenses/filament-toolchain/README.md
```

The same files are in [image/licenses](licenses). The source of the three programs is public, at the tag each was built from:

| Program | Source |
| --- | --- |
| `fsl` | [agent-ix/filament-driver](https://github.com/agent-ix/filament-driver) |
| `quire` | [agent-ix/quire-cli](https://github.com/agent-ix/quire-cli) |
| `quoin` | [agent-ix/quoin](https://github.com/agent-ix/quoin) |

The Ubuntu copyright files stay in `/usr/share/doc/<package>/copyright`. The image build fails when the installed programs are not the versions described in `image/licenses`. After you change a version, run `node tools/licences.mjs collect` on checkouts of the three tags, then `node tools/licences.mjs render`; `make test` fails when the rendered files are stale.
