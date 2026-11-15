// Unit tests for the runner. They use a fake driver and need no FSL binary.
//   node --test tools/
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { buildIndex, discoverSystems, syncGenerated, evaluateCase, findCase, main, parseManifest, promoteText, renderOutput, syncReadmes, verdictOf } from "./run.mjs";

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const RUNNER = resolve(dirname(fileURLToPath(import.meta.url)), "run.mjs");
const OK_DOC = '{"format":"filament-outcome/1","category":"success","result":{"value":{"kind":"integer","decimal":"7"}}}\n';

const MANIFEST = `# Demo
name = "demo"
summary = "A demo."

# A comment between cases.
[[case]]
id = "good"
rule = "R"
statement = "A rule."
file = "rules/r.fsl"
command = "run requests/good.json"
status = "running"
expect_exit = 0
expect_verdict = "holds"

[[case]]
id = "later"
rule = "R2"
statement = "A later rule."
file = "rules/r.fsl"
command = "run requests/later.json"
status = "staged"
expect_exit = 21
ready_exit = 0
waits_on = "QSL-1"
why = "not built yet"
needs = "a feature"
serves = "a page"
`;

test("parseManifest reads header, cases and types", () => {
  const m = parseManifest(MANIFEST);
  assert.equal(m.header.name, "demo");
  assert.equal(m.cases.length, 2);
  assert.equal(m.cases[0].expect_exit, 0);
  assert.equal(m.cases[1].waits_on, "QSL-1");
});

test("parseManifest rejects bad input with a line number", () => {
  assert.throws(() => parseManifest("name = demo\n", "m.toml"), /m\.toml:1: value must be/);
  assert.throws(() => parseManifest("what\n", "m.toml"), /m\.toml:1: cannot read line/);
  assert.throws(() => parseManifest('[[case]]\nid = "a"\nid = "b"\n', "m.toml"), /duplicate key id/);
});

test("parseManifest validates cases", () => {
  const bad = (body, re) => assert.throws(() => parseManifest(`[[case]]\nstatement = "s"\nfile = "f"\n${body}`), re);
  bad('id = "a"\ncommand = "run x"\nstatus = "runs"\nexpect_exit = 0\n', /status must be running or staged/);
  bad('id = "a"\ncommand = "run x"\nstatus = "running"\nexpect_exit = 0\n', /expect_verdict must be a string/);
  bad('id = "a"\ncommand = "run x"\nstatus = "staged"\nexpect_exit = 21\nready_exit = 21\nwaits_on = "QSL-9"\nwhy = "w"\nneeds = "n"\n', /ready_exit equals expect_exit/);
  assert.throws(() => parseManifest(`${MANIFEST}\n[[case]]\nstatement = "s"\nfile = "f"\nid = "good"\ncommand = "c"\nstatus = "running"\nexpect_exit = 0\nexpect_verdict = "v"\n`), /duplicate case id good/);
});

const running = { id: "good", status: "running", expect_exit: 0 };
const staged = { id: "later", status: "staged", expect_exit: 21, ready_exit: 0 };

test("evaluateCase passes a running case that matches", () => {
  const r = evaluateCase(running, { exit: 0, stdout: OK_DOC }, OK_DOC, "demo/good");
  assert.deepEqual(r, { ok: true, message: "demo/good: exit 0 ok" });
});

test("evaluateCase fails a running case on exit, output and missing record", () => {
  assert.match(evaluateCase(running, { exit: 10, stdout: "" }, "", "d/good").message, /exit 10, expected 0/);
  assert.match(evaluateCase(running, { exit: 0, stdout: "{}" }, OK_DOC, "d/good").message, /output differs from expected\/good\.json/);
  assert.match(evaluateCase(running, { exit: 0, stdout: OK_DOC }, null, "d/good").message, /missing; run make record/);
});

test("evaluateCase hints at the wrong binary when output is not an outcome document", () => {
  const r = evaluateCase(running, { exit: 0, stdout: "Usage: quire ..." }, OK_DOC, "d/good");
  assert.match(r.message, /check that FSL names the FSL driver/);
});

test("evaluateCase holds a staged case to its recorded failure", () => {
  assert.equal(evaluateCase(staged, { exit: 21, stdout: "" }, "", "d/later").ok, true);
  assert.match(evaluateCase(staged, { exit: 20, stdout: "" }, "", "d/later").message, /staged case changed: exit 20, expected 21/);
  assert.match(evaluateCase(staged, { exit: 21, stdout: "x" }, "", "d/later").message, /staged case changed: output differs/);
});

test("evaluateCase tells the author to promote a staged case that now passes", () => {
  const r = evaluateCase(staged, { exit: 0, stdout: OK_DOC }, "", "d/later");
  assert.equal(r.ok, false);
  assert.match(r.message, /promote d\/later/);
});

test("promoteText flips only the named case and keeps the rest", () => {
  const out = promoteText(MANIFEST, "later", "holds");
  assert.match(out, /id = "later"[\s\S]*status = "running"\nexpect_exit = 0\nexpect_verdict = "holds"\nserves = "a page"/);
  assert.doesNotMatch(out, /ready_exit|waits_on|why =|needs =/);
  assert.match(out, /# A comment between cases\./);
  assert.equal(parseManifest(out).cases[1].status, "running");
  assert.throws(() => promoteText(MANIFEST, "good", "x"), /no ready_exit/);
  assert.throws(() => promoteText(MANIFEST, "nope", "x"), /not found/);
});

test("verdictOf summarizes outcome documents", () => {
  assert.equal(verdictOf(OK_DOC), "7");
  assert.equal(verdictOf('{"category":"violation"}'), "violated");
  assert.equal(verdictOf('{"category":"refusal","diagnostics":[{"code":"ill_typed"}]}'), "refusal: ill_typed");
  assert.equal(verdictOf("not json"), "unreadable output");
});

test("findCase resolves system/id and bare ids", () => {
  const a = { name: "a", cases: [{ id: "x" }, { id: "y" }] };
  const b = { name: "b", cases: [{ id: "x" }] };
  assert.equal(findCase([a, b], "a/x").sys, a);
  assert.equal(findCase([a, b], "y").c.id, "y");
  assert.throws(() => findCase([a, b], "x"), /ambiguous/);
  assert.throws(() => findCase([a, b], "z"), /no case/);
});

// --- End to end with a fake driver -----------------------------------------

function fixture() {
  const root = mkdtempSync(join(tmpdir(), "runner-"));
  const dir = join(root, "examples", "demo");
  mkdirSync(join(dir, "expected"), { recursive: true });
  writeFileSync(join(dir, "example.toml"), MANIFEST);
  mkdirSync(join(dir, "requests"));
  writeFileSync(join(dir, "requests", "good.json"), "{}");
  writeFileSync(join(dir, "requests", "later.json"), "{}");
  // The fake driver answers by operand from behaviour.json in the example folder.
  const fake = join(root, "fake-fsl");
  writeFileSync(
    fake,
    `#!/usr/bin/env node
const { readFileSync } = require("node:fs");
const b = JSON.parse(readFileSync("behaviour.json", "utf8"));
const r = b[process.argv[3]];
process.stdout.write(r.out);
process.exit(r.exit);
`,
  );
  chmodSync(fake, 0o755);
  const behave = (spec) => writeFileSync(join(dir, "behaviour.json"), JSON.stringify(spec));
  const run = (...args) => spawnSync("node", [RUNNER, ...args], { encoding: "utf8", env: { ...process.env, FSL: fake } });
  return { root, fake, dir, behave, run };
}

test("check, staged, record and promote work end to end", () => {
  const { root, fake, dir, behave, run } = fixture();
  const good = "requests/good.json";
  const later = "requests/later.json";
  writeFileSync(join(dir, "expected", "good.json"), OK_DOC);
  writeFileSync(join(dir, "expected", "later.json"), "");
  behave({ [good]: { exit: 0, out: OK_DOC }, [later]: { exit: 21, out: "" } });

  let r = run("check", dir);
  assert.equal(r.status, 0, r.stderr);
  assert.match(r.stdout, /check: 1 passed, 0 failed/);
  r = run("staged", dir);
  assert.equal(r.status, 0, r.stderr);

  // The language moves: the staged case now passes.
  behave({ [good]: { exit: 0, out: OK_DOC }, [later]: { exit: 0, out: OK_DOC } });
  r = run("staged", dir);
  assert.equal(r.status, 1);
  assert.match(r.stderr, /promote demo\/later/);
  r = run("record", dir);
  assert.equal(r.status, 1, "record refuses a staged case that should be promoted");

  // Promote through main(), which finds the example folders under the fixture root.
  const logs = [];
  const log = console.log;
  console.log = (m) => logs.push(m);
  try {
    assert.equal(main(["promote", "demo/later", "--verdict", "holds"], { FSL: fake }, root), 0);
  } finally {
    console.log = log;
  }
  assert.match(logs[0], /promoted demo\/later: exit 0, verdict "holds"/);
  assert.equal(readFileSync(join(dir, "expected", "later.json"), "utf8"), OK_DOC);
  const promoted = parseManifest(readFileSync(join(dir, "example.toml"), "utf8")).cases[1];
  assert.equal(promoted.status, "running");
  assert.equal(promoted.expect_exit, 0);
  assert.equal(promoted.expect_verdict, "holds");
  assert.equal(run("check", dir).status, 0);
  assert.equal(run("staged", dir).status, 0);
});

test("check fails a running case whose output changed and record then repairs it", () => {
  const { dir, behave, run } = fixture();
  const good = "requests/good.json";
  writeFileSync(join(dir, "expected", "good.json"), OK_DOC);
  writeFileSync(join(dir, "expected", "later.json"), "");
  behave({ [good]: { exit: 0, out: OK_DOC.replace("7", "8") }, "requests/later.json": { exit: 21, out: "" } });
  const r = run("check", dir);
  assert.equal(r.status, 1);
  assert.match(r.stderr, /demo\/good: output differs from expected\/good\.json/);
  assert.equal(run("record", dir).status, 0);
  assert.equal(readFileSync(join(dir, "expected", "good.json"), "utf8"), OK_DOC.replace("7", "8"));
  assert.equal(run("check", dir).status, 0);
});

test("a missing FSL is reported, not ignored", () => {
  const r = spawnSync("node", [RUNNER, "check"], { encoding: "utf8", env: { ...process.env, FSL: "" } });
  assert.equal(r.status, 2);
  assert.match(r.stderr, /FSL is not set/);
});

test("a staged case without a ticket id is rejected", () => {
  const staged = (waits) => `[[case]]\nid = "a"\nstatement = "s"\nfile = "f"\ncommand = "run x"\nstatus = "staged"\nexpect_exit = 21\nready_exit = 0\n${waits}why = "w"\nneeds = "a feature"\n`;
  assert.throws(() => parseManifest(staged("")), /waits_on must be a string/);
  assert.throws(() => parseManifest(staged('waits_on = "no ticket - planning gap"\n')), /waits_on must be a ticket id/);
  assert.equal(parseManifest(staged('waits_on = "QSL-2000"\n')).cases.length, 1);
});

test("a staged case needs a plain-language needs phrase", () => {
  const staged = (needs) => `[[case]]\nid = "a"\nstatement = "s"\nfile = "f"\ncommand = "run x"\nstatus = "staged"\nexpect_exit = 21\nready_exit = 0\nwaits_on = "QSL-9"\nwhy = "w"\n${needs}`;
  assert.throws(() => parseManifest(staged("")), /needs must be a string/);
  assert.throws(() => parseManifest(staged('needs = " "\n')), /needs must be a short phrase/);
  assert.equal(parseManifest(staged('needs = "probability"\n')).cases[0].needs, "probability");
});

test("every case in this repository is complete and every staged case names a ticket", () => {
  for (const dir of discoverSystems(REPO)) {
    const { cases } = parseManifest(readFileSync(join(dir, "example.toml"), "utf8"), dir);
    for (const c of cases) {
      assert.ok(c.statement.length > 0, c.id);
      if (c.status === "staged") {
        assert.match(c.waits_on, /^[A-Z]+-\d+$/, c.id);
        assert.ok(c.needs.length > 0, c.id);
      }
    }
  }
});

test("renderOutput summarizes recorded documents", () => {
  const c = { id: "x", expect_exit: 10 };
  const item = '{"items":[{"category":"violation","cause":null,"clause":"R","request_index":0,"result":"refuted","counterexample":{"a":1}}]}';
  assert.equal(renderOutput(c, item), 'exit 10\n{"category":"violation","clause":"R","result":"refuted","counterexample":"recorded in expected/x.json"}');
  assert.equal(renderOutput({ id: "y", expect_exit: 0 }, OK_DOC), 'exit 0\nresult: {"kind":"integer","decimal":"7"}');
  assert.match(renderOutput(c, '{"category":"refusal","items":[],"diagnostics":[{"code":"ill_typed","message":"bad"}]}'), /refusal: ill_typed: bad/);
});

function demoSystem() {
  const root = mkdtempSync(join(tmpdir(), "gen-"));
  const dir = join(root, "examples", "demo");
  mkdirSync(join(dir, "expected"), { recursive: true });
  mkdirSync(join(dir, "requests"), { recursive: true });
    const text = MANIFEST;
  writeFileSync(join(dir, "example.toml"), text);
  writeFileSync(join(dir, "expected", "good.json"), OK_DOC);
  writeFileSync(join(dir, "expected", "later.json"), "");
  writeFileSync(join(dir, "requests", "good.json"), JSON.stringify({ request: { clause: { observations: { current: "../data/d.json" } } } }));
  writeFileSync(join(dir, "requests", "later.json"), "{}");
  writeFileSync(join(dir, "README.md"), "Text.\n\n<!-- output: good -->\n```\nold\n```\n<!-- /output -->\n");
  return { root, dir, systems: [{ dir, name: "demo", ...parseManifest(text) }] };
}

test("syncReadmes finds and rewrites a stale output region", () => {
  const { dir, systems } = demoSystem();
  assert.equal(syncReadmes(systems, false).length, 1);
  assert.equal(syncReadmes(systems, true).length, 1);
  assert.match(readFileSync(join(dir, "README.md"), "utf8"), /exit 0\nresult: /);
  assert.equal(syncReadmes(systems, false).length, 0);
});

test("buildIndex lists repo-relative paths, verdict, exit and waits_on", () => {
  const { root, systems } = demoSystem();
  const idx = JSON.parse(buildIndex(systems, root));
  const [good, later] = idx.cases;
  assert.equal(good.request_file, "examples/demo/requests/good.json");
  assert.deepEqual(good.data_files, ["examples/demo/data/d.json"]);
  assert.equal(good.expected_file, "examples/demo/expected/good.json");
  assert.equal(good.verdict, "holds");
  assert.equal(good.waits_on, null);
  assert.equal(later.waits_on, "QSL-1");
  assert.equal(later.needs, "a feature");
  assert.equal(good.needs, null);
  assert.equal(later.verdict, null);
  assert.equal(later.statement, "A later rule.");
});

test("a stale index.json says what to do about it", () => {
  const { root, systems } = demoSystem();
  const problems = syncGenerated(systems, root, false);
  assert.ok(problems.includes("index.json is stale: merge origin/main, then run make index"), problems.join("\n"));
});
