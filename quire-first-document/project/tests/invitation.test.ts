import { test } from "node:test";
import assert from "node:assert/strict";
import { open } from "../src/invitation.ts";
import { trace } from "./trace.ts";

const DAY = 24 * 60 * 60 * 1000;

test("an invitation opened after one day is accepted", () => {
  trace("FR-001-AC-1");
  assert.equal(open({ sentAt: 0 }, DAY), "accepted");
});

test("an invitation opened after seven days is refused", () => {
  trace("FR-001-AC-2");
  assert.equal(open({ sentAt: 0 }, 7 * DAY), "expired");
});
