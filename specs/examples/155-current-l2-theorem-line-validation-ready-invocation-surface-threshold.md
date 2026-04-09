# 155 — current L2 theorem line validation-ready invocation-surface threshold

## 目的

`specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md`
までを前提に、

- validation-ready retained bridge に invocation surface をどこまで足すか
- actual notebook exchange rule body と concrete runtime coupling をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の validation-ready invocation-surface threshold** であり、

- actual notebook exchange rule body
- concrete file / blob exchange protocol
- environment-specific runtime coupling and failure body

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current validation-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や overwrite policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref は current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref / exchange rule ref / adapter validation ref も validation-ready retained bridge までは足してよい。
4. actual notebook exchange rule body / environment-specific invocation surface / concrete runtime coupling は still 後段に残している。

したがって current 問いは、
**validation-ready retained bridge に `consumer_invocation_surface_ref` だけを先に足してよいか、それとも actual notebook exchange rule body や concrete runtime coupling とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current validation-ready retained bridge の最小性を壊さないか
2. `adapter_validation_ref` と invocation surface を、actual notebook exchange body から分離できるか
3. concrete runtime coupling / failure body を theorem-line bridge に premature に混ぜないか
4. later reopen で exchange-body threshold へ narrow に進めるか
5. consumer-specific host / path / session binding を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. validation-ready retained bridge を terminal cut にし、invocation surface も bridge 外へ残す

#### 読み

bridge sketch には

- `emitted_artifact_ref`
- `handoff_emitter_ref`
- `consumer_adapter_ref`
- `exchange_rule_ref`
- `adapter_validation_ref`

までを置き、`consumer_invocation_surface_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- concrete runtime coupling と混ざる余地をさらに減らせる

#### 欠点

- validation と invocation surface の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. invocation surface ref だけを持つ invocation-ready retained bridge にする

#### 読み

validation-ready retained bridge に、
actual notebook exchange rule body と concrete runtime coupling を入れずに
**`consumer_invocation_surface_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_invocation_ready_sketch = {
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
  consumer_invocation_surface_ref
}
```

#### 利点

- validation と invocation surface anchor を narrow に橋渡しできる
- actual notebook exchange rule body / concrete runtime coupling / file-blob protocol を still 後段に残せる
- next later reopen を exchange-body threshold へ狭く進めやすい
- invocation surface 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `consumer_invocation_surface_ref` を actual runtime coupling body と誤読されない説明が要る

### 案 3. invocation surface と actual exchange body をまとめて入れる

#### 読み

bridge sketch に

- `consumer_invocation_surface_ref`
- `exchange_rule_body_ref`

をまとめて足し、actual notebook consumer handoff bundle に近い shape へ進める。

#### 利点

- invocation と exchange body を一度に語れる
- concrete consumer handoff pressure に近づく

#### 欠点

- actual exchange body を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- concrete runtime coupling / failure body へ滑りやすい

## current judgment

current L2 で最も自然なのは、
**案 2. invocation surface ref だけを持つ invocation-ready retained bridge にする**
である。

理由は次の通り。

1. validation と invocation surface anchor を narrow に接続できる
2. actual notebook exchange rule body / concrete runtime coupling / file-blob protocol を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を exchange-body threshold へ狭く進めやすい

## minimal invocation-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_invocation_ready_sketch = {
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
  consumer_invocation_surface_ref
}
```

### `consumer_invocation_surface_ref`

`consumer_invocation_surface_ref` は、

- handed-off notebook artifact を downstream consumer が受け取るときの
  **symbolic invocation surface ref**

だけを置く field である。

current task では、

- actual notebook exchange rule body
- concrete file / blob exchange protocol
- environment-specific runtime coupling and failure body

をここに入れない。

## なぜ actual exchange body をまだ入れないか

actual exchange body を current phase で入れると、

- consumer host / path / session binding
- runtime-side adapter invocation protocol
- environment-specific failure family

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず invocation surface ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain invocation-ready retained bridge

```text
proof_notebook_bridge_invocation_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T01:41,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.viewer.default,
  adapter_validation_ref = adapter_validation:proof_notebook.viewer.default,
  consumer_invocation_surface_ref = invocation_surface:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` invocation-ready retained bridge

```text
proof_notebook_bridge_invocation_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T01:41,
  review_session_state = followup_pending,
  retention_state = provisional,
  retained_path_policy_ref = retained_path_policy:phase5.followup,
  emitted_artifact_ref = emitted_artifact:e6.followup.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.followup,
  consumer_adapter_ref = consumer_adapter:proof_notebook.followup_viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.followup.default,
  adapter_validation_ref = adapter_validation:proof_notebook.followup.default,
  consumer_invocation_surface_ref = invocation_surface:proof_notebook.followup.default
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. validation-ready retained bridge の次段では、`consumer_invocation_surface_ref` までは足してよい
2. `consumer_invocation_surface_ref` は symbolic invocation ref に留め、actual notebook exchange body / concrete runtime coupling / file-blob protocol を actual contract にしない
3. next later reopen は、exchange-body threshold の comparison に置く

## この段階でまだ固定しないもの

- `exchange_rule_body_ref`
- actual notebook exchange rule body
- concrete file / blob exchange protocol
- environment-specific runtime coupling and failure body
