# WRK-0022 - WRITE-CROSS failure-generation falsifier

## status

This LAB memo records the registered Lean falsifier for
`mirrorea_canon/working/WRK-0022-write-cross-failure-generation-boundary.md`.
The Canon working record is `L3-open, frozen`; this memo is not a
failure-generation function, a failure-row equality, or an elaboration result.

## registered execution

The registration at `cc8652f9d3dbebf465a28e09bde0e760fc953d66` required the
marker check after push. It confirmed that
`WriteCrossFailureGenerationBoundary.lean` did not already exist. Lean 4.29.1
then checked the existing `ElabDeterminismStatementDraft.lean` successfully.

The allowed transient source imported that existing draft and declared only a
finite six-name failure type, fixed `Allowed` and `Declared` rows, the two
displayed containment predicates, two candidate rows, and four registered
theorem names. It was not committed.

## observed falsifier

The exact registered source command emitted:

```text
samples/lean/lab-statements/obl021/WriteCrossFailureGenerationBoundary.lean:1:0: error: unknown module prefix 'samples'

No directory 'samples' or file 'samples.olean' in the search path entries:
/home/codex/.elan/toolchains/leanprover--lean4---v4.29.1/lib/lean
```

This is the record's explicit `Lean cannot prove both containment instances and
row difference` falsifier. The registered source audit and synchronization test
ran independently after source creation, but neither repairs the failed import
nor supplies countermodel evidence. The transient source and explanation were
removed before this memo was committed.

## root-cause boundary

The bare `lean <path>` command fixed by the record cannot resolve the imported
`samples` module. Changing the import, using a different environment command,
or otherwise making the file compile would repair the frozen registration;
WRK-0022 prohibits that.

## permitted follow-up

A future record may independently select a different source procedure and
falsifier after a fresh non-duplication and consumer screen. It must not claim
to repair, validate, or promote WRK-0022, and it must not select the intended
failure-generation function, row equivalence, diagnostic behavior, Canon
elaboration determinism, or OBL-021 status from this failure.

## non-claims

No finite countermodel, unique-row conclusion, Canon derivation result,
failure-generation function, failure-row equality, E-ROW-001 behavior, OBL
result, implementation behavior, Gate/Phase, or public status follows from the
failed import.
