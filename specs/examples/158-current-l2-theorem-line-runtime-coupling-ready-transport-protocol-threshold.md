# 158 — current L2 theorem line runtime-coupling-ready transport-protocol threshold

## 目的

`specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md`
までを前提に、

- runtime-coupling-ready retained bridge に transport protocol をどこまで足すか
- concrete failure body をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の runtime-coupling-ready transport-protocol threshold** であり、

- concrete failure body
- actual runtime invocation failure semantics

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current runtime-coupling-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や notebook runner command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref / consumer invocation surface ref / exchange rule body ref / runtime coupling ref も runtime-coupling-ready retained bridge までは足してよい。
4. concrete transport protocol と concrete failure body は still 後段に残している。

したがって current 問いは、
**runtime-coupling-ready retained bridge に `transport_protocol_ref` だけを先に足してよいか、それとも `failure_body_ref` とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current runtime-coupling-ready retained bridge の最小性を壊さないか
2. `runtime_coupling_ref` と transport protocol を、failure body から分離できるか
3. concrete failure semantics を theorem-line bridge に premature に混ぜないか
4. later reopen で failure-body threshold へ narrow に進めるか
5. consumer-specific host / session failure wording を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. runtime-coupling-ready retained bridge を terminal cut にし、transport protocol も bridge 外へ残す

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

までを置き、`transport_protocol_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- failure body と transport line の混線をさらに避けられる

#### 欠点

- runtime coupling と transport protocol の境界が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. transport protocol ref だけを持つ transport-ready retained bridge にする

#### 読み

runtime-coupling-ready retained bridge に、
failure body を入れずに
**`transport_protocol_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_transport_ready_sketch = {
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
  transport_protocol_ref
}
```

#### 利点

- runtime coupling と transport protocol の line を narrow に橋渡しできる
- concrete failure body を still 後段に残せる
- next later reopen を failure-body threshold へ狭く進めやすい
- transport protocol 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `transport_protocol_ref` を concrete failure semantics と誤読されない説明が要る

### 案 3. transport protocol と failure body をまとめて入れる

#### 読み

bridge sketch に

- `transport_protocol_ref`
- `failure_body_ref`

をまとめて足し、actual notebook consumer handoff bundle にさらに近い shape へ進める。

#### 利点

- transport / failure の line を一度に語れる
- concrete consumer handoff pressure に近づく

#### 欠点

- concrete failure semantics を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- failure-body family を stage ごとに narrow に reopen しにくい

## current judgment

current L2 で最も自然なのは、
**案 2. transport protocol ref だけを持つ transport-ready retained bridge にする**
である。

理由は次の通り。

1. runtime coupling と transport protocol の line を narrow に接続できる
2. concrete failure body を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を failure-body threshold へ狭く進めやすい

## minimal transport-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_transport_ready_sketch = {
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
  transport_protocol_ref
}
```

### `transport_protocol_ref`

`transport_protocol_ref` は、

- downstream consumer invocation の payload / exchange path を actual transport family に結びつける
  **symbolic transport-protocol ref**

だけを置く field である。

current task では、

- concrete failure body
- concrete runtime invocation failure wording

をここに入れない。

## なぜ failure body をまだ入れないか

failure body を current phase で入れると、

- concrete host / session failure wording
- transport-specific failure detail
- runtime-specific invocation semantics

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず transport protocol ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain transport-ready retained bridge

```text
proof_notebook_bridge_transport_ready_sketch = {
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
  transport_protocol_ref = transport_protocol:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` transport-ready retained bridge

```text
proof_notebook_bridge_transport_ready_sketch = {
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
  transport_protocol_ref = transport_protocol:proof_notebook.followup.default
}
```

## settled / deferred

### settled in current docs-only judgment

- runtime-coupling-ready retained bridge の次段として `transport_protocol_ref` までは current first choice で入れてよい
- そのさらに次段としては `failure_body_ref` を別 package で narrow に比べるのが自然である

### still deferred

- `failure_body_ref` を retained bridge にどこまで入れてよいか
- actual runtime invocation protocol
- concrete notebook exchange runtime / file-blob transport failure semantics

## note

この文書は theorem-line retained bridge の docs-first threshold を扱う。
actual theorem prover notebook runtime、consumer host binding、production handoff protocol、failure wording は固定しない。
