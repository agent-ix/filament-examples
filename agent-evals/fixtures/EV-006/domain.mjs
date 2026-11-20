// Original domain input: the agent authors the complete protocol, not this package.
export default {
  identity: "eval/handshake",
  values: { Ticket: [0, 1] },
  objects: {
    Job: { fields: { ticket: "Ticket" } },
    Client: { fields: { ready: "Boolean" } },
    Server: { fields: { ready: "Boolean" } },
  },
  records: { Message: { ticket: "Ticket" }, Closure: { complete: "Boolean" } },
  populations: { jobs: ["Job"], clients: ["Client"], servers: ["Server"] },
};
