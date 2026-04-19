# CurrentL2LabelModel.lean

## Summary

- Two-point IFC label model with explicit authority-sensitive declassification lemmas.

## Why This File Exists

- This file is the first actual Lean fragment for Package 56. It does not expose final source syntax; it pins the minimal label semantics and authority-sensitive facts that the checker-adjacent IFC line relies on.
- Unlike the generated current-L2 sample stubs, this file contains actual small proofs rather than `sorry`.
- It is still helper-local and non-production. The goal is to pin the first mechanization-ready core, not to freeze the final public type system or verifier contract.
