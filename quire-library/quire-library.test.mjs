// Runs the steps of steps.md in a fresh copy of project/ and compares what each prints with
// expected/<id>.txt. Skipped when cargo or npm is not installed. The build output goes to a
// temporary folder that the test removes.
//   node --test quire-library/quire-library.test.mjs
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const HERE = resolve(dirname(fileURLToPath(import.meta.url)));
const ROOT = resolve(HERE, "..");

const has = (cmd) => spawnSync(cmd, ["--version"], { encoding: "utf8" }).status === 0;
const missing = ["cargo", "npm"].filter((c) => !has(c));

test("the library programs build and print the recorded output", { skip: missing.length ? `${missing.join(" and ")} not installed` : false }, () => {
  const target = mkdtempSync(join(tmpdir(), "quire-library-"));
  try {
    const r = spawnSync(
      process.execPath,
      [join(ROOT, "tools", "steps.mjs"), "run", join(HERE, "steps.md"), "--host", "--copy", join(HERE, "project"), "--expected", join(HERE, "expected")],
      // The steps run cargo, which behaves differently inside a test run, and share one build folder.
      { encoding: "utf8", cwd: ROOT, env: { ...process.env, NODE_TEST_CONTEXT: undefined, CARGO_TARGET_DIR: target } },
    );
    assert.equal(r.status, 0, `${r.stdout}\n${r.stderr}`);
  } finally {
    rmSync(target, { recursive: true, force: true });
  }
});
