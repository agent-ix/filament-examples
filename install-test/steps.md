# Install FSL and run the examples

These are the commands to install the FSL toolchain on a clean Ubuntu 22.04 machine and run the examples. The install test runs every fenced block on this page, in order, in one shell on a bare `ubuntu:22.04` container, so the page cannot drift from what was tested. The commands run as root. On a machine where you are not root, prefix the `apt-get` commands with `sudo`.

You need network access to github.com, crates.io, nodejs.org and static.rust-lang.org. No account or token is needed.

## Install the prerequisites

Install the system packages the build and the examples need. Ubuntu 22.04 has no usable Node.js package, so the next block installs Node.js 24 from nodejs.org.

```bash step=prerequisites
apt-get update
apt-get install -y --no-install-recommends ca-certificates curl git build-essential pkg-config xz-utils jq make
```

```bash step=node
case "$(uname -m)" in aarch64) arch=arm64 ;; x86_64) arch=x64 ;; esac
version=$(curl -sSf https://nodejs.org/dist/index.json | jq -r '[.[] | select(.version | startswith("v24."))][0].version')
curl -sSf "https://nodejs.org/dist/$version/node-$version-linux-$arch.tar.xz" | tar -xJ -C /usr/local --strip-components=1
node --version
```

## Install the FSL driver

The driver is a Rust program. Install Rust 1.98.1 or later with rustup.

```bash step=rust
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. "$HOME/.cargo/env"
cargo --version
```

Build the driver from source. The two `CARGO_PROFILE_RELEASE` settings turn off link-time optimization and use 16 code-generation units. The driver's own release profile needs more than 8 GB of memory and does not finish on a small machine. This step takes a few minutes.

```bash step=driver-source
CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo install --git https://github.com/agent-ix/quire-driver --locked quire-cli
```

A prebuilt driver binary will replace the build when one is released (PLAT-1701). Set `FSL_BINARY_URL` to its address and run this block instead of the build.

```bash step=driver-release
mkdir -p "$HOME/.cargo/bin"
curl -sSf --location --output "$HOME/.cargo/bin/quire" "$FSL_BINARY_URL"
chmod +x "$HOME/.cargo/bin/quire"
```

The driver binary is still named `quire`, and the examples call it `fsl`. Give it the name the examples use.

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
