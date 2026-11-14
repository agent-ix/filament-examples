# Install FSL and run the examples

These are the commands to install the FSL toolchain on macOS or Linux and run the examples. The install test runs every Linux block on this page, in order, in one shell on a bare `ubuntu:22.04` container, so the page cannot drift from what was tested. The commands run as root. On a machine where you are not root, prefix the `apt-get` commands with `sudo`.

You need network access to github.com, nodejs.org and, to build the driver from source, crates.io and static.rust-lang.org. No account or token is needed.

## Install the prerequisites

The examples need `git`, `make`, `curl` and Node.js 20 or later. A block marked for one system is skipped on the other.

On Linux, install the system packages. Ubuntu 22.04 has no usable Node.js package, so the next block installs Node.js 24 from nodejs.org.

```bash step=prerequisites os=linux
apt-get update
apt-get install -y --no-install-recommends ca-certificates curl git build-essential pkg-config xz-utils jq make
```

```bash step=node os=linux
case "$(uname -m)" in aarch64) arch=arm64 ;; x86_64) arch=x64 ;; esac
version=$(curl -sSf https://nodejs.org/dist/index.json | jq -r '[.[] | select(.version | startswith("v24."))][0].version')
curl -sSf "https://nodejs.org/dist/$version/node-$version-linux-$arch.tar.xz" | tar -xJ -C /usr/local --strip-components=1
```

On macOS, `git` and `make` come with the Xcode command line tools (`xcode-select --install`). Install Node.js with Homebrew.

```bash step=prerequisites-macos os=macos
brew install node
```

Check that the tools are there.

```bash step=tools
git --version
make --version | head -1
node --version
```

## Install the FSL driver

The driver is the program that runs the checks. Its release archive for your system holds one file, named `quire` today. The block downloads the archive, keeps the file under the name `fsl`, which the examples use, and puts `$HOME/.local/bin` on the path. To keep `fsl` on the path in new terminals, add the `export` line to your shell profile.

```bash step=driver-release
version=0.1.1
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
  Linux-aarch64) target=aarch64-unknown-linux-gnu ;;
  Darwin-arm64) target=aarch64-apple-darwin ;;
  Darwin-x86_64) target=x86_64-apple-darwin ;;
esac
mkdir -p "$HOME/.local/bin"
curl -sSfL "https://github.com/agent-ix/quire-driver/releases/download/v$version/quire-v$version-$target.tar.gz" | tar -xz -C "$HOME/.local/bin" quire
mv "$HOME/.local/bin/quire" "$HOME/.local/bin/fsl"
export PATH="$HOME/.local/bin:$PATH"
fsl --version
```

## Build the driver from source

Use this route when no archive exists for your system or you want a revision of your own. Install Rust 1.98.1 or later with rustup.

```bash step=rust
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. "$HOME/.cargo/env"
cargo --version
```

Build the driver. The two `CARGO_PROFILE_RELEASE` settings turn off link-time optimization and use 16 code-generation units. The driver's own release profile needs more than 8 GB of memory and does not finish on a small machine. This step takes a few minutes.

```bash step=driver-source
CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo install --git https://github.com/agent-ix/quire-driver --locked quire-cli
```

The built binary is named `quire`. Give it the name the examples use.

```bash step=driver-name
export PATH="$HOME/.cargo/bin:$PATH"
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
