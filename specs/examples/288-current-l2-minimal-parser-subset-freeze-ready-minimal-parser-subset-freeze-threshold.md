# 288 — current L2 minimal-parser-subset-freeze-ready minimal-parser-subset-freeze threshold

## 目的

`specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
で minimal parser subset freeze comparison の first candidate を
stage 1 + stage 2 structural floor に置く判断を fixed した次段として、

- minimal parser subset freeze の minimum をどこまでに留めるか
- accepted cluster / reject cluster / retained-later floor をどの field family で表すべきか
- stage 3 helper-local evidence と future public parser API をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 3 reopen line の
minimal-parser-subset-freeze-ready minimal-parser-subset-freeze threshold** であり、

- public parser API
- final parser grammar
- full request node parse
- richer predicate diagnostics

はまだ固定しない。

## 比較観点

1. accepted cluster / reject cluster / retained-later floor を minimum に分けて書けるか
2. stage 3 helper-local evidence を rejection と誤読させないか
3. stage 1 + stage 2 structural floor の minimalityを保てるか
4. next promoted line を narrow に parser-to-checker reconnect freeze へ進められるか

## 比較対象

### 案 1. `accepted_cluster_refs` だけを持つ

#### shape

```text
minimal_parser_subset_freeze_ready_sketch = {
  accepted_cluster_refs = [
    stage1_chain_declaration_structural_floor,
    stage2_try_rollback_structural_floor
  ]
}
```

#### 利点

- 軽い
- accepted floor だけを素早く見せられる

#### 欠点

- reject / malformed family と retained-later floor が prose 依存になる
- stage 3 helper-local evidence を永久 reject と誤読させやすい

### 案 2. `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` を持つ

#### shape

```text
minimal_parser_subset_freeze_ready_sketch = {
  freeze_kind = stage1_stage2_structural_parser_floor,
  accepted_cluster_refs = [
    stage1_chain_declaration_structural_floor,
    stage2_try_rollback_structural_floor
  ],
  reject_cluster_refs = [
    missing_edge_local_lineage_metadata,
    missing_fallback_body,
    atomic_cut_fallback_placement
  ],
  retention_floor_refs = [
    stage3_admit_slot_branch,
    stage3_request_clause_branch,
    stage3_predicate_fragment_branch
  ]
}
```

#### 利点

- accepted / reject / retained-later の current reading を minimum に反映できる
- stage 3 helper-local evidence を retained-later floor として明示できる
- reconnect freeze に渡す parser gate snapshot が読みやすい

#### 欠点

- 案 1 より少し重い
- `retention_floor_refs` の finer split は later で refinement 余地がある

### 案 3. public parser API / token/span / request head carrier 近傍 field まで minimum に入れる

#### 利点

- future actual parser 実装に一見近づく

#### 欠点

- public parser API と docs-only freeze を premature に混ぜる
- final grammar / diagnostics / request-node parse を既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` を持つ**
である。

理由は次の通り。

1. stage 1 + stage 2 accepted cluster と stage 3 retained-later floor を同時に minimum へ反映できる
2. truly malformed な first-tranche reject family を retained-later family と分けて書ける
3. reconnect freeze の input として使いやすい

## current first choice shape

```text
minimal_parser_subset_freeze_ready_sketch = {
  freeze_kind = stage1_stage2_structural_parser_floor,
  accepted_cluster_refs = [
    stage1_chain_declaration_structural_floor,
    stage2_try_rollback_structural_floor
  ],
  reject_cluster_refs = [
    missing_edge_local_lineage_metadata,
    missing_fallback_body,
    atomic_cut_fallback_placement
  ],
  retention_floor_refs = [
    stage3_admit_slot_branch,
    stage3_request_clause_branch,
    stage3_predicate_fragment_branch
  ]
}
```

### この shape でまだ入れないもの

- public parser API name / module placement
- final token / span / diagnostic carrier
- full request head parse
- multiline attachment concrete grammar
- richer predicate grammar

これらは still later である。

## practical reading

current minimal parser subset freeze が示すのは、

- first actual parser tranche は stage 1 + stage 2 structural floor で十分である
- first-tranche reject family は truly malformed structural family に限る
- stage 3 request/admit/predicate evidence は retained-later floor として維持する

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-parser-subset-freeze-ready parser-to-checker-reconnect-freeze comparison**
に置く。

## open questions

- `retention_floor_refs` を stage 3 branch 内でどこまで細分するか
- stage 3 reopen で request head と clause suite のどちらを先に上げるか
- actual parser crate-local API を 언제 narrow に actualize するか
