export default {
  identity: "eval/safety-counter",
  values: { Level: [0, 4] },
  objects: {
    Counter: {
      fields: { level: "Level" },
      ops: { advance: { modifies: ["level"] } },
    },
  },
  populations: { counters: ["Counter"] },
};
