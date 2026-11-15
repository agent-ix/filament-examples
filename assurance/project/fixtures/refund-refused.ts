import { test } from "node:test";
import assert from "node:assert/strict";
import { refund } from "../src/refund.mjs";
import { trace } from "./trace.ts";

test("a refund above the amount paid is refused", () => {
  trace("FR-001-AC-2");
  assert.equal(refund({ paid: 500 }, 501).accepted, false);
});
