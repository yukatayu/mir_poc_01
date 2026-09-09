# General label dependency (task-local LAB)

`LabelTheory A` supplies a Boolean preorder, bottom and binary least upper
bound. The theory is an explicit parameter, not a Mir axiom or an authority
grant. Antisymmetry, totality, finiteness and distributivity are not required.
General typing/flow checker exactness, confinement, two-run low-state and
ordered projected-write equality reuse the actual ProducerFlow evaluator.
The collector theorem uses the actual Passive.feed with common initial rows
and capacity. All source, store, policy and invocation premises remain explicit.

The diamond instance exercises incomparable labels and nonempty observations.
A lawful collapsed preorder demonstrates that algebraic validity does not
authorize replacement of the intended security policy. Such a replacement also
changes LowEq; it is not a counterexample to the fixed-policy theorem.

`FiniteTheory.Data n` contains total functions on Fin n, a bottom and a join.
Independent relational Laws are equivalent to the explicit finite Boolean
checker, and certify preserves the supplied relation and join. This is not a
materialized table decoder: O(n³) counts relation/join evaluations and gives no
runtime bound for arbitrary supplied functions. Byte decoding, lookup costs,
input/work limits, policy identity/version, current authority and all entry
paths remain implementation obligations. Fin 0 has no Data because bottom
cannot be supplied.

Controls include valid nonempty instances and invalid bottom/join/reflexivity,
plus a transitivity-only failure and an overlarge join that violates leastness.
The all-false relation also violates the bottom law; it is not an isolated
reflexivity test. Fixed decide controls do not replace the general proofs.

Lean 4.29.1 --trust=0 checked the coherent Passive/ProducerFlow/GeneralLabels
cut. Printed dependencies are standard propext and Quot.sound, with no custom
axioms or admissions. Oracle review Q2421976456e3b4e704364a9973cf8f77d1e159f50274db83ebe5b348d6192311
found no scoped theorem gap; main adopted the complexity qualification and
checked the two additional negative controls after review. Commands, hashes
and outputs are in docs/proof-first/FOUNDATION_CHECK.json.

This result is separate from the fallible arithmetic module. It does not cover
arbitrary local proof theories, resources, higher-order values, dynamic policy,
FFI, timing, persistence, runtime refinement or alpha acceptance.
