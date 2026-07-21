# OBL-020 familywise/global boundary experiment

This LAB-only Lean file imports `StepWFStatementDraft.lean` unchanged and
checks three proposition-level facts about its abstract vocabulary:

1. The aggregate global draft implies the family-qualified wrapper because the
   wrapper only adds antecedents before the same `PreservesWF` conclusion.
2. The converse follows from the family-qualified wrapper only when an explicit
   experiment-local coverage premise supplies a canonical family for every
   well-formed actual step under consideration.
3. A finite non-vacuous model has one canonical family and one classified
   preserving step, while an unclassified step reaches a non-well-formed
   configuration. The wrapper holds but the aggregate draft fails.

The model's `ordinary` family, its classification relation, and the coverage
premise are not MirCore terminology, requirements, or a proposed taxonomy.
They exist only to show why a family-local proof cannot establish the aggregate
draft without a stated bridge. This file does not prove OBL-020, bind the
abstract carriers to Canon semantics, change `theory/11`, or require a
familywise proof architecture.
