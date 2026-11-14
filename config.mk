# FSL names the FSL command-line driver. Today the binary is called `quire`;
# after the rename it becomes `fsl`. Change this one line then, or override it
# on the command line: make check FSL=/path/to/driver
FSL ?= fsl
export FSL
