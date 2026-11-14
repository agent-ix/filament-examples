export default {
  identity: "example/payment-spec",
  objects: { Payment: { fields: { charged: "Boolean" }, ops: { charge: { modifies: ["charged"] } } } },
  populations: { payments: ["Payment"] },
};
