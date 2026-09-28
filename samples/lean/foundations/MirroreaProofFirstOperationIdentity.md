# LAB: selection of one existing operation identity

This module checks a bounded lookup condition used by SYS3 owner projection.
It does not define statement identities or a new source execution profile.

`Selects` gives positional rules: select a matching head only if every remaining
entry misses, or skip a nonmatching head. `resolve` filters the input and returns
only a singleton. `resolve_exact` proves soundness and relative completeness for
arbitrary finite lists and Boolean key predicates. `selected_member` retains the
actual entry. General singleton acceptance rules out an all-refusal checker;
`repeated_occurrence_refused` distinguishes two equal entries from one occurrence.

The actual Rust predicate in `evaluation_signature_by_identity` compares name,
kind and owner locus. SYS3 now checks its result before emitting either local or
remote owner fragments, propagating an existing structural diagnostic through
the shared ordinary/provider lowering. The Lean model covers the selector
algorithm; Rust execution, field authenticity, parser/M6 association and snapshot
validation are separate implementation obligations.

In particular, one handler with assignments at two distinct owners can have two
unique typed keys but colliding requester fragment references. The guard does
not solve that problem, source statement ordering, unit acknowledgment, or the
preservation of committed owner effects across requester rejection. Those remain
W4-C obligations. Nor does it add a guard to source-free private image restore.

Lean 4.29.1, `--trust=0 -j1`, `import Std`; no Mir-specific axioms. The owned-module
audit allows only `propext`, `Classical.choice`, `Quot.sound` and checks generated
declarations as well as named theorems. External fresh audit: 23 declarations;
two intentionally false controls (first-match acceptance of duplicates and total
refusal of a matching singleton) fail. They are counterchecks, not general proofs.

See Report2614 and `docs/proof-first/W4_CHECK.json` for exact Rust RED/GREEN,
Oracle advice, positive preservation comparison, and unresolved restore cuts.
