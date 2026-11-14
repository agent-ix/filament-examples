// Runner for the example manifests. Node, no dependencies.
//
//   node tools/run.mjs check   [dir ...]          running cases must match their recorded result
//   node tools/run.mjs staged  [dir ...]          staged cases must still fail as recorded
//   node tools/run.mjs record  [dir ...]          re-record expected/ from the driver
//   node tools/run.mjs promote <case> [--verdict V]   flip a staged case to running
//
// The driver binary comes from the FSL environment variable (the Makefile sets
// it). A <dir> is an example folder holding example.toml; the default is every
// folder under examples/. A <case> is `<system>/<id>` or a bare `<id>`.
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const STATUSES = ["running", "staged"];

// --- Manifest ---------------------------------------------------------------
// example.toml uses a small TOML subset: `key = "string"`, `key = 123`,
// comments, and `[[case]]` tables. Anything else is an error.

export function parseManifest(text, where = "example.toml") {
  const header = {};
  const cases = [];
  let target = header;
  text.split("\n").forEach((raw, i) => {
    const line = raw.trim();
    if (line === "" || line.startsWith("#")) return;
    const fail = (why) => {
      throw new Error(`${where}:${i + 1}: ${why}`);
    };
    if (line === "[[case]]") {
      target = {};
      cases.push(target);
      return;
    }
    const m = /^([a-z_]+) = (.+)$/.exec(line);
    if (!m) fail(`cannot read line: ${line}`);
    let value;
    if (/^-?\d+$/.test(m[2])) value = Number(m[2]);
    else if (m[2].startsWith('"')) {
      try {
        value = JSON.parse(m[2]);
      } catch {
        fail(`bad string: ${m[2]}`);
      }
    } else fail(`value must be a string or an integer: ${m[2]}`);
    if (m[1] in target) fail(`duplicate key ${m[1]}`);
    target[m[1]] = value;
  });
  const seen = new Set();
  for (const c of cases) {
    const need = (key, type) => {
      if (typeof c[key] !== type) throw new Error(`${where}: case ${c.id ?? "?"}: ${key} must be a ${type}`);
    };
    need("id", "string");
    need("command", "string");
    need("expect_exit", "number");
    if (!STATUSES.includes(c.status)) throw new Error(`${where}: case ${c.id}: status must be running or staged`);
    if (seen.has(c.id)) throw new Error(`${where}: duplicate case id ${c.id}`);
    seen.add(c.id);
    if (c.status === "running") need("expect_verdict", "string");
    else {
      need("ready_exit", "number");
      need("waits_on", "string");
      need("why", "string");
      if (c.ready_exit === c.expect_exit) throw new Error(`${where}: case ${c.id}: ready_exit equals expect_exit`);
    }
  }
  return { header, cases };
}

// Flip one staged case to running in the manifest text, line by line so that
// comments and the order of the other cases stay as written.
export function promoteText(text, id, verdict) {
  const lines = text.split("\n");
  const start = lines.findIndex((l) => l === `id = ${JSON.stringify(id)}`);
  if (start < 0) throw new Error(`case ${id} not found in manifest`);
  let end = lines.findIndex((l, i) => i > start && l.trim() === "[[case]]");
  if (end < 0) end = lines.length;
  const block = lines.slice(start, end);
  const ready = block.find((l) => l.startsWith("ready_exit = "));
  if (!ready) throw new Error(`case ${id} has no ready_exit`);
  const out = [];
  for (const l of block) {
    if (/^(ready_exit|waits_on|why) = /.test(l)) continue;
    if (l.startsWith("status = ")) out.push('status = "running"');
    else if (l.startsWith("expect_exit = ")) {
      out.push(`expect_exit = ${ready.slice("ready_exit = ".length)}`);
      out.push(`expect_verdict = ${JSON.stringify(verdict)}`);
    } else out.push(l);
  }
  return [...lines.slice(0, start), ...out, ...lines.slice(end)].join("\n");
}

// A short verdict taken from a quire-outcome/1 document.
export function verdictOf(stdout) {
  let doc;
  try {
    doc = JSON.parse(stdout);
  } catch {
    return "unreadable output";
  }
  if (doc.category === "violation") return "violated";
  if (doc.category === "success" && doc.result?.value) {
    const v = doc.result.value;
    return String(v.value ?? v.decimal ?? "holds");
  }
  if (doc.category === "success") return "holds";
  const d = doc.diagnostics?.[0];
  return d ? `${doc.category}: ${d.code}` : String(doc.category);
}

// --- Running a case ---------------------------------------------------------

export function runCase(dir, c, fsl) {
  const args = [...c.command.split(" ").filter(Boolean), "--format", "json"];
  const r = spawnSync(fsl, args, { cwd: dir, encoding: "utf8", maxBuffer: 256 * 1024 * 1024 });
  if (r.error) throw new Error(`cannot run FSL=${fsl}: ${r.error.message}. Set FSL to the FSL driver binary, for example make check FSL=/path/to/driver`);
  return { exit: r.status ?? -1, stdout: r.stdout };
}

function driverHint(expected, stdout) {
  if (expected === "") return "";
  try {
    if (JSON.parse(stdout).format === "quire-outcome/1") return "";
  } catch {
    // fall through
  }
  return " (the output is not a quire-outcome/1 document: check that FSL names the FSL driver)";
}

// Compare one run with the manifest. Returns { ok, message }.
export function evaluateCase(c, run, expected, label = c.id) {
  if (c.status === "staged" && run.exit === c.ready_exit)
    return { ok: false, message: `${label}: now exits ${run.exit}, as it should once fixed: promote ${label}` };
  if (run.exit !== c.expect_exit) {
    const what = c.status === "staged" ? "staged case changed: " : "";
    return { ok: false, message: `${label}: ${what}exit ${run.exit}, expected ${c.expect_exit}` };
  }
  if (expected === null)
    return { ok: false, message: `${label}: expected/${c.id}.json is missing; run make record` };
  if (run.stdout !== expected) {
    const what = c.status === "staged" ? "staged case changed: " : "";
    return { ok: false, message: `${label}: ${what}output differs from expected/${c.id}.json${driverHint(expected, run.stdout)}` };
  }
  return { ok: true, message: `${label}: exit ${run.exit} ok` };
}

// --- Commands ---------------------------------------------------------------

function loadSystem(dir) {
  const file = join(dir, "example.toml");
  if (!existsSync(file)) throw new Error(`${dir}: no example.toml`);
  const text = readFileSync(file, "utf8");
  return { dir, name: basename(resolve(dir)), file, text, ...parseManifest(text, file) };
}

function readExpected(dir, id) {
  const p = join(dir, "expected", `${id}.json`);
  return existsSync(p) ? readFileSync(p, "utf8") : null;
}

function selectCases(sys, status) {
  return sys.cases.filter((c) => c.status === status);
}

// Run every case of `status`; returns the number of failures.
export function checkSystems(systems, status, fsl, log = console) {
  let passed = 0;
  let failed = 0;
  for (const sys of systems) {
    for (const c of selectCases(sys, status)) {
      const res = evaluateCase(c, runCase(sys.dir, c, fsl), readExpected(sys.dir, c.id), `${sys.name}/${c.id}`);
      if (res.ok) {
        passed += 1;
        log.log(res.message);
      } else {
        failed += 1;
        log.error(res.message);
      }
    }
  }
  log.log(`${status === "running" ? "check" : "staged"}: ${passed} passed, ${failed} failed`);
  return failed;
}

// Re-record expected/ for every case whose exit matches its manifest entry.
export function recordSystems(systems, fsl, log = console) {
  let recorded = 0;
  let failed = 0;
  for (const sys of systems) {
    for (const c of sys.cases) {
      const run = runCase(sys.dir, c, fsl);
      if (c.status === "staged" && run.exit === c.ready_exit) {
        failed += 1;
        log.error(`${sys.name}/${c.id}: now exits ${run.exit}: promote ${sys.name}/${c.id}`);
      } else if (run.exit !== c.expect_exit) {
        failed += 1;
        log.error(`${sys.name}/${c.id}: exit ${run.exit}, manifest says ${c.expect_exit}; update the manifest, then record`);
      } else {
        writeFileSync(join(sys.dir, "expected", `${c.id}.json`), run.stdout);
        recorded += 1;
      }
    }
  }
  log.log(`record: ${recorded} recorded, ${failed} not recorded`);
  return failed;
}

export function findCase(systems, ref) {
  const [sysName, id] = ref.includes("/") ? ref.split("/") : [null, ref];
  const hits = [];
  for (const sys of systems)
    if (sysName === null || sys.name === sysName)
      for (const c of sys.cases) if (c.id === id) hits.push({ sys, c });
  if (hits.length === 0) throw new Error(`no case ${ref}`);
  if (hits.length > 1) throw new Error(`case ${ref} is ambiguous; use <system>/<id>`);
  return hits[0];
}

export function promoteCase(systems, ref, fsl, verdictArg, log = console) {
  const { sys, c } = findCase(systems, ref);
  if (c.status !== "staged") throw new Error(`${sys.name}/${c.id} is not staged`);
  const run = runCase(sys.dir, c, fsl);
  if (run.exit !== c.ready_exit)
    throw new Error(`${sys.name}/${c.id} exits ${run.exit}, not ${c.ready_exit}; it is not ready to promote`);
  const verdict = verdictArg ?? verdictOf(run.stdout);
  writeFileSync(sys.file, promoteText(sys.text, c.id, verdict));
  writeFileSync(join(sys.dir, "expected", `${c.id}.json`), run.stdout);
  log.log(`promoted ${sys.name}/${c.id}: exit ${run.exit}, verdict "${verdict}"`);
  log.log("Check the recorded output against what the case is meant to show, then commit.");
}

export function discoverSystems(root) {
  const base = join(root, "examples");
  return readdirSync(base, { withFileTypes: true })
    .filter((e) => e.isDirectory() && existsSync(join(base, e.name, "example.toml")))
    .map((e) => join(base, e.name))
    .sort();
}

export function main(argv, env, root) {
  const [cmd, ...rest] = argv;
  const fsl = env.FSL;
  if (!["check", "staged", "record", "promote"].includes(cmd)) throw new Error("usage: run.mjs check|staged|record|promote ...");
  if (!fsl) throw new Error("FSL is not set; run through make or set FSL to the FSL driver binary");
  if (cmd === "promote") {
    const args = [...rest];
    const verdictAt = args.indexOf("--verdict");
    const verdict = verdictAt >= 0 ? args.splice(verdictAt, 2)[1] : undefined;
    const ref = args[0];
    if (!ref) throw new Error("usage: run.mjs promote <system>/<case> [--verdict V]");
    promoteCase(discoverSystems(root).map(loadSystem), ref, fsl, verdict);
    return 0;
  }
  const dirs = rest.length ? rest : discoverSystems(root);
  const systems = dirs.map(loadSystem);
  if (cmd === "record") return recordSystems(systems, fsl) === 0 ? 0 : 1;
  return checkSystems(systems, cmd === "check" ? "running" : "staged", fsl) === 0 ? 0 : 1;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
  try {
    process.exit(main(process.argv.slice(2), process.env, root));
  } catch (e) {
    console.error(e.message);
    process.exit(2);
  }
}
