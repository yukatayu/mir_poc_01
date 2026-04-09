# 145 — current L2 theorem line retained bridge review-session link threshold

## 目的

`specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md`
と
`specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md`
までを前提に、

- retained-ready bless bridge に review session linkage をどこまで足すか
- reviewer actor / timestamp / lifecycle state / actual retained path policy をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-bridge review-session-link threshold** であり、

- reviewer actor / timestamp
- review session state machine
- retained notebook overwrite / retention policy
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-ready bless bridge を起点にする。
- detached validation loop の retained path policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. docs-only notebook bridge sketch の次段では compare basis refs までは足してよい。
2. その次段では bless decision state までは足してよい。
3. その次段では review-note refs までは足してよい。
4. その次段では retained-notebook ref までは足してよい。
5. review session id / actor / timestamp / lifecycle state / actual retained path policy は still 後段に残している。

したがって current 問いは、
**retained-ready bless bridge に review session linkage だけを先に足してよいか、それとも session metadata も still bridge 外へ残すべきか**
である。

## 比較観点

1. current retained-ready bless bridge の最小性を壊さないか
2. retained notebook linkage と review session linkage を、actor / timestamp / lifecycle state から分離できるか
3. detached validation loop の retention / bless-update line を premature に混ぜないか
4. later reopen で actor / timestamp / lifecycle state へ narrow に進めるか
5. actual emitted notebook artifact や full review-session bundle より先に進みすぎないか

## 比較対象

### 案 1. retained-ready bless bridge を terminal cut にし、review session linkage も bridge 外へ残す

#### 読み

bridge sketch には

- `bridge_subject_ref`
- `review_units`
- `bridge_goal_text`
- `comparison_basis_refs`
- `bless_decision_state`
- `review_note_refs`
- `retained_notebook_ref`

だけを置き、review session linkage も external note / prose に残す。

#### 利点

- current retained-ready bless bridge を最も動かさない
- actor / timestamp / lifecycle policy と混ざる余地をさらに減らせる

#### 欠点

- retained notebook linkage と review session の対応が prose 依存に残りやすい
- next later reopen の start point が弱い

### 案 2. review-session ref だけを持つ session-linked retained bridge にする

#### 読み

retained-ready bless bridge に、
reviewer actor / timestamp / lifecycle state を入れずに
**`review_session_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_session_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref
}
```

#### 利点

- retained notebook linkage と review session linkage を narrow にまとめられる
- actor / timestamp / lifecycle state を still 後段に残せる
- actual emitted notebook artifact や overwrite policy と切り分けやすい
- next later reopen を session lifecycle state threshold へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- `review_session_ref` を session blob や actor/timestamp carrier と誤読されない説明が要る

### 案 3. full review session metadata をまとめて入れる

#### 読み

bridge sketch に

- `review_session_ref`
- `reviewer_actor`
- `reviewed_at`
- `review_session_state`
- `retention_state`

をまとめて足し、retained notebook handoff bundle に近い shape へ進める。

#### 利点

- review session contract を早く語れる
- retained notebook lifecycle と session lifecycle を一直線に結べる

#### 欠点

- actor / timestamp / state machine を premature に混ぜやすい
- actual retained path policy や emitted artifact pressure と近づきすぎる
- current theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. review-session ref だけを持つ session-linked retained bridge にする**
である。

理由は次の通り。

1. retained notebook linkage と review session linkage を narrow に橋渡しできる
2. actor / timestamp / lifecycle state / retention policy を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を actor / timestamp / lifecycle-state threshold へ狭く進めやすい

## minimal session-linked retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_session_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref
}
```

### `review_session_ref`

`review_session_ref` は、

- bless decision / review-note linkage / retained notebook linkage を束ねる
  **symbolic review session ref**

だけを置く field である。

current task では、

- reviewer actor
- timestamp
- lifecycle state
- retention state

をここに入れない。
session identity と session state machine は別 carrier に切り分け、later reopen に残す。

## なぜ actor / timestamp / lifecycle state をまだ入れないか

actor / timestamp / lifecycle state を current phase で入れると、

- review session state machine
- retained notebook overwrite / retention policy
- actual emitted notebook artifact

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず session linkage を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain session-linked retained bridge

```text
proof_notebook_bridge_session_sketch = {
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
  review_session_ref = review_session:e12.walkthrough
}
```

この段階で必要なのは session linkage までであり、
reviewer actor や reviewed timestamp ではない。

### example B — `try` / `atomic_cut` session-linked retained bridge

```text
proof_notebook_bridge_session_sketch = {
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
  review_session_ref = review_session:e6.rollback_walkthrough
}
```

ここでも current docs-only cut は session linkage までで十分であり、
actor / timestamp / lifecycle state はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. retained-ready bless bridge の次段では、review-session ref までは足してよい
2. minimal session-linked retained bridge は `bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs + bless_decision_state + review_note_refs + retained_notebook_ref + review_session_ref` に留める
3. reviewer actor / timestamp / lifecycle state は still 後段に残す
4. retained notebook actual path / overwrite / retention rule は still 後段に残す
5. actual emitted notebook artifact はさらに後段に残す
6. next later reopen は、session-linked retained bridge に actor / timestamp / lifecycle state をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- reviewer actor / timestamp
- review session lifecycle state
- retained notebook overwrite / retention policy
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
