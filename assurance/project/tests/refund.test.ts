import { test } from "node:test";
import assert from "node:assert/strict";
import { refund } from "../src/refund.mjs";
import { trace } from "./trace.ts";

test("a refund up to the amount paid is accepted", () => {
  trace("FR-001-AC-1");
  assert.equal(refund({ paid: 500 }, 500).accepted, true);
});
