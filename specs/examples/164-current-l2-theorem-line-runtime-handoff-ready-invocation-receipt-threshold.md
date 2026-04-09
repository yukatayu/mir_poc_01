# 164 — current L2 theorem line runtime-handoff-ready invocation-receipt threshold

## 目的

`specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`
までを前提に、

- handoff-ready retained bridge に emitted invocation receipt をどこまで足すか
- runtime transcript family をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の runtime-handoff-ready invocation-receipt threshold** であり、

- runtime transcript family
- concrete channel payload / transcript body

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current handoff-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や notebook runner command は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare / review / retention / path / emitter / adapter / invocation / exchange / runtime / transport / failure / invocation protocol / host binding / failure wording は current first choice に入っている。
2. `actual_runtime_handoff_ref` も handoff-ready retained bridge までは足してよい。
3. runtime transcript family / concrete channel payload は still 後段に残している。

したがって current 問いは、
**handoff-ready retained bridge に `emitted_invocation_receipt_ref` だけを先に足してよいか、それとも runtime transcript family とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual runtime handoff と emitted receipt の line を narrow に切れるか
2. emitted receipt と runtime transcript family を分離できるか
3. concrete transcript body / payload を theorem-line bridge に premature に混ぜないか
4. later reopen を runtime transcript comparison へ narrow に進めるか

## 比較対象

### 案 1. handoff-ready retained bridge を terminal cut にし、receipt も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- transcript family との混線をさらに避けられる

#### 欠点

- actual runtime handoff と emitted receipt の関係が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. emitted invocation receipt ref だけを持つ receipt-ready retained bridge にする

#### 読み

handoff-ready retained bridge に、
runtime transcript family を入れずに
**`emitted_invocation_receipt_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_receipt_ready_sketch = {
  proof_notebook_bridge_handoff_ready_sketch,
  emitted_invocation_receipt_ref
}
```

#### 利点

- actual runtime handoff と emitted receipt の line を narrow に橋渡しできる
- runtime transcript family を still 後段に残せる
- next later reopen を runtime transcript comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- emitted receipt と concrete transcript body を誤読されない説明が要る

### 案 3. emitted receipt と runtime transcript family をまとめて入れる

#### 利点

- runtime-side observable line を一度に語れる
- concrete host payload pressure に近づく

#### 欠点

- transcript family を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. emitted invocation receipt ref だけを持つ receipt-ready retained bridge にする**
である。

理由は次の通り。

1. actual runtime handoff と emitted receipt の line を narrow に接続できる
2. runtime transcript family を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を runtime transcript comparison へ狭く進めやすい

## minimal receipt-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_receipt_ready_sketch = {
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
  emitted_invocation_receipt_ref
}
```

### `emitted_invocation_receipt_ref`

`emitted_invocation_receipt_ref` は、

- actual runtime handoff の結果として
- notebook consumer が emitted receipt row に戻れる
  **symbolic emitted-invocation-receipt ref**

だけを置く field である。

current task では、

- runtime transcript family
- concrete channel payload / transcript body

をここに入れない。

## なぜ runtime transcript family をまだ入れないか

runtime transcript family を current phase で入れると、

- channel transcript row
- host-side payload body
- longer replay / trace family

が theorem-line bridge に premature に近づきやすい。

current pressure はまず emitted receipt row を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain receipt-ready retained bridge

```text
proof_notebook_bridge_receipt_ready_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の runtime handoff receipt を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e12.handoff_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e12.receipt.observation
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
  actual_runtime_handoff_ref = actual_runtime_handoff:proof_notebook.viewer.default,
  emitted_invocation_receipt_ref = emitted_invocation_receipt:proof_notebook.viewer.default
}
```

### example B — runtime try/cut receipt-ready retained bridge

```text
proof_notebook_bridge_receipt_ready_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.write_after_expiry,
    review_unit:e6.try_cut_locality
  ],
  bridge_goal_text = "runtime handoff receipt family を notebook で再確認する",
  comparison_basis_refs = [
    bridge_sketch:e6.handoff_ready
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e6.receipt.observation
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
  actual_runtime_handoff_ref = actual_runtime_handoff:proof_notebook.followup.default,
  emitted_invocation_receipt_ref = emitted_invocation_receipt:proof_notebook.followup.default
}
```

## current conclusion

- handoff-ready retained bridge の次段として `emitted_invocation_receipt_ref` までは current first choice で入れてよい
- runtime transcript family はまだ theorem-line bridge に入れない
- 次の later reopen は、runtime transcript family を theorem-line bridge にどこまで足すかの comparison に置くのが自然である

## unresolved

- runtime transcript family をどこまで retained bridge に寄せてよいか
- concrete transcript body / payload を transcript family から分けるべきか
