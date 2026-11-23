# Licenses and source of the Filament toolchain image

This directory holds the checked-in driver and runner license records and the instructions for notices generated during image build. It is `/usr/share/licenses/filament-toolchain` in the image.

## License

The three toolchain programs `fsl`, `quire` and `quoin` are licensed under the GNU Affero General Public License, version 3 or any later version (SPDX `AGPL-3.0-or-later`). The Filament examples in `/opt/filament-examples` and their runner `fsl-examples` are licensed under the MIT license.

| File | License text of |
| --- | --- |
| `texts/filament-driver.LICENSE` | `fsl`, the FSL driver |
| `texts/filament-examples.LICENSE` | the Filament examples and `fsl-examples`, the example runner |
| `texts/nodejs.LICENSE` | Node.js, with the npm and corepack that come with it |
| `NPM-PACKAGES.txt` | installed npm versions, artifact integrity, licences, source and licence-file digests; generated during image build |
| `THIRD-PARTY.md` | bundled components, with license identifier and source; npm package facts refer to `NPM-PACKAGES.txt` |
| `THIRD-PARTY-LICENSES.txt` | the license texts shipped with the third-party Rust crates built into `fsl`, `quire`, `quoin` and `fsl-examples` |
| `BUILD-FACTS.txt` | the Node.js, npm and Ubuntu package versions this image was built with |

The license of each Ubuntu package is in `/usr/share/doc/<package>/copyright`. The npm packages keep their own licence files under `/opt/npm/node_modules`; `NPM-PACKAGES.txt` records those paths and their digests from the installed tree.

## Where the source code is

The programs in the image are compiled. The corresponding source of each is public on GitHub, at the tag the image was built from. `THIRD-PARTY.md` lists, for each program, the repository and commit of every first-party Rust crate built into it.

| Program | Source repository | Tag |
| --- | --- | --- |
| `fsl` | https://github.com/agent-ix/filament-driver | the tag in `DRIVER-TAG` (`fsl --version` prints the version) |
| `quire` | https://github.com/agent-ix/quire-cli | the source tag in `NPM-PACKAGES.txt` |
| `quoin` | https://github.com/agent-ix/quoin | the source tag in `NPM-PACKAGES.txt` |
| Filament examples and `fsl-examples` (built from `tools/fsl-examples`) | https://github.com/agent-ix/filament-examples | the commit in the image label `org.opencontainers.image.revision` |

`quire --version` and `quoin --version` print the versions. The source of the Rust crates from crates.io is at crates.io under the names and versions in `THIRD-PARTY.md`. The source of Node.js is at https://nodejs.org/dist/ for the version in `BUILD-FACTS.txt`. The source of the Ubuntu packages is in the Ubuntu archive, and `apt-get source <package>` fetches it when the `deb-src` lines of the package sources are enabled.

The image build generates npm package and licence records from its installed tree and lockfile. `npm ci` enforces the versions and integrity in `image/package-lock.json`. The image build and `node tools/licences.mjs check` fail when the example runner's Cargo.lock names crates its licence data does not describe. The driver and runner records are refreshed with `node tools/licences.mjs collect` (or `collect-runner` for a runner change), then `render`.
