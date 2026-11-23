// Tests for the image-build licence collector. They need no Docker, cargo or network.
//   node --test tools/licences.test.mjs
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { closure, collectInstalledNpmToDirectory, lockDrift, lockPackages, releaseVersion, renderTexts, renderThirdParty } from "./licences.mjs";

const program = (values) => ({ targets: ["x86_64-unknown-linux-gnu"], crates: [], ...values });

test("closure covers each requested binary root and skips development dependencies", () => {
  const pkg = (name) => ({ id: name, name });
  const meta = {
    packages: ["root", "host", "shared", "dev"].map(pkg),
    resolve: { nodes: [
      { id: "root", deps: [{ pkg: "shared", dep_kinds: [{ kind: null }] }, { pkg: "dev", dep_kinds: [{ kind: "dev" }] }] },
      { id: "host", deps: [{ pkg: "shared", dep_kinds: [{ kind: "build" }] }] },
      { id: "shared", deps: [] },
      { id: "dev", deps: [] },
    ] },
  };
  assert.deepEqual(closure(meta, ["root", "host"]).map((p) => p.name).sort(), ["host", "root", "shared"]);
});

test("runtime notice renders source records for all built programs without reading committed notice files", () => {
  const root = (name) => ({ name, version: "1.0.0", licence: "AGPL-3.0-or-later", source: "workspace", repository: null, commit: null, licenceFiles: [] });
  const thirdParty = { name: "serde", version: "2.0.0", licence: "MIT", source: "crates.io", repository: null, commit: null, licenceFiles: ["h1"] };
  const data = {
    programs: [
      program({ program: "filament-driver", binary: "fsl", version: "0.2.0", repository: "https://github.com/agent-ix/filament-driver", tag: "v0.2.0", commit: "a".repeat(40), crates: [root("fsl-cli"), root("filament-plugin-host"), thirdParty] }),
      program({ program: "quire-cli", binary: "quire", repository: "https://github.com/agent-ix/quire-cli", crates: [thirdParty] }),
      program({ program: "quoin", binary: "quoin", repository: "https://github.com/agent-ix/quoin", crates: [thirdParty] }),
      program({ program: "fsl-examples", binary: "fsl-examples", version: "0.1.0", repository: "https://github.com/agent-ix/filament-examples", path: "tools/fsl-examples", crates: [{ ...root("fsl-examples"), licence: "MIT" }, thirdParty] }),
    ],
    texts: { h1: "MIT text\n" },
  };
  const md = renderThirdParty(data);
  assert.match(md, /filament-driver.*v0\.2\.0/);
  assert.match(md, /`quire` \(quire-cli\).*NPM-PACKAGES\.txt/);
  assert.match(md, /`quoin` \(quoin\).*NPM-PACKAGES\.txt/);
  assert.match(md, /filament-plugin-host/);
  assert.match(md, /fsl-examples/);
  assert.match(renderTexts(data), /MIT text/);
  assert.equal(existsSync(join(import.meta.dirname, "..", "image", "licenses", "THIRD-PARTY.md")), false);
});

test("npm records and license text copies come from installed packages and the lockfile", () => {
  const rootDir = mkdtempSync(join(tmpdir(), "installed-npm-"));
  const modules = join(rootDir, "node_modules");
  const pkgDir = join(modules, "@agent-ix", "sample");
  const output = join(rootDir, "notices");
  mkdirSync(pkgDir, { recursive: true });
  writeFileSync(join(rootDir, "package.json"), JSON.stringify({ dependencies: { "@agent-ix/sample": "1.0.0" } }));
  writeFileSync(join(rootDir, "package-lock.json"), JSON.stringify({ packages: {
    "node_modules/@agent-ix/sample": { version: "1.0.0", resolved: "https://registry.npmjs.org/sample.tgz", integrity: "sha512-artifact" },
    "node_modules/@agent-ix/sample-linux-x64": { version: "1.0.0", resolved: "https://registry.npmjs.org/sample-linux-x64.tgz", integrity: "sha512-optional" },
  } }));
  writeFileSync(join(pkgDir, "package.json"), JSON.stringify({ name: "@agent-ix/sample", version: "1.0.0", license: "MIT" }));
  writeFileSync(join(pkgDir, "LICENSE"), "sample license\n");
  mkdirSync(join(pkgDir, "licenses"));
  writeFileSync(join(pkgDir, "licenses", "NOTICE.txt"), "sample notice\n");
  collectInstalledNpmToDirectory(modules, join(rootDir, "package-lock.json"), output);
  const record = readFileSync(join(output, "NPM-PACKAGES.txt"), "utf8");
  assert.match(record, /@agent-ix\/sample 1\.0\.0/);
  assert.match(record, /sha512-artifact/);
  assert.match(record, /npm\/@agent-ix\/sample\/LICENSE sha256:[0-9a-f]{64}/);
  assert.match(record, /npm\/@agent-ix\/sample\/licenses\/NOTICE\.txt sha256:[0-9a-f]{64}/);
  assert.doesNotMatch(record, /sample-linux-x64/);
  assert.equal(readFileSync(join(output, "npm", "@agent-ix", "sample", "LICENSE"), "utf8"), "sample license\n");
  assert.equal(readFileSync(join(output, "npm", "@agent-ix", "sample", "licenses", "NOTICE.txt"), "utf8"), "sample notice\n");
});

test("lock drift reports missing and unexpected runner packages", () => {
  const lock = '[[package]]\nname = "a"\nversion = "1.0.0"\n\n[[package]]\nname = "dev"\nversion = "2.0.0"\ndependencies = [\n "a",\n]\n';
  assert.deepEqual(lockPackages(lock), ["a@1.0.0", "dev@2.0.0"]);
  const data = { path: "tools/r", crates: [{ name: "a", version: "1.0.0" }], notBuilt: ["dev@2.0.0"] };
  assert.deepEqual(lockDrift(data, lock), []);
  assert.deepEqual(lockDrift(data, lock.replace('"1.0.0"', '"1.1.0"')), [
    "tools/r/Cargo.lock has a@1.1.0, which the licence data does not describe",
    "the licence data describes a@1.0.0, which tools/r/Cargo.lock does not have",
  ]);
});

test("release versions are derived from source tags", () => {
  assert.equal(releaseVersion("v0.32.0"), "0.32.0");
  assert.equal(releaseVersion("0.1.2"), "0.1.2");
});
