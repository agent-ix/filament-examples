# Install FSL and run the examples

These are the commands to install the FSL toolchain and run the examples. The install test runs every fenced block on this page, in order, in one shell on a bare `ubuntu:26.04` container, so the page cannot drift from what was tested. The commands run as root. On a machine where you are not root, prefix the `apt-get` commands with `sudo`.

The FSL driver comes from a prebuilt release for Linux on x86_64 and arm64 and for macOS on Apple silicon and Intel. A second path builds the driver from source. You need network access to github.com, crates.io and static.rust-lang.org. No account or token is needed.

## Install the prerequisites

The examples need `git`, `make` and Rust 1.98.1 or later with a C toolchain: the example runner in `tools/fsl-examples` is a Rust program that `make check` builds on its first run. The release download needs `curl` and `tar`.

On macOS, `git`, `make`, `curl`, `tar` and the C toolchain come with the Xcode command line tools; install Rust with rustup as in the third block. Then continue with the driver download.

```bash step=prerequisites
apt-get update
apt-get install -y --no-install-recommends ca-certificates curl git make
```

```bash step=build-tools
apt-get install -y --no-install-recommends build-essential pkg-config
```

```bash step=rust
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. "$HOME/.cargo/env"
cargo --version
```

## Install the FSL driver from a release

Each release of the driver has one archive per platform and a `SHA256SUMS.txt` file with the checksum of every archive. Run the four steps of this section in one terminal session. Each step uses names that the step before it sets, and the first step moves into a new temporary folder.

The first block finds the newest release and the archive name for your platform. It stops with a message when the driver has no release.

```bash step=driver-target
repo=https://github.com/agent-ix/filament-driver
tag=$(curl -sSf --location --output /dev/null --write-out '%{url_effective}' "$repo/releases/latest")
tag=${tag##*/}
case "$tag" in
  v*) ;;
  *) echo "No release of the FSL driver was found at $repo/releases" >&2; false ;;
esac
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
  Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
  Darwin-arm64) target=aarch64-apple-darwin ;;
  Darwin-x86_64) target=x86_64-apple-darwin ;;
esac
archive="fsl-$tag-$target.tar.gz"
cd "$(mktemp -d)"
echo "$archive"
```

Download the archive and the checksum file.

```bash step=driver-download
url=$repo/releases/download/$tag
curl -sSf --location --remote-name "$url/$archive"
curl -sSf --location --remote-name "$url/SHA256SUMS.txt"
```

Check the archive against its checksum. The command fails if the file differs. It uses `sha256sum` where that exists and `shasum` otherwise.

```bash step=driver-verify
if command -v sha256sum >/dev/null; then
  grep " $archive\$" SHA256SUMS.txt | sha256sum --check -
else
  grep " $archive\$" SHA256SUMS.txt | shasum -a 256 --check -
fi
```

The archive holds two sibling programs, `fsl` and `fsl-plugin-launch`. Install both in a folder on your path; the driver uses the launcher when it starts plugin providers.

```bash step=driver-install
mkdir -p "$HOME/.local/bin"
tar -xzf "$archive" -C "$HOME/.local/bin" fsl fsl-plugin-launch
export PATH="$HOME/.local/bin:$PATH"
cd "$HOME"
fsl --version
```

Add `export PATH="$HOME/.local/bin:$PATH"` to your shell profile to keep `fsl` on the path in new terminals. The commands work in bash and zsh.

## Install the FSL driver from source

The source install is the second path. Skip this section if you installed the release. It installs both driver programs from the same revision, beside each other in Cargo's bin directory.

The two `CARGO_PROFILE_RELEASE` settings turn off link-time optimization and use 16 code-generation units. The driver's own release profile needs more than 8 GB of memory and does not finish on a small machine. This step takes several minutes.

```bash step=driver-source
FSL_REVISION=158bc4735167e75347afbfa883e03c1013d45f2d
CARGO_INCREMENTAL=0 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false \
  CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo install --git https://github.com/agent-ix/filament-driver --rev "$FSL_REVISION" --locked fsl-cli
CARGO_INCREMENTAL=0 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false \
  CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo install --git https://github.com/agent-ix/filament-driver --rev "$FSL_REVISION" --locked filament-plugin-host
fsl --version
command -v fsl-plugin-launch
```

## Run the examples

Clone the examples and run every running and staged case.

```bash step=clone
git clone https://github.com/agent-ix/filament-examples
cd filament-examples
```

```bash step=check-release
FSL="$(command -v fsl)" cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- check --exclude-case shop/staged-charge-adds-amount
FSL="$(command -v fsl)" cargo run --quiet --locked --manifest-path tools/fsl-examples/Cargo.toml -- staged
```

```bash step=check-source
make check
make staged
```
