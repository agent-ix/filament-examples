export default {
  identity: "example/payment-client",
  values: { Count: [0, 5] },
  objects: {
    Payment: {
      fields: { attempts: "Count", keySeen: "Boolean" },
      ops: {
        send: { modifies: ["attempts"] },
        retry: { modifies: ["attempts"] },
        apply: { modifies: ["keySeen"] },
        dropDuplicate: { modifies: [] },
      },
    },
  },
  populations: { payments: ["Payment"] },
};
