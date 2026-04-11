# 292 — current L2 phase1-semantics-closeout-ready minimal-phase1-semantics-closeout threshold

## 目的

`specs/examples/291-current-l2-parser-to-checker-reconnect-freeze-ready-phase1-semantics-closeout-comparison.md`
で Phase 1 closeout comparison の first candidate を
narrow closeout bundle に置く判断を fixed した次段として、

- Phase 1 closeout の minimum をどこまでに留めるか
- semantics / invariant bridge / notation status を minimum にどう反映するか
- parser grammar / type system / external schema をどこまで still later に残すか

を比較する。

ここで固定するのは
**current L2 phase1-semantics-closeout-ready minimal-phase1-semantics-closeout threshold**
であり、

- final parser grammar
- final type system
- actual external verifier schema
- actual emitted artifact

はまだ固定しない。

## 比較観点

1. semantics closeout と parser/type finalization を distinguish できるか
2. invariants と proof-obligation wording の bridge を minimum に反映できるか
3. notation family boundaryを minimum に含めつつ lexical finalization を still later に残せるか
4. next promoted line を narrow に Phase 2 closeout へ進められるか

## 比較対象

### 案 1. `core_semantics_refs + invariant_bridge_refs` だけを持つ

#### shape

```text
phase1_semantics_closeout_ready_sketch = {
  core_semantics_refs = [
    fallback_lease_chain_semantics,
    try_atomic_cut_semantics
  ],
  invariant_bridge_refs = [
    invariants_8_9_to_canonical_normalization_law,
    invariant_11_to_rollback_cut_non_interference
  ]
}
```

#### 利点

- 軽い
- semantics と invariant bridge は見える

#### 欠点

- notation family boundary が minimum に現れない
- Phase 1 closeout が lexical finalization ではないことを示しにくい

### 案 2. `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs + retained_later_refs` を持つ

#### shape

```text
phase1_semantics_closeout_ready_sketch = {
  closeout_kind = current_l2_semantics_closeout,
  core_semantics_refs = [
    fallback_lease_chain_semantics,
    try_atomic_cut_semantics
  ],
  invariant_bridge_refs = [
    invariants_8_9_to_canonical_normalization_law,
    invariant_11_to_rollback_cut_non_interference
  ],
  notation_status_refs = [
    explicit_edge_row_family,
    a2_polished_first_choice,
    a1_companion_shorthand
  ],
  retained_later_refs = [
    final_parser_grammar,
    final_type_system,
    actual_external_schema
  ]
}
```

#### 利点

- semantics closeout の kind が明示される
- invariant bridge と notation boundary を minimum に同時に反映できる
- parser/type/schema finalization を retained-later line に残せる

#### 欠点

- 案 1 より field が増える

### 案 3. parser grammar / type / schema initial-finalization ref まで minimum に入れる

#### 利点

- closeout の外形は大きく見える

#### 欠点

- Phase 1 closeout を超えて finalization pressure を minimum に混ぜる
- current roadmap / autonomy gate と合わない

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. closeout minimum に必要なのは semantic redesign ではなく bridge と boundary の固定である
2. notation drift correction を minimum に反映できる
3. parser grammar / type system / schema finalization を still later に残しやすい

## current first choice shape

```text
phase1_semantics_closeout_ready_sketch = {
  closeout_kind = current_l2_semantics_closeout,
  core_semantics_refs = [
    fallback_lease_chain_semantics,
    try_atomic_cut_semantics
  ],
  invariant_bridge_refs = [
    invariants_8_9_to_canonical_normalization_law,
    invariant_11_to_rollback_cut_non_interference
  ],
  notation_status_refs = [
    explicit_edge_row_family,
    a2_polished_first_choice,
    a1_companion_shorthand
  ],
  retained_later_refs = [
    final_parser_grammar,
    final_type_system,
    actual_external_schema
  ]
}
```

### この shape でまだ入れないもの

- final reserved keyword / punctuation
- final type lattice / capability compatibility completion
- actual theorem / model-check contract
- actual emitted verifier artifact

これらは still later である。

## practical reading

current minimal Phase 1 closeout が示すのは、

- semantics core は settled である
- invariant bridge は proof-side naming まで fixed した
- notation boundary は explicit edge-row family + A2/A1 relation まで fixed した
- lexical finalization や parser/type/schema finalization は later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**phase1-semantics-closeout-ready phase2-parser-free-poc-closeout comparison**
に置く。

## open questions

- parser grammar で `lineage(...)` token をどう最終化するか
- type system / verifier schema reopen をどの順で行うか
- proof / model-check actual handoff を tool-neutral export から始めるか
