// Runs the program of steps.md in a fresh copy of ../project and compares what it prints with
// expected/<id>.txt. Skipped when python3 cannot import quire: the wheel is built by the first step of
// steps.md, which this test leaves out.
//   node --test quire-library/python/quire-library-python.test.mjs
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const HERE = resolve(dirname(fileURLToPath(import.meta.url)));
const ROOT = resolve(HERE, "..", "..");

const hasQuire = spawnSync("python3", ["-I", "-c", "import quire"], { encoding: "utf8" }).status === 0;

test("the Python program prints the recorded output", { skip: hasQuire ? false : "python3 cannot import quire" }, () => {
  const r = spawnSync(
    process.execPath,
    [
      join(ROOT, "tools", "steps.mjs"), "run", join(HERE, "steps.md"), "--host", "--skip", "python-wheel",
      "--copy", join(HERE, "..", "project"), "--expected", join(HERE, "expected"),
    ],
    { encoding: "utf8", cwd: ROOT, env: { ...process.env, NODE_TEST_CONTEXT: undefined } },
  );
  assert.equal(r.status, 0, `${r.stdout}\n${r.stderr}`);
});
