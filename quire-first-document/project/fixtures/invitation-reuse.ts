import { test } from "node:test";
import assert from "node:assert/strict";
import { open } from "../src/invitation.ts";
import { trace } from "./trace.ts";

test("an invitation that was already accepted is refused", () => {
  trace("FR-001-AC-3");
  const invitation = { sentAt: 0 };
  open(invitation, 1000);
  assert.equal(open(invitation, 2000), "used");
});
