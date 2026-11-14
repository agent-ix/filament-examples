// Unit tests for the install-test runner. They need no Docker.
//   node --test tools/
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { buildScript, extractSteps, parseArgs, readMarkers, selectSteps } from "./steps.mjs";

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), "..");

const PAGE = `# Page

Text with an inline \`code\` span.

\`\`\`bash step=one
echo one
\`\`\`

\`\`\`bash
echo not a step
\`\`\`

\`\`\`bash step=two-b
echo two
echo again
\`\`\`
`;

test("extractSteps reads only fenced blocks that carry a step id", () => {
  const steps = extractSteps(PAGE);
  assert.deepEqual(steps.map((s) => s.id), ["one", "two-b"]);
  assert.equal(steps[1].body, "echo two\necho again");
});

test("extractSteps rejects a duplicate id and an unclosed block", () => {
  assert.throws(() => extractSteps("```bash step=a\nx\n```\n```bash step=a\ny\n```\n"), /duplicate step id a/);
  assert.throws(() => extractSteps("```bash step=a\nx\n"), /not closed/);
});

test("selectSteps drops skipped ids and rejects unknown ones", () => {
  const steps = extractSteps(PAGE);
  assert.deepEqual(selectSteps(steps, ["one"]).map((s) => s.id), ["two-b"]);
  assert.throws(() => selectSteps(steps, ["nope"]), /unknown step nope/);
});

test("buildScript runs the steps in order and stops at the first failure", () => {
  const steps = extractSteps("```bash step=a\necho first\n```\n```bash step=b\nfalse\n```\n```bash step=c\necho never\n```\n");
  const run = spawnSync("bash", ["-s"], { input: buildScript(steps), encoding: "utf8" });
  assert.notEqual(run.status, 0);
  assert.match(run.stdout, /first/);
  assert.doesNotMatch(run.stdout, /never/);
  assert.equal(readMarkers(run.stdout.split("\n")).last, "b");
});

test("shell state carries from one step to the next, as in one terminal", () => {
  const steps = extractSteps("```bash step=a\nx=7\ncd /\n```\n```bash step=b\necho \"$x $PWD\"\n```\n");
  const run = spawnSync("bash", ["-s"], { input: buildScript(steps), encoding: "utf8" });
  assert.equal(run.status, 0);
  assert.match(run.stdout, /^7 \/$/m);
});

test("readMarkers turns marker lines into step durations", () => {
  const { records, last } = readMarkers(["@@STEP a 100", "noise", "@@STEP b 130", "@@STEP end 135"]);
  assert.deepEqual(records, [{ id: "a", seconds: 30 }, { id: "b", seconds: 5 }]);
  assert.equal(last, "end");
});

test("parseArgs reads skip and image", () => {
  assert.deepEqual(parseArgs(["run", "f.md", "--image", "ubuntu:22.04", "--skip", "a,b"]), {
    command: "run", file: "f.md", skip: ["a", "b"], image: "ubuntu:22.04",
  });
  assert.throws(() => parseArgs(["run", "f.md", "--bogus"]), /unknown argument/);
});

test("the real steps file has the documented steps, each with a bash body", () => {
  const steps = extractSteps(readFileSync(resolve(REPO, "install-test/steps.md"), "utf8"));
  assert.deepEqual(steps.map((s) => s.id), [
    "prerequisites", "node", "rust", "driver-source", "driver-release", "driver-name", "clone", "check",
  ]);
  for (const s of steps) assert.ok(s.body.trim().length > 0, s.id);
  // A source install and a release install differ only in the two driver steps.
  buildScript(selectSteps(steps, ["driver-release"]));
  buildScript(selectSteps(steps, ["rust", "driver-source"]));
});
