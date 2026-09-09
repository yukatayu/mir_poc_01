# Finite graph validation consumer (LAB)

This proof consumes the support closure and tracked validation candidates inside
W1. It concerns one explicit directed relation graph; it does not collapse
existence, support, module dependency, relation and causal graphs into one DAG.
No production code, public graph representation or lifecycle state is selected.

`Path` independently defines reflexive finite paths. `Acyclic` forbids an edge
whose target can reach its source. `reachForms` translates a finite Boolean graph
into positive support formulas; the existing bounded closure checker is proved
exact for Path. The translation is used to calculate reachability, not to make
authority or lifetime claims about graph vertices.

For an initially acyclic graph, `checkAdd_exact` proves that rejecting a path from
target to source is necessary and sufficient for adding exactly that one edge.
`acyclic_subgraph` covers deletion. These facts do not validate arbitrary extra
changes under the name of one edge. The self-loop countermodel reproduced against
F0.3 is retained as a guard: checking its designated noncyclic edge succeeds,
whereas the whole-graph checker rejects the additional self-loop.

The smallest alternative for unrestricted finite edge patches is `checkAcyclic`,
which checks every present edge. `checkAcyclic_exact` proves soundness and
completeness for the independent Acyclic property. `scan` reads every finite
adjacency row, including absence. `scan_evaluates` and `sampled_exact` connect its
actual Query evaluation with the reconstructed graph. `dagQuery_exact` then
proves property-specific validator adequacy. Finally `committed_graph_acyclic`
composes this with tracked logical commit to prove the actual returned state
acyclic. No initial acyclicity is needed for this whole-result checker.
`interleaved_graph_publication` establishes the same property from an actual
PatchPreparation trace and final stamps, without a preparation-start snapshot.
The immutable patch and logical atomic reads/publication remain explicit.

This temporary consumer uses `Fin n` vertices and typed optional adjacency lists.
Absent rows mean no outgoing edges; repeated adjacency entries do not create
parallel-edge semantics. `n` is fixed for each theorem; dynamic universe changes,
external decoder validation, stored derived indexes, physical atomicity,
authorization, preparation issuance/consumption, restore/current-head selection,
resource limits and production simulation remain open. The whole-graph query
conservatively reads all n rows. The selective F0.3 traversal has not been proved
to translate into that Query; it is only a finite comparison reference.

Lean4.29.1 --trust=0 checked the general proofs with standard propext,
Classical.choice and Quot.sound (some structural lemmas need no axioms).
Fixed positive/negative controls passed. Main also compared all 512 directed
three-vertex graphs, including self-loops: the Lean checker agrees with the copied
F0.3 traversal when that traversal is applied to every present edge. This is
finite differential evidence, not a general Python refinement theorem.

Oracle source review `mirrorea-graph-consumer-review` completed; main checked
its proposed controls in MirroreaProofFirstGraphReview.lean with Lean4.29.1
--trust=0. No contradiction to the quantified statements was found. This closes
only the pure LAB graph dependency, not production or formal lifecycle acceptance.
The snapshot whole-graph checker rejects any lawful intervening graph-row write;
it has no unrelated-graph-write advantage over a same-domain global revision.
The live theorem protects each row from its observation time, not preparation
entry. An actual two-vertex preparation deletes a reverse edge before reading
that row, accepts the final graph, and rejects the preparation-start overlay.
No acceptance equivalence with F3's earlier destination-stamp capture is claimed.
Same-version fabricated current values, duplicate-key publication with a cyclic
prefix, absent destination existence, per-kind union, and secret-dependent
validation results are separate controls. The graph Boolean is not automatically
an authorized public observation. See report2611 and the check records.
