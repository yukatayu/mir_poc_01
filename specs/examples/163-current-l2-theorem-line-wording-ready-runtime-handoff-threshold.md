# 163 — current L2 theorem line wording-ready runtime-handoff threshold

## 目的

`specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md`
までを前提に、

- wording-ready retained bridge に actual notebook runtime handoff actualization をどこまで足すか
- emitted invocation receipt / runtime transcript family をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の wording-ready runtime-handoff threshold** であり、

- actual emitted invocation receipt
- runtime transcript family
- concrete channel payload / host-side transcript body

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current wording-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や notebook runner command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref / consumer invocation surface ref / exchange rule body ref / runtime coupling ref / transport protocol ref / failure body ref / actual invocation protocol ref / consumer-host-binding ref / failure-wording ref も wording-ready retained bridge までは足してよい。
4. actual emitted invocation receipt / runtime transcript family / concrete channel payload は still 後段に残している。

したがって current 問いは、
**wording-ready retained bridge に `actual_runtime_handoff_ref` だけを先に足してよいか、それとも emitted invocation receipt とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current wording-ready retained bridge の最小性を壊さないか
2. actual runtime handoff を emitted receipt / transcript family から分離できるか
3. concrete channel payload / transcript body を theorem-line bridge に premature に混ぜないか
4. later reopen を emitted invocation receipt comparison へ narrow に進めるか
5. notebook-oriented handoff step と host-local result blob を混同しないか

## 比較対象

### 案 1. wording-ready retained bridge を terminal cut にし、runtime handoff も bridge 外へ残す

#### 読み

bridge sketch には

- `failure_wording_ref`

までを置き、`actual_runtime_handoff_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- emitted receipt / transcript family との混線をさらに避けられる

#### 欠点

- failure wording と actual runtime handoff の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. runtime-handoff ref だけを持つ handoff-ready retained bridge にする

#### 読み

wording-ready retained bridge に、
emitted invocation receipt / runtime transcript family を入れずに
**`actual_runtime_handoff_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_handoff_ready_sketch = {
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
  actual_runtime_handoff_ref
}
```

#### 利点

- wording-ready bridge と actual runtime handoff の line を narrow に橋渡しできる
- emitted invocation receipt / runtime transcript family を still 後段に残せる
- next later reopen を emitted invocation receipt comparison へ狭く進めやすい
- actual runtime handoff 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `actual_runtime_handoff_ref` を actual emitted receipt や transcript family と誤読されない説明が要る

### 案 3. runtime handoff と emitted invocation receipt をまとめて入れる

#### 読み

bridge sketch に

- `actual_runtime_handoff_ref`
- emitted invocation receipt family

をまとめて足し、actual notebook runtime handoff bundle にさらに近い shape へ進める。

#### 利点

- runtime handoff / receipt の line を一度に語れる
- concrete runtime handoff pressure に近づく

#### 欠点

- emitted receipt / transcript family を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- emitted invocation receipt comparison を stage ごとに narrow に reopen しにくい

## current judgment

current L2 で最も自然なのは、
**案 2. runtime-handoff ref だけを持つ handoff-ready retained bridge にする**
である。

理由は次の通り。

1. wording-ready bridge と actual runtime handoff の line を narrow に接続できる
2. emitted invocation receipt / runtime transcript family を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を emitted invocation receipt comparison へ狭く進めやすい

## minimal handoff-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_handoff_ready_sketch = {
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
  actual_runtime_handoff_ref
}
```

### `actual_runtime_handoff_ref`

`actual_runtime_handoff_ref` は、

- proof notebook consumer に対して
- actual invocation protocol / host binding / failure wording の次段で
- runtime handoff step が actualized された notebook-side handoff bundle へ戻れる
  **symbolic actual-runtime-handoff ref**

だけを置く field である。

current task では、

- actual emitted invocation receipt
- runtime transcript family
- concrete channel payload / transcript body

をここに入れない。

## なぜ emitted invocation receipt をまだ入れないか

emitted invocation receipt を current phase で入れると、

- concrete emitted receipt row
- runtime transcript family
- host-side channel payload

が theorem-line bridge に premature に近づきやすい。

current pressure はまず actual runtime handoff 自体を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain handoff-ready retained bridge

```text
proof_notebook_bridge_handoff_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T03:20,
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
  actual_runtime_handoff_ref = actual_runtime_handoff:proof_notebook.viewer.default
}
```

### example B — runtime try/cut handoff-ready retained bridge

```text
proof_notebook_bridge_handoff_ready_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.write_after_expiry,
    review_unit:e6.try_cut_locality
  ],
  bridge_goal_text = "runtime write-after-expiry family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e6.previous
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e6.write_after_expiry.observation
  ],
  retained_notebook_ref = retained_notebook:e6.latest,
  review_session_ref = review_session:e6.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-10T03:20,
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
  actual_runtime_handoff_ref = actual_runtime_handoff:proof_notebook.followup.default
}
```

## current conclusion

- wording-ready retained bridge の次段として `actual_runtime_handoff_ref` までは current first choice で入れてよい
- emitted invocation receipt / runtime transcript family はまだ theorem-line bridge に入れない
- 次の later reopen は、emitted invocation receipt を theorem-line bridge にどこまで足すかの comparison に置くのが自然である

## unresolved

- emitted invocation receipt をどこまで retained bridge に寄せてよいか
- runtime transcript family を receipt と分けて扱うべきか
- concrete channel payload / transcript body をどこまで actual handoff materialization に含めるか
