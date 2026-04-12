# 352 — current L2 actual-e22-contrast-row-source-actualization-ready minimal-actual-e22-contrast-row threshold

## 目的

`specs/examples/351-current-l2-second-source-sample-cluster-sequencing-ready-actual-e22-contrast-row-source-actualization-comparison.md`
で `e22` actualization の current first choice を fixed した次段として、

- actual `e22` package の minimum をどこまでに留めるか
- source row / bridge extension / reached-stage をどう一体で残すか
- broader static malformed cluster と public surface inventory をどう handoff するか

を比較する。

ここで固定するのは
**current L2 actual-e22-contrast-row-source-actualization-ready minimal-actual-e22-contrast-row threshold**
であり、

- stable static malformed subcluster selection
- final public parser / checker / runtime API
- concrete theorem / model-check tool binding

はまだ固定しない。

## 比較観点

1. `e22` actualization を minimum に残せるか
2. nested `place` block 受理の helper-local extension を過不足なく残せるか
3. reached-stage と kept-later line を narrow に handoff できるか

## 比較対象

### 案 1. source file path と reached-stage だけを残す

#### 利点

- 軽い。

#### 欠点

- stage 2 bridge extension が抜け、`e22` がなぜ current source path で通るのかが失われる。

### 案 2. `actualization_kind + entry_criteria_refs + actualized_row + bridge_extension_refs + reached_stage_refs + kept_later_refs` を持つ

#### 利点

- source row、current parser/runtime bridge extension、reached-stage、next broader cluster を lossless に残せる。
- helper-local extension と final public parser API defer を同時に明示できる。
- stable static malformed post-contrast sequencing と public surface inventoryへ narrow に handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. stable static malformed sequencing や public surface inventory まで minimum に含める

#### 利点

- next line は見えやすい。

#### 欠点

- threshold ではなく later package を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + actualized_row + bridge_extension_refs + reached_stage_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. `e22` actualization の key change は source row だけでなく stage 2 bridge extension にある。
2. `e22` は current first post-sextet runtime contrast pair を source-backed に閉じる row であり、reached-stage を明示する価値が高い。
3. stable static malformed sequencing と public surface inventory は明確な next package であり、threshold 側では kept-later に残すのが最小である。

## current first choice shape

```text
e22_actualization = {
  actualization_kind = current_l2_runtime_contrast_row_source_actualization,
  entry_criteria_refs = [
    phase6_current_authored_rows_e1_e21_e3,
    phase6_second_source_sample_cluster_sequencing
  ],
  actualized_row = {
    sample_stem = e22_try_atomic_cut_place_mismatch,
    source_status = source_authored,
    runner_status = accepted_source_sample,
    regression_status = included_in_authored_inventory
  },
  bridge_extension_refs = [
    keep_current_stage2_try_rollback_floor,
    fold_nested_place_block_as_top_level_other_statement_head,
    defer_final_public_parser_contract
  ],
  reached_stage_refs = [
    static_gate_reached_valid,
    interpreter_reached_success,
    runtime_formal_hook_reached_runtime_try_cut_cluster
  ],
  kept_later_refs = [
    stable_static_malformed_post_contrast_sequencing,
    parser_checker_runtime_public_surface_inventory,
    model_check_public_checker_second_reserve_inventory
  ]
}
```

## practical reading

current minimal `e22` actualization が示すのは、

- `e22-try-atomic-cut-place-mismatch` は current authored septet に入った
- current helper-local source path では nested `place` block を stage 2 bridge の top-level `Other` として受ける
- `e22` は `runtime_try_cut_cluster` formal-hook top の内側に留まる
- broader static malformed family と public surface inventory は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**actual-e22-contrast-row-source-actualization-ready stable-static-malformed-post-contrast-sequencing comparison**
に置く。

## open questions

- stable static malformed family で first reopen row を `e4/e19` typed reason-code sideから切るか
- public surface inventory で parser-free public helper stack と parser/checker/runtime helper-local line をどう二層化するか
- `e22` stage 2 bridge extension を final public parser surface へ上げるべきか、それとも helper-local floor のまま維持するべきか
