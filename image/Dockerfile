# Filament toolchain image. Build from the repository root:
#   docker build -f image/Dockerfile -t filament-toolchain .
#   docker run --rm filament-toolchain smoke.sh
#
# The driver comes from one input. By default it is built from source. Pass
# --build-arg FSL_BINARY_URL=<url> to download a prebuilt binary instead; the
# URL may contain {arch}, replaced by x86_64 or aarch64. Nothing else changes.

ARG UBUNTU=22.04

# ------------------------------------------------------------ driver stage
FROM ubuntu:${UBUNTU} AS driver
ARG RUST_VERSION=1.98.1
ARG FSL_BINARY_URL=""
ENV DEBIAN_FRONTEND=noninteractive
# The driver's own release profile (thin LTO, one codegen unit) exhausts the
# memory of a 8 GB machine. These two overrides use cargo's environment
# variables, change no file, and build the same package in a few minutes.
ENV CARGO_PROFILE_RELEASE_LTO=off \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
RUN set -eu; \
    apt-get update; \
    mkdir -p /opt/fsl-driver/bin; \
    if [ -n "$FSL_BINARY_URL" ]; then \
      apt-get install -y --no-install-recommends ca-certificates curl; \
      url=$(printf '%s' "$FSL_BINARY_URL" | sed "s/{arch}/$(uname -m)/"); \
      curl -sSf --location --output /opt/fsl-driver/bin/quire "$url"; \
      chmod +x /opt/fsl-driver/bin/quire; \
    else \
      apt-get install -y --no-install-recommends ca-certificates curl git build-essential pkg-config; \
      curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain "$RUST_VERSION"; \
      PATH=/root/.cargo/bin:$PATH cargo install --git https://github.com/agent-ix/quire-driver \
        --locked --root /opt/fsl-driver quire-cli; \
      strip /opt/fsl-driver/bin/quire; \
    fi; \
    rm -rf /var/lib/apt/lists/*

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
# Markdown tool. The driver binary stays in its own directory, off PATH, and is
# reached as `fsl`. Nothing inside either package is renamed.
RUN npm i -g @agent-ix/quire-cli @agent-ix/quoin && npm cache clean --force
COPY --from=driver /opt/fsl-driver /opt/fsl-driver
RUN ln -s /opt/fsl-driver/bin/quire /usr/local/bin/fsl

COPY examples /opt/filament-examples/examples
COPY tools /opt/filament-examples/tools
COPY Makefile config.mk index.json /opt/filament-examples/
COPY image/smoke.sh /usr/local/bin/smoke.sh

WORKDIR /opt/filament-examples
CMD ["smoke.sh"]
