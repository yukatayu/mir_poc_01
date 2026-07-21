# plan/164 - OBL-001 result/write coverage boundary

## Role and authority

This is LAB repository memory. `mirrorea_canon/` remains authoritative for
THM-001, BND-001, the Core, every OBL status, Gates, and Phases. The evidence
here does not define a Core/result carrier or change Canon semantics.

## Trigger

The foundation audit correctly left the prior no-candidate triage as a priority
heuristic. A fresh reviewer then compared Canon THM-001's phrase "every write
in `c`" with the current LAB OBL-001 draft, whose soundness quantifier ranges
only over `GeneratedWrite result write`.

The draft has an opaque `Result` carrier and no relation that says every write
represented in that result is a `GeneratedWrite`. This is distinct from
T-RESEARCH-001: that audit put a write inside `GeneratedWrite` and removed its
justification; the present model keeps every generated-write obligation vacuous
and puts the experiment-only write outside the predicate's domain.

## Evidence

WRK-0007 pre-registered the result/write enumeration question at `cb83300e`.
Evidence commit `8d28ed89` adds an imported Lean countermodel in the existing
`samples/lean/lab-statements/obl001` lane. Its two experiment-only carriers
have one result/write pair:

```text
ExperimentOnlyWriteMembership(untrackedCross, only)
GeneratedWrite(untrackedCross, only) = false
ElaboratesAssignment(..., untrackedCross) = true
THM001StatementDraft(V, P) = true
```

Lean proves the draft, the successful untracked result, absence of a generated
write witness, and negation of the experiment-only enumeration implication.
The source imports an unchanged copy of the draft compiled to a fresh external
`.olean` directory. It requires no new runner, schema, helper, public surface,
or Canon carrier.

## Current disposition

1. The current LAB statement shape does not itself entail coverage from every
   experiment-local result write to `GeneratedWrite`.
2. This is not a counterexample to THM-001. It is not evidence that Canon
   permits a direct cross-locus write, and it does not identify the experiment
   label with Core `c`.
3. A future proof-facing package must stop before choosing any actual
   Core/result enumeration, traversal, inversion relation, or theorem premise.
   That future choice is reserved by ADR-0014.
4. The outcome-totality decision in PROPOSAL-008 remains independent. The new
   result neither resolves it nor depends on totality.

## Non-claims

No BND-001/THM-001 wording, ledger entry, OBL status, Core IR exchange form,
grammar, request semantics, equality, conformance, Gate/Phase, runtime, or
public claim changed. WRK-0007 remains L3 `not-promoted`; L2 remains
fail-closed pending the owner-authenticated trust anchor.
