# 143 — current L2 theorem line bless-ready bridge review-session threshold

## 目的

`specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md`
と
`specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md`
までを前提に、

- bless-ready bridge sketch に review-session metadata をどこまで足すか
- retained notebook path / reviewer actor / timestamp / actual emitted artifact をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の bless-ready-bridge review-session threshold** であり、

- retained notebook path / overwrite / retention policy
- reviewer actor / timestamp / session lifecycle
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current bless-ready bridge sketch を起点にする。
- current named review unit bundle は維持する。
- detached validation loop の bless / retention policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. review unit の current first cut は docs-only named review unit bundle に置く。
2. named review unit の次段は docs-only notebook bridge sketch に置く。
3. bridge sketch の次段では compare basis refs までは足してよい。
4. compare-ready bridge sketch の次段では bless decision state までは足してよい。
5. retained notebook path / reviewer actor / timestamp / actual file policy は still 後段に残している。

したがって current 問いは、
**bless-ready bridge sketch に review notes ref だけを先に足してよいか、それとも review-session metadata も still bridge 外へ残すべきか**
である。

## 比較観点

1. current bless-ready bridge sketch の最小性を壊さないか
2. bless decision と human-facing review note linkage を、retention / lifecycle policy から分離できるか
3. detached validation loop の bless / retained path line を premature に混ぜないか
4. later reopen で retained notebook path / session lifecycle へ narrow に進めるか
5. `proof_assistant_adapter` line より先に進みすぎないか

## 比較対象

### 案 1. bless-ready bridge sketch を terminal cut にし、review-session metadata は bridge 外へ残す

#### 読み

bridge sketch には

- `bridge_subject_ref`
- `review_units`
- `bridge_goal_text`
- `comparison_basis_refs`
- `bless_decision_state`

だけを置き、review notes も session metadata も review prose / external note に残す。

#### 利点

- current bless-ready bridge sketch を最も動かさない
- retention / lifecycle policy と混ざる余地をさらに減らせる

#### 欠点

- bless decision の根拠や review observation の linkage まで prose 依存に残りやすい
- next later reopen の start point が弱い

### 案 2. review-note refs だけを持つ review-linked bless bridge にする

#### 読み

bless-ready bridge sketch に、
retained path や reviewer actor / timestamp を入れずに
**review_note_refs**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_review_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs
}
```

#### 利点

- bless decision の根拠となる human-facing note への linkage を narrow に導入できる
- note 本体や session lifecycle を still 後段に残せる
- retained notebook path / actor / timestamp と切り分けやすい
- later reopen を retention / lifecycle threshold へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- `review_note_refs` を actual retained file path と誤読されない説明が要る

### 案 3. full review session metadata をまとめて入れる

#### 読み

bridge sketch に

- `review_note_refs`
- `review_session_ref`
- `reviewer_actor`
- `reviewed_at`
- `retained_notebook_path`

をまとめて足し、review session bundle に近い shape へ進める。

#### 利点

- review session contract を早く語れる
- retained notebook handoff や later artifact management に直接つながる

#### 欠点

- detached validation loop の bless / retention / overwrite policy を premature に混ぜやすい
- actual emitted notebook artifact や retained path pressure と近づきすぎる
- current theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. review-note refs だけを持つ review-linked bless bridge にする**
である。

理由は次の通り。

1. bless decision と human-facing observation の linkage を narrow に bridge-level へ引き上げられる
2. note blob / session lifecycle / retained path を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first disciplineを壊さない
4. next later reopen を retained notebook path / review session lifecycle threshold へ狭く進めやすい

## minimal review-linked bless bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_review_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs
}
```

### `review_note_refs`

`review_note_refs` は、

- bridge walkthrough に対する human-facing observation / correction / caveat
- ただし note blob 自体ではなく
  **symbolic ref family**

だけを置く field である。

current task では、

- retained notebook path
- reviewer actor
- timestamp
- session id

をここに入れない。
retained artifact path や session identity をここへ押し込まず、later reopen の別 carrier に残す。

## なぜ retained path / actor / timestamp をまだ入れないか

retained path / actor / timestamp を current phase で入れると、

- notebook retention / overwrite rule
- review session lifecycle
- actual emitted notebook artifact

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず note linkage を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain review-linked bless bridge

```text
proof_notebook_bridge_review_sketch = {
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
  ]
}
```

この段階で必要なのは note への linkage までであり、
review session id や retained notebook path ではない。

### example B — `try` / `atomic_cut` review-linked bless bridge

```text
proof_notebook_bridge_review_sketch = {
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
  ]
}
```

ここでも current docs-only cut は review-note refs までで十分であり、
retained notebook file や session timestamp はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. bless-ready bridge sketch の次段では review-note refs までは足してよい
2. minimal review-linked bless bridge は `bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs + bless_decision_state + review_note_refs` に留める
3. retained notebook path / reviewer actor / timestamp / review session metadata は second step に残す
4. actual emitted notebook artifact はさらに後段に残す
5. next later reopen は、review-linked bless bridge に retained notebook path / review session lifecycle をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- retained notebook path / overwrite / retention rule
- reviewer actor / timestamp / session id
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
