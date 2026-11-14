# Licenses and source of the Filament toolchain image

This directory holds the license texts of the programs in the image, a notice for the components bundled with them, and the places where their source code is published. It is `/usr/share/licenses/filament-toolchain` in the image.

## License

The three toolchain programs are licensed under the GNU Affero General Public License, version 3 or any later version (SPDX `AGPL-3.0-or-later`). The Filament examples in `/opt/filament-examples` are licensed under the MIT license.

| File | License text of |
| --- | --- |
| `texts/filament-driver.LICENSE` | `fsl`, the FSL driver |
| `texts/quire-cli.LICENSE` | `quire`, the Markdown tool |
| `texts/quoin.LICENSE` | `quoin` |
| `texts/filament-examples.LICENSE` | the Filament examples |
| `texts/nodejs.LICENSE` | Node.js, with the npm and corepack that come with it |
| `THIRD-PARTY.md` | every bundled component, with version, license identifier and source |
| `THIRD-PARTY-LICENSES.txt` | the license texts shipped with the third-party Rust crates built into the three programs |
| `BUILD-FACTS.txt` | the Node.js, npm and Ubuntu package versions this image was built with |

The license of each Ubuntu package is in `/usr/share/doc/<package>/copyright`. The npm packages keep their own `LICENSE` files in their folders under `/usr/local/lib/node_modules`.

## Where the source code is

The programs in the image are compiled. The corresponding source of each is public on GitHub, at the tag the image was built from. `THIRD-PARTY.md` lists, for each program, the repository and commit of every first-party Rust crate built into it.

| Program | Source repository | Tag |
| --- | --- | --- |
| `fsl` | https://github.com/agent-ix/filament-driver | the tag in `DRIVER-TAG` (`fsl --version` prints the version) |
| `quire` | https://github.com/agent-ix/quire-cli | `v` and the version of `@agent-ix/quire-cli` |
| `quoin` | https://github.com/agent-ix/quoin | `v` and the version of `@agent-ix/quoin` |
| Filament examples | https://github.com/agent-ix/filament-examples | the commit in the image label `org.opencontainers.image.revision` |

`quire --version` and `quoin --version` print the versions. The source of the Rust crates from crates.io is at crates.io under the names and versions in `THIRD-PARTY.md`. The source of Node.js is at https://nodejs.org/dist/ for the version in `BUILD-FACTS.txt`. The source of the Ubuntu packages is in the Ubuntu archive, and `apt-get source <package>` fetches it when the `deb-src` lines of the package sources are enabled.

The image build fails when the installed `quire` and `quoin` packages or the downloaded driver are not the versions this directory describes. The files here are updated, with `node tools/licences.mjs collect` and `render`, in the same change that updates a version.
