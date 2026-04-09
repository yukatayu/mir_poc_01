# 147 — current L2 theorem line observed-session lifecycle threshold

## 目的

`specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md`
と
`specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md`
までを前提に、

- observed session-linked retained bridge に review session lifecycle state をどこまで足すか
- retention state / actual retained path policy / emitted artifact pressure をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の observed-session lifecycle threshold** であり、

- retention state / overwrite policy
- actual retained path / emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current observed session-linked retained bridge を起点にする。
- detached validation loop の retained path policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. review actor / timestamp も observed-session bridge までは足してよい。
3. review session lifecycle state / retention state / actual retained path policy は still 後段に残している。

したがって current 問いは、
**observed session-linked retained bridge に lifecycle state だけを先に足してよいか、それとも session progression も still bridge 外へ残すべきか**
である。

## 比較観点

1. current observed session-linked retained bridge の最小性を壊さないか
2. review observation と session progression を、retention state / actual retained path policy から分離できるか
3. detached validation loop の bless / retention line を premature に混ぜないか
4. later reopen で retention state / actual emitted artifact threshold へ narrow に進めるか
5. full session machine を actual contract に誤昇格しないか

## 比較対象

### 案 1. observed session-linked retained bridge を terminal cut にし、lifecycle state も bridge 外へ残す

#### 読み

bridge sketch には

- `reviewed_by_ref`
- `reviewed_at_ref`

までを置き、`review_session_state` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- retention state / actual path policy と混ざる余地をさらに減らせる

#### 欠点

- review observation と session progression の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. lifecycle state だけを持つ lifecycle-ready retained bridge にする

#### 読み

observed session-linked retained bridge に、
retention state や actual retained path policy を入れずに
**`review_session_state`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_lifecycle_sketch = {
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
  review_session_state
}
```

#### 利点

- review observation と session progression を narrow に橋渡しできる
- retention state / actual path policy / emitted artifact を still 後段に残せる
- next later reopen を retention / emitted-artifact threshold へ狭く進めやすい
- `review_session_state` を symbolic state tag に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `review_session_state` を full state machine / transition policy と誤読されない説明が要る

### 案 3. lifecycle state と retention state をまとめて入れる

#### 読み

bridge sketch に

- `review_session_state`
- `retention_state`
- `retained_path_policy_ref`

をまとめて足し、retained notebook lifecycle bundle に近い shape へ進める。

#### 利点

- session progression と retention progression を一度に語れる
- actual handoff / artifact management へ近づく

#### 欠点

- retention / overwrite / actual path policy を premature に混ぜやすい
- emitted artifact pressure と近づきすぎる
- current theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. lifecycle state だけを持つ lifecycle-ready retained bridge にする**
である。

理由は次の通り。

1. observation と lifecycle を narrow に接続できる
2. retention state / actual path policy / emitted artifact を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を retention / emitted-artifact threshold へ狭く進めやすい

## minimal lifecycle-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_lifecycle_sketch = {
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
  review_session_state
}
```

### `review_session_state`

`review_session_state` は、

- current review walkthrough がどの progress tag にいるかを示す
  **symbolic state tag**

だけを置く field である。

current task では、

- transition policy
- retention state
- actual retained path policy
- emitted artifact state

をここに入れない。
session progression の tag と lifecycle rule 本体は別 carrier に切り分け、later reopen に残す。

## なぜ retention state をまだ入れないか

retention state を current phase で入れると、

- retained notebook overwrite / retention rule
- actual retained path policy
- emitted notebook artifact management

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず session progression tag を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain lifecycle-ready retained bridge

```text
proof_notebook_bridge_lifecycle_sketch = {
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
  review_session_state = blessed
}
```

### example B — `try` / `atomic_cut` lifecycle-ready retained bridge

```text
proof_notebook_bridge_lifecycle_sketch = {
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
  review_session_state = followup_pending
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. observed session-linked retained bridge の次段では、`review_session_state` までは足してよい
2. `review_session_state` は symbolic state tag に留め、transition policy や full state machine を actual contract にしない
3. retention state / actual retained path policy / emitted notebook artifact は still 後段に残す
4. next later reopen は、retention state / actual retained path policy / emitted artifact threshold の comparison に置く

## この段階でまだ固定しないもの

- retention state / overwrite policy
- actual retained path / emitted artifact id
- lifecycle transition policy
- `proof_assistant_adapter` specific schema
- public checker API
