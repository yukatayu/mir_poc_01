# 326 — current L2 source-sample-authoring-bless-regression-policy-ready minimal-source-sample-authoring-bless-regression-policy threshold

## 目的

`specs/examples/325-current-l2-verification-ladder-wiring-ready-source-sample-authoring-bless-regression-policy-comparison.md`
で source-sample authoring / bless / regression policy の current first choice を fixed した次段として、

- policy minimum をどこまでに留めるか
- current authored set / deferred set / regression flow を minimum にどう残すか
- next theorem-first pilot line をどこまで minimum に反映するか

を比較する。

ここで固定するのは
**current L2 source-sample-authoring-bless-regression-policy-ready minimal-source-sample-authoring-bless-regression-policy threshold**
であり、

- deferred authored row widen
- final public CLI
- retained artifact bless / archive policy
- concrete theorem/model-check tool binding

はまだ固定しない。

## 比較観点

1. first authored trio と deferred row を minimum に同時に残せるか
2. authoring flow と regression flow を minimum で読めるか
3. bless を repo-local reviewed sync に留め、後段 policy と混ぜないか

## 比較対象

### 案 1. regression helper ref だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- authored/deferred boundary と manual authoring steps が minimum から落ちる。

### 案 2. `policy_kind + authored_rows + deferred_rows + authoring_step_refs + regression_step_refs + bless_reading + guard_refs` を持つ

#### 利点

- current authored/deferred boundary を lossless に渡せる。
- manual review と repo-local regression helper の split を minimum に残せる。
- next theorem-first pilot line へ明確に handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. public CLI / retained artifact fields まで minimum に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- current threshold ではなく future reopen line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `policy_kind + authored_rows + deferred_rows + authoring_step_refs + regression_step_refs + bless_reading + guard_refs` を持つ**
である。

理由は次の通り。

1. current task の本体は source-sample path の repo-local運用を narrow に閉じることであり、authored/deferred boundary と step refs を minimum に残す必要がある。
2. public CLI や retained artifact policy は still later であり、minimum に混ぜるべきではない。
3. next line は theorem-first concrete tool pilot なので、policy minimum ではそこへの guard/handoff だけがあればよい。

## current first choice shape

```text
source_sample_authoring_policy = {
  policy_kind = current_l2_first_trio_repo_local_authoring_policy,
  authored_rows = [
    e2_try_fallback,
    e4_malformed_lineage,
    e23_malformed_try_fallback_missing_fallback_body
  ],
  deferred_rows = [
    e1_place_atomic_cut,
    e3_option_admit_chain,
    e21_try_atomic_cut_frontier
  ],
  authoring_step_refs = [
    update_source_text,
    keep_fixture_mapping_aligned,
    keep_readme_matrix_aligned,
    keep_ladder_row_aligned,
    record_report_and_snapshot_sync
  ],
  regression_step_refs = [
    current_l2_source_sample_inventory_check,
    current_l2_source_lowering_test,
    current_l2_source_sample_runner_test,
    current_l2_source_sample_verification_ladder_test,
    current_l2_formal_hook_support_test,
    current_l2_formal_hook_smoke_first_trio
  ],
  bless_reading = reviewed_repo_local_sync_without_retained_artifact_policy,
  guard_refs = [
    keep_first_trio_as_current_authored_set,
    avoid_public_cli_premature_surface,
    avoid_retained_artifact_policy_premature_merge,
    handoff_to_theorem_first_concrete_tool_pilot
  ]
}
```

## practical reading

current minimal source-sample authoring policy が示すのは、

- current authored rows は first trio に留める
- deferred row は target-only / not-yet-authored のまま残す
- authoring は manual review を主にし、regression は thin helper で束ねる
- bless は repo-local reviewed sync と regression success の確認に留める
- next line は theorem-first concrete tool pilot に置く

という最小 cut である。

## next promoted line

next promoted line は、
**source-sample-authoring-bless-regression-policy-ready theorem-first-concrete-tool-pilot comparison**
に置く。

## open questions

- widen task で authored row と regression helper accepted set をどう同期するか
- retained detached artifact bless policy をどの reopen line で actualize するか
- proof notebook / theorem-first pilot が source-sample policy に要求する metadata が増えるか
