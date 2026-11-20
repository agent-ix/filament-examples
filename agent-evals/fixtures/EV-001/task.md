Author a genuine native, source-owned object model in `model.fsl`, then check it with `fsl check` and save that command's JSON outcome as `check.json`. Use the shipped native grammar reference in docs/ and the public FSL executable. Do not use a model generator, external domain package, handwritten semantic IR, or a substitute model.

The model describes a storage bin:
- Exactly one object declaration named StorageBin. It is concrete, without inheritance, and its identity is the field key.
- Exactly four required scalar fields: key is an integer from 1 through 16; limit is an integer from 1 through 12; occupied is an integer from 0 through 12; locked is Boolean. You may use native bounded integer types or aliases to them.
- The object declares its closed population bins.
- Its sole operation reserve takes one parameter amount, an integer from 1 through 12, returns Boolean, and declares its write frame as exactly the receiver's occupied field. It creates/deletes nothing.
- Its sole state clause is the current invariant WithinCapacity applies to StorageBin: occupied must not exceed limit. Write that relationship directly over the two self fields; it must hold for every allowed pair, independent of key/locked. Equivalent Boolean/arithmetic formulations are welcome; comments or a trivially true predicate are not a model of this requirement.

Use language ix:native edition 1-draft and select filament.model.complete/v1. The native grammar uses `object Name identity (field) { ... }`, object members include fields/populations/operations, and a current invariant has `invariant Name using alias on Type at current { ... }`. Discover the full syntax in the grammar rather than substituting another route. Treat docs/ as read-only reference material. Work only in this workspace; docs/ contains references to shipped documentation and the published native grammar. Do not inspect the evaluator, reference answers, or other source repositories.

When the authored model actually checks, your final action must be `echo '<<<EVAL-COMPLETE>>>'`. Print that marker only at the end. If blocked, explain the concrete public-tool problem and finish with `echo '<<<EVAL-FAILED>>>'` instead.
