# 159 — current L2 theorem line transport-ready failure-body threshold

## 目的

`specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md`
までを前提に、

- transport-ready retained bridge に failure body をどこまで足すか
- actual runtime invocation protocol / host binding / failure wording をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の transport-ready failure-body threshold** であり、

- actual runtime invocation protocol
- consumer-host-specific binding
- concrete failure wording

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current transport-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や notebook runner command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref / consumer invocation surface ref / exchange rule body ref / runtime coupling ref / transport protocol ref も transport-ready retained bridge までは足してよい。
4. actual runtime invocation protocol / host binding / concrete failure wording は still 後段に残している。

したがって current 問いは、
**transport-ready retained bridge に `failure_body_ref` だけを先に足してよいか、それとも actual runtime invocation protocol / host binding とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current transport-ready retained bridge の最小性を壊さないか
2. `transport_protocol_ref` と failure body を、actual runtime invocation protocol から分離できるか
3. concrete host / session binding と failure wording を theorem-line bridge に premature に混ぜないか
4. later reopen で actual invocation protocol threshold へ narrow に進めるか
5. consumer-specific failure channel / retry wording を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. transport-ready retained bridge を terminal cut にし、failure body も bridge 外へ残す

#### 読み

bridge sketch には

- `emitted_artifact_ref`
- `handoff_emitter_ref`
- `consumer_adapter_ref`
- `exchange_rule_ref`
- `adapter_validation_ref`
- `consumer_invocation_surface_ref`
- `exchange_rule_body_ref`
- `runtime_coupling_ref`
- `transport_protocol_ref`

までを置き、`failure_body_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- concrete invocation protocol との混線をさらに避けられる

#### 欠点

- transport protocol と failure body の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. failure body ref だけを持つ failure-ready retained bridge にする

#### 読み

transport-ready retained bridge に、
actual runtime invocation protocol / host binding / failure wording を入れずに
**`failure_body_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_failure_ready_sketch = {
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
  failure_body_ref
}
```

#### 利点

- transport protocol と failure body の line を narrow に橋渡しできる
- actual runtime invocation protocol / host binding / failure wording を still 後段に残せる
- next later reopen を actual-invocation-protocol threshold へ狭く進めやすい
- failure body 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `failure_body_ref` を concrete host binding や failure wording と誤読されない説明が要る

### 案 3. failure body と actual invocation protocol をまとめて入れる

#### 読み

bridge sketch に

- `failure_body_ref`
- `actual_invocation_protocol_ref`

をまとめて足し、actual notebook consumer handoff bundle にさらに近い shape へ進める。

#### 利点

- failure / invocation の line を一度に語れる
- concrete consumer handoff pressure に近づく

#### 欠点

- concrete host / session binding を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- actual invocation protocol family を stage ごとに narrow に reopen しにくい

## current judgment

current L2 で最も自然なのは、
**案 2. failure body ref だけを持つ failure-ready retained bridge にする**
である。

理由は次の通り。

1. transport protocol と failure body の line を narrow に接続できる
2. actual runtime invocation protocol / host binding / failure wording を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を actual-invocation-protocol threshold へ狭く進めやすい

## minimal failure-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_failure_ready_sketch = {
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
  failure_body_ref
}
```

### `failure_body_ref`

`failure_body_ref` は、

- downstream consumer invocation の failure branch を actual failure-body family に結びつける
  **symbolic failure-body ref**

だけを置く field である。

current task では、

- actual runtime invocation protocol
- consumer-host-specific binding
- concrete failure wording

をここに入れない。

## なぜ actual invocation protocol をまだ入れないか

actual invocation protocol を current phase で入れると、

- concrete host / session binding
- environment-specific channel shape
- runtime-specific invocation / retry semantics

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず failure body ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain failure-ready retained bridge

```text
proof_notebook_bridge_failure_ready_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の normalization family を walkthrough する",
  comparison_basis_refs = [
    bridge_sketch:e12.previous
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e12.normalization.observation
  ],
  retained_notebook_ref = retained_notebook:e12.latest,
  review_session_ref = review_session:e12.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T01:52,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.viewer.default,
  adapter_validation_ref = adapter_validation:proof_notebook.viewer.default,
  consumer_invocation_surface_ref = invocation_surface:proof_notebook.viewer.default,
  exchange_rule_body_ref = exchange_body:proof_notebook.viewer.default,
  runtime_coupling_ref = runtime_coupling:proof_notebook.viewer.default,
  transport_protocol_ref = transport_protocol:proof_notebook.viewer.default,
  failure_body_ref = failure_body:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` failure-ready retained bridge

```text
proof_notebook_bridge_failure_ready_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.rollback_cut_non_interference,
    review_unit:e6.rollback_frontier_reading
  ],
  bridge_goal_text = "rollback family を side-by-side で見直す",
  comparison_basis_refs = [
    runtime_cluster:e6,
    static_gate:e6
  ],
  bless_decision_state = revise_requested,
  review_note_refs = [
    review_note:e6.rollback.followup
  ],
  retained_notebook_ref = retained_notebook:e6.followup,
  review_session_ref = review_session:e6.rollback_walkthrough,
  reviewed_by_ref = actor:maintainer_2,
  reviewed_at_ref = timestamp:2026-04-10T01:52,
  review_session_state = followup_pending,
  retention_state = provisional,
  retained_path_policy_ref = retained_path_policy:phase5.followup,
  emitted_artifact_ref = emitted_artifact:e6.followup.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.followup,
  consumer_adapter_ref = consumer_adapter:proof_notebook.followup_viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.followup.default,
  adapter_validation_ref = adapter_validation:proof_notebook.followup.default,
  consumer_invocation_surface_ref = invocation_surface:proof_notebook.followup.default,
  exchange_rule_body_ref = exchange_body:proof_notebook.followup.default,
  runtime_coupling_ref = runtime_coupling:proof_notebook.followup.default,
  transport_protocol_ref = transport_protocol:proof_notebook.followup.default,
  failure_body_ref = failure_body:proof_notebook.followup.default
}
```

## settled / deferred

### settled in current docs-only judgment

- transport-ready retained bridge の次段として `failure_body_ref` までは current first choice で入れてよい
- そのさらに次段としては `actual_invocation_protocol_ref` を別 package で narrow に比べるのが自然である

### still deferred

- `actual_invocation_protocol_ref` を retained bridge にどこまで入れてよいか
- consumer-host-specific binding
- concrete notebook exchange runtime failure wording

## note

この文書は theorem-line retained bridge の docs-first threshold を扱う。
actual theorem prover notebook runtime、consumer host binding、production handoff protocol、failure wording は固定しない。
