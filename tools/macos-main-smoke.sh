#!/usr/bin/env bash
# Consume one exact successful native main build; no FSL build runs here.
set -euo pipefail
export GIT_TERMINAL_PROMPT=0

if [[ "$(uname -s)-$(uname -m)" != Darwin-arm64 ]]; then
  echo 'This lane requires a native macOS arm64 host.' >&2
  exit 1
fi
: "${RUNNER_TEMP:?GitHub runner temporary directory is required}"
: "${DRIVER_RUN_ID:?Select a successful driver main-build run}"
: "${DRIVER_SHA:?Select its exact driver commit}"
: "${ARTIFACT_DIGEST:?Select the expected sha256 workflow artifact digest}"
: "${GH_TOKEN:?The workflow token is required to download the public artifact}"
[[ "$DRIVER_RUN_ID" =~ ^[0-9]+$ ]]
[[ "$DRIVER_SHA" =~ ^[0-9a-f]{40}$ ]]
[[ "$ARTIFACT_DIGEST" =~ ^sha256:[0-9a-f]{64}$ ]]
df -h /

run_root=$(mktemp -d "$RUNNER_TEMP/fsl-macos-artifact.XXXXXX")
trap 'rm -rf "$run_root"' EXIT
api=repos/agent-ix/filament-driver/actions
gh api "$api/runs/$DRIVER_RUN_ID" > "$run_root/run.json"
jq -e --arg sha "$DRIVER_SHA" '
  .head_sha == $sha and .head_branch == "main" and
  .event == "workflow_dispatch" and .status == "completed" and
  .conclusion == "success" and .path == ".github/workflows/release.yml"
' "$run_root/run.json" > /dev/null
gh api "$api/runs/$DRIVER_RUN_ID/artifacts" > "$run_root/artifacts.json"
jq -e --arg digest "$ARTIFACT_DIGEST" '
  [.artifacts[] | select(.name == "fsl-aarch64-apple-darwin" and .expired == false and .digest == $digest)] |
  if length == 1 then .[0] else error("expected one unexpired main-build artifact with the pinned digest") end
' "$run_root/artifacts.json" > "$run_root/artifact.json"
artifact_id=$(jq -r '.id' "$run_root/artifact.json")
[[ "$artifact_id" =~ ^[0-9]+$ ]]
gh api "$api/artifacts/$artifact_id/zip" > "$run_root/artifact.zip"
actual_digest="sha256:$(shasum -a 256 "$run_root/artifact.zip" | awk '{ print $1 }')"
if [[ "$actual_digest" != "$ARTIFACT_DIGEST" ]]; then
  echo 'Downloaded workflow artifact differs from its pinned digest.' >&2
  exit 1
fi
archive_name=$(unzip -Z1 "$run_root/artifact.zip")
[[ "$archive_name" =~ ^fsl-v[0-9]+\.[0-9]+\.[0-9]+-aarch64-apple-darwin\.tar\.gz$ ]]
unzip -q "$run_root/artifact.zip" -d "$run_root"
archive="$run_root/$archive_name"
[[ "$(tar -tzf "$archive")" == fsl ]]
[[ "$(tar -tvzf "$archive")" == -* ]]
mkdir "$run_root/bin"
tar -xzf "$archive" -C "$run_root/bin" fsl
lipo -verify_arch arm64 "$run_root/bin/fsl"

git init --quiet "$run_root/driver"
git -C "$run_root/driver" remote add origin https://github.com/agent-ix/filament-driver
git -C "$run_root/driver" fetch --quiet --depth 1 origin "$DRIVER_SHA"
git -C "$run_root/driver" checkout --quiet --detach FETCH_HEAD
[[ "$(git -C "$run_root/driver" rev-parse HEAD)" == "$DRIVER_SHA" ]]
printf 'driver_run %s\ndriver_sha %s\nartifact_id %s\nartifact_digest %s\nplatform %s\n' \
  "$DRIVER_RUN_ID" "$DRIVER_SHA" "$artifact_id" "$actual_digest" \
  "$(uname -s)-$(uname -m)" > "$RUNNER_TEMP/fsl-macos-cohort.txt"

FSL="$run_root/bin/fsl" \
FSL_DRIVER_DIR="$run_root/driver" \
FSL_SMOKE_PLATFORM=macos-arm64 \
CARGO_TARGET_DIR="$run_root/examples-target" \
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 \
  cargo test --locked --manifest-path tools/fsl-examples/Cargo.toml \
    --test platform_smoke quickstart_runs_on_the_selected_native_platform \
    -- --ignored --exact --nocapture --test-threads=1 \
    --skip two_protocols_declaring_one_channel_name_keep_their_queues_apart \
    --skip a_recovery_predicate_over_the_recovery_view_is_evaluated \
    2>&1 | tee "$RUNNER_TEMP/fsl-macos-smoke.log"
