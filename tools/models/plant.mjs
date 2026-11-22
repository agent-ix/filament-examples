export default {
  identity: "example/plant",
  values: { Percent: [0, 100] },
  objects: {
    Valve: { fields: { open: "Boolean" } },
    Pump: { fields: { running: "Boolean", discharge: "Valve" } },
    Tank: { fields: { level: "Percent" } },
  },
  populations: { valves: ["Valve"], pumps: ["Pump"], tanks: ["Tank"] },
};
