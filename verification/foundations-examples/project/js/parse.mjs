import {
  parseDocument, extractFrontmatter, section, sections, parseTable,
  parseBulletList, extractDiagrams, search, updateSection,
} from "@agent-ix/quire";

const markdown = `---
title: "Checkout"
tags: [a, b]
---
Intro text.

## Scope

- **In** — orders and payments
- **Out** — refunds

### 1.1 Details

| Name | Kind |
| --- | --- |
| a | x |
| b | y |

## Flow

\`\`\`mermaid
%% @type: sequence
sequenceDiagram
  A->>B: pay
\`\`\`
`;

const doc = parseDocument(markdown);
console.log("frontmatter:", JSON.stringify(doc.frontmatter));
console.log("preamble:", JSON.stringify(doc.preamble));
console.log("top-level sections:", doc.sections.map((s) => s.heading).join(", "));
console.log("section 'details':", section(doc, "details").heading);
console.log("level 2:", sections(doc, { level: 2 }).map((s) => s.heading).join(", "));
console.log("table:", JSON.stringify(parseTable(section(doc, "Details").content)));
const items = parseBulletList(section(doc, "Scope").content);
console.log("bullets:", items.map((i) => `${i.title} = ${i.description}`).join("; "));
const diagrams = extractDiagrams(doc);
console.log("diagram:", diagrams[0].language, diagrams[0].section, diagrams[0].tag);
console.log("search:", JSON.stringify(search(doc, "orders").map((r) => r.matches)));
console.log("updated:", JSON.stringify(updateSection(doc, "Scope", "- replaced").split("\n").slice(5, 11)));
console.log("bad frontmatter:", JSON.stringify(extractFrontmatter("---\nbad line\n---\nbody")));
try {
  updateSection(doc, "Missing", "x");
} catch (e) {
  console.log("missing section:", e.message);
}
