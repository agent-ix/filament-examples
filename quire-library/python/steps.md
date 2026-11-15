# Steps of the Quire Python page

The program of the Python page of the Quire documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of [../project](../project), and compares the output of each step with [expected](expected). Run it with `make quire-library-python-test`.

The `quire` wheel is not published on a public index, so the first step builds it from the `v0.60.0` tag of quire-rs with maturin and installs it into a new virtual environment. The step needs Python 3.9 or later, Rust, git and network access to github.com, crates.io and pypi.org, and it takes a few minutes. The test skips it and runs the program when `python3` can already import `quire`.

## Build and install the wheel

```bash step=python-wheel output=ignore
mkdir -p build && cd build
git clone --quiet --depth 1 --branch v0.60.0 --recurse-submodules --shallow-submodules https://github.com/agent-ix/quire-rs.git
python3 -m venv venv
venv/bin/pip install --quiet maturin
(cd quire-rs && ../venv/bin/maturin build --release --out ../wheel)
venv/bin/pip install --quiet wheel/*.whl
. venv/bin/activate
cd ..
```

## Run the program

```bash step=python-demo
python3 -I examples/api_demo.py
```
