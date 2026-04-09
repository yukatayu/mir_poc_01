# 162 — current L2 theorem line binding-ready failure-wording threshold

## 目的

`specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md`
までを前提に、

- binding-ready retained bridge に failure wording をどこまで足すか
- actual notebook runtime handoff actualization をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の binding-ready failure-wording threshold** であり、

- actual notebook runtime handoff actualization
- actual emitted invocation receipt / runtime transcript family

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current binding-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や notebook runner command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref / consumer invocation surface ref / exchange rule body ref / runtime coupling ref / transport protocol ref / failure body ref / actual invocation protocol ref / consumer-host-binding ref も binding-ready retained bridge までは足してよい。
4. actual notebook runtime handoff actualization / actual emitted invocation receipt / runtime transcript family は still 後段に残している。

したがって current 問いは、
**binding-ready retained bridge に `failure_wording_ref` だけを先に足してよいか、それとも actual notebook runtime handoff actualization とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current binding-ready retained bridge の最小性を壊さないか
2. host binding と failure wording を、actual runtime handoff actualization から分離できるか
3. actual emitted invocation receipt / runtime transcript family を theorem-line bridge に premature に混ぜないか
4. later reopen を actual notebook runtime handoff actualization comparison へ narrow に進めるか
5. operator-facing wording と concrete runtime result carrier を混同しないか

## 比較対象

### 案 1. binding-ready retained bridge を terminal cut にし、failure wording も bridge 外へ残す

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
- `failure_body_ref`
- `actual_invocation_protocol_ref`
- `consumer_host_binding_ref`

までを置き、`failure_wording_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- actual runtime handoff actualization との混線をさらに避けられる

#### 欠点

- host binding と failure wording の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. failure wording ref だけを持つ wording-ready retained bridge にする

#### 読み

binding-ready retained bridge に、
actual notebook runtime handoff actualization を入れずに
**`failure_wording_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_wording_ready_sketch = {
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
  failure_wording_ref
}
```

#### 利点

- host binding と failure wording の line を narrow に橋渡しできる
- actual notebook runtime handoff actualization を still 後段に残せる
- next later reopen を actual runtime handoff actualization comparison へ狭く進めやすい
- failure wording 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `failure_wording_ref` を actual emitted invocation receipt / runtime transcript と誤読されない説明が要る

### 案 3. failure wording と actual notebook runtime handoff actualization をまとめて入れる

#### 読み

bridge sketch に

- `failure_wording_ref`
- actual emitted invocation receipt / runtime transcript family

をまとめて足し、actual notebook runtime handoff bundle にさらに近い shape へ進める。

#### 利点

- wording / handoff actualization の line を一度に語れる
- concrete notebook runtime handoff pressure に近づく

#### 欠点

- concrete runtime result carrier を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- actual notebook runtime handoff actualization comparison を stage ごとに narrow に reopen しにくい

## current judgment

current L2 で最も自然なのは、
**案 2. failure wording ref だけを持つ wording-ready retained bridge にする**
である。

理由は次の通り。

1. host binding と failure wording の line を narrow に接続できる
2. actual notebook runtime handoff actualization を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を actual notebook runtime handoff actualization comparison へ狭く進めやすい

## minimal wording-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_wording_ready_sketch = {
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
  failure_wording_ref
}
```

### `failure_wording_ref`

`failure_wording_ref` は、

- downstream consumer invocation の failure presentation / wording family を actual wording family に結びつける
  **symbolic failure-wording ref**

だけを置く field である。

current task では、

- actual notebook runtime handoff actualization
- actual emitted invocation receipt / runtime transcript family

をここに入れない。

## なぜ actual notebook runtime handoff actualization をまだ入れないか

actual notebook runtime handoff actualization を current phase で入れると、

- emitted runtime receipt
- concrete runtime transcript / channel artifact
- host-specific result carrier

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず failure wording ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain wording-ready retained bridge

```text
proof_notebook_bridge_wording_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T02:58,
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
  failure_body_ref = failure_body:proof_notebook.viewer.default,
  actual_invocation_protocol_ref = actual_invocation_protocol:proof_notebook.viewer.default,
  consumer_host_binding_ref = consumer_host_binding:proof_notebook.viewer.default,
  failure_wording_ref = failure_wording:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` wording-ready retained bridge

```text
proof_notebook_bridge_wording_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T02:58,
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
  failure_body_ref = failure_body:proof_notebook.followup.default,
  actual_invocation_protocol_ref = actual_invocation_protocol:proof_notebook.followup.default,
  consumer_host_binding_ref = consumer_host_binding:proof_notebook.followup.default,
  failure_wording_ref = failure_wording:proof_notebook.followup.default
}
```

## settled / deferred

### settled in current docs-only judgment

- binding-ready retained bridge の次段として `failure_wording_ref` までは current first choice で入れてよい
- 次の later reopen は、actual notebook runtime handoff actualization を theorem-line bridge にどこまで近づけるかの comparison に置くのが自然である

### still deferred

- actual notebook runtime handoff actualization
- actual emitted invocation receipt / runtime transcript family
- `proof_assistant_adapter` pressure を `proof_notebook` line より先に practical reopen に上げる条件

## note

この文書は theorem-line retained bridge の docs-first threshold を扱う。
actual theorem prover notebook runtime、runtime receipt / transcript artifact、production handoff actualizationは固定しない。
