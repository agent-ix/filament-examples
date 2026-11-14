export default {
  identity: "example/rover",
  values: { Speed: [0, 2000], Range: [0, 20000], Millis: [0, 3600000] },
  enums: { Mode: ["Driving", "Degraded", "SafeStop", "Failsafe"] },
  objects: {
    Rover: {
      fields: {
        mode: "Mode", speed: "Speed", stopped: "Boolean", lidarOk: "Boolean",
        rangeA: "Range", rangeB: "Range", rangeC: "Range", clock: "Millis", voteWrong: "Boolean",
      },
      ops: {
        loseLidar: { returns: "Boolean", modifies: ["mode", "lidarOk", "speed"] },
        brake: { returns: "Boolean", modifies: ["mode", "speed"] },
        halt: { returns: "Boolean", modifies: ["speed", "stopped"] },
        enterFailsafe: { returns: "Boolean", modifies: ["mode"] },
        sample: { returns: "Boolean", params: { faultA: "Boolean", faultB: "Boolean", faultC: "Boolean" }, modifies: ["voteWrong"] },
      },
    },
  },
  populations: { rovers: ["Rover"] },
};
