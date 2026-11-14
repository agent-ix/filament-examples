# Library

The library example is a small model written from scratch, with one rule compiled against it. It is the worked example of the "Write a model" guide.

A book has a number of copies and a status, available or on loan, and an operation `lend`. The model is in [model/library.semantic-ir.json](model/library.semantic-ir.json), generated from [tools/models/library.mjs](../../tools/models/library.mjs).

## Cases

The case `compile-ok` compiles the rule `OnLoanHasCopies` against the model. The case `compile-misspelled` compiles the same rule with a field the model does not have, and the checker refuses it.
