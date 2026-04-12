# 317 — current L2 fixed-subset-source-sample-corpus-scope-and-file-layout-ready representative-fixture-source-mapping-matrix comparison

## 目的

`specs/examples/315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
と
`specs/examples/316-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-minimal-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold.md`
で source-sample corpus の scope / path / naming を fixed した次段として、

- representative prose
- machine-readable fixture corpus
- fixed-subset source sample

の 3 層を、どの matrix shape で 1 表に束ねるかを比較する。

ここで固定するのは
**current L2 fixed-subset-source-sample-corpus-scope-and-file-layout-ready representative-fixture-source-mapping-matrix comparison**
であり、

- actual source file content authoring
- actual parser-to-`Program` lowering
- sample ごとの reached stage inventory

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...316` で fixed 済みの source-corpus scope / layout とする。
- root source は `specs/examples/00...02`、fixture corpus、`samples/current-l2/README.md`、`plan/08` を主に扱う。
- current initial cluster は `e1` / `e2` / `e3` / `e4` / `e21` / `e23` に留める。

## current 前提

current repo では次が成立している。

1. representative prose は explanation-first layer であり、parser-ready final grammar 正本ではない。
2. fixture corpus は machine-check baseline であり、`fixture_id` / `source_example_id` を already 持つ。
3. source-sample corpus は repo-root `samples/current-l2/` flat `.txt` layer として fixed 済みだが、actual file content authoring はまだ始めていない。
4. `e3-option-admit-chain` fixture は `source_example_id = E3-variant` を持ち、plain `E3` と同一ではない。
5. `e23-malformed-try-fallback-missing-fallback-body` fixture は `source_example_id = E23` を already 持つが、`specs/examples/00-representative-mir-programs.md` には current representative prose row がない。

したがって current 問いは、
**sample stem / representative anchor / fixture identity / source target path を 1 row に束ねる minimum matrix をどこまで fixed するか**
である。

## 比較観点

1. representative prose / fixture corpus / source target の 3 層責務を崩さないか
2. `e3` variant と `e23` unresolved anchor を silent repair せず表せるか
3. next task の lowering / runner に自然に渡せるか
4. reached stage inventory や bless policy を premature に混ぜないか

## 比較対象

### 案 1. `fixture_id + source_sample_target_ref` だけを持つ

#### 利点

- 軽い。

#### 欠点

- representative prose との関係が見えない。
- `e3` variant と `e23` unresolved anchor を区別しにくい。

### 案 2. `ladder_order + sample_stem + representative_anchor_ref + representative_status + source_example_id + fixture_ref + fixture_id + fixture_mode + source_sample_target_ref + coverage_focus + expected_static_verdict + expected_runtime_outcome` を持つ

#### shape

```text
representative_fixture_source_mapping_matrix = {
  matrix_kind = current_l2_fixed_subset_representative_fixture_source_mapping,
  rows = [
    {
      ladder_order = 1,
      sample_stem = e1_place_atomic_cut,
      representative_anchor_ref = representative_program_E1,
      representative_status = direct,
      source_example_id = E1,
      fixture_ref = fixture_e1_place_atomic_cut,
      fixture_id = e1_place_atomic_cut,
      fixture_mode = runtime_fixture,
      source_sample_target_ref = samples_current_l2_e1_place_atomic_cut_txt,
      coverage_focus = place_atomic_cut_post_cut_failure,
      expected_static_verdict = valid,
      expected_runtime_outcome = explicit_failure
    },
    {
      ladder_order = 2,
      sample_stem = e2_try_fallback,
      representative_anchor_ref = representative_program_E2,
      representative_status = direct,
      source_example_id = E2,
      fixture_ref = fixture_e2_try_fallback,
      fixture_id = e2_try_fallback,
      fixture_mode = runtime_fixture,
      source_sample_target_ref = samples_current_l2_e2_try_fallback_txt,
      coverage_focus = local_try_fallback_rollback,
      expected_static_verdict = valid,
      expected_runtime_outcome = success
    },
    {
      ladder_order = 3,
      sample_stem = e3_option_admit_chain,
      representative_anchor_ref = representative_program_E3_variant,
      representative_status = variant,
      source_example_id = E3_variant,
      fixture_ref = fixture_e3_option_admit_chain,
      fixture_id = e3_option_admit_chain,
      fixture_mode = runtime_fixture,
      source_sample_target_ref = samples_current_l2_e3_option_admit_chain_txt,
      coverage_focus = option_local_admit_non_admissible_skip,
      expected_static_verdict = valid,
      expected_runtime_outcome = success
    },
    {
      ladder_order = 4,
      sample_stem = e4_malformed_lineage,
      representative_anchor_ref = representative_program_E4,
      representative_status = direct,
      source_example_id = E4,
      fixture_ref = fixture_e4_malformed_lineage,
      fixture_id = e4_malformed_lineage,
      fixture_mode = static_only_fixture,
      source_sample_target_ref = samples_current_l2_e4_malformed_lineage_txt,
      coverage_focus = edge_local_lineage_mismatch_static_stop,
      expected_static_verdict = malformed,
      expected_runtime_outcome = not_evaluated
    },
    {
      ladder_order = 5,
      sample_stem = e21_try_atomic_cut_frontier,
      representative_anchor_ref = representative_program_E21,
      representative_status = direct,
      source_example_id = E21,
      fixture_ref = fixture_e21_try_atomic_cut_frontier,
      fixture_id = e21_try_atomic_cut_frontier,
      fixture_mode = runtime_fixture,
      source_sample_target_ref = samples_current_l2_e21_try_atomic_cut_frontier_txt,
      coverage_focus = try_body_atomic_cut_frontier_update,
      expected_static_verdict = valid,
      expected_runtime_outcome = success
    },
    {
      ladder_order = 6,
      sample_stem = e23_malformed_try_fallback_missing_fallback_body,
      representative_anchor_ref = unresolved_representative_anchor,
      representative_status = unresolved,
      source_example_id = E23,
      fixture_ref = fixture_e23_malformed_try_fallback_missing_fallback_body,
      fixture_id = e23_malformed_try_fallback_missing_fallback_body,
      fixture_mode = static_only_fixture,
      source_sample_target_ref = samples_current_l2_e23_malformed_try_fallback_missing_fallback_body_txt,
      coverage_focus = empty_fallback_body_structural_malformed_floor,
      expected_static_verdict = malformed,
      expected_runtime_outcome = not_evaluated
    }
  ]
}
```

#### 利点

- 3 層の責務を 1 row に分けて残せる。
- `e3` は `variant`、`e23` は `unresolved` として explicit に表せる。
- lowering / runner に渡したい coverage / verdict expectation も narrow に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. reached stage / bless policy / owner notes まで matrix に含める

#### 利点

- 後段 task との繋がりは見えやすい。

#### 欠点

- mapping matrix task を越えて verification ladder / authoring policy threshold を先取りする。
- Task 4 / Task 5 の boundary を壊す。

## current judgment

current L2 で最も自然なのは、
**案 2. `ladder_order + sample_stem + representative_anchor_ref + representative_status + source_example_id + fixture_ref + fixture_id + fixture_mode + source_sample_target_ref + coverage_focus + expected_static_verdict + expected_runtime_outcome` を持つ**
である。

理由は次の通り。

1. representative prose / fixture corpus / source target の 3 層対応を current task で明示したい。
2. `e3` variant と `e23` unresolved anchor を silent repair せず表すには `representative_status` が要る。
3. reached stage と bless policy は next task chain へ押し出したままでも、coverage / verdict expectation までは mapping row に残してよい。

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

current mapping matrix judgment が示すのは、次の 6 点である。

1. matrix row は sample stem を軸に 3 層を束ねる
2. representative anchor と fixture identity は別 field に残す
3. `e3` は `E3-variant` として扱う
4. `e23` は `source_example_id = E23` を保ちつつ representative anchor unresolved にする
5. source sample は current task では target path ref として扱い、actual file content はまだ要求しない
6. reached stage / bless policy は Task 4 / Task 5 に押し出す

## next promoted line

next promoted line は、
**representative-fixture-source-mapping-matrix-ready actual-parser-to-Program-lowering-first-cut comparison**
に置く。

## open questions

- `e23` representative anchor を later に prose row へ actualize するか
- `e1` / `e3` / `e21` sample file content を lowering first cut のどの task で authoring するか
- reached stage inventory を Task 4 でどの schema に mirror するか
