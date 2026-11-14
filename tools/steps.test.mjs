// Unit tests for the install-test runner. They need no Docker.
//   node --test tools/
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { buildScript, checkRecords, extractSteps, parseArgs, readMarkers, selectSteps, shellCommand } from "./steps.mjs";

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

test("a step marked expect-exit runs with errexit off and reports its status", () => {
  const md = "```bash step=a\necho before\n```\n```bash step=b expect-exit=3\necho oops >&2\nexit_three() { return 3; }\nexit_three\n```\n```bash step=c\necho after\n```\n";
  const steps = extractSteps(md);
  assert.equal(steps[1].expectExit, 3);
  assert.throws(() => extractSteps("```bash step=a expect-exit=x\n```\n"), /expect-exit must be an integer/);
  const run = spawnSync("bash", ["-s"], { input: buildScript(steps), encoding: "utf8" });
  assert.equal(run.status, 0);
  const { records } = readMarkers(run.stdout.split("\n"));
  assert.deepEqual(records.map((r) => [r.id, r.exit, r.output]), [["a", null, "before\n"], ["b", 3, "oops\n"], ["c", null, "after\n"]]);
  assert.deepEqual(checkRecords(steps, records, {}), []);
  assert.match(checkRecords(extractSteps(md.replace("expect-exit=3", "expect-exit=4")), records, {})[0], /b: exit 3, expected 4/);
});

test("checkRecords compares output with the expected folder", () => {
  const dir = mkdtempSync(resolve(tmpdir(), "expected-"));
  writeFileSync(resolve(dir, "a.txt"), "one\n");
  const steps = extractSteps("```bash step=a\n```\n```bash step=b\n```\n");
  const rec = (a) => [{ id: "a", output: a, exit: null }, { id: "b", output: "anything\n", exit: null }];
  assert.deepEqual(checkRecords(steps, rec("one\n"), { expected: dir }), []);
  assert.match(checkRecords(steps, rec("two\n"), { expected: dir })[0], /a: output differs/);
  assert.deepEqual(checkRecords(steps, rec("two\n"), { expected: dir, record: true }), []);
});

test("readMarkers turns marker lines into step durations", () => {
  const { records, last } = readMarkers(["@@STEP a 100", "noise", "@@STEP b 130", "@@STEP end 135"]);
  assert.deepEqual(records, [
    { id: "a", seconds: 30, output: "noise\n", exit: null },
    { id: "b", seconds: 5, output: "", exit: null },
  ]);
  assert.equal(last, "end");
});

test("parseArgs reads skip and image", () => {
  assert.deepEqual(parseArgs(["run", "f.md", "--image", "ubuntu:22.04", "--skip", "a,b"]), {
    command: "run", file: "f.md", skip: ["a", "b"], image: "ubuntu:22.04", shell: "bash", host: false, copy: null, expected: null, record: false,
  });
  assert.deepEqual(parseArgs(["run", "f.md", "--host", "--shell", "zsh"]).shell, "zsh");
  assert.throws(() => parseArgs(["run", "f.md", "--bogus"]), /unknown argument/);
  assert.throws(() => parseArgs(["run", "f.md", "--shell", "fish"]), /--shell must be bash or zsh/);
});

test("the real steps file has the documented steps, each with a bash body", () => {
  const steps = extractSteps(readFileSync(resolve(REPO, "install-test/steps.md"), "utf8"));
  assert.deepEqual(steps.map((s) => s.id), [
    "prerequisites", "node", "driver-target", "driver-download", "driver-verify", "driver-install",
    "build-tools", "rust", "driver-source", "clone", "check",
  ]);
  for (const s of steps) assert.ok(s.body.trim().length > 0, s.id);
  // The release path and the source path differ only in their driver steps.
  const release = ["driver-target", "driver-download", "driver-verify", "driver-install"];
  const source = ["build-tools", "rust", "driver-source"];
  buildScript(selectSteps(steps, source));
  buildScript(selectSteps(steps, release));
});

test("shellCommand starts zsh in a bare container after installing it, and on the host directly", () => {
  const zsh = shellCommand({ host: false, image: "ubuntu:22.04", shell: "zsh" });
  assert.equal(zsh.cmd, "docker");
  assert.match(zsh.args.at(-1), /apt-get install -y -qq zsh .*exec zsh -f -s$/);
  assert.match(shellCommand({ host: false, image: "i", shell: "bash" }).args.at(-1), /^exec bash -s$/);
  assert.deepEqual(shellCommand({ host: true, shell: "zsh" }), { cmd: "zsh", args: ["-f", "-s"], cwd: null });
  const copy = shellCommand({ host: false, image: "i", shell: "bash", copy: "." });
  assert.match(copy.args.join(" "), /:\/src:ro .* cp -R \/src \/work && cd \/work && exec bash -s$/);
});

// The checksum step ran only under bash for a while and failed every time under
// zsh, the macOS default shell. Run the real step under every shell present.
for (const shell of ["bash", "zsh"]) {
  const present = spawnSync(shell, ["-c", "true"]).status === 0;
  test(`the driver-verify step passes a good archive and fails a changed one under ${shell}`, { skip: !present }, () => {
    const steps = extractSteps(readFileSync(resolve(REPO, "install-test/steps.md"), "utf8"));
    const verify = steps.find((s) => s.id === "driver-verify");
    const dir = mkdtempSync(resolve(tmpdir(), "verify-"));
    const archive = "fsl-v1-x86_64-unknown-linux-gnu.tar.gz";
    writeFileSync(resolve(dir, archive), "archive bytes");
    const sum = createHash("sha256").update("archive bytes").digest("hex");
    writeFileSync(resolve(dir, "SHA256SUMS.txt"), `${sum}  ${archive}\n`);
    const run = () => spawnSync(shell, shell === "zsh" ? ["-f", "-s"] : ["-s"], {
      input: `set -eu\ncd ${dir}\narchive=${archive}\n${verify.body}\n`, encoding: "utf8",
    });
    const good = run();
    assert.equal(good.status, 0, good.stderr);
    assert.match(good.stdout, /: OK/);
    writeFileSync(resolve(dir, archive), "changed bytes");
    assert.notEqual(run().status, 0);
  });
}

test("the driver-target step stops with a message when the release lookup lands on no tag", () => {
  const steps = extractSteps(readFileSync(resolve(REPO, "install-test/steps.md"), "utf8"));
  const target = steps.find((s) => s.id === "driver-target");
  assert.match(target.body, /No release of the FSL driver was found/);
  // Same guard, run on its own with a tag that is not a version.
  const guard = target.body.split("\n").slice(3, 7).join("\n");
  const run = spawnSync("bash", ["-s"], { input: `repo=r\ntag=releases\n${guard}\n`, encoding: "utf8" });
  assert.notEqual(run.status, 0);
  assert.match(run.stderr, /No release of the FSL driver was found at r\/releases/);
});
