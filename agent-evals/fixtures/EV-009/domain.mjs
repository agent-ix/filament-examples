// Original user-provided inventory domain, not an agent-authored output.
export default {
  identity: "eval/blind-stock",
  values: { Count: [0, 4] },
  objects: {
    Stock: { fields: { count: "Count" }, ops: { add: { params: {}, modifies: ["count"] } } },
  },
  populations: { stocks: ["Stock"] },
};
