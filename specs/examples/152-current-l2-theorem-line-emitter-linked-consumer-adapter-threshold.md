# 152 — current L2 theorem line emitter-linked consumer-adapter threshold

## 目的

`specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md`
までを前提に、

- emitter-linked retained bridge に consumer adapter をどこまで足すか
- actual notebook exchange rule をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の emitter-linked consumer-adapter threshold** であり、

- actual notebook exchange rule
- concrete file / blob exchange protocol
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current emitter-linked retained bridge を起点にする。
- detached validation loop の concrete emitter command や file emission policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. review actor / timestamp / session lifecycle state も observed-session bridge までは足してよい。
3. retention state / retained path policy ref / emitted artifact ref / handoff emitter ref も emitter-linked bridge までは足してよい。
4. actual consumer adapter / notebook exchange rule / concrete file exchange protocol は still 後段に残している。

したがって current 問いは、
**emitter-linked retained bridge に consumer adapter ref だけを先に足してよいか、それとも actual notebook exchange rule とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current emitter-linked retained bridge の最小性を壊さないか
2. `handoff_emitter_ref` と consumer adapter を、actual notebook exchange rule から分離できるか
3. actual file / blob exchange protocol を theorem-line bridge に premature に混ぜないか
4. later reopen で notebook exchange rule threshold へ narrow に進めるか
5. `proof_assistant_adapter` specific schema を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. emitter-linked retained bridge を terminal cut にし、consumer adapter も bridge 外へ残す

#### 読み

bridge sketch には

- `emitted_artifact_ref`
- `handoff_emitter_ref`

までを置き、`consumer_adapter_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- actual exchange rule と混ざる余地をさらに減らせる

#### 欠点

- handoff emitter と consumer adapter anchor の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. consumer adapter ref だけを持つ adapter-linked retained bridge にする

#### 読み

emitter-linked retained bridge に、
actual notebook exchange rule を入れずに
**`consumer_adapter_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_adapter_linked_sketch = {
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
  consumer_adapter_ref
}
```

#### 利点

- handoff emitter と consumer adapter anchor を narrow に橋渡しできる
- actual notebook exchange rule / file exchange protocol を still 後段に残せる
- next later reopen を exchange-rule threshold へ狭く進めやすい
- consumer adapter 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `consumer_adapter_ref` を actual exchange rule / adapter-specific schema と誤読されない説明が要る

### 案 3. consumer adapter と notebook exchange rule をまとめて入れる

#### 読み

bridge sketch に

- `consumer_adapter_ref`
- `exchange_rule_ref`

をまとめて足し、actual notebook exchange bundle に近い shape へ進める。

#### 利点

- adapter と exchange line を一度に語れる
- concrete exchange pressure に近づく

#### 欠点

- actual notebook exchange rule を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- `proof_assistant_adapter` specific schema へ滑りやすい

## current judgment

current L2 で最も自然なのは、
**案 2. consumer adapter ref だけを持つ adapter-linked retained bridge にする**
である。

理由は次の通り。

1. handoff emitter と consumer adapter anchor を narrow に接続できる
2. actual notebook exchange rule / file exchange protocol を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を exchange-rule threshold へ狭く進めやすい

## minimal adapter-linked retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_adapter_linked_sketch = {
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
  consumer_adapter_ref
}
```

### `consumer_adapter_ref`

`consumer_adapter_ref` は、

- handed-off notebook artifact を downstream consumer shape に受け渡す
  **symbolic consumer adapter ref**

だけを置く field である。

current task では、

- actual notebook exchange rule
- concrete file / blob exchange protocol
- adapter-local validation rule

をここに入れない。

## なぜ exchange rule をまだ入れないか

actual notebook exchange rule を current phase で入れると、

- file / blob transport rule
- adapter-local exchange validation
- environment / consumer specific invocation surface

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず consumer adapter ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain adapter-linked retained bridge

```text
proof_notebook_bridge_adapter_linked_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T00:58,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default,
  consumer_adapter_ref = consumer_adapter:proof_notebook.viewer
}
```

### example B — `try` / `atomic_cut` adapter-linked retained bridge

```text
proof_notebook_bridge_adapter_linked_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T00:58,
  review_session_state = followup_pending,
  retention_state = provisional,
  retained_path_policy_ref = retained_path_policy:phase5.followup,
  emitted_artifact_ref = emitted_artifact:e6.followup.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.followup,
  consumer_adapter_ref = consumer_adapter:proof_notebook.followup_viewer
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. emitter-linked retained bridge の次段では、`consumer_adapter_ref` までは足してよい
2. `consumer_adapter_ref` は symbolic adapter ref に留め、actual notebook exchange rule / file exchange protocol / adapter-local validation を actual contract にしない
3. next later reopen は、notebook exchange rule threshold の comparison に置く

## この段階でまだ固定しないもの

- `exchange_rule_ref`
- actual notebook exchange rule
- concrete file / blob exchange protocol
- `proof_assistant_adapter` specific schema
