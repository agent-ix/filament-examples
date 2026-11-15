import quire

text = open("runbooks/RB-002-rotate-keys.md", encoding="utf-8").read()
result = quire.validate_document("Runbook", "modules/runbook", text)
print("valid:", result["is_valid"], result["errors"], result["warnings"])
broken = text.replace("## Purpose", "## Overview")
result = quire.validate_document("Runbook", "modules/runbook", broken)
print("errors:", [(e["reason"], e["message"]) for e in result["errors"]])

extracted = quire.extract("Runbook", "modules/runbook", text)
print("purpose:", extracted["extraction"][0]["purpose"])
print("edges:", extracted["edges"])

doc = quire.parse_document(text)
print("sections:", [s["heading"] for s in doc["sections"][0]["children"]])

try:
    quire.validate("Runbook", "modules/runbook", {"id": "bad", "title": "", "type": "Runbook"})
except quire.QuireValidationError as error:
    print("rejected:", error)

spec = quire.Spec.from_path("runbooks")
print("Runbook documents:", spec.by_type("Runbook"))
print("referencing RB-001:", spec.referencing("RB-001"))
print("dangling:", spec.dangling())
