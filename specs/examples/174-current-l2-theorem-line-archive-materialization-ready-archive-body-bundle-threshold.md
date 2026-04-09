# 174 — current L2 theorem line archive-materialization-ready archive-body-bundle threshold

## 目的

`specs/examples/173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md`
までを前提に、

- archive-materialization-ready retained bridge に actual archive body / bundle family をどこまで足すか
- actual archive bundle family をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-materialization-ready archive-body-bundle threshold** であり、

- actual archive bundle family

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-materialization-ready retained bridge を起点にする。
- actual archive retention policy、bundle retention layout、bless/update policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. retained file body family と archive materialization family までは current first choice に入っている。
2. actual archive body / bundle family は still 後段に残している。

したがって current 問いは、
**archive-materialization-ready retained bridge に `archive_body_ref` だけを先に足してよいか、それとも actual archive bundle family とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. archive materialization family と actual archive body family の line を narrow に切れるか
2. actual archive body family と actual archive bundle family を分離できるか
3. actual archive bundle family を theorem-line bridge に premature に混ぜないか
4. next later reopen を archive bundle comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-materialization-ready retained bridge を terminal cut にし、actual archive body も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual archive bundle family との混線をさらに避けられる

#### 欠点

- archive materialization family と actual archive body family の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. actual archive body ref だけを持つ archive-body-ready retained bridge にする

#### 読み

archive-materialization-ready retained bridge に、
actual archive bundle family を入れずに
**`archive_body_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_archive_body_ready_sketch = {
  proof_notebook_bridge_archive_materialization_ready_sketch,
  archive_body_ref
}
```

#### 利点

- archive materialization family と actual archive body family の line を narrow に橋渡しできる
- actual archive bundle family を still 後段に残せる
- next later reopen を archive bundle comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- archive body family と archive bundle family を誤読されない説明が要る

### 案 3. actual archive body と actual archive bundle family をまとめて入れる

#### 利点

- notebook consumer が actual archive family を一度に追える
- archive retrieval / bless / retention pressure に近づく

#### 欠点

- actual archive body family と actual archive bundle family を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. actual archive body ref だけを持つ archive-body-ready retained bridge にする**
である。

理由は次の通り。

1. archive materialization family と actual archive body family の line を narrow に接続できる
2. actual archive bundle family を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を archive bundle comparison へ狭く進めやすい

## minimal archive-body-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_archive_body_ready_sketch = {
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
  retained_file_body_ref,
  archive_materialization_ref,
  archive_body_ref
}
```

### `archive_body_ref`

`archive_body_ref` は、

- archive materialization family の次段で
- notebook consumer が **actual archive body family** に戻れる
  symbolic archive-body ref

だけを置く field である。

current task では、

- actual archive bundle family

をここに入れない。

## なぜ actual archive bundle family をまだ入れないか

actual archive bundle family を current phase で入れると、

- actual archive body family
- actual archive bundle family
- retention layout / bundle manifest / bless policy

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual archive body family を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain archive-body-ready retained bridge

```text
proof_notebook_bridge_archive_body_ready_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の actual archive body family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e12.archive_materialization_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e12.archive_body.observation
  ],
  retained_notebook_ref = retained_notebook:e12.latest,
  review_session_ref = review_session:e12.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T07:12,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
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
  emitted_invocation_receipt_ref = emitted_invocation_receipt:e12.latest,
  runtime_transcript_ref = runtime_transcript:e12.latest,
  materialized_runtime_handoff_ref = materialized_runtime_handoff:e12.latest,
  concrete_payload_ref = concrete_payload:e12.latest,
  concrete_transcript_body_ref = concrete_transcript_body:e12.latest,
  serialized_channel_body_ref = serialized_channel_body:e12.latest,
  emitted_attachment_body_ref = emitted_attachment_body:e12.latest,
  emitted_attachment_blob_ref = emitted_attachment_blob:e12.latest,
  retained_file_body_ref = retained_file_body:e12.latest,
  archive_materialization_ref = archive_materialization:e12.latest,
  archive_body_ref = archive_body:e12.latest
}
```

ここで notebook bridge が知るのは
archive body に戻る ref までであり、
bundle manifest や multi-file retention layout まではまだ bridge に入れない。

### example B — witnessed draw archive-body-ready retained bridge

```text
proof_notebook_bridge_archive_body_ready_sketch = {
  bridge_subject_ref = witnessed_draw_room:sugoroku.room_1,
  review_units = [
    review_unit:sugoroku.draw.seriality,
    review_unit:sugoroku.draw.witness_consistency
  ],
  bridge_goal_text = "authoritative draw room の actual archive body family を notebook で追う",
  comparison_basis_refs = [
    bridge_sketch:sugoroku.draw.archive_materialization_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:sugoroku.archive_body.observation
  ],
  retained_notebook_ref = retained_notebook:sugoroku.draw.latest,
  review_session_ref = review_session:sugoroku.draw.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T07:12,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:sugoroku.draw.notebook,
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
  emitted_invocation_receipt_ref = emitted_invocation_receipt:sugoroku.draw.latest,
  runtime_transcript_ref = runtime_transcript:sugoroku.draw.latest,
  materialized_runtime_handoff_ref = materialized_runtime_handoff:sugoroku.draw.latest,
  concrete_payload_ref = concrete_payload:sugoroku.draw.latest,
  concrete_transcript_body_ref = concrete_transcript_body:sugoroku.draw.latest,
  serialized_channel_body_ref = serialized_channel_body:sugoroku.draw.latest,
  emitted_attachment_body_ref = emitted_attachment_body:sugoroku.draw.latest,
  emitted_attachment_blob_ref = emitted_attachment_blob:sugoroku.draw.latest,
  retained_file_body_ref = retained_file_body:sugoroku.draw.latest,
  archive_materialization_ref = archive_materialization:sugoroku.draw.latest,
  archive_body_ref = archive_body:sugoroku.draw.latest
}
```

draw result body へ戻る ref は notebook bridge で追えるが、
archive bundle family までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で archive body / bundle family comparison を切り、current first choice は `archive_body_ref` のみを足す archive-body-ready retained bridge である
- current first choice は `archive_body_ref` のみを足す archive-body-ready retained bridge である
- next later reopen は actual archive bundle comparison である
- actual archive bundle family は still 後段に残す

### not decided

- actual archive bundle family の最小 shape
- archive retention layout / bless/update policy への接続
