// Runs the steps of steps.md in a copy of project/ and compares what each prints
// with expected/<id>.txt. Skipped when quoin or quire is not installed.
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
const have = (cmd) => spawnSync(cmd, ["--version"], { encoding: "utf8" }).status === 0;
const missing = ["quoin", "quire"].filter((c) => !have(c));

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

test("every step has a recorded output", () => {
  for (const s of steps) assert.ok(existsSync(join(HERE, "expected", `${s.id}.txt`)), `expected/${s.id}.txt is missing`);
});

test("the staged files exist", () => {
  for (const s of steps) for (const f of s.stage) assert.ok(existsSync(join(HERE, "staged", f)), `staged/${f} is missing`);
});

test("the steps print the recorded output", { skip: missing.length ? `${missing.join(" and ")} not installed` : false }, () => {
  const dir = mkdtempSync(join(tmpdir(), "quoin-first-spec-"));
  try {
    cpSync(join(HERE, "project"), dir, { recursive: true });
    const dirs = [dir, realpathSync(dir)].sort((a, b) => b.length - a.length);
    for (const s of steps.filter((s) => !s.manual)) {
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
    rmSync(dir, { recursive: true, force: true });
  }
});
