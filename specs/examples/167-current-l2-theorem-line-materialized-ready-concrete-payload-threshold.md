# 167 — current L2 theorem line materialized-ready concrete-payload threshold

## 目的

`specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`
までを前提に、

- materialized-ready retained bridge に concrete payload family をどこまで足すか
- transcript body / actual serialized channel body をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の materialized-ready concrete-payload threshold** であり、

- concrete transcript body
- actual serialized channel body

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current materialized-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や notebook runner command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare / review / retention / path / emitter / adapter / invocation / exchange / runtime / transport / failure / host binding / wording / runtime handoff / emitted receipt / runtime transcript / materialized runtime handoff は current first choice に入っている。
2. concrete payload / concrete transcript body / actual serialized channel body は still 後段に残している。

したがって current 問いは、
**materialized-ready retained bridge に `concrete_payload_ref` だけを先に足してよいか、それとも transcript body / serialized channel body とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. materialized handoff fact と concrete payload family の line を narrow に切れるか
2. concrete payload family と transcript body を分離できるか
3. actual serialized channel body を theorem-line bridge に premature に混ぜないか
4. next later reopen を transcript body comparison へ narrow に進めるか

## 比較対象

### 案 1. materialized-ready retained bridge を terminal cut にし、payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- transcript body / serialized channel body との混線をさらに避けられる

#### 欠点

- materialized handoff fact と concrete payload family の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. concrete-payload ref だけを持つ payload-ready retained bridge にする

#### 読み

materialized-ready retained bridge に、
concrete transcript body / actual serialized channel body を入れずに
**`concrete_payload_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_payload_ready_sketch = {
  proof_notebook_bridge_materialized_ready_sketch,
  concrete_payload_ref
}
```

#### 利点

- materialized handoff fact と concrete payload family の line を narrow に橋渡しできる
- transcript body / actual serialized channel body を still 後段に残せる
- next later reopen を transcript body comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- payload family と transcript body を誤読されない説明が要る

### 案 3. concrete payload と transcript body をまとめて入れる

#### 利点

- notebook consumer に渡る materialized content を一度に語れる
- concrete notebook replay / inspection pressure に近づく

#### 欠点

- transcript body / serialized channel body を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. concrete-payload ref だけを持つ payload-ready retained bridge にする**
である。

理由は次の通り。

1. materialized handoff fact と concrete payload family の line を narrow に接続できる
2. transcript body / actual serialized channel body を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を transcript body comparison へ狭く進めやすい

## minimal payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_payload_ready_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref,
  reviewed_by_ref,
  reviewed_at_ref,
  review_session_state,
  retention_state,
  retained_path_policy_ref,
  emitted_artifact_ref,
  handoff_emitter_ref,
  consumer_adapter_ref,
  exchange_rule_ref,
  adapter_validation_ref,
  consumer_invocation_surface_ref,
  exchange_rule_body_ref,
  runtime_coupling_ref,
  transport_protocol_ref,
  failure_body_ref,
  actual_invocation_protocol_ref,
  consumer_host_binding_ref,
  failure_wording_ref,
  actual_runtime_handoff_ref,
  emitted_invocation_receipt_ref,
  runtime_transcript_ref,
  materialized_runtime_handoff_ref,
  concrete_payload_ref
}
```

### `concrete_payload_ref`

`concrete_payload_ref` は、

- materialized runtime handoff artifact の次段で
- notebook consumer が **payload family** に戻れる
  symbolic concrete-payload ref

だけを置く field である。

current task では、

- concrete transcript body
- actual serialized channel body

をここに入れない。

## なぜ transcript body をまだ入れないか

concrete transcript body を current phase で入れると、

- payload family
- transcript formatting / rendering body
- serialized channel body

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず concrete payload family を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain payload-ready retained bridge

```text
proof_notebook_bridge_payload_ready_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の concrete payload family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e12.materialized_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e12.payload.observation
  ],
  retained_notebook_ref = retained_notebook:e12.latest,
  review_session_ref = review_session:e12.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T04:08,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.viewer.default,
  adapter_validation_ref = adapter_validation:proof_notebook.viewer.default,
  consumer_invocation_surface_ref = consumer_invocation_surface:proof_notebook.viewer.default,
  exchange_rule_body_ref = exchange_rule_body:proof_notebook.viewer.default,
  runtime_coupling_ref = runtime_coupling:proof_notebook.viewer.default,
  transport_protocol_ref = transport_protocol:proof_notebook.viewer.default,
  failure_body_ref = failure_body:proof_notebook.viewer.default,
  actual_invocation_protocol_ref = actual_invocation_protocol:proof_notebook.viewer.default,
  consumer_host_binding_ref = consumer_host_binding:proof_notebook.viewer.default,
  failure_wording_ref = failure_wording:proof_notebook.viewer.default,
  actual_runtime_handoff_ref = actual_runtime_handoff:proof_notebook.viewer.default,
  emitted_invocation_receipt_ref = emitted_invocation_receipt:proof_notebook.viewer.default,
  runtime_transcript_ref = runtime_transcript:proof_notebook.viewer.default,
  materialized_runtime_handoff_ref = materialized_runtime_handoff:proof_notebook.viewer.default,
  concrete_payload_ref = concrete_payload:proof_notebook.viewer.default
}
```

### example B — runtime try/cut payload-ready retained bridge

```text
proof_notebook_bridge_payload_ready_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.write_after_expiry,
    review_unit:e6.try_cut_locality
  ],
  bridge_goal_text = "runtime payload family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e6.materialized_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e6.payload.observation
  ],
  retained_notebook_ref = retained_notebook:e6.latest,
  review_session_ref = review_session:e6.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T04:08,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e6.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.followup,
  exchange_rule_ref = exchange_rule:proof_notebook.followup.default,
  adapter_validation_ref = adapter_validation:proof_notebook.followup.default,
  consumer_invocation_surface_ref = consumer_invocation_surface:proof_notebook.followup.default,
  exchange_rule_body_ref = exchange_rule_body:proof_notebook.followup.default,
  runtime_coupling_ref = runtime_coupling:proof_notebook.followup.default,
  transport_protocol_ref = transport_protocol:proof_notebook.followup.default,
  failure_body_ref = failure_body:proof_notebook.followup.default,
  actual_invocation_protocol_ref = actual_invocation_protocol:proof_notebook.followup.default,
  consumer_host_binding_ref = consumer_host_binding:proof_notebook.followup.default,
  failure_wording_ref = failure_wording:proof_notebook.followup.default,
  actual_runtime_handoff_ref = actual_runtime_handoff:proof_notebook.followup.default,
  emitted_invocation_receipt_ref = emitted_invocation_receipt:proof_notebook.followup.default,
  runtime_transcript_ref = runtime_transcript:proof_notebook.followup.default,
  materialized_runtime_handoff_ref = materialized_runtime_handoff:proof_notebook.followup.default,
  concrete_payload_ref = concrete_payload:proof_notebook.followup.default
}
```

## deferred

current task では次を fixed judgment に上げない。

- concrete transcript body
- actual serialized channel body
- concrete payload / transcript body / serialized body の single bundled field

## summary

- materialized-ready retained bridge の次段として `concrete_payload_ref` までは current first choice で入れてよい
- concrete transcript body と actual serialized channel body はまだ theorem-line bridge に入れない
- 次の later reopen は、concrete transcript body をどこまで theorem-line bridge に寄せるかの comparison に置くのが自然である

## OPEN

- `concrete_payload_ref` を payload family 単位に留めるか、proof-side consumer ごとに finer split するか
- concrete transcript body を separate ref にするか、payload family attachment とみなすか
- actual serialized channel body を transcript family の一部とみなすか、transport artifact family とみなすか
