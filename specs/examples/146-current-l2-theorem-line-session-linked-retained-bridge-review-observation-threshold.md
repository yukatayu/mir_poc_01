# 146 — current L2 theorem line session-linked retained bridge review-observation threshold

## 目的

`specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md`
と
`specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md`
までを前提に、

- session-linked retained bridge に review actor / review timestamp をどこまで足すか
- review session lifecycle state / retention state / actual retained path policy をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の session-linked-retained-bridge review-observation threshold** であり、

- review session lifecycle state
- retention state / overwrite policy
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current session-linked retained bridge を起点にする。
- detached validation loop の retained path policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. reviewer actor / reviewed timestamp / lifecycle state / retention state / actual retained path policy は still 後段に残している。

したがって current 問いは、
**session-linked retained bridge に review actor / timestamp だけを先に足してよいか、それとも observation metadata も still bridge 外へ残すべきか**
である。

## 比較観点

1. current session-linked retained bridge の最小性を壊さないか
2. review session identity と review observation を、session lifecycle state から分離できるか
3. detached validation loop の retention / bless-update line を premature に混ぜないか
4. later reopen で lifecycle state へ narrow に進めるか
5. actual retained path policy や emitted artifact pressure と混ざらないか

## 比較対象

### 案 1. session-linked retained bridge を terminal cut にし、review actor / timestamp も bridge 外へ残す

#### 読み

bridge sketch には

- `bridge_subject_ref`
- `review_units`
- `bridge_goal_text`
- `comparison_basis_refs`
- `bless_decision_state`
- `review_note_refs`
- `retained_notebook_ref`
- `review_session_ref`

だけを置き、review actor / timestamp も external note / prose に残す。

#### 利点

- current session-linked retained bridge を最も動かさない
- lifecycle state / retention state と混ざる余地をさらに減らせる

#### 欠点

- session ref が誰の review を指すかが prose 依存に残りやすい
- next later reopen の start point が弱い

### 案 2. review observation refs だけを持つ observed session-linked retained bridge にする

#### 読み

session-linked retained bridge に、
session lifecycle state を入れずに

- `reviewed_by_ref`
- `reviewed_at_ref`

だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_observed_session_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref,
  reviewed_by_ref,
  reviewed_at_ref
}
```

#### 利点

- session identity と review observation を narrow に橋渡しできる
- lifecycle state / retention state を still 後段に残せる
- actor/timestamp ref であることを明示しつつ actual actor blob を持ち込まない
- next later reopen を lifecycle-state threshold へ狭く進めやすい

#### 欠点

- bridge-level field が 2 つ増える
- `reviewed_at_ref` を actual wall-clock policy や replay ordering carrier と誤読されない説明が要る

### 案 3. review observation と lifecycle state をまとめて入れる

#### 読み

bridge sketch に

- `reviewed_by_ref`
- `reviewed_at_ref`
- `review_session_state`
- `retention_state`

をまとめて足し、full review-session bundle に近い shape へ進める。

#### 利点

- review observation と lifecycle を一度に語れる
- retained notebook lifecycle と session progression を一直線に結べる

#### 欠点

- session state machine / retention state を premature に混ぜやすい
- actual retained path policy や emitted artifact pressure と近づきすぎる
- current theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. review observation refs だけを持つ observed session-linked retained bridge にする**
である。

理由は次の通り。

1. review session ref と review observation を narrow に橋渡しできる
2. lifecycle state / retention state / actual path policy を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を lifecycle-state threshold へ狭く進めやすい

## minimal observed session-linked retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_observed_session_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref,
  reviewed_by_ref,
  reviewed_at_ref
}
```

### `reviewed_by_ref`

`reviewed_by_ref` は、

- session-linked retained bridge に関与した reviewer / maintainer principal
- ただし actor blob や auth policy stack ではなく
  **symbolic actor ref**

だけを置く。

### `reviewed_at_ref`

`reviewed_at_ref` は、

- review observation の timing anchor
- ただし replay ordering rule や wall-clock policy ではなく
  **symbolic timestamp ref**

だけを置く。

current task では、

- review session lifecycle state
- retention state
- actual retained path policy

をまだここへ入れない。

## なぜ lifecycle state をまだ入れないか

lifecycle state を current phase で入れると、

- review session progression rule
- retention state
- actual emitted notebook artifact

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず review observation を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain observed session-linked retained bridge

```text
proof_notebook_bridge_observed_session_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-09T23:59
}
```

ここで必要なのは observation への linkage までであり、
session state machine や retention state ではない。

### example B — `try` / `atomic_cut` observed session-linked retained bridge

```text
proof_notebook_bridge_observed_session_sketch = {
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
  reviewed_at_ref = timestamp:2026-04-10T00:05
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. session-linked retained bridge の次段では、`reviewed_by_ref + reviewed_at_ref` までは足してよい
2. review observation は symbolic ref に留め、actor blob / time policy を actual contract にしない
3. session lifecycle state / retention state / actual retained path policy は still 後段に残す
4. next later reopen は、observed session-linked retained bridge に lifecycle state をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- review session lifecycle state
- retention state / overwrite policy
- actual retained path / emitted artifact id
- `proof_assistant_adapter` specific schema
- public checker API
