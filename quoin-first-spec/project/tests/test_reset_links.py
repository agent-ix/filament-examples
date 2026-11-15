from datetime import datetime, timedelta

import pytest

from reset_links import ResetLink, open_link

ISSUED = datetime(2026, 1, 1, 12, 0)


@pytest.mark.trace("FR-001-AC-1")
def test_link_issued_29_minutes_ago_is_accepted():
    link = ResetLink(issued_at=ISSUED)
    assert open_link(link, ISSUED + timedelta(minutes=29)) == "accepted"


@pytest.mark.trace("FR-001-AC-2")
def test_link_issued_30_minutes_ago_is_expired():
    link = ResetLink(issued_at=ISSUED)
    assert open_link(link, ISSUED + timedelta(minutes=30)) == "expired"
