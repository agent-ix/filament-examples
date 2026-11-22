export default {
  identity: "example/plant-line",
  objects: {
    Line: {
      fields: { valveOpen: "Boolean", pumpRunning: "Boolean" },
      ops: {
        openValve: { returns: "Boolean", modifies: ["valveOpen"] },
        closeValve: { returns: "Boolean", modifies: ["valveOpen"] },
        startPump: { returns: "Boolean", modifies: ["pumpRunning"] },
        stopPump: { returns: "Boolean", modifies: ["pumpRunning"] },
      },
    },
  },
  populations: { lines: ["Line"] },
};
