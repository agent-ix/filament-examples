# Runs every example. The driver binary comes from FSL (config.mk).
include config.mk

NODE ?= node
RUN = $(NODE) tools/run.mjs

.PHONY: check staged record promote readme index models test install-test install-test-host
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
	$(NODE) tools/model.mjs tools/models/lock.mjs examples/lock/model/lock.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/two-locks.mjs examples/lock/model/two-locks.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/library.mjs examples/library/model/library.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/payment-spec.mjs examples/shop/model/payment-spec.semantic-ir.json
	$(NODE) tools/model.mjs tools/models/payment-client.mjs examples/shop/model/payment-client.semantic-ir.json

# Unit tests for the runner (they use a fake driver and need no FSL binary) and the Quoin first-spec check
# (skipped when quoin and quire are not installed).
test:
	$(NODE) --test tools/*.test.mjs quoin-first-spec/*.test.mjs

# Run the documented install steps on a bare ubuntu:22.04 container (needs Docker).
# INSTALL_DRIVER=release (default) downloads the driver release; source builds it.
# INSTALL_SHELL=zsh runs the steps in zsh, the macOS default shell.
INSTALL_DRIVER ?= release
INSTALL_SHELL ?= bash
INSTALL_SKIP_release = build-tools,rust,driver-source
INSTALL_SKIP_source = driver-target,driver-download,driver-verify,driver-install
install-test:
	$(NODE) tools/steps.mjs run install-test/steps.md --image ubuntu:22.04 --shell $(INSTALL_SHELL) --skip $(INSTALL_SKIP_$(INSTALL_DRIVER))

# Run the release steps on this machine, in zsh (for a macOS runner, which has
# Node.js, git, make and jq already). It installs the driver under $HOME.
install-test-host:
	$(NODE) tools/steps.mjs run install-test/steps.md --host --shell zsh --skip prerequisites,node,$(INSTALL_SKIP_release)
