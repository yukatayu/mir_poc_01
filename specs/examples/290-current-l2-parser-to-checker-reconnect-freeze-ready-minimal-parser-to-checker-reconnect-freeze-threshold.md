# 290 — current L2 parser-to-checker-reconnect-freeze-ready minimal-parser-to-checker-reconnect-freeze threshold

## 目的

`specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
で parser-to-checker reconnect freeze comparison の first candidate を
stage 1 summary + stage 2 structural contract に置く判断を fixed した次段として、

- parser-to-checker reconnect freeze の minimum をどこまでに留めるか
- parser subset freeze と checker floor bridge をどの field family で表すべきか
- stage 3 request/predicate line、`e19`、`E21` / `E22` をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 3 -> 5 bridge line の
parser-to-checker-reconnect-freeze-ready minimal-parser-to-checker-reconnect-freeze threshold** であり、

- public parser/checker API
- stage 3 request reconnect
- runtime / proof boundary actualization

はまだ固定しない。

## 比較観点

1. parser subset freeze と checker floor bridge の relation を minimum に反映できるか
2. stage 1 / stage 2 bridge と retained-later floor を minimum に分けて書けるか
3. stage 3 request/predicate line、`e19`、`E21` / `E22` を premature に minimum へ混ぜないか
4. Phase 3 reconnect line を一区切りとして close できるか

## 比較対象

### 案 1. `parser_subset_freeze_ref + checker_floor_refs` だけを持つ

#### shape

```text
parser_to_checker_reconnect_freeze_ready_sketch = {
  parser_subset_freeze_ref = minimal_parser_subset_freeze_ready_sketch,
  checker_floor_refs = [
    stage1_reconnect_summary_floor,
    stage2_try_rollback_structural_floor
  ]
}
```

#### 利点

- 軽い
- parser subset から checker floor へ戻れる

#### 欠点

- reconnect 自体の kind が prose 依存になる
- retained-later floor が minimum に見えない

### 案 2. `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` を持つ

#### shape

```text
parser_to_checker_reconnect_freeze_ready_sketch = {
  reconnect_kind = stage1_stage2_checker_floor_bridge,
  parser_subset_freeze_ref = minimal_parser_subset_freeze_ready_sketch,
  checker_floor_refs = [
    stage1_reconnect_summary_floor,
    stage2_try_rollback_structural_floor
  ],
  retained_helper_refs = [
    stage3_request_predicate_reconnect_line,
    stage1_direct_target_mismatch_redesign_line,
    runtime_contrast_e21_e22_line
  ]
}
```

#### 利点

- current reconnect bridge が stage 1 + stage 2 に留まると minimum に反映できる
- stage 3 request/predicate line、`e19`、`E21` / `E22` を retained-later floor として明示できる
- Phase 3 reconnect freeze の stop line が読みやすい

#### 欠点

- 案 1 より少し重い
- retained helper line の finer split は later refinement の余地がある

### 案 3. stage 3 reconnect family や runtime-facing contrast detail まで minimum に入れる

#### 利点

- future compile path に一見近づく

#### 欠点

- current Phase 3 reconnect freeze と later reopen line を混ぜる
- parser / checker / runtime / proof boundary を premature に一体化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` を持つ**
である。

理由は次の通り。

1. stage 1 + stage 2 bridge と retained-later floor を minimum に分けて書ける
2. current reconnect freeze の stop line が見えやすい
3. Phase 3 self-driven line を一区切りとして close しやすい

## current first choice shape

```text
parser_to_checker_reconnect_freeze_ready_sketch = {
  reconnect_kind = stage1_stage2_checker_floor_bridge,
  parser_subset_freeze_ref = minimal_parser_subset_freeze_ready_sketch,
  checker_floor_refs = [
    stage1_reconnect_summary_floor,
    stage2_try_rollback_structural_floor
  ],
  retained_helper_refs = [
    stage3_request_predicate_reconnect_line,
    stage1_direct_target_mismatch_redesign_line,
    runtime_contrast_e21_e22_line
  ]
}
```

### この shape でまだ入れないもの

- public parser/checker API name
- stage 3 request contract reconnect
- `e19` direct target mismatch redesign
- `E21` / `E22` runtime contrast actual bridge
- runtime / proof boundary actualization

これらは still later である。

## practical reading

current minimal parser-to-checker reconnect freeze が示すのは、

- minimal parser subset freeze の current bridge は stage 1 + stage 2 checker floor まででよい
- stage 3 request/predicate line、`e19`、`E21` / `E22` は retained helper line に残す
- Phase 3 self-driven reconnect line はここで一区切りとして読める

という最小 cut である。

## next promoted line

next promoted line は、
**Phase 1 semantics / invariants / notation final sweep**
に置く。

## open questions

- compile-ready first tranche で stage 1 / stage 2 bridge をどの crate-local contract へ寄せるか
- stage 3 reconnect と `e19` redesign をどの順に reopen するか
- runtime / proof boundary contrast をどの tool-neutral shape へ戻すか
