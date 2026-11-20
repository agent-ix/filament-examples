# Runs every example. The driver binary comes from FSL and the runner from FSL_EXAMPLES (config.mk).
include config.mk

NODE ?= node
RUN = $(FSL_EXAMPLES)

.PHONY: tutorial-test tutorial-first-rule-test tutorial-write-model-test
tutorial-test: tutorial-first-rule-test tutorial-write-model-test
tutorial-first-rule-test:
	$(NODE) tools/steps.mjs run tutorials/first-rule/steps.md --host --copy examples/shop
tutorial-write-model-test:
	$(NODE) tools/steps.mjs run tutorials/write-a-model/steps.md --host --copy .

.PHONY: check staged record promote readme index models test runner-test install-test install-test-host assurance-test assurance-record quire-test quire-record tl-examples-test tl-examples-record quire-library-test quire-library-record quire-library-python-test quire-library-python-record foundations-examples-test foundations-examples-record contracts-examples-test contracts-examples-record
check:
	@$(RUN) check
staged:
	@$(RUN) staged
record:
	@$(RUN) record
promote:
	@$(if $(CASE),,$(error CASE is required, for example: make promote CASE=rover/vote-two-of-three))
	@$(RUN) promote "$(CASE)" $(if $(VERDICT),--verdict "$(VERDICT)")

# Rewrite the recorded-output regions of the READMEs and index.json from the manifests
# and expected/. make check fails when either is stale.
readme index:
	@$(RUN) $@

# Regenerate the Semantic IR models from tools/models (interim, see tools/README.md).
models:
	$(NODE) tools/model.mjs tools/models/shop.mjs examples/shop/model/shop.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/architecture.mjs examples/architecture/model/architecture.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/parts.mjs examples/parts-saga/model/parts-saga.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/cell.mjs examples/cell/model/cell.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/rover.mjs examples/rover/model/rover.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/rover-spec.mjs examples/rover/model/rover-spec.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/rover-deadline.mjs examples/rover/model/rover-deadline.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/lock.mjs examples/lock/model/lock.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/two-locks.mjs examples/lock/model/two-locks.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/library.mjs examples/library/model/library.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/payment-spec.mjs examples/shop/model/payment-spec.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/payment-client.mjs examples/shop/model/payment-client.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/shop-outcomes.mjs examples/shop/model/shop-outcomes.semantic-ir.json

# Tests of the example runner in tools/fsl-examples. They use a stand-in driver and need cargo, not an FSL binary.
runner-test:
	cargo test --locked --manifest-path tools/fsl-examples/Cargo.toml

# Unit tests for the Node tools (steps, licences, identifier rewrite), the Quoin first-spec check (skipped
# when quoin and quire are not installed) and the example projects that run their steps: the temporal logic
# and the Quire library tests build Rust programs from git and need cargo, npm and network access, and skip
# themselves when cargo or npm is missing.
test:
	$(NODE) --test tools/*.test.mjs quoin-first-spec/*.test.mjs quire-first-document/*.test.mjs verification/tl-examples/*.test.mjs verification/foundations-examples/*.test.mjs verification/contracts-examples/*.test.mjs quire-library/*.test.mjs quire-library/python/*.test.mjs

# Run the documented install steps on a bare ubuntu:22.04 container (needs Docker).
# INSTALL_DRIVER=release (default) downloads the driver release; source builds it.
# INSTALL_SHELL=zsh runs the steps in zsh, the macOS default shell.
INSTALL_DRIVER ?= release
INSTALL_SHELL ?= bash
INSTALL_SKIP_release = driver-source,check-source
INSTALL_SKIP_source = driver-target,driver-download,driver-verify,driver-install,check-release
install-test:
	INSTALL_DRIVER=$(INSTALL_DRIVER) $(NODE) tools/steps.mjs run install-test/steps.md --image ubuntu:22.04 --shell $(INSTALL_SHELL) --skip $(INSTALL_SKIP_$(INSTALL_DRIVER))

# Run the release steps on this machine, in zsh (for a macOS runner, which has
# git, make and Rust already). It installs the driver under $HOME.
install-test-host:
	INSTALL_DRIVER=release $(NODE) tools/steps.mjs run install-test/steps.md --host --shell zsh --skip prerequisites,build-tools,rust,$(INSTALL_SKIP_release)

# Run the Assurance steps (assurance/steps.md) on this machine against a fresh
# copy of assurance/project and compare each step's output with
# assurance/expected. Needs Node.js 22.18 or later, npm and git, and network
# access to npmjs.org and github.com. HOME is a new temporary folder and npm
# installs into it, so the tools and spec modules never touch your own. The
# folder is removed when the run ends.
ASSURANCE_ENV = assurance_home="$$(mktemp -d)" && trap 'rm -rf "$$assurance_home"' EXIT && export HOME="$$assurance_home" && export npm_config_prefix="$$HOME/npm" && export PATH="$$HOME/npm/bin:$$PATH" &&
ASSURANCE_RUN = $(NODE) tools/steps.mjs run assurance/steps.md --host --copy assurance/project --expected assurance/expected
assurance-test:
	$(ASSURANCE_ENV) $(ASSURANCE_RUN)

# Record the outputs of the Assurance steps again.
assurance-record:
	$(ASSURANCE_ENV) $(ASSURANCE_RUN) --record

# Run the Quire steps (quire-first-document/steps.md) on this machine against a fresh copy of
# quire-first-document/project and compare each step's output with quire-first-document/expected.
# Needs Node.js 22.18 or later, npm and git, and network access to npmjs.org and github.com. HOME is a
# new temporary folder and npm installs into it, so the tools and spec modules never touch your own.
QUIRE_ENV = quire_home="$$(mktemp -d)" && trap 'rm -rf "$$quire_home"' EXIT && export HOME="$$quire_home" && export npm_config_prefix="$$HOME/npm" && export PATH="$$HOME/npm/bin:$$PATH" &&
QUIRE_RUN = $(NODE) tools/steps.mjs run quire-first-document/steps.md --host --skip get-project --copy quire-first-document/project --expected quire-first-document/expected
quire-test:
	$(QUIRE_ENV) $(QUIRE_RUN)

# Record the outputs of the Quire steps again.
quire-record:
	$(QUIRE_ENV) $(QUIRE_RUN) --record

# Run the Rust programs of the temporal logic pages (verification/tl-examples/steps.md) in a
# fresh copy of verification/tl-examples/project and compare each step's output with
# verification/tl-examples/expected. Needs cargo and network access to github.com and crates.io;
# it prints a message and succeeds when cargo is not installed.
TL_EXAMPLES_RUN = $(NODE) tools/steps.mjs run verification/tl-examples/steps.md --host --copy verification/tl-examples/project --expected verification/tl-examples/expected
tl-examples-test:
	@if command -v cargo >/dev/null 2>&1; then $(TL_EXAMPLES_RUN); else echo "tl-examples-test skipped: cargo is not installed"; fi

# Record the outputs of the temporal logic steps again.
tl-examples-record:
	$(TL_EXAMPLES_RUN) --record

# Run the Rust and WebAssembly programs of the Quire library pages (quire-library/steps.md) in a fresh copy of
# quire-library/project and compare each step's output with quire-library/expected. Needs cargo, Node.js and npm,
# and network access to github.com, crates.io and npmjs.org. The build output goes to a temporary folder that is
# removed when the run ends. It prints a message and succeeds when cargo is not installed.
QUIRE_LIBRARY_ENV = quire_library_target="$$(mktemp -d)" && trap 'rm -rf "$$quire_library_target"' EXIT && export CARGO_TARGET_DIR="$$quire_library_target" &&
QUIRE_LIBRARY_RUN = $(NODE) tools/steps.mjs run quire-library/steps.md --host --skip get-project --copy quire-library/project --expected quire-library/expected
quire-library-test:
	@if command -v cargo >/dev/null 2>&1; then $(QUIRE_LIBRARY_ENV) $(QUIRE_LIBRARY_RUN); else echo "quire-library-test skipped: cargo is not installed"; fi

# Record the outputs of the library steps again.
quire-library-record:
	$(QUIRE_LIBRARY_ENV) $(QUIRE_LIBRARY_RUN) --record

# Build the quire wheel from the v0.60.0 tag of quire-rs and run the Python program of the Python page
# (quire-library/python/steps.md). Needs Python 3.9 or later, Rust, git and network access to github.com,
# crates.io and pypi.org, and takes a few minutes. The build output goes to a temporary folder that is removed
# when the run ends.
QUIRE_LIBRARY_PYTHON_RUN = $(NODE) tools/steps.mjs run quire-library/python/steps.md --host --skip get-project --copy quire-library --expected quire-library/python/expected
quire-library-python-test:
	$(QUIRE_LIBRARY_ENV) $(QUIRE_LIBRARY_PYTHON_RUN)

# Record the output of the Python step again.
quire-library-python-record:
	$(QUIRE_LIBRARY_ENV) $(QUIRE_LIBRARY_PYTHON_RUN) --record

# Run the Rust programs of the foundation libraries pages (verification/foundations-examples/steps.md)
# in a fresh copy of verification/foundations-examples/project and compare each step's output with
# verification/foundations-examples/expected. Needs cargo and network access to github.com and
# crates.io; it prints a message and succeeds when cargo is not installed.
FOUNDATIONS_EXAMPLES_RUN = $(NODE) tools/steps.mjs run verification/foundations-examples/steps.md --host --copy verification/foundations-examples/project --expected verification/foundations-examples/expected
foundations-examples-test:
	@if command -v cargo >/dev/null 2>&1; then $(FOUNDATIONS_EXAMPLES_RUN); else echo "foundations-examples-test skipped: cargo is not installed"; fi

# Record the outputs of the foundation libraries steps again.
foundations-examples-record:
	$(FOUNDATIONS_EXAMPLES_RUN) --record

# Run the Rust programs of the contract libraries pages (verification/contracts-examples/steps.md)
# in a fresh copy of verification/contracts-examples/project and compare each step's output with
# verification/contracts-examples/expected. Needs cargo and network access to github.com and
# crates.io; it prints a message and succeeds when cargo is not installed.
CONTRACTS_EXAMPLES_RUN = $(NODE) tools/steps.mjs run verification/contracts-examples/steps.md --host --copy verification/contracts-examples/project --expected verification/contracts-examples/expected
contracts-examples-test:
	@if command -v cargo >/dev/null 2>&1; then $(CONTRACTS_EXAMPLES_RUN); else echo "contracts-examples-test skipped: cargo is not installed"; fi

# Record the outputs of the contract libraries steps again.
contracts-examples-record:
	$(CONTRACTS_EXAMPLES_RUN) --record
