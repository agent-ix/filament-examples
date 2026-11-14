export default {
  identity: "example/two-locks",
  values: { Holder: [0, 2], Process: [1, 2] },
  objects: {
    Locks: {
      fields: { a: "Holder", b: "Holder" },
      ops: {
        lockFirst: { params: { p: "Process" }, modifies: ["a", "b"] },
        lockSecond: { params: { p: "Process" }, modifies: ["a", "b"] },
        unlock: { params: { p: "Process" }, modifies: ["a", "b"] },
      },
    },
  },
  populations: { locks: ["Locks"] },
};
