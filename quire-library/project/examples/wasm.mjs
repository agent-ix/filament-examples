import { readFileSync } from "node:fs";
import { initSync, parseDocument, extractFromBlob, validateFromBlob } from "@agent-ix/quire-wasm";

// Load the .wasm file from disk. A browser calls the default export instead.
const wasm = new URL("../node_modules/@agent-ix/quire-wasm/pkg/quire_wasm_bg.wasm", import.meta.url);
initSync({ module: readFileSync(wasm) });

const text = readFileSync("runbooks/RB-002-rotate-keys.md", "utf8");
const moduleBlob = {
  manifest: readFileSync("modules/runbook/manifest.yaml", "utf8"),
  schemas: { "schemas/runbook.schema.json": readFileSync("modules/runbook/schemas/runbook.schema.json", "utf8") },
};

const doc = parseDocument(text);
console.log(doc.sections[0].heading, doc.sections[0].children.map((s) => s.heading));
const { records, diagnostics } = extractFromBlob("Runbook", moduleBlob, text);
console.log(JSON.stringify(records[0]), diagnostics.length);

validateFromBlob("Runbook", moduleBlob, { id: "RB-002", title: "Rotate the signing keys", type: "Runbook" });
console.log("record is valid");
try {
  validateFromBlob("Runbook", moduleBlob, { id: "bad", title: "", type: "Runbook" });
} catch (e) {
  console.log("rejected:", String(e));
}
