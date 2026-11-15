# FSL names the FSL command-line driver, the `fsl` binary. Override it on the command line to use another
# build: make check FSL=/path/to/driver
FSL ?= fsl
export FSL
