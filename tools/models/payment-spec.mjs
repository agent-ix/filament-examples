export default {
  identity: "example/payment-spec",
  objects: { Payment: { fields: { charged: "Boolean" }, ops: { charge: { returns: "Boolean", modifies: ["charged"] } } } },
  populations: { payments: ["Payment"] },
};
