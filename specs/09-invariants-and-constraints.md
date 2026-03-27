# 09 — Invariants and Constraints

This document lists constraints that should be treated as strong unless and until explicitly revised.

## Graph / causality invariants

1. The semantic event structure is directed and acyclic.
2. Cuts (`atomic_cut`, `durable_cut`) create explicit global decision boundaries.
3. If a mechanism would allow hidden backward edges in meaning, it is suspect by default.

## Evolution invariants

4. Default evolution mode is downstream addition, not arbitrary upstream rewiring.
5. API shadowing is forbidden as a default design rule.
6. Compatibility-preserving overlays must not silently narrow domain behavior.

## Ownership / lifetime invariants

7. Linear resources must not be duplicated by continuation tricks or patching tricks.
8. Lifetime degradation is monotone.
9. Preference / fallback chains must normalize into an unambiguous monotone form.

## Failure / rollback invariants

10. Failure should remain semantically explicit.
11. Rollback may not cross finalizing cuts unless transformed into compensation.
12. Non-rollbackable effects must be marked, isolated, or compensated.

## Integration invariants

13. PrismCascade runtime internals must not be forced into Mir runtime semantics if doing so would weaken Prism's planning guarantees.
14. Legacy integration must state clearly which guarantees remain inside Mir semantics and which do not.
15. Engine-specific concepts must not leak into core language semantics.

## Tooling invariants

16. Any major design change should have a report.
17. Specs must say what is unresolved.
18. Visual tools must be explainable from the same underlying graph/contract model, not from ad-hoc hidden state.
