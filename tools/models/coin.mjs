export default {
  identity: "example/coin",
  objects: {
    Coin: {
      fields: { heads: "Boolean" },
      ops: { flip: { params: { ok: "Boolean" }, modifies: ["heads"] } },
    },
  },
  populations: { coins: ["Coin"] },
};
