export default {
  identity: "my/library",
  values: { Copies: [0, 1000] },
  enums: { Status: ["Available", "OnLoan"] },
  objects: {
    Book: {
      fields: { copies: "Copies", status: "Status" },
      ops: {
        lend: { params: { count: "Copies" }, modifies: ["copies", "status"] },
      },
    },
  },
  populations: { books: ["Book"] },
};
