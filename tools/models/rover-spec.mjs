export default {
  identity: "example/rover-spec",
  objects: { Vehicle: { fields: { stopped: "Boolean" }, ops: { stop: { returns: "Boolean", modifies: ["stopped"] } } } },
  populations: { vehicles: ["Vehicle"] },
};
