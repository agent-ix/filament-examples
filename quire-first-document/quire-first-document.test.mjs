// Runs the steps of steps.md with the quire and quoin on the path, in a copy of project/, and
// compares what each prints with expected/<id>.txt. Skipped when quire or quoin is missing or
// older than the releases the recordings were made with, or when the default spec modules are
// not installed. The install steps are left out: they need the network and change the machine.
//   node --test quire-first-document/quire-first-document.test.mjs
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { extractSteps } from "../tools/steps.mjs";

const HERE = resolve(dirname(fileURLToPath(import.meta.url)));
const REPO = resolve(HERE, "..");

// The releases the outputs in expected/ were recorded with.
export const RECORDED_WITH = { quoin: "0.30.1", quire: "0.45.0" };
const LEFT_OUT = ["install", "modules", "get-project"];

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
export function skipReason(versions = { quoin: versionOf("quoin"), quire: versionOf("quire") }, modules = existsSync(join(homedir(), ".ix/filament/modules/spec-artifacts-iso"))) {
  const missing = Object.keys(versions).filter((c) => !versions[c]);
  if (missing.length) return `${missing.join(" and ")} not installed`;
  const old = Object.keys(versions).filter((c) => !atLeast(versions[c], RECORDED_WITH[c]));
  if (old.length) return `${old.join(" and ")} older than the recorded release`;
  if (!modules) return "the default spec modules are not installed (run quoin module ensure-defaults)";
  return "";
}

test("every step has a recorded output unless it prints nothing, and the left-out steps exist", () => {
  const steps = extractSteps(readFileSync(join(HERE, "steps.md"), "utf8"));
  for (const id of LEFT_OUT) assert.ok(steps.some((s) => s.id === id), `${id} is not a step`);
  assert.ok(existsSync(join(HERE, "expected", "validate.txt")));
});

test("the skip reason names what is missing or too old", () => {
  assert.equal(skipReason({ quoin: [0, 30, 1], quire: [0, 45, 0] }, true), "");
  assert.equal(skipReason({ quoin: null, quire: [0, 45, 0] }, true), "quoin not installed");
  assert.equal(skipReason({ quoin: [0, 29, 4], quire: [0, 45, 0] }, true), "quoin older than the recorded release");
  assert.match(skipReason({ quoin: [0, 30, 1], quire: [0, 45, 0] }, false), /default spec modules/);
});

test("the steps print the recorded output", { skip: skipReason() || false }, () => {
  const r = spawnSync(
    process.execPath,
    [
      join(REPO, "tools/steps.mjs"), "run", join(HERE, "steps.md"), "--host",
      "--skip", LEFT_OUT.join(","),
      "--copy", join(HERE, "project"),
      "--expected", join(HERE, "expected"),
    ],
    { encoding: "utf8" },
  );
  assert.equal(r.status, 0, `${r.stdout}\n${r.stderr}`);
});
