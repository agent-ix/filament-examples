export default {
  identity: "example/rover-spec",
  objects: { Vehicle: { fields: { stopped: "Boolean" }, ops: { stop: { modifies: ["stopped"] } } } },
  populations: { vehicles: ["Vehicle"] },
};
