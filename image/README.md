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

The image puts five commands on the path.

| Command | Program |
| --- | --- |
| `fsl` | The FSL driver |
| `fsl-plugin-launch` | The driver's resource-limited launcher for plugin providers; it must stay beside `fsl` |
| `fsl-examples` | The example runner from [tools/fsl-examples](../tools/README.md), which `make check` and `make staged` call |
| `quire` | The Markdown tool from the npm package `@agent-ix/quire-cli` |
| `quoin` | The quoin command-line tool |

The driver binary is `fsl`, with its sibling `fsl-plugin-launch`. The Markdown tool keeps its own name, `quire`, and no file inside either package is renamed.

## Building and testing

Build from the repository root and run the smoke test:

```
docker build -f image/Dockerfile -t filament-toolchain .
docker run --rm filament-toolchain smoke.sh
```

By default, the image downloads the driver v0.2.0 release archive and verifies its checksum. The manual workflow defaults to the `0.2.0` image tag and this driver version.

To build the driver from source, set `FSL_REVISION` to the exact current driver commit and select `FSL_SOURCE=source`. `FSL_REVISION` has no default, so a source build cannot silently use an old revision:

```
docker build -f image/Dockerfile --build-arg FSL_SOURCE=source --build-arg FSL_REVISION="$FSL_REVISION" -t filament-toolchain .
```

The release build checks the archive against the `SHA256SUMS.txt` file for the selected release. To select a different driver release, pass another `FSL_VERSION`:

```
docker build -f image/Dockerfile --build-arg FSL_SOURCE=release --build-arg FSL_VERSION=0.2.0 -t filament-toolchain .
```

The source compile uses link-time optimization off and takes several minutes. The manual workflow [image.yml](../.github/workflows/image.yml) builds each platform on a native runner, runs `smoke.sh` on each build, and publishes one multi-platform tag for each tag in its `tags` input. The inputs `fsl_revision`, `fsl_version`, and `fsl_source` select the driver.

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
