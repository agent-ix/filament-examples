# QSL-2102 affected bundle receipt

The independently reviewed runnable slice passes: 22 running cases, 0 failures. This is partial QSL-2102 delivery against the accepted cohort, not completion of every requested form or acceptance of current-main model and dimension features.

Tested head: `09cf046d8773abb928eb7dc2d92b823ff3d4339d`. Source/request/expectation fixes were reviewed PASS at `386467a`; `09cf046` adds the unchanged SR-1515 and SR-1516 review artifacts. The following receipt commit changes evidence only.

From the repository worktree, the final affected gate command was:

```sh
FSL=/home/peter/dev/.worktrees/luna-fsl-l3-main/target/debug/fsl \
FSL_TEST_PROVIDER=/home/peter/dev/.worktrees/luna-fsl-l3-main/target/debug/fsl-test-provider \
CARGO_TARGET_DIR=target CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=1 \
target/debug/fsl-examples check language-values
```

Exit 0: `check: 22 passed, 0 failed`. The unordered Bag dropFirst case correctly exited 20; the other 21 cases exited 0. The OrderedSet ordering mismatch returned the independently expected Boolean false. No FSL or runner build ran for this gate; it used the warm runner and accepted external driver cohort `a260b3b26277bfa907ce77373c4f15dac6671083`, locked FSL `8877b38b6397055fa11dc2e7882ad87d02ecb654`.

Generated manifest/index consistency:

```sh
target/debug/fsl-examples index
git diff --exit-code -- README.md index.json
```

Both exited 0; generation wrote 0 files. No existing example suite, full make check, product rebuild, version/hash check, main refresh, matrix or further review ran.

The confined fix receipt at `386467a` predates the final gate:

```sh
FSL=/home/peter/dev/.worktrees/luna-fsl-l3-main/target/debug/fsl \
target/debug/fsl-examples check language-values \
  --case text-constructor --case function-selection --case index-ordered-set
```

Exit 0: 3 passed, 0 failed. Text returns the independently expected `{"kind":"text","text":"café"}`. Function selection requests `selected`, expecting `{"integer":"42"}`, while `example` returns 99. OrderedSet indexing uses `orderedSet[8,8,3,6]`, so retaining duplicates changes the selected value. Initial authoring receipts for all 22 cases remain weaker pre-fix evidence; the final gate above supersedes them for this slice.

Dimension equivalence, delivery/release declarations, finite hyper, abstraction/guarantee/compose, method/dispatch, protocol reference, created/deleted frame identity, Text snapshot data, window aggregates, temporal captures and role projection/realizability remain unclaimed. No staged placeholder or passing-coverage claim was added for those forms. QSL-2102 remains In Progress.
