# Tracked validation candidate (LAB)

This scoped W1 dependency studies validation replay under tracked reads. It does not
select ordinary Mir read consistency, transactions across owners, or Q-18.
The direct consumer is finite support/DAG update validation, including absent
rows and adjacency indexes. No production source has changed.

`Query K V A` is a well-founded read-dependent computation: result or read with
a continuation for every optional value. `Evaluates` independently specifies
its meaning; `evaluate_exact` proves correspondence with the executable evaluator.
`tracked_reads_suffice` proves equal result and read path when two stores agree
on the first run's actual footprint. This does not assume that all keys agree.
Arbitrary impure Python callbacks, divergence, effects, hidden reads and resources
are outside this query language. A compiler/checker translation remains open.

Cells retain natural-number versions even for absent values. Every `write`
increments its key's version, including deletion and equal-value replacement.
For any finite sequence of these writes, versions are monotone, and equal
versions imply the identical cell. `checkStamps_exact` connects executable version
comparison to that premise. `validated_query_preserved` composes this invariant
with tracked evaluation. A reset/overflow/restore path is not among the transitions.

A finite first-key-wins overlay models a proposed patch for validation.
`requiredKeys` contains actual query reads plus all blind-write keys;
`checked_overlay_replay` proves replay against intervening state with unchanged
stamps, and `checked_blind_write_unchanged` proves those target cells unchanged.
This overtracks patched reads conservatively. The logical `publish` applies the tail before the head and is proved to
produce exactly these first-key-wins values, even with duplicates. It is also
a finite sequence of the declared incrementing writes. `commit` checks the
prepared Boolean query and required stamps before logical publication;
`commit_rechecks_result` proves that accepted output satisfies the same query.
This is not an invariant-adequacy theorem: authorization, issued preparation
origin, one-use identity, and physical atomic check/write remain unproved. F0.3 uses unique-key dictionaries; no general Python refinement
is claimed. Same result alone is not a state-invariant preservation theorem.

Candidate A tracks actual rows; the smallest alternative B checks one global
state revision. B also needs a monotone all-mutator revision discipline and
rejects unrelated changes that A permits. Neither may trust a restored old head.

Fixed controls cover a data-dependent read, absent row, unrelated positive write,
delete/recreate ABA, omitted index/absence footprint, blind-write omission, and
version-reset counterexample outside the proved write language. General theorems
use induction, not fixed `decide` examples. The successful Lean4.29.1 --trust=0
cut uses only standard propext/Quot.sound axioms; no Mir-specific axiom or admission.
Kernel/toolchain and explicit current-state selection are distinct trust boundaries.
The first frozen Oracle review completed with no theorem counterexample under
its premises; it did not execute Lean. Main reproduced extra-edge self-loop,
captured-patch mutation, unrelated invariant-breaking mutation, and hidden-capture
countermodels against the copied F0.3 Store. An issued preparation can match its
record exactly while recording a different patch from the one validated: Python
copies the caller patch twice. No production adoption follows.

The extension models actual preparation interleavings with atomic `Cell` reads
and version-incrementing writes. `PreparationTrace` determines each observation's
origin; origin and reachability are proved by induction. If final observed stamps
match, `interleaved_validation_sound` proves the same result/path in the final
store, without a preparation-start snapshot. A torn old-value/new-version control
passes stamp comparison alone; `forged_has_no_origin` proves it has no lawful
preparation trace from any starting store for that query and final state.

`writes_version_exact` counts writes to each key. `checkStamps_no_writes` proves
that the specified stamp check accepts exactly when no write in its specified
common-reference history targets a required key. This is not full commit
acceptance (which also requires a true query), nor zero writes throughout a
live preparation interval: a write before a key is observed may be accepted. This is
not maximal semantic permissiveness: equal-value writes are intentionally
conflicts, and B's global revision would additionally reject unrelated writes.
No global head authentication follows.

The same immutable query, captured parameters, and patch must bind preparation
and commit. Query success requires a separate property-specific adequacy proof.
For DAG updates, validating one named edge is insufficient for arbitrary patches
or invariant-breaking other mutators. Absent/index representation, all mutators,
store lifetime, restore, authority and physical linearization remain explicit
implementation/proof obligations. The first interleaving/publication/count review completed without a theorem
counterexample. Main corrected the stamp-versus-commit wording and added
repeated-read/last-only-log, false-query, duplicate-version and unlawful-history
controls. Its missing live-overlay and graph bridge findings are addressed by
newer proofs subsequently covered by the completed graph-consumer review. See
report2611, TRACKED_VALIDATION_CHECK.json and the GraphValidation companion.

The latest extension also treats a fixed patch during live preparation.
`PatchPreparation` reads each raw Cell atomically and supplies the corresponding
patched value to the same Query; concrete writes may interleave. Origins and
observed semantics are derived from that trace. `interleaved_patch_replay` and
`patch_publication_replay` prove the result/path against final overlay and actual
logical published values, respectively. The patch term is identical throughout;
captured caller mutation is not an allowed transition. This does not establish
issued-preparation origin, blind-write conflict history for unread destinations,
authority, physical read atomicity or publication linearization. The whole-graph
consumer reads all destinations; no generic guarantee is inferred for other
queries. The subsequent `mirrorea-graph-consumer-review` covered these patch and
graph lemmas; main checked the returned controls in a coherent imported cut.
This permits scoped mathematical dependency use only. It does not discharge
physical publication, actual identity/current-state selection or source refinement.
