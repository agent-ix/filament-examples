// Runs the steps of steps.md in a fresh copy of project/ and compares what each prints with
// expected/<id>.txt. Skipped when cargo is not installed.
//   node --test verification/foundations-examples/foundations-examples.test.mjs
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const HERE = resolve(dirname(fileURLToPath(import.meta.url)));
const ROOT = resolve(HERE, "..", "..");

const hasCargo = spawnSync("cargo", ["--version"], { encoding: "utf8" }).status === 0;

test("the foundation libraries examples build and print the recorded output", { skip: hasCargo ? false : "cargo is not installed" }, () => {
  const r = spawnSync(
    process.execPath,
    [
      join(ROOT, "tools", "steps.mjs"),
      "run",
      join(HERE, "steps.md"),
      "--host",
      "--copy",
      join(HERE, "project"),
      "--expected",
      join(HERE, "expected"),
    ],
    { encoding: "utf8", cwd: ROOT },
  );
  assert.equal(r.status, 0, `${r.stdout}\n${r.stderr}`);
});
