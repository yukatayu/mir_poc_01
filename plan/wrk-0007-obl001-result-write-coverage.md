# WRK-0007 - OBL-001 result/write coverage countermodel

## Position

- The normative source is `mirrorea_canon/`; this is LAB experiment memory.
- The pre-registration is `mirrorea_canon/working/WRK-0007-obl001-result-write-coverage.md`.
- The target is the completeness of the current LAB draft's `GeneratedWrite`
  quantifier, not the truth of Canon THM-001 or a definition of Canon Core.

## Question

Can the unchanged LAB `THM001StatementDraft` hold for a successful
experiment-local Result which contains an experiment-only write but has no
`GeneratedWrite` witness for that write?

If it can, the evidence establishes only that the current draft does not encode
the result/write enumeration bridge. If it cannot, the existing draft has a
bridge or an equivalent restriction that must be recorded without strengthening
the Canon theorem.

## Method

1. Import the unchanged statement draft through a fresh external `.olean` path.
2. Use a two-case `ExperimentResult` and one `ExperimentWrite` only inside the
   LAB source. `ExperimentOnlyWriteMembership` labels one pair; it is not a
   Core AST, IR field, equality, or semantic relation.
3. Make the precondition and elaboration predicates true, every
   `GeneratedWrite` false, and all unrelated result-level postconditions true.
4. Check that the draft holds, that the untracked result elaborates and contains
   the labeled write, that no generated-write witness exists, and that the
   experiment-only coverage implication is false.
5. Run the forbidden-token audit and existing Lean synchronization test.

## Boundary

This is not a counterexample to Canon THM-001. The result does not define
whether the eventual theory should use an enumeration, traversal, inversion
lemma, or another Core/result bridge. It does not amend BND-001, select Core
or request semantics, reopen PROPOSAL-008, prove/discharge OBL-001/002, or
change the ledger, Gates, Phases, SCNs, implementation, or public API.
