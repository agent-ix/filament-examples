# Filament toolchain image. Build from the repository root:
#   docker build -f image/Dockerfile -t filament-toolchain .
#   docker run --rm filament-toolchain smoke.sh
#
# The driver is downloaded from the v0.2.0 release by default. The source
# option requires an explicit driver revision.
# Build arguments:
#   FSL_VERSION  release to download, such as 0.2.0; "latest" is the newest release
#   FSL_SOURCE   "release" (default) or "source" to build a source revision
#   FSL_REVISION exact filament-driver commit built when FSL_SOURCE=source
#   FSL_REPO     the driver repository

ARG UBUNTU=26.04
ARG RUST_VERSION=1.100.0
ARG NODE_MAJOR=24

# ------------------------------------------------------------ driver stage
FROM ubuntu:${UBUNTU} AS driver
ARG FSL_SOURCE=release
ARG FSL_VERSION=0.2.0
ARG FSL_REVISION
ARG FSL_REPO=https://github.com/agent-ix/filament-driver
ARG RUST_VERSION
ENV DEBIAN_FRONTEND=noninteractive
# The source build: the driver's own release profile (thin LTO, one codegen
# unit) exhausts the memory of an 8 GB machine. These two overrides use cargo's
# environment variables, change no file, and build the same package in a few
# minutes.
ENV CARGO_PROFILE_RELEASE_LTO=off \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
    CARGO_PROFILE_RELEASE_DEBUG=0 \
    CARGO_PROFILE_RELEASE_INCREMENTAL=false \
    CARGO_INCREMENTAL=0
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
      tar -xzf "$archive" -C /opt/fsl-driver/bin fsl fsl-plugin-launch; \
      echo "$tag" > /opt/fsl-driver/TAG; \
      echo "$tag" > /opt/fsl-driver/REVISION; \
    else \
      if [ -z "$FSL_REVISION" ]; then echo "FSL_REVISION is required when FSL_SOURCE=source" >&2; exit 1; fi; \
      apt-get install -y --no-install-recommends ca-certificates curl git build-essential pkg-config; \
      curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain "$RUST_VERSION"; \
      PATH=/root/.cargo/bin:$PATH cargo install --git "$FSL_REPO" --rev "$FSL_REVISION" \
        --locked --root /opt/fsl-driver fsl-cli; \
      PATH=/root/.cargo/bin:$PATH cargo install --git "$FSL_REPO" --rev "$FSL_REVISION" \
        --locked --root /opt/fsl-driver filament-plugin-host; \
      strip /opt/fsl-driver/bin/fsl; \
      strip /opt/fsl-driver/bin/fsl-plugin-launch; \
      echo source > /opt/fsl-driver/TAG; \
      echo "$FSL_REVISION" > /opt/fsl-driver/REVISION; \
    fi; \
    rm -rf /var/lib/apt/lists/* /tmp/*

# ------------------------------------------------------------ runner stage
# The example runner, tools/fsl-examples, built from this repository. `make check`
# and `make staged` call it through FSL_EXAMPLES, so the image needs no cargo.
FROM ubuntu:${UBUNTU} AS runner
ARG RUST_VERSION
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl build-essential \
 && curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain "$RUST_VERSION" \
 && rm -rf /var/lib/apt/lists/*
COPY tools/fsl-examples /src/fsl-examples
RUN PATH=/root/.cargo/bin:$PATH cargo install --locked --path /src/fsl-examples --root /opt/fsl-examples \
 && strip /opt/fsl-examples/bin/fsl-examples

# ------------------------------------------------------------ Node base stage
FROM ubuntu:${UBUNTU} AS node-base
ARG NODE_MAJOR
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl jq xz-utils \
 && rm -rf /var/lib/apt/lists/*

# Node from nodejs.org keeps the selected Node major independent of Ubuntu's
# packaged version. The tarball names use arm64 and x64, not aarch64 and x86_64.
# Docs and headers are dropped; the LICENSE file stays with the binary.
RUN set -eu; \
    case "$(uname -m)" in aarch64) a=arm64;; x86_64) a=x64;; esac; \
    v=$(curl -sSf https://nodejs.org/dist/index.json \
        | jq -r "[.[] | select(.version | startswith(\"v${NODE_MAJOR}.\"))][0].version"); \
    curl -sSf https://nodejs.org/dist/$v/node-$v-linux-$a.tar.xz \
      | tar -xJ -C /usr/local --strip-components=1; \
    rm -rf /usr/local/include /usr/local/share/doc /usr/local/share/man /usr/local/*.md

# ---------------------------------------------------------- npm package stage
FROM node-base AS npm-deps
COPY image/package.json image/package-lock.json /opt/npm/
RUN npm ci --prefix /opt/npm --omit=dev --no-audit --no-fund && npm cache clean --force

# -------------------------------------------------------- license data stage
FROM npm-deps AS licenses
ARG RUST_VERSION
ENV RUSTUP_TOOLCHAIN=${RUST_VERSION}
ARG FSL_REPO=https://github.com/agent-ix/filament-driver
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends git build-essential pkg-config \
 && curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain "$RUST_VERSION" \
 && rm -rf /var/lib/apt/lists/*
ENV PATH=/root/.cargo/bin:$PATH
COPY tools/licences.mjs /tmp/licences/tools/licences.mjs
COPY tools/fsl-examples /tmp/licences/tools/fsl-examples
COPY image/licenses/README.md /tmp/licences/image/licenses/README.md
COPY LICENSE /tmp/licences/image/licenses/texts/filament-examples.LICENSE
COPY --from=driver /opt/fsl-driver/TAG /tmp/driver/TAG
COPY --from=driver /opt/fsl-driver/REVISION /tmp/driver/REVISION
RUN set -eu; \
    mkdir -p /tmp/driver-source; \
    git -C /tmp/driver-source init --quiet; \
    git -C /tmp/driver-source remote add origin "$FSL_REPO"; \
    git -C /tmp/driver-source fetch --quiet --depth 1 origin "$(cat /tmp/driver/REVISION)"; \
    git -C /tmp/driver-source checkout --quiet --detach FETCH_HEAD; \
    quire_version=$(node -p "require('/opt/npm/node_modules/@agent-ix/quire-cli/package.json').version"); \
    quoin_version=$(node -p "require('/opt/npm/node_modules/@agent-ix/quoin/package.json').version"); \
    git clone --quiet --depth 1 --branch "v$quire_version" https://github.com/agent-ix/quire-cli.git /tmp/quire-cli-source; \
    git clone --quiet --depth 1 --branch "v$quoin_version" https://github.com/agent-ix/quoin.git /tmp/quoin-source; \
    node /tmp/licences/tools/licences.mjs collect \
      --driver /tmp/driver-source --driver-tag "$(cat /tmp/driver/TAG)" \
      --quire-cli /tmp/quire-cli-source --quoin /tmp/quoin-source; \
    node /tmp/licences/tools/licences.mjs collect-installed \
      /opt/npm/node_modules /opt/npm/package-lock.json /tmp/licences/image/licenses; \
    node /tmp/licences/tools/licences.mjs render; \
    driver_revision=$(git -C /tmp/driver-source rev-parse HEAD); \
    node /tmp/licences/tools/licences.mjs verify-installed "$(cat /tmp/driver/TAG)" "$driver_revision"; \
    cp /tmp/driver/TAG /tmp/licences/image/licenses/DRIVER-TAG; \
    echo "$driver_revision" > /tmp/licences/image/licenses/DRIVER-REVISION

# ----------------------------------------------------------- runtime stage
FROM ubuntu:${UBUNTU}
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl git jq make xz-utils \
 && rm -rf /var/lib/apt/lists/*
COPY --from=node-base /usr/local /usr/local
COPY --from=npm-deps /opt/npm /opt/npm
ENV PATH=/opt/npm/node_modules/.bin:$PATH
# The Markdown tool and quoin install to /usr/local/bin, where `quire` is the
# Markdown tool. The driver binaries are `fsl` and its sibling launcher.
COPY --from=driver /opt/fsl-driver/bin/fsl /usr/local/bin/fsl
COPY --from=driver /opt/fsl-driver/bin/fsl-plugin-launch /usr/local/bin/fsl-plugin-launch
COPY --from=runner /opt/fsl-examples/bin/fsl-examples /usr/local/bin/fsl-examples
ENV FSL_EXAMPLES=fsl-examples

COPY --from=licenses /tmp/licences/image/licenses /usr/share/licenses/filament-toolchain
RUN set -eu; \
    d=/usr/share/licenses/filament-toolchain; \
    cp /usr/local/LICENSE $d/texts/nodejs.LICENSE; \
    { echo "Facts recorded while this image was built."; echo; \
      echo "node $(node --version)"; echo "npm $(npm --version)"; echo "driver tag: $(cat $d/DRIVER-TAG)"; \
      echo "driver revision: $(cat $d/DRIVER-REVISION)"; echo; \
      echo "Installed npm package licence and integrity records: NPM-PACKAGES.txt"; echo; \
      echo "Ubuntu packages (name, version); licences are in /usr/share/doc/<name>/copyright:"; \
      dpkg-query -W -f='${binary:Package} ${Version}\n'; \
    } > $d/BUILD-FACTS.txt; \
    ls /usr/share/doc/*/copyright > /dev/null

# Image metadata. GHCR links the package to the repository in
# org.opencontainers.image.source. The licence expression names the licences of
# the first-party parts: the toolchain programs and the examples; every other
# component keeps its own licence, listed under /usr/share/licenses.
ARG IMAGE_VERSION=unversioned
ARG IMAGE_REVISION=unknown
LABEL org.opencontainers.image.title="Filament toolchain" \
      org.opencontainers.image.description="FSL driver, Markdown tool, quoin, jq, make, git and Node for the Filament examples and tutorials" \
      org.opencontainers.image.source="https://github.com/agent-ix/filament-examples" \
      org.opencontainers.image.licenses="AGPL-3.0-or-later AND MIT" \
      org.opencontainers.image.version="$IMAGE_VERSION" \
      org.opencontainers.image.revision="$IMAGE_REVISION" \
      org.opencontainers.image.documentation="https://github.com/agent-ix/filament-examples/blob/main/image/README.md" \
      io.agent-ix.licenses.notices="/usr/share/licenses/filament-toolchain"

COPY examples /opt/filament-examples/examples
COPY tools /opt/filament-examples/tools
COPY Makefile config.mk index.json README.md rust-toolchain.toml /opt/filament-examples/
COPY image/smoke.sh /usr/local/bin/smoke.sh

WORKDIR /opt/filament-examples
CMD ["smoke.sh"]
