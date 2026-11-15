# Steps of the Quire Python page

The program of the Python page of the Quire documentation. The test runs every fenced block marked `step=<id>`, in order, in one shell, in a fresh copy of the `quire-library` folder, and compares the output of each step with [expected](expected). Run it with `make quire-library-python-test`.

The program, [api_demo.py](api_demo.py), reads the module and the documents of the Rust project in [../project](../project). The `quire` wheel is not published on a public index, so the first step builds it from the `v0.60.0` tag of quire-rs with maturin and installs it into a new virtual environment. That step needs Python 3.9 or later, Rust, git and network access to github.com, crates.io and pypi.org, and it takes a few minutes. The test skips it and runs the program when `python3` can already import `quire`.

The test starts in a copy of the `quire-library` folder, so it skips this step.

```bash step=get-project
git clone https://github.com/agent-ix/filament-examples
cd filament-examples/quire-library
```

## Build and install the wheel

```bash step=python-wheel output=ignore
mkdir -p python/build && cd python/build
git clone --quiet --depth 1 --branch v0.60.0 --recurse-submodules --shallow-submodules https://github.com/agent-ix/quire-rs.git
python3 -m venv venv
venv/bin/pip install --quiet maturin
(cd quire-rs && ../venv/bin/maturin build --release --out ../wheel)
venv/bin/pip install --quiet wheel/*.whl
. venv/bin/activate
cd ../..
```

## Run the program

```bash step=python-demo
cd python
python3 api_demo.py
```
