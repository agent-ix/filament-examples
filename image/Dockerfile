# Filament toolchain image. Build from the repository root:
#   docker build -f image/Dockerfile -t filament-toolchain .
#   docker run --rm filament-toolchain smoke.sh
#
# The driver comes from a release by default: the build downloads the release
# archive for its platform and checks it against SHA256SUMS.txt from the same
# release. Build arguments:
#   FSL_VERSION  release to download, such as 0.1.1; "latest" is the newest release
#   FSL_SOURCE   "release" (default) or "source" to build the driver from its repository
#   FSL_REPO     the driver repository

ARG UBUNTU=22.04

# ------------------------------------------------------------ driver stage
FROM ubuntu:${UBUNTU} AS driver
ARG FSL_SOURCE=release
ARG FSL_VERSION=latest
ARG FSL_REPO=https://github.com/agent-ix/filament-driver
ARG RUST_VERSION=1.98.1
ENV DEBIAN_FRONTEND=noninteractive
# The source build: the driver's own release profile (thin LTO, one codegen
# unit) exhausts the memory of an 8 GB machine. These two overrides use cargo's
# environment variables, change no file, and build the same package in a few
# minutes.
ENV CARGO_PROFILE_RELEASE_LTO=off \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
RUN set -eu; \
    apt-get update; \
    mkdir -p /opt/fsl-driver/bin; \
    if [ "$FSL_SOURCE" = release ]; then \
      apt-get install -y --no-install-recommends ca-certificates curl; \
      if [ "$FSL_VERSION" = latest ]; then \
        tag=$(curl -sSf --location --output /dev/null --write-out '%{url_effective}' "$FSL_REPO/releases/latest"); \
        tag=${tag##*/}; \
        case "$tag" in v*) ;; *) echo "No release of the FSL driver was found at $FSL_REPO/releases" >&2; exit 1;; esac; \
      else tag=v${FSL_VERSION#v}; fi; \
      archive="fsl-$tag-$(uname -m)-unknown-linux-gnu.tar.gz"; \
      cd /tmp; \
      curl -sSf --location --remote-name "$FSL_REPO/releases/download/$tag/$archive"; \
      curl -sSf --location --remote-name "$FSL_REPO/releases/download/$tag/SHA256SUMS.txt"; \
      grep " $archive\$" SHA256SUMS.txt | sha256sum --check -; \
      tar -xzf "$archive" -C /opt/fsl-driver/bin fsl; \
    else \
      apt-get install -y --no-install-recommends ca-certificates curl git build-essential pkg-config; \
      curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain "$RUST_VERSION"; \
      PATH=/root/.cargo/bin:$PATH cargo install --git "$FSL_REPO" \
        --locked --root /opt/fsl-driver fsl-cli; \
      strip /opt/fsl-driver/bin/fsl; \
    fi; \
    rm -rf /var/lib/apt/lists/* /tmp/*

# ----------------------------------------------------------- runtime stage
FROM ubuntu:${UBUNTU}
ARG NODE_MAJOR=24
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl git jq make xz-utils \
 && rm -rf /var/lib/apt/lists/*

# Node from nodejs.org: the Ubuntu 22.04 package is too old. The tarball names
# use arm64 and x64, not aarch64 and x86_64. Docs and headers are dropped.
RUN set -eu; \
    case "$(uname -m)" in aarch64) a=arm64;; x86_64) a=x64;; esac; \
    v=$(curl -sSf https://nodejs.org/dist/index.json \
        | jq -r "[.[] | select(.version | startswith(\"v${NODE_MAJOR}.\"))][0].version"); \
    curl -sSf https://nodejs.org/dist/$v/node-$v-linux-$a.tar.xz \
      | tar -xJ -C /usr/local --strip-components=1; \
    rm -rf /usr/local/include /usr/local/share/doc /usr/local/share/man /usr/local/*.md /usr/local/LICENSE

# The Markdown tool and quoin install to /usr/local/bin, where `quire` is the
# Markdown tool. The driver binary is `fsl`. Nothing inside either package is
# renamed.
RUN npm i -g @agent-ix/quire-cli @agent-ix/quoin && npm cache clean --force
COPY --from=driver /opt/fsl-driver/bin/fsl /usr/local/bin/fsl

COPY examples /opt/filament-examples/examples
COPY tools /opt/filament-examples/tools
COPY Makefile config.mk index.json /opt/filament-examples/
COPY image/smoke.sh /usr/local/bin/smoke.sh

WORKDIR /opt/filament-examples
CMD ["smoke.sh"]
