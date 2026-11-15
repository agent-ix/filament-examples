// Runs the steps of steps.md in a copy of project/ (or of module/) and compares what each
// prints with expected/<id>.txt. Skipped when quoin or quire is missing, or older than the
// releases the recordings were made with.
//   node --test quoin-first-spec/quoin-first-spec.test.mjs
//   RECORD=1 node --test quoin-first-spec/quoin-first-spec.test.mjs    (rewrites expected/)
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const HERE = resolve(dirname(fileURLToPath(import.meta.url)));
const RECORD = process.env.RECORD === "1";

// The releases the outputs in expected/ were recorded with.
export const RECORDED_WITH = { quoin: "0.30.1", quire: "0.45.0" };

const versionOf = (cmd) => {
  const r = spawnSync(cmd, ["--version"], { encoding: "utf8" });
  if (r.status !== 0) return null;
  return /(\d+)\.(\d+)\.(\d+)/.exec(r.stdout)?.slice(1).map(Number) ?? null;
};
const atLeast = (have, want) => {
  const w = want.split(".").map(Number);
  for (let i = 0; i < 3; i++) if (have[i] !== w[i]) return have[i] > w[i];
  return true;
};

// Why the steps cannot be compared, or "" when they can.
export function skipReason(versions = { quoin: versionOf("quoin"), quire: versionOf("quire") }) {
  const missing = Object.keys(versions).filter((c) => !versions[c]);
  if (missing.length) return `${missing.join(" and ")} not installed`;
  const old = Object.keys(versions).filter((c) => !atLeast(versions[c], RECORDED_WITH[c]));
  if (old.length)
    return `${old.map((c) => `${c} ${versions[c].join(".")}`).join(" and ")} older than the recorded ${old
      .map((c) => `${c} ${RECORDED_WITH[c]}`)
      .join(" and ")}`;
  return "";
}

export function parseSteps(markdown) {
  const steps = [];
  const lines = markdown.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const open = /^```(\w+) step=([a-z0-9-]+)(?:\s+(.*))?$/.exec(lines[i]);
    if (!open) continue;
    const body = [];
    for (i++; i < lines.length && !lines[i].startsWith("```"); i++) body.push(lines[i]);
    const words = (open[3] ?? "").split(/\s+/).filter(Boolean);
    const option = (name) => words.find((w) => w.startsWith(`${name}=`))?.slice(name.length + 1);
    steps.push({
      id: open[2],
      shell: open[1],
      body: body.join("\n"),
      manual: words.includes("manual"),
      exit: Number(option("expect-exit") ?? 0),
      stage: (option("stage") ?? "").split(",").filter(Boolean),
      dir: option("dir") ?? "project",
    });
  }
  return steps;
}

// Quire prints notices about the installed modules on standard error, and they
// differ between releases. A result is what a command prints on standard output
// and, when it fails, the lines on standard error that name a document or count
// the documents that failed.
const diagnostic = (line) => /\.md: |document\(s\) failed/.test(line);

export function recorded(stdout, stderr, dirs, status) {
  let text = stdout;
  if (status !== 0) text += stderr.split("\n").filter(diagnostic).join("\n");
  for (const dir of dirs) text = text.split(`${dir}/`).join("");
  return text.replace(/\n+$/, "") + "\n";
}

const steps = parseSteps(readFileSync(join(HERE, "steps.md"), "utf8"));
const skip = skipReason();

test("every step has a recorded output", () => {
  for (const s of steps) assert.ok(existsSync(join(HERE, "expected", `${s.id}.txt`)), `expected/${s.id}.txt is missing`);
});

test("the staged files exist", () => {
  for (const s of steps) for (const f of s.stage) assert.ok(existsSync(join(HERE, "staged", f)), `staged/${f} is missing`);
});

test("the version check names the versions", () => {
  assert.equal(skipReason({ quoin: [0, 30, 1], quire: [0, 45, 0] }), "");
  assert.equal(skipReason({ quoin: [0, 29, 4], quire: [0, 38, 3] }), "quoin 0.29.4 and quire 0.38.3 older than the recorded quoin 0.30.1 and quire 0.45.0");
  assert.equal(skipReason({ quoin: null, quire: [0, 45, 0] }), "quoin not installed");
});

test(`the steps print the recorded output (recorded with quoin ${RECORDED_WITH.quoin}, quire ${RECORDED_WITH.quire})`, { skip: skip || false }, () => {
  const roots = {};
  try {
    for (const name of ["project", "module"]) {
      roots[name] = mkdtempSync(join(tmpdir(), `quoin-first-spec-${name}-`));
      cpSync(join(HERE, name), roots[name], { recursive: true });
    }
    for (const s of steps.filter((s) => !s.manual)) {
      const dir = roots[s.dir];
      const dirs = [dir, realpathSync(dir)].sort((a, b) => b.length - a.length);
      for (const f of s.stage) {
        mkdirSync(dirname(join(dir, f)), { recursive: true });
        cpSync(join(HERE, "staged", f), join(dir, f));
      }
      const r = spawnSync("sh", ["-c", s.body], { cwd: dir, encoding: "utf8" });
      assert.equal(r.status, s.exit, `${s.id} exited ${r.status}: ${r.stderr}`);
      const got = recorded(r.stdout, r.stderr, dirs, r.status);
      const file = join(HERE, "expected", `${s.id}.txt`);
      if (RECORD) writeFileSync(file, got === "\n" ? "" : got);
      else assert.equal(got === "\n" ? "" : got, readFileSync(file, "utf8"), `${s.id} printed something else`);
    }
  } finally {
    for (const dir of Object.values(roots)) rmSync(dir, { recursive: true, force: true });
  }
});
