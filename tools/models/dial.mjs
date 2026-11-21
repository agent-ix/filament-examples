export default {
  identity: "example/dial",
  values: { Level: [0, 3] },
  objects: {
    Dial: {
      fields: { n: "Level" },
      ops: {
        up: { modifies: ["n"] },
        skip: { modifies: ["n"] },
        down: { modifies: ["n"] },
        reset: { modifies: ["n"] },
      },
    },
  },
  populations: { dials: ["Dial"] },
};
