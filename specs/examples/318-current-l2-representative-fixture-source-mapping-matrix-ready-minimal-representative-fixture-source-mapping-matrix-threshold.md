# 318 — current L2 representative-fixture-source-mapping-matrix-ready minimal-representative-fixture-source-mapping-matrix threshold

## 目的

`specs/examples/317-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-representative-fixture-source-mapping-matrix-comparison.md`
で representative / fixture / source mapping matrix の current first choice を fixed した次段として、

- mapping matrix の minimum をどこまでに留めるか
- `e3` variant / `e23` unresolved anchor を minimum にどう反映するか
- next line の lowering first cut へどこまで渡すか

を比較する。

ここで固定するのは
**current L2 representative-fixture-source-mapping-matrix-ready minimal-representative-fixture-source-mapping-matrix threshold**
であり、

- actual sample file content
- reached stage inventory
- bless / regression policy

はまだ固定しない。

## 比較観点

1. 3 層対応を minimum で読めるか
2. `e3` variant / `e23` unresolved anchor を minimum で失わないか
3. lowering / runner へ渡す information が不足しないか

## 比較対象

### 案 1. `sample_stem + fixture_id + source_sample_target_ref` だけを持つ

#### 利点

- 軽い。

#### 欠点

- representative layer が消える。
- `e3` variant / `e23` unresolved anchor を表せない。

### 案 2. `matrix_kind + rows(ladder_order + sample_stem + representative_anchor_ref + representative_status + source_example_id + fixture_ref + fixture_id + fixture_mode + source_sample_target_ref + coverage_focus + expected_static_verdict + expected_runtime_outcome)` を持つ

#### 利点

- 3 層対応を minimum で読める。
- current initial cluster の差分点を lossless に残せる。
- lowering / runner の first cut に渡す expected verdict / outcome も読める。

#### 欠点

- 案 1 よりは fields が多い。

### 案 3. reached stage / bless policy まで minimum に含める

#### 利点

- next task chain との繋がりは見えやすい。

#### 欠点

- mapping threshold ではなく Task 4 / Task 5 threshold を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `matrix_kind + rows(ladder_order + sample_stem + representative_anchor_ref + representative_status + source_example_id + fixture_ref + fixture_id + fixture_mode + source_sample_target_ref + coverage_focus + expected_static_verdict + expected_runtime_outcome)` を持つ**
である。

理由は次の通り。

1. current task は source target と representative prose の対応を失わずに固定する task である。
2. `e3` variant / `e23` unresolved anchor を minimum に残さないと、next lowering line の coverage 判断が曖昧になる。
3. reached stage / bless policy は next threshold へ分離したままでよい。

## current first choice shape

```text
representative_fixture_source_mapping_matrix = {
  matrix_kind = current_l2_fixed_subset_representative_fixture_source_mapping,
  rows = [
    { ladder_order = 1, sample_stem = e1_place_atomic_cut, representative_anchor_ref = representative_program_E1, representative_status = direct, source_example_id = E1, fixture_ref = fixture_e1_place_atomic_cut, fixture_id = e1_place_atomic_cut, fixture_mode = runtime_fixture, source_sample_target_ref = samples_current_l2_e1_place_atomic_cut_txt, coverage_focus = place_atomic_cut_post_cut_failure, expected_static_verdict = valid, expected_runtime_outcome = explicit_failure },
    { ladder_order = 2, sample_stem = e2_try_fallback, representative_anchor_ref = representative_program_E2, representative_status = direct, source_example_id = E2, fixture_ref = fixture_e2_try_fallback, fixture_id = e2_try_fallback, fixture_mode = runtime_fixture, source_sample_target_ref = samples_current_l2_e2_try_fallback_txt, coverage_focus = local_try_fallback_rollback, expected_static_verdict = valid, expected_runtime_outcome = success },
    { ladder_order = 3, sample_stem = e3_option_admit_chain, representative_anchor_ref = representative_program_E3_variant, representative_status = variant, source_example_id = E3_variant, fixture_ref = fixture_e3_option_admit_chain, fixture_id = e3_option_admit_chain, fixture_mode = runtime_fixture, source_sample_target_ref = samples_current_l2_e3_option_admit_chain_txt, coverage_focus = option_local_admit_non_admissible_skip, expected_static_verdict = valid, expected_runtime_outcome = success },
    { ladder_order = 4, sample_stem = e4_malformed_lineage, representative_anchor_ref = representative_program_E4, representative_status = direct, source_example_id = E4, fixture_ref = fixture_e4_malformed_lineage, fixture_id = e4_malformed_lineage, fixture_mode = static_only_fixture, source_sample_target_ref = samples_current_l2_e4_malformed_lineage_txt, coverage_focus = edge_local_lineage_mismatch_static_stop, expected_static_verdict = malformed, expected_runtime_outcome = not_evaluated },
    { ladder_order = 5, sample_stem = e21_try_atomic_cut_frontier, representative_anchor_ref = representative_program_E21, representative_status = direct, source_example_id = E21, fixture_ref = fixture_e21_try_atomic_cut_frontier, fixture_id = e21_try_atomic_cut_frontier, fixture_mode = runtime_fixture, source_sample_target_ref = samples_current_l2_e21_try_atomic_cut_frontier_txt, coverage_focus = try_body_atomic_cut_frontier_update, expected_static_verdict = valid, expected_runtime_outcome = success },
    { ladder_order = 6, sample_stem = e23_malformed_try_fallback_missing_fallback_body, representative_anchor_ref = unresolved_representative_anchor, representative_status = unresolved, source_example_id = E23, fixture_ref = fixture_e23_malformed_try_fallback_missing_fallback_body, fixture_id = e23_malformed_try_fallback_missing_fallback_body, fixture_mode = static_only_fixture, source_sample_target_ref = samples_current_l2_e23_malformed_try_fallback_missing_fallback_body_txt, coverage_focus = empty_fallback_body_structural_malformed_floor, expected_static_verdict = malformed, expected_runtime_outcome = not_evaluated }
  ]
}
```

## practical reading

current minimal mapping matrix が示すのは、

- sample stem を軸に 3 層対応を current initial cluster 6 本へ固定する
- representative layer の direct / variant / unresolved status を minimum に残す
- source target path と expected verdict / outcome を next lowering / runner line へ渡す
- reached stage / bless policy はまだ minimum に含めない

という cut である。

## next promoted line

next promoted line は、
**representative-fixture-source-mapping-matrix-ready actual-parser-to-Program-lowering-first-cut comparison**
に置く。

## open questions

- `e23` representative anchor を later prose row に寄せるか
- source sample file authoring を lowering task と runner task のどちらで揃えるか
- verification ladder inventory をどの row schema で固定するか
