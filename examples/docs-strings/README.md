---
type: reference
---

# Exact String documentation cases

The String cases exercise exact scalar sequences through complete source programs and public run or compile requests. They check equality, case differences, composed and combining forms, supplementary scalars, all JSON short escapes, NUL and the empty value.

Malformed escapes, unpaired surrogates and unterminated literals must refuse during source parsing. Ordering and mixed String/Text comparisons must refuse during checking. Successful calls have explicit Boolean or String value oracles, including a false equality result.

Run this family from the repository root with `fsl-examples check docs-strings`, setting `FSL` to the driver binary. These cases support the exact String section of the documentation; bounded Text normalization remains separate scope.
