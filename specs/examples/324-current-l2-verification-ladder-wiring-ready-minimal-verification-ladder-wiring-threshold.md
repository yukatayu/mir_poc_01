# 324 — current L2 verification-ladder-wiring-ready minimal-verification-ladder-wiring threshold

## 目的

`specs/examples/323-current-l2-syntax-backed-sample-runner-first-cut-ready-verification-ladder-wiring-comparison.md`
で verification ladder wiring の current first choice を fixed した次段として、

- verification ladder の minimum をどこまでに留めるか
- authored sample row と deferred source target row を minimum にどう残すか
- current top と next line をどこまで minimum に反映するか

を比較する。

ここで固定するのは
**current L2 verification-ladder-wiring-ready minimal-verification-ladder-wiring threshold**
であり、

- source-sample bless / regression update flow
- current authored set の widen
- concrete theorem/model-check tool binding

はまだ固定しない。

## 比較観点

1. current authored trio の reached stage を minimum で読めるか
2. source target only row を failure ではなく deferred authored set として残せるか
3. tool-neutral formal hook top と next policy line を guard に残せるか

## 比較対象

### 案 1. authored sample rows だけを持つ

#### 利点

- 軽い。

#### 欠点

- `e1` / `e3` / `e21` の current status が minimum に残らない。
- next authoring/policy line への handoff が弱い。

### 案 2. `ladder_kind + fixed_entry_criteria_refs + authored_sample_rows + deferred_source_target_rows + guard_refs` を持つ

#### 利点

- current authored trio と deferred authored set を minimum に同時に残せる。
- current top が tool-neutral formal hook であることを guard として読める。
- next authoring/policy line に必要な status を lossless に渡せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. update flow や concrete tool choice まで minimum に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- ladder threshold ではなく policy / tool pilot threshold を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `ladder_kind + fixed_entry_criteria_refs + authored_sample_rows + deferred_source_target_rows + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は reached-stage inventory を fixed する task であり、authoring flow と concrete tool choice は next package へ分けるべきである。
2. authored sample rows だけでは current initial cluster の deferred status を失う。
3. tool-neutral formal hook top と authored set guard を minimum に残す必要がある。

## current first choice shape

```text
source_sample_verification_ladder = {
  ladder_kind = current_l2_source_sample_verification_ladder_first_trio,
  fixed_entry_criteria_refs = [
    phase6_compile_ready_formal_hook,
    phase6_source_mapping_matrix,
    phase6_source_lowering_first_cut,
    phase6_source_runner_first_cut
  ],
  authored_sample_rows = [
    { sample_stem = e2_try_fallback, authored_status = source_authored, static_gate_stage = reached_valid_static_gate, interpreter_stage = reached_runtime_success, formal_hook_stage = reached_runtime_try_cut_cluster, evidence_refs = [current_l2_source_sample_runner_test_e2, current_l2_source_sample_verification_ladder_test_e2, current_l2_formal_hook_runtime_smoke_e2] },
    { sample_stem = e4_malformed_lineage, authored_status = source_authored, static_gate_stage = reached_malformed_static_gate, interpreter_stage = not_reached_static_stop, formal_hook_stage = reached_fixture_static_cluster, evidence_refs = [current_l2_source_sample_runner_test_e4, current_l2_source_sample_verification_ladder_test_e4, current_l2_formal_hook_static_smoke_e4] },
    { sample_stem = e23_malformed_try_fallback_missing_fallback_body, authored_status = source_authored, static_gate_stage = reached_malformed_static_gate, interpreter_stage = not_reached_static_stop, formal_hook_stage = reached_fixture_static_cluster, evidence_refs = [current_l2_source_sample_runner_test_e23, current_l2_source_sample_verification_ladder_test_e23, current_l2_formal_hook_static_smoke_e23] }
  ],
  deferred_source_target_rows = [
    { sample_stem = e1_place_atomic_cut, authored_status = source_target_only_not_yet_authored },
    { sample_stem = e3_option_admit_chain, authored_status = source_target_only_not_yet_authored },
    { sample_stem = e21_try_atomic_cut_frontier, authored_status = source_target_only_not_yet_authored }
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_current_top,
    keep_first_trio_as_current_authored_set,
    avoid_authoring_policy_premature_merge,
    avoid_concrete_tool_binding_backpressure
  ]
}
```

## practical reading

current minimal verification ladder wiring が示すのは、

- current authored sample rows は first trio に留める
- current initial cluster の残り 3 本は deferred authored set に残す
- current top は tool-neutral formal hook である
- next line は source-sample authoring / bless / regression policy に置く

という最小 cut である。

## next promoted line

next promoted line は、
**verification-ladder-wiring-ready source-sample-authoring-bless-regression-policy comparison**
に置く。

## open questions

- authoring/policy task で update flow と regression command をどこまで minimum に入れるか
- deferred authored set を widen するとき、accepted sample surface と README matrix をどの task で同期するか
- theorem-first concrete pilot へ formal hook row family をどこまで渡すか
