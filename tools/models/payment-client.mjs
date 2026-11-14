export default {
  identity: "example/payment-client",
  values: { Count: [0, 5] },
  objects: {
    Payment: {
      fields: { attempts: "Count", keySeen: "Boolean" },
      ops: {
        send: { returns: "Boolean", modifies: ["attempts"] },
        retry: { returns: "Boolean", modifies: ["attempts"] },
        apply: { returns: "Boolean", modifies: ["keySeen"] },
        dropDuplicate: { returns: "Boolean", modifies: [] },
      },
    },
  },
  populations: { payments: ["Payment"] },
};
