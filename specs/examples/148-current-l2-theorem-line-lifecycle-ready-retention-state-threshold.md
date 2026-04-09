# 148 — current L2 theorem line lifecycle-ready retention-state threshold

## 目的

`specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md`
と
`specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md`
までを前提に、

- lifecycle-ready retained bridge に retention state をどこまで足すか
- actual retained path policy / emitted artifact pressure をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の lifecycle-ready retention-state threshold** であり、

- actual retained path policy
- emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current lifecycle-ready retained bridge を起点にする。
- detached validation loop の retained path policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. review actor / timestamp も observed-session bridge までは足してよい。
3. review session lifecycle state も lifecycle-ready bridge までは足してよい。
4. retention state / actual retained path policy / emitted artifact pressure は still 後段に残している。

したがって current 問いは、
**lifecycle-ready retained bridge に retention state だけを先に足してよいか、それとも retention progression も still bridge 外へ残すべきか**
である。

## 比較観点

1. current lifecycle-ready retained bridge の最小性を壊さないか
2. session progression と retention progression を、actual retained path policy から分離できるか
3. detached validation loop の retained path / bless-update line を premature に混ぜないか
4. later reopen で retained path policy / emitted artifact threshold へ narrow に進めるか
5. full retention machine を actual contract に誤昇格しないか

## 比較対象

### 案 1. lifecycle-ready retained bridge を terminal cut にし、retention state も bridge 外へ残す

#### 読み

bridge sketch には

- `reviewed_by_ref`
- `reviewed_at_ref`
- `review_session_state`

までを置き、`retention_state` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- actual retained path policy や emitted artifact と混ざる余地をさらに減らせる

#### 欠点

- session progression と retention progression の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. retention state だけを持つ retention-ready retained bridge にする

#### 読み

lifecycle-ready retained bridge に、
actual retained path policy や emitted artifact pressure を入れずに
**`retention_state`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retention_sketch = {
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
  retention_state
}
```

#### 利点

- session progression と retention progression を narrow に橋渡しできる
- actual retained path policy / emitted artifact を still 後段に残せる
- next later reopen を path-policy / emitted-artifact threshold へ狭く進めやすい
- `retention_state` を symbolic state tag に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `retention_state` を overwrite policy / concrete storage rule と誤読されない説明が要る

### 案 3. retention state と path policy をまとめて入れる

#### 読み

bridge sketch に

- `retention_state`
- `retained_path_policy_ref`
- `emitted_artifact_ref`

をまとめて足し、retained notebook lifecycle bundle に近い shape へ進める。

#### 利点

- retention progression と path handling を一度に語れる
- actual handoff / artifact management へ近づく

#### 欠点

- actual retained path policy / emitted artifact pressure を premature に混ぜやすい
- current theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. retention state だけを持つ retention-ready retained bridge にする**
である。

理由は次の通り。

1. session progression と retention progression を narrow に接続できる
2. actual retained path policy / emitted artifact を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を retained path policy / emitted artifact threshold へ狭く進めやすい

## minimal retention-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retention_sketch = {
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
  retention_state
}
```

### `retention_state`

`retention_state` は、

- current retained notebook linkage がどの retention progress tag にいるかを示す
  **symbolic state tag**

だけを置く field である。

current task では、

- actual retained path policy
- overwrite rule
- emitted artifact state

をここに入れない。
retention progression の tag と retention policy 本体は別 carrier に切り分け、later reopen に残す。

## なぜ path policy をまだ入れないか

path policy を current phase で入れると、

- retained notebook overwrite / relocation rule
- actual retained path policy
- emitted notebook artifact management

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず retention progression tag を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retention-ready retained bridge

```text
proof_notebook_bridge_retention_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-09T23:59,
  review_session_state = blessed,
  retention_state = retained
}
```

### example B — `try` / `atomic_cut` retention-ready retained bridge

```text
proof_notebook_bridge_retention_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T00:05,
  review_session_state = followup_pending,
  retention_state = provisional
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. lifecycle-ready retained bridge の次段では、`retention_state` までは足してよい
2. `retention_state` は symbolic state tag に留め、overwrite / relocation / concrete storage rule を actual contract にしない
3. actual retained path policy / emitted notebook artifact は still 後段に残す
4. next later reopen は、retained path policy / emitted artifact threshold の comparison に置く

## この段階でまだ固定しないもの

- `retained_path_policy_ref`
- actual retained path string / URI / path prefix
- overwrite / relocation / GC rule
- actual emitted notebook artifact family
- `proof_assistant_adapter` specific schema
