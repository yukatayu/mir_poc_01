# 157 — current L2 theorem line exchange-body-ready runtime-coupling threshold

## 目的

`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md`
までを前提に、

- exchange-body-ready retained bridge に runtime coupling をどこまで足すか
- concrete transport protocol と failure body をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の exchange-body-ready runtime-coupling threshold** であり、

- concrete file / blob transport protocol
- concrete failure body

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current exchange-body-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や runtime adapter invocation command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref / consumer invocation surface ref / exchange rule body ref も exchange-body-ready retained bridge までは足してよい。
4. concrete runtime coupling / concrete file-blob transport / concrete failure body は still 後段に残している。

したがって current 問いは、
**exchange-body-ready retained bridge に `runtime_coupling_ref` だけを先に足してよいか、それとも transport protocol や failure body とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current exchange-body-ready retained bridge の最小性を壊さないか
2. `exchange_rule_body_ref` と runtime coupling を、transport protocol / failure body から分離できるか
3. concrete transport protocol と failure semantics を theorem-line bridge に premature に混ぜないか
4. later reopen で transport / failure threshold へ narrow に進めるか
5. consumer-specific host / path / session binding を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. exchange-body-ready retained bridge を terminal cut にし、runtime coupling も bridge 外へ残す

#### 読み

bridge sketch には

- `emitted_artifact_ref`
- `handoff_emitter_ref`
- `consumer_adapter_ref`
- `exchange_rule_ref`
- `adapter_validation_ref`
- `consumer_invocation_surface_ref`
- `exchange_rule_body_ref`

までを置き、`runtime_coupling_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- transport / failure body と混ざる余地をさらに減らせる

#### 欠点

- exchange body と runtime coupling の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. runtime coupling ref だけを持つ runtime-coupling-ready retained bridge にする

#### 読み

exchange-body-ready retained bridge に、
transport protocol と failure body を入れずに
**`runtime_coupling_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_runtime_coupling_ready_sketch = {
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
  runtime_coupling_ref
}
```

#### 利点

- exchange body と runtime coupling anchor を narrow に橋渡しできる
- concrete transport protocol / failure body を still 後段に残せる
- next later reopen を transport / failure threshold へ狭く進めやすい
- runtime coupling 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `runtime_coupling_ref` を concrete transport / failure semantics と誤読されない説明が要る

### 案 3. runtime coupling と transport / failure body をまとめて入れる

#### 読み

bridge sketch に

- `runtime_coupling_ref`
- `transport_protocol_ref`
- `failure_body_ref`

をまとめて足し、actual notebook consumer handoff bundle にさらに近い shape へ進める。

#### 利点

- runtime / transport / failure の line を一度に語れる
- concrete consumer handoff pressure に近づく

#### 欠点

- concrete runtime semantics を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- transport / failure family を stage ごとに narrow に reopen しにくい

## current judgment

current L2 で最も自然なのは、
**案 2. runtime coupling ref だけを持つ runtime-coupling-ready retained bridge にする**
である。

理由は次の通り。

1. exchange body と runtime coupling anchor を narrow に接続できる
2. concrete transport protocol / failure body を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を transport / failure threshold へ狭く進めやすい

## minimal runtime-coupling-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_runtime_coupling_ready_sketch = {
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
  runtime_coupling_ref
}
```

### `runtime_coupling_ref`

`runtime_coupling_ref` は、

- downstream consumer invocation を actual runtime / notebook runner に結びつける
  **symbolic runtime-coupling ref**

だけを置く field である。

current task では、

- concrete file / blob transport protocol
- concrete failure body

をここに入れない。

## なぜ transport / failure body をまだ入れないか

transport / failure body を current phase で入れると、

- concrete host / path / session binding
- transport-specific behavior
- runtime-specific failure semantics

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず runtime coupling ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain runtime-coupling-ready retained bridge

```text
proof_notebook_bridge_runtime_coupling_ready_sketch = {
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
  runtime_coupling_ref = runtime_coupling:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` runtime-coupling-ready retained bridge

```text
proof_notebook_bridge_runtime_coupling_ready_sketch = {
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
  runtime_coupling_ref = runtime_coupling:proof_notebook.followup.default
}
```

## settled / deferred

### settled in current docs-only judgment

- invocation-ready retained bridge の次段として `exchange_rule_body_ref` までは current first choice で入れてよい
- そのさらに次段としては `runtime_coupling_ref` だけを先に足すのが自然である
- transport protocol / failure body は still later reopen に残す

### still deferred

- `transport_protocol_ref` を retained bridge にどこまで入れてよいか
- `failure_body_ref` を retained bridge にどこまで入れてよいか
- actual runtime invocation protocol
- concrete notebook exchange runtime / file-blob transport

## note

この文書は theorem-line retained bridge の docs-first threshold を扱う。
actual theorem prover notebook runtime、consumer host binding、production handoff protocol は固定しない。
