# 350 — current L2 second-source-sample-cluster-sequencing-ready minimal-second-source-sample-cluster-sequencing threshold

## 目的

`specs/examples/349-current-l2-proof-model-check-first-concrete-tool-pilot-ready-second-source-sample-cluster-sequencing-comparison.md`
で second source-sample cluster sequencing の current first choice を fixed した次段として、

- first post-sextet cluster minimum をどこまでに留めるか
- source / fixture / formal-hook / regression の 4 層接続を minimum にどう残すか
- next `e22` actualization line と broader follow-up cluster をどう handoff するか

を比較する。

ここで固定するのは
**current L2 second-source-sample-cluster-sequencing-ready minimal-second-source-sample-cluster-sequencing threshold**
であり、

- actual `e22` source file
- runner accepted set widening
- stable static malformed subcluster sequencing
- public surface inventory

はまだ固定しない。

## 比較観点

1. first post-sextet cluster を minimum に残せるか
2. source / fixture / formal-hook / regression の 4 層接続を minimum に保てるか
3. `e22` actualization と broader follow-up cluster へ narrow に handoff できるか

## 比較対象

### 案 1. cluster 名と sample stem だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current formal-hook guard と next actualization route が弱い。

### 案 2. `sequencing_kind + fixed_entry_criteria_refs + selected_cluster_rows + guard_refs + kept_later_refs` を持つ

#### 利点

- current post-sextet first cluster と next `e22` actualization line を lossless に残せる。
- source / fixture / formal-hook / regression の 4 層を narrow に同期できる。
- broader follow-up cluster と public surface inventory を threshold 側から still later に押し分けられる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. `e22` actualization や post-contrast follow-up cluster まで minimum に含める

#### 利点

- 次段との接続は見えやすい。

#### 欠点

- threshold ではなく later actualization / broader sequencing を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `sequencing_kind + fixed_entry_criteria_refs + selected_cluster_rows + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current task は first post-sextet cluster の sequencing close であり、actual widening は次段へ残すべきである。
2. `e22` は next actualization target だが、current key judgment は cluster family を narrow に決めることにある。
3. broader follow-up cluster と public surface inventory は threshold 側で kept-later として明示するのが最小である。

## current first choice shape

```text
second_source_sample_cluster = {
  sequencing_kind = current_l2_first_post_sextet_runtime_contrast_cluster,
  fixed_entry_criteria_refs = [
    phase6_actual_e3_authored_row,
    phase6_proof_model_check_first_concrete_tool_pilot
  ],
  selected_cluster_rows = [
    {
      sample_stem = e21_try_atomic_cut_frontier,
      current_status = already_source_authored,
      fixture_anchor_ref = current_runtime_contrast_anchor,
      formal_hook_status = reached_runtime_try_cut_cluster
    },
    {
      sample_stem = e22_try_atomic_cut_place_mismatch,
      current_status = deferred_target_only,
      fixture_anchor_ref = current_runtime_contrast_pair,
      next_actualization_kind = next_source_authored_runtime_contrast_row
    }
  ],
  guard_refs = [
    keep_runtime_try_cut_cluster_as_current_formal_hook_top,
    keep_e3_guarded_non_reached_row_unchanged,
    defer_expiry_and_request_contract_families,
    defer_stable_static_malformed_followup_cluster
  ],
  kept_later_refs = [
    actual_e22_contrast_row_actualization,
    stable_static_malformed_post_contrast_cluster,
    parser_checker_runtime_public_surface_inventory,
    model_check_public_checker_second_reserve_inventory
  ]
}
```

## practical reading

current minimal second-source-sample cluster sequencing が示すのは、

- first post-sextet cluster は `e21` / `e22` try-rollback locality contrast に置く
- next actual source row は `e22` に置く
- current formal-hook top は `runtime_try_cut_cluster` のまま維持する
- broader runtime family widening、static malformed broader cluster、public surface inventory は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**second-source-sample-cluster-sequencing-ready actual-e22-contrast-row-actualization comparison**
に置く。

## open questions

- `e22` source row の exact textual shape を helper-compatible subset にどこまで寄せるか
- `e22` actualization の直後に stable static malformed family のどこを second broader cluster に置くか
- public surface inventory を Macro 4 widening のどこで差し込むか
