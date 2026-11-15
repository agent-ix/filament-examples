from datetime import datetime

import pytest

from reset_links import ResetLink, open_link


@pytest.mark.trace("FR-001")
def test_link_that_was_already_used_is_refused_tagged_with_the_requirement():
    link = ResetLink(issued_at=datetime(2026, 1, 1, 12, 0), used=True)
    assert open_link(link, datetime(2026, 1, 1, 12, 5)) == "used"
