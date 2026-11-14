# Install FSL and run the examples

These are the commands to install the FSL toolchain and run the examples. The install test runs every fenced block on this page, in order, in one shell on a bare `ubuntu:22.04` container, so the page cannot drift from what was tested. The commands run as root. On a machine where you are not root, prefix the `apt-get` commands with `sudo`.

The FSL driver comes from a prebuilt release for Linux on x86_64 and arm64 and for macOS on Apple silicon and Intel. A second path builds the driver from source. You need network access to github.com and nodejs.org. The source build also needs crates.io and static.rust-lang.org. No account or token is needed.

## Install the prerequisites

The examples need `git`, `make`, `jq` and Node.js 20 or later. The release download needs `curl` and `tar`. Ubuntu 22.04 has no usable Node.js package, so the second block installs Node.js 24 from nodejs.org.

On macOS, install Node.js 20 or later and `jq` with Homebrew (the `node` and `jq` packages). `git`, `make`, `curl` and `tar` come with the Xcode command line tools. Then continue with the driver download.

```bash step=prerequisites
apt-get update
apt-get install -y --no-install-recommends ca-certificates curl git jq make xz-utils
```

```bash step=node
case "$(uname -m)" in aarch64) arch=arm64 ;; x86_64) arch=x64 ;; esac
version=$(curl -sSf https://nodejs.org/dist/index.json | jq -r '[.[] | select(.version | startswith("v24."))][0].version')
curl -sSf "https://nodejs.org/dist/$version/node-$version-linux-$arch.tar.xz" | tar -xJ -C /usr/local --strip-components=1
node --version
```

## Install the FSL driver from a release

Each release of the driver has one archive per platform and a `SHA256SUMS.txt` file with the checksum of every archive. The first block finds the newest release and the archive name for your platform.

```bash step=driver-target
tag=$(curl -sSf --location --output /dev/null --write-out '%{url_effective}' https://github.com/agent-ix/quire-driver/releases/latest)
tag=${tag##*/}
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
  Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
  Darwin-arm64) target=aarch64-apple-darwin ;;
  Darwin-x86_64) target=x86_64-apple-darwin ;;
esac
archive="quire-$tag-$target.tar.gz"
cd "$(mktemp -d)"
echo "$archive"
```

Download the archive and the checksum file.

```bash step=driver-download
url=https://github.com/agent-ix/quire-driver/releases/download/$tag
curl -sSf --location --remote-name "$url/$archive"
curl -sSf --location --remote-name "$url/SHA256SUMS.txt"
```

Check the archive against its checksum. The command fails if the file differs.

```bash step=driver-verify
if command -v sha256sum >/dev/null; then check="sha256sum --check -"; else check="shasum -a 256 --check -"; fi
grep " $archive\$" SHA256SUMS.txt | $check
```

The archive holds one program, still named `quire`. The examples call the driver `fsl`, so install it under that name in a folder on your path.

```bash step=driver-install
mkdir -p "$HOME/.local/bin"
tar -xzf "$archive" -C "$HOME/.local/bin" quire
mv "$HOME/.local/bin/quire" "$HOME/.local/bin/fsl"
export PATH="$HOME/.local/bin:$PATH"
cd "$HOME"
fsl --version
```

Add `export PATH="$HOME/.local/bin:$PATH"` to your shell profile to keep `fsl` on the path in new terminals.

## Build the FSL driver from source

The source build is the second path. Skip this section if you installed the release. The driver is a Rust program that needs a C toolchain and Rust 1.98.1 or later.

```bash step=build-tools
apt-get install -y --no-install-recommends build-essential pkg-config
```

```bash step=rust
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. "$HOME/.cargo/env"
cargo --version
```

Build the driver. The two `CARGO_PROFILE_RELEASE` settings turn off link-time optimization and use 16 code-generation units. The driver's own release profile needs more than 8 GB of memory and does not finish on a small machine. This step takes several minutes.

```bash step=driver-source
CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo install --git https://github.com/agent-ix/quire-driver --locked quire-cli
```

The built program is named `quire`. Give it the name the examples use.

```bash step=driver-name
ln -sf "$HOME/.cargo/bin/quire" "$HOME/.cargo/bin/fsl"
fsl --version
```

## Run the examples

Clone the examples and run every running and staged case.

```bash step=clone
git clone https://github.com/agent-ix/filament-examples
cd filament-examples
```

```bash step=check
make check
make staged
```
