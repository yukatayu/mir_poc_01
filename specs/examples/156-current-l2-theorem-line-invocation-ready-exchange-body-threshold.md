# 156 — current L2 theorem line invocation-ready exchange-body threshold

## 目的

`specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md`
までを前提に、

- invocation-ready retained bridge に exchange body をどこまで足すか
- concrete runtime coupling と file / blob transport をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の invocation-ready exchange-body threshold** であり、

- concrete file / blob exchange protocol
- environment-specific runtime coupling
- concrete failure body

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current invocation-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や runtime adapter invocation は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref / consumer invocation surface ref も invocation-ready retained bridge までは足してよい。
4. actual notebook exchange body / concrete runtime coupling / concrete file-blob transport は still 後段に残している。

したがって current 問いは、
**invocation-ready retained bridge に `exchange_rule_body_ref` だけを先に足してよいか、それとも runtime coupling や transport protocol とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current invocation-ready retained bridge の最小性を壊さないか
2. `consumer_invocation_surface_ref` と exchange body を、concrete runtime coupling から分離できるか
3. file / blob transport と runtime failure body を theorem-line bridge に premature に混ぜないか
4. later reopen で runtime-coupling threshold へ narrow に進めるか
5. consumer-specific host / path / session binding を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. invocation-ready retained bridge を terminal cut にし、exchange body も bridge 外へ残す

#### 読み

bridge sketch には

- `emitted_artifact_ref`
- `handoff_emitter_ref`
- `consumer_adapter_ref`
- `exchange_rule_ref`
- `adapter_validation_ref`
- `consumer_invocation_surface_ref`

までを置き、`exchange_rule_body_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- concrete runtime coupling と混ざる余地をさらに減らせる

#### 欠点

- invocation surface と exchange body の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. exchange body ref だけを持つ exchange-body-ready retained bridge にする

#### 読み

invocation-ready retained bridge に、
concrete runtime coupling と transport protocol を入れずに
**`exchange_rule_body_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_exchange_body_ready_sketch = {
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
  exchange_rule_body_ref
}
```

#### 利点

- invocation surface と exchange body anchor を narrow に橋渡しできる
- concrete runtime coupling / file-blob transport / failure body を still 後段に残せる
- next later reopen を runtime-coupling threshold へ狭く進めやすい
- exchange body 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `exchange_rule_body_ref` を concrete runtime body と誤読されない説明が要る

### 案 3. exchange body と runtime coupling をまとめて入れる

#### 読み

bridge sketch に

- `exchange_rule_body_ref`
- `runtime_coupling_ref`

をまとめて足し、actual notebook consumer handoff bundle に近い shape へ進める。

#### 利点

- exchange と runtime line を一度に語れる
- concrete consumer handoff pressure に近づく

#### 欠点

- concrete runtime coupling を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- transport protocol と failure body へ滑りやすい

## current judgment

current L2 で最も自然なのは、
**案 2. exchange body ref だけを持つ exchange-body-ready retained bridge にする**
である。

理由は次の通り。

1. invocation surface と exchange body anchor を narrow に接続できる
2. concrete runtime coupling / transport protocol / failure body を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を runtime-coupling threshold へ狭く進めやすい

## minimal exchange-body-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_exchange_body_ready_sketch = {
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
  exchange_rule_body_ref
}
```

### `exchange_rule_body_ref`

`exchange_rule_body_ref` は、

- handed-off notebook artifact を downstream consumer へ渡すときの
  **symbolic exchange body ref**

だけを置く field である。

current task では、

- concrete file / blob exchange protocol
- environment-specific runtime coupling
- concrete failure body

をここに入れない。

## なぜ runtime coupling をまだ入れないか

runtime coupling を current phase で入れると、

- consumer host / path / session binding
- concrete invocation protocol
- runtime-specific failure semantics

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず exchange body ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain exchange-body-ready retained bridge

```text
proof_notebook_bridge_exchange_body_ready_sketch = {
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
  exchange_rule_body_ref = exchange_body:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` exchange-body-ready retained bridge

```text
proof_notebook_bridge_exchange_body_ready_sketch = {
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
  exchange_rule_body_ref = exchange_body:proof_notebook.followup.default
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. invocation-ready retained bridge の次段では、`exchange_rule_body_ref` までは足してよい
2. `exchange_rule_body_ref` は symbolic body ref に留め、concrete runtime coupling / transport protocol / failure body を actual contract にしない
3. next later reopen は、runtime-coupling threshold の comparison に置く

## この段階でまだ固定しないもの

- `runtime_coupling_ref`
- concrete file / blob exchange protocol
- environment-specific runtime coupling
- concrete failure body
