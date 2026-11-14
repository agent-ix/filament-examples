// Install-test runner. Node, no dependencies.
//
//   node tools/steps.mjs extract <steps.md> [--skip id,id]
//   node tools/steps.mjs run <steps.md> --image ubuntu:22.04 [--shell bash|zsh] [--skip id,id]
//   node tools/steps.mjs run <steps.md> --host [--shell bash|zsh] [--skip id,id]
//
// A steps file is Markdown. Every fenced block whose info string is
// `<language> step=<id>` is a step. `extract` prints the steps as one shell
// script. `run` starts the image, streams that script into one bash session
// (the way a reader types the commands into one terminal) or, with --host, into
// a shell on this machine, in bash or zsh, and reports the
// duration of each step. The first failing step stops the run and is named.
import { spawn } from "node:child_process";
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const MARK = "@@STEP";

export function extractSteps(markdown) {
  const steps = [];
  const lines = markdown.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const open = /^```(\w+) step=([a-z0-9-]+)\s*$/.exec(lines[i]);
    if (!open) continue;
    const body = [];
    let closed = false;
    for (i++; i < lines.length; i++) {
      if (lines[i].startsWith("```")) {
        closed = true;
        break;
      }
      body.push(lines[i]);
    }
    if (!closed) throw new Error(`step ${open[2]}: fenced block is not closed`);
    if (steps.some((s) => s.id === open[2])) throw new Error(`duplicate step id ${open[2]}`);
    steps.push({ id: open[2], language: open[1], body: body.join("\n") });
  }
  return steps;
}

export function selectSteps(steps, skip = []) {
  for (const id of skip) {
    if (!steps.some((s) => s.id === id)) throw new Error(`--skip names unknown step ${id}`);
  }
  return steps.filter((s) => !skip.includes(s.id));
}

// One script, with a marker line before each step. The shell stops at the first
// failure (set -e) and a step id is printed with its start time.
export function buildScript(steps) {
  const parts = ["set -eu -o pipefail"];
  for (const s of steps) {
    if (s.language !== "bash") throw new Error(`step ${s.id}: only bash steps are supported`);
    parts.push(`echo "${MARK} ${s.id} $(date +%s)"`, s.body);
  }
  parts.push(`echo "${MARK} end $(date +%s)"`);
  return parts.join("\n") + "\n";
}

// Turn marker lines into step records: { id, seconds }. `end` closes the last step.
export function readMarkers(lines) {
  const marks = [];
  for (const line of lines) {
    const m = new RegExp(`^${MARK} (\\S+) (\\d+)$`).exec(line);
    if (m) marks.push({ id: m[1], at: Number(m[2]) });
  }
  const records = [];
  for (let i = 0; i + 1 < marks.length; i++) {
    records.push({ id: marks[i].id, seconds: marks[i + 1].at - marks[i].at });
  }
  return { records, last: marks.length ? marks[marks.length - 1].id : null };
}

export function parseArgs(argv) {
  const [command, file, ...rest] = argv;
  const opts = { command, file, skip: [], image: null, shell: "bash", host: false };
  for (let i = 0; i < rest.length; i++) {
    if (rest[i] === "--skip") opts.skip = (rest[++i] ?? "").split(",").filter(Boolean);
    else if (rest[i] === "--image") opts.image = rest[++i];
    else if (rest[i] === "--shell") opts.shell = rest[++i];
    else if (rest[i] === "--host") opts.host = true;
    else throw new Error(`unknown argument ${rest[i]}`);
  }
  if (!["bash", "zsh"].includes(opts.shell)) throw new Error(`--shell must be bash or zsh, not ${opts.shell}`);
  return opts;
}

// The command that reads the script from stdin. In a container, zsh is
// installed first: the image is bare, and zsh is not part of the steps.
export function shellCommand(opts) {
  if (opts.host) return { cmd: opts.shell, args: opts.shell === "zsh" ? ["-f", "-s"] : ["-s"] };
  const exec = opts.shell === "zsh" ? "exec zsh -f -s" : "exec bash -s";
  const prepare = opts.shell === "zsh" ? "apt-get update -qq >/dev/null && apt-get install -y -qq zsh >/dev/null && " : "";
  return { cmd: "docker", args: ["run", "--rm", "-i", opts.image, "sh", "-c", prepare + exec] };
}

function runScript(opts, script) {
  return new Promise((resolve) => {
    const { cmd, args } = shellCommand(opts);
    // On the host the steps start in a new empty folder, as a reader's would.
    const cwd = opts.host ? mkdtempSync(join(tmpdir(), "install-test-")) : undefined;
    const child = spawn(cmd, args, {
      stdio: ["pipe", "pipe", "inherit"],
      cwd,
    });
    const lines = [];
    let pending = "";
    child.stdout.on("data", (chunk) => {
      pending += chunk;
      const parts = pending.split("\n");
      pending = parts.pop();
      for (const line of parts) {
        lines.push(line);
        console.log(line);
      }
    });
    child.on("close", (code) => {
      if (pending) lines.push(pending);
      resolve({ code, lines });
    });
    child.stdin.end(script);
  });
}

async function main(argv) {
  const opts = parseArgs(argv);
  const steps = selectSteps(extractSteps(readFileSync(opts.file, "utf8")), opts.skip);
  const script = buildScript(steps);
  if (opts.command === "extract") {
    process.stdout.write(script);
    return 0;
  }
  if (opts.command !== "run" || (!opts.image && !opts.host)) throw new Error("usage: steps.mjs run <steps.md> (--image <image> | --host) [--shell bash|zsh] [--skip id,id]");
  const { code, lines } = await runScript(opts, script);
  const { records, last } = readMarkers(lines);
  console.log("\nstep durations");
  for (const r of records) console.log(`  ${r.id.padEnd(16)} ${r.seconds}s`);
  if (code !== 0) {
    console.error(`install test FAILED in step ${last} (exit ${code})`);
    return 1;
  }
  console.log(`install test passed: ${records.length} steps`);
  return 0;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main(process.argv.slice(2)).then(
    (code) => process.exit(code),
    (err) => {
      console.error(err.message);
      process.exit(2);
    },
  );
}
