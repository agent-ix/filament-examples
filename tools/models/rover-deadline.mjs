export default {
  identity: "example/rover-deadline",
  enums: { Mode: ["Degraded", "SafeStop"] },
  objects: {
    Rover: {
      fields: { mode: "Mode", stopped: "Boolean", x: "Clock" },
      ops: {
        brake: { returns: "Boolean", modifies: ["mode", "x"] },
        halt: { returns: "Boolean", modifies: ["stopped"] },
      },
    },
  },
  populations: { rovers: ["Rover"] },
};
