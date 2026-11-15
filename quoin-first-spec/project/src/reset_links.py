"""Password reset links: issued once, valid for 30 minutes."""
from dataclasses import dataclass
from datetime import datetime, timedelta

LIFETIME = timedelta(minutes=30)


@dataclass
class ResetLink:
    issued_at: datetime
    used: bool = False


def open_link(link: ResetLink, now: datetime) -> str:
    """Return "accepted", "expired" or "used" for a link opened at `now`."""
    if link.used:
        return "used"
    if now - link.issued_at >= LIFETIME:
        return "expired"
    link.used = True
    return "accepted"
