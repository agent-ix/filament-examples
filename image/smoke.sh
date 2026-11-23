#!/bin/sh
# Smoke test, run inside the image: tool versions, the baked-in examples, and
# the exit code of verbs the driver does not support yet.
set -eu

fsl --version
command -v fsl-plugin-launch >/dev/null
quire --version | grep '^quire '   # the Markdown tool, not the driver
quoin --version
jq --version
make --version | head -1
git --version
node --version

for verb in analyze prove generate monitor replay; do
  status=0
  fsl "$verb" x >/dev/null 2>&1 || status=$?
  [ "$status" -eq 21 ] || { echo "fsl $verb exited $status, expected 21"; exit 1; }
done

# The examples write into their folders, so run a scratch copy.
work=$(mktemp -d)
cp -R /opt/filament-examples/. "$work"
make -C "$work" check staged

# Licence notices: the licence directory, Node's licence and the Ubuntu copyright files.
lic=/usr/share/licenses/filament-toolchain
for f in README.md THIRD-PARTY.md THIRD-PARTY-LICENSES.txt BUILD-FACTS.txt DRIVER-TAG \
         texts/filament-driver.LICENSE texts/quire-cli.LICENSE texts/quoin.LICENSE \
         texts/filament-examples.LICENSE texts/nodejs.LICENSE; do
  [ -s "$lic/$f" ] || { echo "missing $lic/$f"; exit 1; }
done
[ -s /usr/local/LICENSE ] || { echo "missing /usr/local/LICENSE"; exit 1; }
[ -s /usr/share/doc/git/copyright ] || { echo "missing /usr/share/doc/git/copyright"; exit 1; }
