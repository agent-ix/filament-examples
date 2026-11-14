export default {
  identity: "example/lock",
  values: { Holder: [0, 2] },
  objects: {
    Lock: {
      fields: { owner: "Holder" },
      ops: {
        acquire: { params: { p: "Holder" }, modifies: ["owner"] },
        release: { modifies: ["owner"] },
      },
    },
  },
  populations: { locks: ["Lock"] },
};
