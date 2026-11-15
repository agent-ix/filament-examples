// Adds the criterion ids each test is tagged with to a JUnit report written by
// `node --test --test-reporter=junit`, in the form `quoin evidence record
// --adapter junit` reads: a <property name="trace"> child on each test case.
// Node's own JUnit reporter does not write it.
//
//   node tools/junit-trace.mjs <node-junit.xml> <out.xml>
//
// The ids come from `quire trace --file <test file> --json`, which reads the
// tags in the test source, so the report and the test matrix agree. The script
// also makes each `file` path relative to the repository, sets `classname` to
// the test name (the symbol quire reports), and drops timings, so the same
// tests give the same report on every machine.
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { relative } from "node:path";

const [input, output] = process.argv.slice(2);
if (!input || !output) {
  console.error("usage: node tools/junit-trace.mjs <node-junit.xml> <out.xml>");
  process.exit(2);
}

const ids = new Map(); // "<path>\u0000<test name>" -> criterion ids
function idsFor(path, name) {
  if (!ids.has(path)) {
    const report = JSON.parse(execFileSync("quire", ["trace", "--file", path, "--json"], { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }));
    for (const claim of report.claims.verifies) {
      const key = `${claim.path}\u0000${claim.symbol}`;
      ids.set(key, [...(ids.get(key) ?? []), claim.trace_id]);
    }
    ids.set(path, true);
  }
  return ids.get(`${path}\u0000${name}`) ?? [];
}

const attr = (attrs, key) => new RegExp(`\\b${key}="([^"]*)"`).exec(attrs)?.[1];
const xml = readFileSync(input, "utf8")
  .replace(/\s*<!-- duration_ms [^>]*-->/g, "")
  .replace(/<testcase ([^>]*?)(\/?)>/g, (_, attrs, selfClosing) => {
    const name = attr(attrs, "name");
    const file = relative(process.cwd(), attr(attrs, "file") ?? "");
    const traced = idsFor(file, name);
    const head = `<testcase name="${name}" classname="${name}" file="${file}">`;
    const mark = traced.length ? `<property name="trace" value="${traced.join(",")}"/>` : "";
    return selfClosing ? `${head}${mark}</testcase>` : `${head}${mark}`;
  });
writeFileSync(output, xml);
