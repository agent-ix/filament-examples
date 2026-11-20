#!/usr/bin/env bash
# Build one explicitly selected source cohort on the native Mac runner, then
# exercise the driver's own README through the existing platform smoke lane.
set -euo pipefail
export GIT_TERMINAL_PROMPT=0

if [[ "$(uname -s)-$(uname -m)" != Darwin-arm64 ]]; then
  echo 'This lane requires a native macOS arm64 host.' >&2
  exit 1
fi
: "${RUNNER_TEMP:?GitHub runner temporary directory is required}"
: "${DRIVER_REF:?Select the driver revision}"
: "${LANGUAGE_REF:?Select the FSL revision}"
: "${CODEGEN_REF:?Select the codegen revision}"
: "${IR_REF:?Select the matching contract IR revision}"
df -h /
free_kib=$(df -Pk / | awk 'NR == 2 { print $4 }')
if (( free_kib < 20 * 1024 * 1024 )); then
  echo 'Native build blocked: less than 20 GiB is free on the runner.' >&2
  exit 1
fi

source_root=$(mktemp -d "$RUNNER_TEMP/fsl-macos-source.XXXXXX")
trap 'rm -rf "$source_root"' EXIT

checkout() {
  local repository=$1 revision=$2 directory=$3
  git init --quiet "$directory"
  git -C "$directory" remote add origin "https://github.com/agent-ix/$repository"
  git -C "$directory" fetch --quiet --depth 1 origin "$revision"
  git -C "$directory" checkout --quiet --detach FETCH_HEAD
  printf '%s %s\n' "$repository" "$(git -C "$directory" rev-parse HEAD)" >> "$RUNNER_TEMP/fsl-macos-cohort.txt"
}

: > "$RUNNER_TEMP/fsl-macos-cohort.txt"
checkout filament-driver "$DRIVER_REF" "$source_root/driver"
checkout filament-spec-language "$LANGUAGE_REF" "$source_root/language"
checkout filament-contract-codegen "$CODEGEN_REF" "$source_root/codegen"
checkout filament-contract-ir "$IR_REF" "$source_root/ir"

# Cargo's native path patches select the same source packages throughout the
# graph. These are build-only overrides; no upstream manifest or pin is edited.
config="$source_root/source-cohort.toml"
{
  printf '[patch."https://github.com/agent-ix/filament-spec-language"]\n'
  for manifest in "$source_root"/language/fsl-*/Cargo.toml; do
    package=$(sed -n 's/^name = "\([^"]*\)".*/\1/p' "$manifest" | head -n 1)
    printf '%s = { path = "%s" }\n' "$package" "${manifest%/Cargo.toml}"
  done
  printf '\n[patch."https://github.com/agent-ix/filament-contract-codegen"]\n'
  printf 'filament-contract-codegen = { path = "%s/codegen" }\n' "$source_root"
  printf '\n[patch."https://github.com/agent-ix/filament-contract-ir"]\n'
  printf 'filament-contract-model = { path = "%s/ir/crates/filament-contract-model" }\n' "$source_root"
} > "$config"

export CARGO_TARGET_DIR="$source_root/driver/target"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
(
  cd "$source_root/driver"
  cargo build --config "$config" -p fsl-cli --bin fsl
) 2>&1 | tee "$RUNNER_TEMP/fsl-macos-build.log"

FSL="$CARGO_TARGET_DIR/debug/fsl" \
FSL_DRIVER_DIR="$source_root/driver" \
FSL_SMOKE_PLATFORM=macos-arm64 \
CARGO_TARGET_DIR="$source_root/examples-target" \
  cargo test --locked --manifest-path tools/fsl-examples/Cargo.toml \
    --test platform_smoke quickstart_runs_on_the_selected_native_platform \
    -- --ignored --exact --nocapture --test-threads=1 \
    --skip two_protocols_declaring_one_channel_name_keep_their_queues_apart \
    --skip a_recovery_predicate_over_the_recovery_view_is_evaluated \
    2>&1 | tee "$RUNNER_TEMP/fsl-macos-smoke.log"
