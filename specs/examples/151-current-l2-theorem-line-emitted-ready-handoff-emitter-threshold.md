# 151 — current L2 theorem line emitted-ready handoff-emitter threshold

## 目的

`specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md`
までを前提に、

- emitted-ready retained bridge に actual handoff emitter をどこまで足すか
- consumer adapter pressure をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の emitted-ready handoff-emitter threshold** であり、

- actual consumer adapter contract
- concrete notebook exchange rule
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current emitted-ready retained bridge を起点にする。
- detached validation loop の concrete emitter command や file emission policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. review actor / timestamp / session lifecycle state も observed-session bridge までは足してよい。
3. retention state / retained path policy ref も path-ready bridge までは足してよい。
4. `emitted_artifact_ref` も emitted-ready bridge までは足してよい。
5. actual handoff emitter / consumer adapter / concrete notebook exchange rule は still 後段に残している。

したがって current 問いは、
**emitted-ready retained bridge に handoff emitter ref だけを先に足してよいか、それとも consumer adapter pressure とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. current emitted-ready retained bridge の最小性を壊さないか
2. `emitted_artifact_ref` と handoff emitter を、consumer adapter contract から分離できるか
3. actual emitter command / file exchange rule を theorem-line bridge に premature に混ぜないか
4. later reopen で consumer adapter threshold へ narrow に進めるか
5. `proof_assistant_adapter` specific schema を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. emitted-ready retained bridge を terminal cut にし、handoff emitter も bridge 外へ残す

#### 読み

bridge sketch には

- `retained_path_policy_ref`
- `emitted_artifact_ref`

までを置き、`handoff_emitter_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- consumer adapter pressure と混ざる余地をさらに減らせる

#### 欠点

- emitted artifact と handoff emitter anchor の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. handoff emitter ref だけを持つ emitter-linked retained bridge にする

#### 読み

emitted-ready retained bridge に、
actual consumer adapter policy を入れずに
**`handoff_emitter_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_emitter_linked_sketch = {
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
  handoff_emitter_ref
}
```

#### 利点

- emitted artifact と handoff emitter anchor を narrow に橋渡しできる
- actual consumer adapter / notebook exchange rule を still 後段に残せる
- next later reopen を consumer adapter threshold へ狭く進めやすい
- handoff emitter 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `handoff_emitter_ref` を actual emitter command / adapter contract と誤読されない説明が要る

### 案 3. handoff emitter と consumer adapter をまとめて入れる

#### 読み

bridge sketch に

- `handoff_emitter_ref`
- `consumer_adapter_ref`

をまとめて足し、actual notebook handoff bundle に近い shape へ進める。

#### 利点

- emitter と consumer handoff line を一度に語れる
- concrete consumer pressure に近づく

#### 欠点

- actual consumer adapter policy を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある
- `proof_assistant_adapter` specific schema へ滑りやすい

## current judgment

current L2 で最も自然なのは、
**案 2. handoff emitter ref だけを持つ emitter-linked retained bridge にする**
である。

理由は次の通り。

1. emitted artifact と handoff emitter anchor を narrow に接続できる
2. actual consumer adapter / notebook exchange rule を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を consumer adapter threshold へ狭く進めやすい

## minimal emitter-linked retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_emitter_linked_sketch = {
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
  handoff_emitter_ref
}
```

### `handoff_emitter_ref`

`handoff_emitter_ref` は、

- emitted notebook artifact を downstream consumer へ handoff する
  **symbolic emitter ref**

だけを置く field である。

current task では、

- actual emitter command / invocation rule
- consumer adapter contract
- actual file / blob exchange protocol

をここに入れない。

## なぜ consumer adapter をまだ入れないか

actual consumer adapter を current phase で入れると、

- proof assistant / checker / exporter specific adapter surface
- adapter-local validation rule
- actual notebook exchange rule

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず handoff emitter ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain emitter-linked retained bridge

```text
proof_notebook_bridge_emitter_linked_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T00:47,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default,
  emitted_artifact_ref = emitted_artifact:e12.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.default
}
```

### example B — `try` / `atomic_cut` emitter-linked retained bridge

```text
proof_notebook_bridge_emitter_linked_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T00:47,
  review_session_state = followup_pending,
  retention_state = provisional,
  retained_path_policy_ref = retained_path_policy:phase5.followup,
  emitted_artifact_ref = emitted_artifact:e6.followup.notebook,
  handoff_emitter_ref = handoff_emitter:proof_notebook.followup
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. emitted-ready retained bridge の次段では、`handoff_emitter_ref` までは足してよい
2. `handoff_emitter_ref` は symbolic emitter ref に留め、actual emitter command / consumer adapter contract / file exchange rule を actual contract にしない
3. next later reopen は、consumer adapter threshold の comparison に置く

## この段階でまだ固定しないもの

- `consumer_adapter_ref`
- actual emitter command / invocation rule
- actual file / blob exchange protocol
- `proof_assistant_adapter` specific schema
