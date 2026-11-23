// Tests for the licence notice generator. They need no Docker, cargo or network.
//   node --test tools/
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { closure, collectInstalledNpm, lockDrift, lockPackages, parseArgs, releaseVersion, renderTexts, renderThirdParty } from "./licences.mjs";

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const LICENSES = join(REPO, "image", "licenses");
const read = (p) => readFileSync(join(REPO, p), "utf8");

test("the committed notice is what render writes from the committed data", () => {
  const out = mkdtempSync(join(tmpdir(), "licences-"));
  const r = spawnSync(process.execPath, [join(REPO, "tools", "licences.mjs"), "render", "--out", out], { encoding: "utf8" });
  assert.equal(r.status, 0, r.stderr);
  for (const f of ["THIRD-PARTY.md", "THIRD-PARTY-LICENSES.txt"]) {
    assert.equal(readFileSync(join(out, f), "utf8"), readFileSync(join(LICENSES, f), "utf8"), `${f} is stale: run node tools/licences.mjs render`);
  }
  const c = spawnSync(process.execPath, [join(REPO, "tools", "licences.mjs"), "check"], { encoding: "utf8" });
  assert.equal(c.status, 0, c.stderr);
});

test("the notice keeps npm package versions out of checked-in records", () => {
  const md = read("image/licenses/THIRD-PARTY.md");
  assert.match(md, /https:\/\/github\.com\/agent-ix\/filament-driver \\| v[0-9.]+ \\| [0-9a-f]{40} \\|/);
  assert.match(md, /`quire` \(quire-cli\).*NPM-PACKAGES\.txt/);
  assert.match(md, /`quoin` \(quoin\).*NPM-PACKAGES\.txt/);
  for (const file of ["image/licenses/data/quire-cli.json", "image/licenses/data/quoin.json"]) {
    const record = JSON.parse(read(file));
    assert.equal("version" in record, false);
    assert.equal("tag" in record, false);
    assert.equal("commit" in record, false);
  }
});

test("closure follows normal and build dependencies and skips development ones", () => {
  const pkg = (name) => ({ id: name, name });
  const meta = {
    packages: ["root", "a", "b", "dev"].map(pkg),
    resolve: {
      nodes: [
        { id: "root", deps: [{ pkg: "a", dep_kinds: [{ kind: null }] }, { pkg: "dev", dep_kinds: [{ kind: "dev" }] }] },
        { id: "a", deps: [{ pkg: "b", dep_kinds: [{ kind: "build" }] }] },
        { id: "b", deps: [] },
        { id: "dev", deps: [] },
      ],
    },
  };
  assert.deepEqual(closure(meta, "root").map((p) => p.name).sort(), ["a", "b", "root"]);
});

test("first-party crates are separated from third-party crates", () => {
  const data = {
    programs: [{
      program: "filament-driver", binary: "fsl", version: "1", repository: "https://github.com/agent-ix/filament-driver", tag: "v1", commit: "c".repeat(40), targets: ["t"],
      crates: [
        { name: "fsl-cli", version: "1", licence: "AGPL-3.0-or-later", source: "workspace", repository: null, commit: null, licenceFiles: [] },
        { name: "serde", version: "2", licence: "MIT OR Apache-2.0", source: "crates.io", repository: null, commit: null, licenceFiles: ["h1"] },
        { name: "odd", version: "3", licence: null, source: "crates.io", repository: null, commit: null, licenceFiles: [] },
      ],
    }],
    texts: { h1: "MIT text\n" },
  };
  const md = renderThirdParty(data);
  assert.match(md, /1 first-party and 2 third-party/);
  assert.match(md, /\| none declared \| 1 \|/);
  const txt = renderTexts(data);
  assert.match(txt, /serde/);
  assert.match(txt, /odd/);
});

test("npm records use installed package metadata and lockfile integrity", () => {
  const root = mkdtempSync(join(tmpdir(), "installed-npm-"));
  const modules = join(root, "node_modules");
  const pkgDir = join(modules, "@agent-ix", "sample");
  mkdirSync(pkgDir, { recursive: true });
  writeFileSync(join(root, "package.json"), JSON.stringify({ dependencies: { "@agent-ix/sample": "1.0.0" } }));
  writeFileSync(join(root, "package-lock.json"), JSON.stringify({ packages: {
    "node_modules/@agent-ix/sample": {
      version: "1.0.0", resolved: "https://registry.npmjs.org/sample.tgz", integrity: "sha512-artifact", license: "MIT",
    },
    "node_modules/@agent-ix/sample-linux-x64": {
      version: "1.0.0", resolved: "https://registry.npmjs.org/sample-linux-x64.tgz", integrity: "sha512-optional",
    },
  } }));
  writeFileSync(join(pkgDir, "package.json"), JSON.stringify({ name: "@agent-ix/sample", version: "1.0.0", license: "MIT" }));
  writeFileSync(join(pkgDir, "LICENSE"), "sample license\n");
  const output = collectInstalledNpm({ modules, lockPath: join(root, "package-lock.json") });
  assert.match(output, /@agent-ix\/sample 1\.0\.0/);
  assert.match(output, /sha512-artifact/);
  assert.match(output, /LICENSE sha256:[0-9a-f]{64}/);
  assert.doesNotMatch(output, /sample-linux-x64/);
});

test("the Dockerfile keeps the licence files and carries the labels", () => {
  const dockerfile = read("image/Dockerfile");
  assert.doesNotMatch(dockerfile, /rm -rf[^\n]*\/usr\/local\/LICENSE/);
  assert.doesNotMatch(dockerfile, /\/usr\/share\/doc\/\*\/copyright[^\n]*rm|rm[^\n]*\/usr\/share\/doc\/\*/);
  for (const label of ["image.source", "image.description", "image.licenses", "image.version", "image.revision"]) {
    assert.ok(dockerfile.includes(`org.opencontainers.${label}=`), label);
  }
});

test("parseArgs reads options and words", () => {
  assert.deepEqual(parseArgs(["collect", "--driver", "d"]), { _: ["collect"], driver: "d" });
});

test("the shipped version is the release tag, not the root crate version", () => {
  assert.equal(releaseVersion("v0.32.0"), "0.32.0");
  assert.equal(releaseVersion("0.1.2"), "0.1.2");
});

test("the notice lists the example runner the image builds, with its third-party crates", () => {
  const md = read("image/licenses/THIRD-PARTY.md");
  assert.match(md, /\| `fsl-examples` \(tools\/fsl-examples\) \| [0-9.]+ \| MIT \| `texts\/filament-examples.LICENSE` \| https:\/\/github.com\/agent-ix\/filament-examples \|/);
  const section = md.slice(md.indexOf("## Rust crates built into `fsl-examples`"));
  for (const crate of ["serde", "serde_json", "toml", "thiserror"]) assert.match(section, new RegExp(`\\| ${crate} \\| [^|]+ \\| [^|]*MIT`));
  assert.doesNotMatch(section, /\| tempfile \|/, "development dependencies are not shipped");
  assert.match(read("image/licenses/THIRD-PARTY-LICENSES.txt"), /serde_json/);
});

test("lockDrift reports lock packages the licence data does not describe, and the reverse", () => {
  const lock = '[[package]]\nname = "a"\nversion = "1.0.0"\n\n[[package]]\nname = "dev"\nversion = "2.0.0"\ndependencies = [\n "a",\n]\n';
  assert.deepEqual(lockPackages(lock), ["a@1.0.0", "dev@2.0.0"]);
  const data = { path: "tools/r", crates: [{ name: "a", version: "1.0.0" }], notBuilt: ["dev@2.0.0"] };
  assert.deepEqual(lockDrift(data, lock), []);
  assert.deepEqual(lockDrift(data, lock.replace('"1.0.0"', '"1.1.0"')), [
    "tools/r/Cargo.lock has a@1.1.0, which the licence data does not describe",
    "the licence data describes a@1.0.0, which tools/r/Cargo.lock does not have",
  ]);
});

test("the image build checks the runner's Cargo.lock against the licence data", () => {
  const dockerfile = read("image/Dockerfile");
  assert.match(dockerfile, /COPY tools\/fsl-examples\/Cargo.lock \/tmp\/licences\/tools\/fsl-examples\/Cargo.lock/);
  assert.match(dockerfile, /npm ci --prefix \/opt\/npm/);
  assert.match(dockerfile, /collect-installed \/opt\/npm\/node_modules \/opt\/npm\/package-lock\.json/);
  assert.doesNotMatch(dockerfile, /@agent-ix\/(?:quire-cli|quoin)@[0-9]/);
});
