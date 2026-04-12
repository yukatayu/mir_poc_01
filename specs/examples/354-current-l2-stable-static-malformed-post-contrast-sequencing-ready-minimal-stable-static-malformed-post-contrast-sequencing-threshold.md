# 354 — current L2 stable-static-malformed-post-contrast-sequencing-ready minimal-stable-static-malformed-post-contrast-sequencing threshold

## 目的

`specs/examples/353-current-l2-actual-e22-contrast-row-source-actualization-ready-stable-static-malformed-post-contrast-sequencing-comparison.md`
で post-contrast static malformed sequencing の current first choice を fixed した次段として、

- second broader cluster sequencing の minimum をどこまでに留めるか
- stable cluster / duplicate cluster / try-rollback malformed-static line の境界をどう残すか
- repo-level next package と Macro 4 next reopen point をどう handoff するか

を比較する。

ここで固定するのは
**current L2 stable-static-malformed-post-contrast-sequencing-ready minimal-stable-static-malformed-post-contrast-sequencing threshold**
であり、

- `e4/e19` first reopen actualization
- parser / checker / runtime public surface inventory の具体形
- duplicate cluster の actual promotion

はまだ固定しない。

## 比較観点

1. sequencing close を minimum に残せるか
2. stable vs duplicate vs try/rollback malformed-static の境界を明示できるか
3. next repo-level package と later Macro 4 reopen point を両立して handoff できるか

## 比較対象

### 案 1. cluster 名と next line だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- `e4/e19` first reopen point や excluded families が抜け、later reopen 条件が弱い。

### 案 2. `sequencing_kind + entry_criteria_refs + selected_cluster_refs + selected_first_reopen_rows + guard_refs + kept_later_refs` を持つ

#### 利点

- stable reason-code cluster選択、`e4/e19` first reopen point、excluded families、repo-level next packageを lossless に残せる。
- duplicate cluster と try/rollback malformed-static family を still later として明示できる。
- public surface inventory と stable-static edge-pair reopen の両方に handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. `e4/e19` actualization や public surface inventory の shape まで minimum に含める

#### 利点

- 次段は見えやすい。

#### 欠点

- threshold ではなく later package を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `sequencing_kind + entry_criteria_refs + selected_cluster_refs + selected_first_reopen_rows + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は sequencing close であり、actual reopen は次段へ残すべきである。
2. `e4/e19` first reopen point と excluded families を threshold 側で残さないと、stable cluster selection の意味が薄くなる。
3. repo-level next line は public surface inventoryだが、Macro 4 next reopen point は stable-static edge-pair sideに残るため、両方を handoff する field が必要である。

## current first choice shape

```text
stable_static_post_contrast = {
  sequencing_kind = current_l2_post_contrast_stable_reason_code_fixture_static_cluster,
  entry_criteria_refs = [
    phase6_actual_e22_contrast_row,
    current_stable_reason_code_inventory
  ],
  selected_cluster_refs = [
    stable_reason_code_fixture_static_cluster
  ],
  selected_first_reopen_rows = [
    e4_malformed_lineage,
    e19_malformed_target_mismatch
  ],
  guard_refs = [
    keep_duplicate_cluster_outside_stable_inventory,
    keep_try_rollback_malformed_static_family_later,
    keep_broader_runtime_families_later
  ],
  kept_later_refs = [
    parser_checker_runtime_public_surface_inventory,
    stable_static_edge_pair_first_reopen,
    mirrorea_shared_space_docs_first_reentry,
    model_check_public_checker_second_reserve_inventory
  ]
}
```

## practical reading

current minimal post-contrast sequencing が示すのは、

- `e22` の次段は stable reason-code / fixture-static cluster に置く
- その first reopen point は `e4` / `e19` edge-pair sideに置く
- duplicate cluster と try/rollback malformed-static family は still later に残す
- repo-level next package は public surface inventory に進む

という最小 cut である。

## next promoted line

next promoted line は、
**stable-static-malformed-post-contrast-sequencing-ready parser-checker-runtime-public-surface-inventory comparison**
に置く。

## open questions

- `e4/e19` first reopen のあとに missing-option / capability family をどう束ねるか
- stable-static edge-pair reopen と public surface inventory の後先をいつ再評価するか
- duplicate cluster を長期的に stable source-of-truth line へ接続するか
