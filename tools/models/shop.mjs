export default {
  identity: "example/shop",
  values: { Cents: [0, 1000000], Qty: [1, 99], Key: [0, 1000000] },
  enums: { OrderStatus: ["Open", "Paid", "Shipped", "Cancelled"] },
  objects: {
    Staff: { fields: { badge: "Key" } },
    Charge: { fields: { key: "Key", amount: "Cents" } },
    Refund: { fields: { amount: "Cents", approvedBy: "Staff", secondApprover: { type: "Staff", optional: true } } },
    Order: {
      fields: {
        total: "Cents", charged: "Cents", status: "OrderStatus",
        charges: { type: "Charge", many: [0, 8] },
        refunds: { type: "Refund", many: [0, 8] },
      },
      ops: {
        charge: { params: { amount: "Cents" }, modifies: ["charged", "status"] },
        refund: { params: { amount: "Cents", approvers: { type: "Staff", many: [1, 2] } }, modifies: ["refunds"] },
        note: { params: { secondApprover: { type: "Staff", optional: true } }, modifies: [] },
      },
    },
  },
  populations: { orders: ["Order"], charges: ["Charge"], refunds: ["Refund"], staff: ["Staff"] },
};
