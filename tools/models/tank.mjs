export default {
  identity: "example/tank",
  values: { Level: [0, 3] },
  objects: {
    Tank: {
      fields: { level: "Level" },
      ops: { fill: { modifies: ["level"] }, drain: { modifies: ["level"] } },
    },
  },
  populations: { tanks: ["Tank"] },
};
