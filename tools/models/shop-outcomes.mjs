export default {
  identity: "example/shop-outcomes",
  objects: { Order: { fields: {} } },
  populations: { orders: ["Order"], lookupTargets: ["Order"] },
};
