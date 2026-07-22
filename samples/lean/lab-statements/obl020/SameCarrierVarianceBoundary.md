# Same-carrier relation variance experiment

This LAB-only file contains three generic Lean lemmas. It has no imports,
inductive data, local relation definitions, or downstream importers.

1. If each intended transition is contained in a model relation, model
   invariant preservation transfers to the intended relation.
2. If intended success/failure relations are contained in model relations,
   model coherence and success/failure exclusion transfer to the intended
   relations.
3. If each model success/failure witness is realizable by the intended
   relation, model outcome existence transfers to the intended relation.

The third direction is deliberately opposite to the first two. These are only
conditional same-carrier transfer laws. They do not establish a real model to
Canon relation, select a carrier or output representation, define scheduling or
fairness, enumerate writes, prove an obligation, or change Canon status.
