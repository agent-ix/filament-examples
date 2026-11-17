import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const FSL = process.env.FSL;
const FIXTURES = resolve(REPO, "verification/fr865-text/expected");

const probes = [
  {
    name: "shop check renders FR-865 text by default",
    cwd: "examples/shop",
    args: ["check", "rules/values.fsl"],
    expected: "shop-check.stdout.txt",
  },
  {
    name: "library compile renders FR-865 text by default",
    cwd: "examples/library",
    args: ["compile", "requests/compile-ok.json"],
    expected: "library-compile.stdout.txt",
  },
  {
    name: "shop run renders FR-865 text by default",
    cwd: "examples/shop",
    args: ["run", "requests/values-line-total.json"],
    expected: "shop-run.stdout.txt",
  },
];

test("the three FR-865 CLI probes match their default text transcripts", { skip: !FSL }, async (t) => {
  for (const probe of probes) {
    await t.test(probe.name, () => {
      const run = spawnSync(FSL, probe.args, {
        cwd: resolve(REPO, probe.cwd),
        encoding: "utf8",
      });
      assert.equal(run.error, undefined, run.error?.message);
      assert.equal(run.status, 0, run.stderr);
      assert.equal(run.stdout, readFileSync(resolve(FIXTURES, probe.expected), "utf8"));
    });
  }
});
