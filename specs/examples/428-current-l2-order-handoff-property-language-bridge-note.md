# 428 — current L2 order handoff property-language bridge note

## 目的

order / handoff line で、

- cut-family comparison
- relation decomposition
- adequacy corpus
- property-to-boundary matrix
- candidate reduction

を property-language bridge に接続する current cut を整理する。

ここで fixed するのは
**higher-level kept/second candidate と boundary matrix をどう bridge するか**
であり、

- final property language
- final source-surface handoff syntax
- concrete theorem / protocol tool binding

は still later に残す。

## source-backed floor

- cut family は
  local finalization / ordering-only barrier / snapshot-only observation / durable commit
  に分けて比較済みである。
- relation decomposition は
  `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before`
  まで docs-first working vocabulary に上がっている。
- adequacy corpus と 4-way boundary matrix は fixed 済みである。
- candidate reduction は
  `authority_serial_transition_family` first、
  `witness_aware_commit_family` second、
  `event_tree_execution_view` derived/debug、
  low-level `memory_order` family retained-later reference
  に置く。

## current bridge shape

```text
order_handoff_property_language_bridge = {
  property_family_refs,
  relation_refs,
  kept_candidate_refs,
  second_candidate_refs,
  boundary_matrix_refs,
  adequacy_corpus_refs,
  guard_refs,
  kept_later_refs
}
```

## current judgment

1. current property-language bridge は
   **`authority_serial_transition_family` を kept higher-level candidate**
   に置く。
2. `witness_aware_commit_family` は second candidate に残し、
   `event_tree_execution_view` は derived/debug candidate に留める。
3. property-language bridge は theorem/protocol/runtime-policy boundary への docs-first handoff であり、
   low-level `memory_order` exact API を source surface に import しない。
4. publication omission、witness omission、replay / epoch drift、provider / authority collapse、fairness overclaim を
   negative corpus family として separate に保つ。

## guard

- low-level `memory_order` exact API を source surface に入れない。
- final source-surface handoff syntax を fixed しない。
- final property language を fixed しない。
- theorem / protocol concrete tool binding を fixed しない。

## next promoted line

next promoted line は、
**modal promotion-threshold note**
に置く。

## what is not decided here

- final property language
- final emitted artifact schema
- concrete theorem / protocol tool binding
- final `snapshot_cut` naming
