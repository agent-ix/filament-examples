#!/bin/sh
# Smoke test, run inside the image: tool versions, the baked-in examples, and
# the driver's typed response for verbs this build does not route.
set -eu

fsl --version
command -v fsl-plugin-launch >/dev/null
quire --version | grep '^quire '   # the Markdown tool, not the driver
quoin --version
jq --version
make --version | head -1
git --version
node --version

# These are the driver's build's unrouted verbs (fsl-cli/tests/verbs.rs).
for verb in prove generate inspect render; do
  status=0
  result=$(fsl "$verb" --format json) || status=$?
  [ "$status" -eq 21 ] || { echo "fsl $verb exited $status, expected 21"; exit 1; }
  printf '%s\n' "$result" | jq -e --arg verb "$verb" '
    .operation == $verb and
    .category == "unsupported" and
    .diagnostics[0].code == "unsupported_operation" and
    .diagnostics[0].cause == "operation-not-routed" and
    .diagnostics[0].detail.verb == $verb
  ' >/dev/null || { echo "fsl $verb returned an unexpected unsupported response"; exit 1; }
done

# The examples write into their folders, so run a scratch copy.
work=$(mktemp -d)
cp -R /opt/filament-examples/. "$work"
make -C "$work" check staged

# Licence notices: the licence directory, Node's licence and the Ubuntu copyright files.
lic=/usr/share/licenses/filament-toolchain
for f in README.md THIRD-PARTY.md THIRD-PARTY-LICENSES.txt BUILD-FACTS.txt DRIVER-TAG DRIVER-REVISION \
         NPM-PACKAGES.txt texts/filament-driver.LICENSE \
         texts/filament-examples.LICENSE texts/nodejs.LICENSE; do
  [ -s "$lic/$f" ] || { echo "missing $lic/$f"; exit 1; }
done
grep -q '^@agent-ix/quire-cli ' "$lic/NPM-PACKAGES.txt" || { echo "missing quire package record"; exit 1; }
grep -q '^@agent-ix/quoin ' "$lic/NPM-PACKAGES.txt" || { echo "missing quoin package record"; exit 1; }
[ -s /opt/npm/node_modules/@agent-ix/quire-cli/LICENSE ] || { echo "missing quire-cli licence"; exit 1; }
[ -s /opt/npm/node_modules/@agent-ix/quoin/LICENSE ] || { echo "missing quoin licence"; exit 1; }
[ -s "$lic/npm/@agent-ix/quire-cli/LICENSE" ] || { echo "missing copied quire-cli licence"; exit 1; }
[ -s "$lic/npm/@agent-ix/quoin/LICENSE" ] || { echo "missing copied quoin licence"; exit 1; }
[ -s /usr/local/LICENSE ] || { echo "missing /usr/local/LICENSE"; exit 1; }
[ -s /usr/share/doc/git/copyright ] || { echo "missing /usr/share/doc/git/copyright"; exit 1; }
