export default {
  identity: "example/architecture",
  values: { Count: [0, 50] },
  enums: { Tier: ["Edge", "Core", "Data"], Classification: ["Public", "Internal", "Personal"] },
  objects: {
    Team: { fields: { size: "Count" } },
    Store: { fields: { classification: "Classification", owner: "Service" } },
    Instance: { fields: { serving: "Boolean", release: "Count" } },
    Service: {
      fields: {
        tier: "Tier",
        owner: { type: "Team", optional: true },
        dependsOn: { type: "Service", many: [0, 8], ordered: true, unique: false },
        reads: { type: "Store", many: [0, 8] },
        handlesPersonalData: "Boolean",
        minServing: "Count",
        instances: { type: "Instance", many: [0, 10] },
        up: "Boolean",
      },
      ops: { call: { params: { ok: "Boolean" }, modifies: ["up"] } },
    },
  },
  populations: { teams: ["Team"], stores: ["Store"], instances: ["Instance"], services: ["Service"] },
};
