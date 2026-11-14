// Tests for the licence notice generator. They need no Docker, cargo or network.
//   node --test tools/
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { closure, parseArgs, renderTexts, renderThirdParty } from "./licences.mjs";

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

test("the notice names every shipped program with its repository, tag and commit", () => {
  const md = read("image/licenses/THIRD-PARTY.md");
  for (const repo of ["filament-driver", "quire-cli", "quoin"]) {
    assert.match(md, new RegExp(`https://github.com/agent-ix/${repo} \\| v[0-9.]+ \\| [0-9a-f]{40} \\|`));
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
  assert.match(txt, /serde 2/);
  assert.match(txt, /odd 3/);
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
