export default {
  identity: "example/cell",
  values: { Speed: [0, 3000], Distance: [0, 10000] },
  enums: { PressMode: ["Stopped", "Setup", "Run"] },
  objects: {
    Press: {
      fields: { guardClosed: "Boolean", ramMoving: "Boolean", mode: "PressMode", ramSpeed: "Speed", estop: "Boolean" },
      ops: {
        start: { returns: "Boolean", modifies: ["ramMoving", "mode", "ramSpeed"] },
        openGuard: { returns: "Boolean", modifies: ["guardClosed", "ramMoving", "mode", "ramSpeed"] },
      },
    },
    Robot: { fields: { speed: "Speed", separation: "Distance", personInCell: "Boolean" } },
  },
  populations: { presses: ["Press"], robots: ["Robot"] },
};
