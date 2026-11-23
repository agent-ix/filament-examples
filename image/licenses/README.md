# Licenses and source of the Filament toolchain image

This directory contains the instructions for the license notices generated during image build. The generated directory is `/usr/share/licenses/filament-toolchain` in the image.

## License

The three toolchain programs `fsl`, `quire` and `quoin` are licensed under the GNU Affero General Public License, version 3 or any later version (SPDX `AGPL-3.0-or-later`). The Filament examples in `/opt/filament-examples` and their runner `fsl-examples` are licensed under the MIT license.

| Generated file | Contents |
| --- | --- |
| `texts/` | program and dependency license texts from the built or installed artifacts |
| `NPM-PACKAGES.txt` | installed npm versions, artifact integrity, licenses, source and license-file digests |
| `THIRD-PARTY.md` | bundled components, licenses and source commits |
| `THIRD-PARTY-LICENSES.txt` | license texts from the Rust crates built into the programs |
| `BUILD-FACTS.txt` | Node.js, npm, driver source and Ubuntu package versions for this image |

The license of each Ubuntu package is in `/usr/share/doc/<package>/copyright`. npm licenses are copied from the installed tree to `npm/`; `NPM-PACKAGES.txt` records their paths and digests.

## Where the source code is

The programs in the image are compiled. The corresponding source of each is public on GitHub. `THIRD-PARTY.md` records the source commit of every first-party Rust crate built into each program.

| Program | Source repository | Tag |
| --- | --- | --- |
| `fsl` | https://github.com/agent-ix/filament-driver | `DRIVER-TAG` and exact source commit in `DRIVER-REVISION` |
| `quire` | https://github.com/agent-ix/quire-cli | the source tag in `NPM-PACKAGES.txt` |
| `quoin` | https://github.com/agent-ix/quoin | the source tag in `NPM-PACKAGES.txt` |
| Filament examples and `fsl-examples` (built from `tools/fsl-examples`) | https://github.com/agent-ix/filament-examples | the commit in the image label `org.opencontainers.image.revision` |

`quire --version` and `quoin --version` print their versions. The source of crates.io dependencies is at crates.io under the names and versions in `THIRD-PARTY.md`. The source of Node.js is at https://nodejs.org/dist/ for the version in `BUILD-FACTS.txt`. The source of Ubuntu packages is in the Ubuntu archive; `apt-get source <package>` fetches a package when the `deb-src` lines are enabled.

During image build, the license stage runs `cargo metadata --locked` against the exact driver source, installed npm package release sources, and built runner source. It renders the crate inventory and license texts into the image. `npm ci` installs the locked npm packages and checks their artifact integrity. The image contains generated notices only; this repository keeps the collector, tests and this guide.
