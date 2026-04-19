# e5-underdeclared-lineage

## Summary

- Static underdeclared omission sample with two representative proof obligations.
- This is the narrow static-side anchor for current theorem review-unit and Lean-stub generation.

## What This Lean File Means

- This file is generated from the repo-local theorem bridge and was accepted by `lean`.
- The generated theorem bodies still contain `sorry`, so the current guarantee is **artifact well-formedness and bridge alignment**, not full mathematical discharge.
- In concrete terms, the repo has verified that the review-unit to Lean-stub route produces syntactically valid Lean text for this sample and that the sample stays on the current theorem-first bridge floor.
- This is **not the final public theorem contract** and not the final proof-object schema.

## Why It Is Still Useful

- It preserves an inspectable snapshot of the actual Lean text attached to the current sample.
- It makes the current proof obligations concrete enough to compare across `e5`, `p06`, `p07`, and `p08`.
- It keeps the distinction between "Lean accepted the generated file" and "the domain theorem is fully proved" explicit.
