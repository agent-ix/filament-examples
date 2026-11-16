# FSL names the FSL command-line driver, the `fsl` binary. Override it on the command line to use another
# build: make check FSL=/path/to/driver
FSL ?= fsl
export FSL

# FSL_EXAMPLES runs the example runner (tools/fsl-examples). The default builds and runs it with cargo;
# set it to an installed fsl-examples binary to skip the build, as the toolchain image does.
EXAMPLES_ROOT := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))
FSL_EXAMPLES ?= cargo run --quiet --locked --manifest-path $(EXAMPLES_ROOT)/tools/fsl-examples/Cargo.toml --
