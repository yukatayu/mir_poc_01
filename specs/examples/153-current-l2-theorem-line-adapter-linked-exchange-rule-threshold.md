# 153 — current L2 theorem line adapter-linked exchange-rule threshold

## 目的

`specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md`
までを前提に、

- adapter-linked retained bridge に exchange rule をどこまで足すか
- adapter-local validation をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の adapter-linked exchange-rule threshold** であり、

- adapter-local validation rule
- concrete file / blob exchange protocol
- environment-specific consumer invocation surface

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current adapter-linked retained bridge を起点にする。
- detached validation loop の concrete emitter command や file overwrite policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. review actor / timestamp / session lifecycle state、retention state / retained path policy ref も current first choice に入っている。
3. emitted artifact ref / handoff emitter ref / consumer adapter ref も adapter-linked retained bridge までは足してよい。
4. actual notebook exchange rule / adapter-local validation / concrete file exchange protocol は still 後段に残している。

したがって current 問いは、
**adapter-linked retained bridge に exchange-rule ref だけを先に足してよいか、それとも adapter-local validation とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current adapter-linked retained bridge の最小性を壊さないか
2. `consumer_adapter_ref` と exchange rule を、adapter-local validation から分離できるか
3. concrete file / blob exchange protocol を theorem-line bridge に premature に混ぜないか
4. later reopen で adapter-local validation threshold へ narrow に進めるか
5. environment-specific invocation surface を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. adapter-linked retained bridge を terminal cut にし、exchange rule も bridge 外へ残す

#### 読み

bridge sketch には

- `emitted_artifact_ref`
- `handoff_emitter_ref`
- `consumer_adapter_ref`

までを置き、`exchange_rule_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- adapter-local validation と混ざる余地をさらに減らせる

#### 欠点

- consumer adapter と exchange rule の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. exchange rule ref だけを持つ exchange-ready retained bridge にする

#### 読み

adapter-linked retained bridge に、
adapter-local validation を入れずに
**`exchange_rule_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_exchange_ready_sketch = {
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
  exchange_rule_ref
}
```

#### 利点

- consumer adapter と exchange line を narrow に橋渡しできる
- adapter-local validation / concrete file exchange protocol を still 後段に残せる
- next later reopen を adapter-validation threshold へ狭く進めやすい
- exchange rule 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `exchange_rule_ref` を actual validation rule / environment-specific invocation surface と誤読されない説明が要る

### 案 3. exchange rule と adapter-local validation をまとめて入れる

#### 読み

bridge sketch に

- `exchange_rule_ref`
- `adapter_validation_ref`

をまとめて足し、actual notebook exchange bundle に近い shape へ進める。

#### 利点

- exchange と validation line を一度に語れる
- concrete consumer handoff pressure に近づく

#### 欠点

- adapter-local validation を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- environment-specific invocation surface へ滑りやすい

## current judgment

current L2 で最も自然なのは、
**案 2. exchange rule ref だけを持つ exchange-ready retained bridge にする**
である。

理由は次の通り。

1. consumer adapter と exchange rule anchor を narrow に接続できる
2. adapter-local validation / concrete file exchange protocol を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を adapter-validation threshold へ狭く進めやすい

## minimal exchange-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_exchange_ready_sketch = {
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
  exchange_rule_ref
}
```

### `exchange_rule_ref`

`exchange_rule_ref` は、

- handed-off notebook artifact を downstream consumer へ受け渡すときの
  **symbolic notebook exchange rule ref**

だけを置く field である。

current task では、

- adapter-local validation rule
- concrete file / blob exchange protocol
- environment-specific invocation surface

をここに入れない。

## なぜ adapter-local validation をまだ入れないか

adapter-local validation を current phase で入れると、

- exchange precondition / postcondition
- consumer environment specific validation
- invocation-surface specific failure family

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず exchange rule ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain exchange-ready retained bridge

```text
proof_notebook_bridge_exchange_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T01:14,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.viewer.default
}
```

### example B — `try` / `atomic_cut` exchange-ready retained bridge

```text
proof_notebook_bridge_exchange_ready_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T01:14,
  review_session_state = followup_pending,
  retention_state = provisional,
  retained_path_policy_ref = retained_path_policy:phase5.followup,
  emitted_artifact_ref = emitted_artifact:e6.followup.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.followup,
  consumer_adapter_ref = consumer_adapter:proof_notebook.followup_viewer,
  exchange_rule_ref = exchange_rule:proof_notebook.followup.default
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. adapter-linked retained bridge の次段では、`exchange_rule_ref` までは足してよい
2. `exchange_rule_ref` は symbolic rule ref に留め、adapter-local validation / concrete file-blob exchange / environment-specific invocation surface を actual contract にしない
3. next later reopen は、adapter-validation threshold の comparison に置く

## この段階でまだ固定しないもの

- `adapter_validation_ref`
- actual notebook exchange rule
- concrete file / blob exchange protocol
- environment-specific consumer invocation surface
