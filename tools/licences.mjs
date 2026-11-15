// Licence notice generator for the toolchain image. Node, no dependencies.
//
//   node tools/licences.mjs collect --driver <dir> --quire-cli <dir> --quoin <dir>
//   node tools/licences.mjs render [--out <dir>]
//   node tools/licences.mjs check
//   node tools/licences.mjs verify-installed <node_modules dir> <driver tag>
//
// `collect` reads checkouts of the three programs at the tags the image ships
// (filament-driver, quire-cli and quoin) and writes image/licenses/data/ with one
// JSON file for each program: its repository, tag and commit, and every Rust crate
// that cargo builds into it (from `cargo metadata --locked`, for the two Linux
// targets of the image, development dependencies left out), with the licence
// identifier the crate declares and the hashes of the licence files in the crate.
// It also writes the licence texts (texts.json) and the licence file of each
// program (image/licenses/texts/). It needs cargo and the network for git
// dependencies, and is run by hand when a shipped version changes.
//
// `render` writes THIRD-PARTY.md and THIRD-PARTY-LICENSES.txt from that data.
// `check` fails when the files committed in image/licenses differ from what
// `render` writes, so the notice cannot drift from its data (make test).
// `verify-installed` runs inside the image build: it fails when the installed npm
// programs or the downloaded driver are not the versions the data describes. A
// driver built from source (tag "source") is not checked.
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const DIR = join(ROOT, "image", "licenses");
const TARGETS = ["aarch64-unknown-linux-gnu", "x86_64-unknown-linux-gnu"];
const PROGRAMS = [
  { key: "driver", id: "filament-driver", binary: "fsl", root: "fsl-cli", manifestDir: "." },
  { key: "quire-cli", id: "quire-cli", binary: "quire", root: "quire-cli", manifestDir: "." },
  { key: "quoin", id: "quoin", binary: "quoin", root: "quoin-cli", manifestDir: "rust" },
];
const FIRST_PARTY = "https://github.com/agent-ix/";
const LICENCE_FILE = /^(licen[sc]e|copying|unlicense|notice)/i;

const sha = (text) => createHash("sha256").update(text).digest("hex").slice(0, 16);
const git = (dir, ...args) => {
  const r = spawnSync("git", args, { cwd: dir, encoding: "utf8" });
  if (r.status !== 0) throw new Error(`git ${args.join(" ")} failed in ${dir}: ${r.stderr}`);
  return r.stdout.trim();
};

// The crates cargo builds into `root` for one target: everything reachable
// through normal and build dependencies.
export function closure(meta, root) {
  const byId = new Map(meta.packages.map((p) => [p.id, p]));
  const nodes = new Map(meta.resolve.nodes.map((n) => [n.id, n]));
  const start = meta.packages.find((p) => p.name === root);
  if (!start) throw new Error(`no package ${root}`);
  const seen = new Set([start.id]);
  const queue = [start.id];
  while (queue.length) {
    for (const dep of nodes.get(queue.pop()).deps) {
      if (!dep.dep_kinds.some((k) => k.kind !== "dev") || seen.has(dep.pkg)) continue;
      seen.add(dep.pkg);
      queue.push(dep.pkg);
    }
  }
  return [...seen].map((id) => byId.get(id));
}

// The version a program ships under is its release tag, which is what npm and the
// driver archive carry. The root crate's own Cargo version can lag it (quoin
// v0.32.0 is built from crates at 0.31.0), so it is not used.
export const releaseVersion = (tag) => tag.replace(/^v/, "");

function crateRecord(p, texts) {
  const source = p.source ?? "workspace";
  let repo = null;
  let commit = null;
  const m = /^git\+(https:\/\/[^?#]+?)(?:\.git)?(?:\?[^#]*)?#([0-9a-f]{40})$/.exec(source);
  if (m) [, repo, commit] = m;
  const files = [];
  for (const name of readdirSync(dirname(p.manifest_path)).sort()) {
    if (!LICENCE_FILE.test(name)) continue;
    let text;
    try {
      text = readFileSync(join(dirname(p.manifest_path), name), "utf8");
    } catch {
      continue;
    }
    const hash = sha(text);
    texts[hash] = text;
    files.push(hash);
  }
  return {
    name: p.name,
    version: p.version,
    licence: p.license ?? null,
    source: source.startsWith("registry+") ? "crates.io" : source === "workspace" ? "workspace" : "git",
    repository: repo ?? p.repository ?? null,
    commit,
    licenceFiles: files,
  };
}

function collect(args) {
  const texts = {};
  mkdirSync(join(DIR, "data"), { recursive: true });
  mkdirSync(join(DIR, "texts"), { recursive: true });
  for (const prog of PROGRAMS) {
    if (!args[prog.key]) throw new Error(`--${prog.key} <checkout> is required`);
    const dir = resolve(args[prog.key]);
    const manifestDir = join(dir, prog.manifestDir);
    const tag = git(dir, "describe", "--tags", "--exact-match");
    const commit = git(dir, "rev-parse", "HEAD");
    const perCrate = new Map();
    for (const target of TARGETS) {
      const r = spawnSync("cargo", ["metadata", "--locked", "--format-version", "1", "--filter-platform", target], {
        cwd: manifestDir, encoding: "utf8", maxBuffer: 1 << 29,
      });
      if (r.status !== 0) throw new Error(`cargo metadata failed for ${prog.id}: ${r.stderr}`);
      for (const p of closure(JSON.parse(r.stdout), prog.root)) perCrate.set(`${p.name}@${p.version}`, crateRecord(p, texts));
    }
    const crates = [...perCrate.values()].sort((a, b) => (a.name + a.version).localeCompare(b.name + b.version));
    const data = {
      program: prog.id,
      binary: prog.binary,
      version: releaseVersion(tag),
      repository: `${FIRST_PARTY}${prog.id}`,
      tag,
      commit,
      targets: TARGETS,
      crates,
    };
    writeFileSync(join(DIR, "data", `${prog.id}.json`), JSON.stringify(data, null, 2) + "\n");
    copyFileSync(join(dir, "LICENSE"), join(DIR, "texts", `${prog.id}.LICENSE`));
  }
  const sorted = Object.fromEntries(Object.entries(texts).sort(([a], [b]) => a.localeCompare(b)));
  writeFileSync(join(DIR, "data", "texts.json"), JSON.stringify(sorted, null, 2) + "\n");
}

function loadData() {
  const read = (f) => JSON.parse(readFileSync(join(DIR, "data", f), "utf8"));
  return { programs: PROGRAMS.map((p) => read(`${p.id}.json`)), texts: read("texts.json") };
}

const isFirstParty = (c) =>
  c.licence === "AGPL-3.0-or-later" && (c.source === "workspace" || (c.repository ?? "").startsWith(FIRST_PARTY));

function kinds(crates) {
  const by = new Map();
  for (const c of crates) {
    const k = c.licence ?? "none declared";
    by.set(k, (by.get(k) ?? 0) + 1);
  }
  return [...by].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]));
}

const cell = (s) => s.replaceAll("|", "\\|");

export function renderThirdParty({ programs }) {
  const L = [];
  L.push("# Components of the Filament toolchain image", "");
  L.push("This file is generated by `node tools/licences.mjs render` from the data in `image/licenses/data/`. Do not edit it by hand.", "");
  L.push("It lists what the image ships, with each component's licence identifier as its own repository or package declares it, and where its source is. It records facts and is not legal advice.", "");
  L.push("## Programs of the toolchain", "");
  L.push("| Program | Version | Licence | Licence text in the image | Source repository | Tag | Commit |", "| --- | --- | --- | --- | --- | --- | --- |");
  for (const p of programs) {
    const rootName = PROGRAMS.find((x) => x.id === p.program).root;
    const root = p.crates.find((c) => c.name === rootName);
    L.push(`| \`${p.binary}\` (${p.program}) | ${p.version} | ${root.licence} | \`texts/${p.program}.LICENSE\` | ${p.repository} | ${p.tag} | ${p.commit} |`);
  }
  L.push("| Filament examples (`/opt/filament-examples`) | this image | MIT | `texts/filament-examples.LICENSE` | https://github.com/agent-ix/filament-examples | the commit in the image label `org.opencontainers.image.revision` | |", "");
  L.push("## Other components", "");
  L.push("| Component | Licence | Where its notices are |", "| --- | --- | --- |");
  L.push("| Node.js, with the npm and corepack that come with it | as listed in Node's own licence file | `texts/nodejs.LICENSE`; the version is in `BUILD-FACTS.txt` |");
  L.push("| Ubuntu 22.04 packages | the licence of each package | `/usr/share/doc/<package>/copyright`; the package list is in `BUILD-FACTS.txt` |", "");
  for (const p of programs) {
    const first = p.crates.filter(isFirstParty);
    const third = p.crates.filter((c) => !isFirstParty(c));
    L.push(`## Rust crates built into \`${p.binary}\` (${p.program} ${p.tag})`, "");
    L.push(`Taken from \`cargo metadata --locked\` of ${p.repository}/tree/${p.tag}, for ${p.targets.join(" and ")}, normal and build dependencies. There are ${p.crates.length} crates: ${first.length} first-party and ${third.length} third-party. Build-time crates (procedural macros, build scripts) are included, so the list may name a few crates that are not linked into the binary.`, "");
    L.push("Licences of the third-party crates, as the crates declare them:", "");
    L.push("| Declared licence | Crates |", "| --- | --- |");
    for (const [k, n] of kinds(third)) L.push(`| ${cell(k)} | ${n} |`);
    L.push("", "First-party crates (AGPL-3.0-or-later). The corresponding source is at the repository and commit shown:", "");
    L.push("| Crate | Version | Repository | Commit |", "| --- | --- | --- | --- |");
    for (const c of first) L.push(`| ${c.name} | ${c.version} | ${c.repository ?? p.repository} | ${c.commit ?? `${p.commit} (tag ${p.tag})`} |`);
    L.push("", "Third-party crates:", "");
    L.push("| Crate | Version | Declared licence |", "| --- | --- | --- |");
    for (const c of third) L.push(`| ${c.name} | ${c.version} | ${cell(c.licence ?? "none declared")} |`);
    L.push("");
  }
  L.push("The licence texts shipped inside those crates are in `THIRD-PARTY-LICENSES.txt`.", "");
  return L.join("\n");
}

export function renderTexts({ programs, texts }) {
  const users = new Map();
  const missing = new Set();
  for (const p of programs) {
    for (const c of p.crates) {
      if (isFirstParty(c)) continue;
      if (c.licenceFiles.length === 0) missing.add(`${c.name} ${c.version}`);
      for (const h of c.licenceFiles) {
        if (!users.has(h)) users.set(h, new Set());
        users.get(h).add(`${c.name} ${c.version}`);
      }
    }
  }
  const bar = "=".repeat(78);
  const out = [
    "Licence texts shipped with the third-party Rust crates built into fsl, quire and quoin.",
    "Generated by `node tools/licences.mjs render`. Each block is a licence file of the listed crates, verbatim.",
    "",
  ];
  const blocks = [...users].map(([h, set]) => [[...set].sort().join(", "), texts[h]]);
  for (const [who, text] of blocks.sort((a, b) => a[0].localeCompare(b[0]))) out.push(bar, who, bar, text.trimEnd(), "");
  if (missing.size) {
    out.push(bar, "Crates whose package carries no licence file", bar);
    out.push("These crates declare a licence in Cargo.toml but their package has no licence file:", [...missing].sort().join(", "), "");
  }
  return out.join("\n");
}

function render(out) {
  const data = loadData();
  mkdirSync(out, { recursive: true });
  writeFileSync(join(out, "THIRD-PARTY.md"), renderThirdParty(data));
  writeFileSync(join(out, "THIRD-PARTY-LICENSES.txt"), renderTexts(data));
}

function check() {
  const data = loadData();
  const want = { "THIRD-PARTY.md": renderThirdParty(data), "THIRD-PARTY-LICENSES.txt": renderTexts(data) };
  const bad = Object.entries(want)
    .filter(([f, text]) => !existsSync(join(DIR, f)) || readFileSync(join(DIR, f), "utf8") !== text)
    .map(([f]) => f);
  if (bad.length) throw new Error(`${bad.join(", ")} in image/licenses is stale: run node tools/licences.mjs render`);
}

// Run in the image build. `modules` is the global node_modules directory.
function verifyInstalled(modules, driverTag) {
  const { programs } = loadData();
  const errors = [];
  for (const p of programs) {
    if (p.program === "filament-driver") {
      if (driverTag !== "source" && driverTag !== p.tag) errors.push(`the driver is ${driverTag}, the licence data describes ${p.tag}`);
      continue;
    }
    const base = join(modules, "@agent-ix", p.program);
    const pkg = JSON.parse(readFileSync(join(base, "package.json"), "utf8"));
    if (pkg.version !== p.version) errors.push(`@agent-ix/${p.program} is ${pkg.version}, the licence data describes ${p.version}`);
    if (readFileSync(join(base, "LICENSE"), "utf8") !== readFileSync(join(DIR, "texts", `${p.program}.LICENSE`), "utf8"))
      errors.push(`the LICENSE of @agent-ix/${p.program} differs from texts/${p.program}.LICENSE`);
  }
  if (errors.length) throw new Error(`${errors.join("; ")}. Update image/licenses with node tools/licences.mjs collect.`);
}

export function parseArgs(argv) {
  const args = { _: [] };
  for (let i = 0; i < argv.length; i++) {
    if (argv[i].startsWith("--")) args[argv[i].slice(2)] = argv[++i];
    else args._.push(argv[i]);
  }
  return args;
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? "").href) {
  const args = parseArgs(process.argv.slice(2));
  const [cmd] = args._;
  try {
    if (cmd === "collect") collect(args);
    else if (cmd === "render") render(args.out ?? DIR);
    else if (cmd === "check") check();
    else if (cmd === "verify-installed") verifyInstalled(args._[1], args._[2]);
    else throw new Error("usage: licences.mjs collect|render|check|verify-installed");
  } catch (e) {
    console.error(e.message);
    process.exit(1);
  }
}
