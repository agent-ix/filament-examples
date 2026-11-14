export default {
  identity: "example/parts",
  values: { PartId: [0, 100000], Count: [0, 10] },
  enums: { PartStatus: ["Released", "Machined", "Passed", "Rework", "Scrapped", "Assembled"] },
  records: {
    WorkOrder: { part: "Integer", ok: "Boolean" },
    Handoff: { part: "Integer", ok: "Boolean", pass: "Boolean", rework: "Boolean" },
    AttemptView: { ok: "Boolean", id: "Integer", previous: "Integer" },
    EffectView: { ok: "Boolean", part: "Integer" },
    TriggerView: { scrapped: "Boolean" },
    RecoveryView: { complete: "Boolean" },
    CommitView: { ok: "Boolean" },
    ClosureView: { complete: "Boolean" },
  },
  objects: {
    Job: { fields: { part: "Integer", status: "PartStatus", reworks: "Count" } },
    PlanningService: { fields: { open: "Count" } },
    MachiningStation: {
      fields: { reserved: "Boolean", busy: "Boolean" },
      ops: {
        reserve: { modifies: ["reserved"] },
        process: { modifies: ["busy"] },
        release: { modifies: ["reserved", "busy"] },
      },
    },
    InspectionStation: { fields: { queue: "Count" } },
    AssemblyStation: {
      fields: { reserved: "Boolean" },
      ops: { assemble: { modifies: ["reserved"] } },
    },
  },
  populations: { jobs: ["Job"], planning: ["PlanningService"], machining: ["MachiningStation"], inspection: ["InspectionStation"], assembly: ["AssemblyStation"] },
};
