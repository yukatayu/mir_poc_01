# 172 — current L2 theorem line attachment-blob-ready retained-file-body threshold

## 目的

`specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`
までを前提に、

- attachment-blob-ready retained bridge に actual retained file body family をどこまで足すか
- actual archive materialization をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の attachment-blob-ready retained-file-body threshold** であり、

- actual archive materialization

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current attachment-blob-ready retained bridge を起点にする。
- detached validation loop の archive retention policy や bless/update policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare / review / retention / path / emitter / adapter / invocation / exchange / runtime / transport / failure / host binding / wording / runtime handoff / emitted receipt / runtime transcript / materialized runtime handoff / concrete payload / concrete transcript body / serialized channel body / emitted attachment body / emitted attachment blob は current first choice に入っている。
2. actual retained file body family / actual archive materialization は still 後段に残している。

したがって current 問いは、
**attachment-blob-ready retained bridge に `retained_file_body_ref` だけを先に足してよいか、それとも actual archive materialization とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. emitted attachment blob family と retained file body family の line を narrow に切れるか
2. retained file body family と archive materialization を分離できるか
3. actual archive materialization を theorem-line bridge に premature に混ぜないか
4. next later reopen を archive materialization comparison へ narrow に進めるか

## 比較対象

### 案 1. attachment-blob-ready retained bridge を terminal cut にし、retained file body も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- archive materialization との混線をさらに避けられる

#### 欠点

- emitted attachment blob family と retained file body family の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. retained-file-body ref だけを持つ retained-file-body-ready retained bridge にする

#### 読み

attachment-blob-ready retained bridge に、
actual archive materialization を入れずに
**`retained_file_body_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_file_body_ready_sketch = {
  proof_notebook_bridge_attachment_blob_ready_sketch,
  retained_file_body_ref
}
```

#### 利点

- emitted attachment blob family と retained file body family の line を narrow に橋渡しできる
- actual archive materialization を still 後段に残せる
- next later reopen を archive materialization comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- retained file body family と archive materialization を誤読されない説明が要る

### 案 3. retained file body と archive materialization をまとめて入れる

#### 利点

- notebook consumer が retained file line と archive line を一度に追える
- archive / bless / retention pressure に近づく

#### 欠点

- retained file body family と archive materialization を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. retained-file-body ref だけを持つ retained-file-body-ready retained bridge にする**
である。

理由は次の通り。

1. emitted attachment blob family と retained file body family の line を narrow に接続できる
2. actual archive materialization を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を archive materialization comparison へ狭く進めやすい

## minimal retained-file-body-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_file_body_ready_sketch = {
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
  concrete_payload_ref,
  concrete_transcript_body_ref,
  serialized_channel_body_ref,
  emitted_attachment_body_ref,
  emitted_attachment_blob_ref,
  retained_file_body_ref
}
```

### `retained_file_body_ref`

`retained_file_body_ref` は、

- emitted attachment blob family の次段で
- notebook consumer が **retained file body family** に戻れる
  symbolic retained-file-body ref

だけを置く field である。

current task では、

- actual archive materialization

をここに入れない。

## なぜ actual archive materialization をまだ入れないか

actual archive materialization を current phase で入れると、

- retained file body family
- archive materialization
- bless / retention / archive policy

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず retained file body family を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-file-body-ready retained bridge

```text
proof_notebook_bridge_retained_file_body_ready_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の retained file body family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e12.attachment_blob_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e12.retained_file_body.observation
  ],
  retained_notebook_ref = retained_notebook:e12.latest,
  review_session_ref = review_session:e12.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T06:31,
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
  concrete_payload_ref = concrete_payload:proof_notebook.viewer.default,
  concrete_transcript_body_ref = concrete_transcript_body:proof_notebook.viewer.default,
  serialized_channel_body_ref = serialized_channel_body:proof_notebook.viewer.default,
  emitted_attachment_body_ref = emitted_attachment_body:proof_notebook.viewer.default,
  emitted_attachment_blob_ref = emitted_attachment_blob:proof_notebook.viewer.default,
  retained_file_body_ref = retained_file_body:proof_notebook.viewer.default
}
```

### example B — runtime try/cut retained-file-body-ready retained bridge

```text
proof_notebook_bridge_retained_file_body_ready_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.write_after_expiry,
    review_unit:e6.try_cut_locality
  ],
  bridge_goal_text = "runtime retained file body family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e6.attachment_blob_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e6.retained_file_body.observation
  ],
  retained_notebook_ref = retained_notebook:e6.latest,
  review_session_ref = review_session:e6.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T06:31,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e6.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.viewer.default,
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
  concrete_payload_ref = concrete_payload:proof_notebook.viewer.default,
  concrete_transcript_body_ref = concrete_transcript_body:proof_notebook.viewer.default,
  serialized_channel_body_ref = serialized_channel_body:proof_notebook.viewer.default,
  emitted_attachment_body_ref = emitted_attachment_body:proof_notebook.viewer.default,
  emitted_attachment_blob_ref = emitted_attachment_blob:proof_notebook.viewer.default,
  retained_file_body_ref = retained_file_body:proof_notebook.viewer.default
}
```

## deferred line

current task では次を still 後段に残す。

- actual archive materialization
- actual bless/update action for retained file replacement
- detached validation loop の final bless / retention policy

## summary

- current first choice は、attachment-blob-ready retained bridge に `retained_file_body_ref` だけを足す cut である。
- actual archive materialization は still theorem-line bridge 外に残す。
- next later reopen は archive materialization comparison である。
