# Filament toolchain image

The Filament toolchain image is a Docker image for running the Filament examples and your own FSL projects. It holds the FSL driver, the Markdown tool, quoin, jq, make, git and Node.js, and is published as `ghcr.io/agent-ix/filament-toolchain` for linux/amd64 and linux/arm64.

You run the checks of the examples in the image without installing a toolchain, and you run the checks of your own project by mounting its folder.

## Getting started

You need Docker. Run the examples that are baked into the image:

```
docker run --rm ghcr.io/agent-ix/filament-toolchain make check staged
```

## Usage

Run the checks of a project in your current folder:

```
docker run --rm --volume "$PWD:/work" --workdir /work ghcr.io/agent-ix/filament-toolchain make check
```

## Commands

The image puts three commands on the path.

| Command | Program |
| --- | --- |
| `fsl` | The FSL driver, a link to the driver binary |
| `quire` | The Markdown tool from the npm package `@agent-ix/quire-cli` |
| `quoin` | The quoin command-line tool |

The driver binary is still named `quire`, the same name as the Markdown tool. The image keeps the driver binary in `/opt/fsl-driver/bin`, off the path, and links it as `fsl`, so `quire` stays the Markdown tool and no file inside either package is renamed.

## Building and testing

Build from the repository root and run the smoke test:

```
docker build -f image/Dockerfile -t filament-toolchain .
docker run --rm filament-toolchain smoke.sh
```

The build downloads the prebuilt driver from the [driver releases](https://github.com/agent-ix/quire-driver/releases). The `FSL_BINARY_URL` build argument names the release archive, a `.tar.gz` file that holds the `quire` binary, and `{target}` becomes `x86_64-unknown-linux-gnu` or `aarch64-unknown-linux-gnu`. To compile the driver from the public [quire-driver](https://github.com/agent-ix/quire-driver) repository instead, pass an empty value. The build then takes a few minutes, with link-time optimization off:

```
docker build -f image/Dockerfile --build-arg FSL_BINARY_URL= -t filament-toolchain .
```

The manual workflow [image.yml](../.github/workflows/image.yml) builds each platform on a native runner, runs `smoke.sh` on each build, and publishes one multi-platform tag for each tag in its `tags` input.

## License

MIT. See the [license file](../LICENSE).
