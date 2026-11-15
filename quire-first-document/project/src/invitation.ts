import { trace } from "../tests/trace.ts";

// An invitation can be accepted once, within seven days of the day it was sent.
const SEVEN_DAYS = 7 * 24 * 60 * 60 * 1000;

export function open(invitation: { sentAt: number; used?: boolean }, now: number): string {
  if (invitation.used) return "used";
  trace("FR-001-AC-2");
  if (now - invitation.sentAt >= SEVEN_DAYS) return "expired";
  invitation.used = true;
  return "accepted";
}
