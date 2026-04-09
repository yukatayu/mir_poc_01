# 144 — current L2 theorem line review-linked bless bridge retained-notebook threshold

## 目的

`specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md`、
`specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md`、
`specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md`
までを前提に、

- review-linked bless bridge に retained notebook linkage をどこまで足すか
- review session lifecycle / reviewer actor / timestamp / actual emitted notebook artifact をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の review-linked-bless-bridge retained-notebook threshold** であり、

- review session id / actor / timestamp / lifecycle state
- retained notebook overwrite / retention policy
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current review-linked bless bridge を起点にする。
- current named review unit bundle は維持する。
- detached validation loop の retained path policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. review unit の current first cut は docs-only named review unit bundle に置く。
2. named review unit の次段は docs-only notebook bridge sketch に置く。
3. bridge sketch の次段では compare basis refs までは足してよい。
4. compare-ready bridge sketch の次段では bless decision state までは足してよい。
5. bless-ready bridge sketch の次段では review-note refs までは足してよい。
6. retained notebook path / review session id / actor / timestamp / actual file policy は still 後段に残している。

したがって current 問いは、
**review-linked bless bridge に retained notebook への linkage だけを先に足してよいか、それとも retained pressure も still bridge 外へ残すべきか**
である。

## 比較観点

1. current review-linked bless bridge の最小性を壊さないか
2. review note linkage と retained notebook linkage を、retention / overwrite / lifecycle policy から分離できるか
3. detached validation loop の retained path / bless-update line を premature に混ぜないか
4. later reopen で review session lifecycle へ narrow に進めるか
5. actual emitted notebook artifact や `proof_assistant_adapter` line より先に進みすぎないか

## 比較対象

### 案 1. review-linked bless bridge を terminal cut にし、retained notebook linkage も bridge 外へ残す

#### 読み

bridge sketch には

- `bridge_subject_ref`
- `review_units`
- `bridge_goal_text`
- `comparison_basis_refs`
- `bless_decision_state`
- `review_note_refs`

だけを置き、retained notebook linkage も external note / prose に残す。

#### 利点

- current review-linked bless bridge を最も動かさない
- retention / overwrite policy と混ざる余地をさらに減らせる

#### 欠点

- bless decision と review notes を後で何に retain したかの linkage が still prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. retained-notebook ref だけを持つ retained-ready bless bridge にする

#### 読み

review-linked bless bridge に、
review session actor / timestamp / lifecycle を入れずに
**`retained_notebook_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref
}
```

#### 利点

- bless decision / review-note linkage と retained notebook linkage を narrow に橋渡しできる
- actual path string / overwrite / retention policy を still 後段に残せる
- review session lifecycle と分離したまま later reopen を狭く進めやすい
- detached validation loop の path policy line と theorem-line bridge を同一視しなくてよい

#### 欠点

- bridge-level field が 1 つ増える
- `retained_notebook_ref` を actual file path や emitted notebook artifact id と誤読されない説明が要る

### 案 3. retained notebook linkage と review session metadata をまとめて入れる

#### 読み

bridge sketch に

- `retained_notebook_ref`
- `review_session_ref`
- `reviewer_actor`
- `reviewed_at`

をまとめて足し、review-session bundle に近い shape へ進める。

#### 利点

- review-linked bless bridge から retained notebook handoff まで一直線につながる
- review session contract を早く語れる

#### 欠点

- review session lifecycle / retained path / overwrite policy を premature に混ぜやすい
- actual emitted notebook artifact pressure と近づきすぎる
- current theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. retained-notebook ref だけを持つ retained-ready bless bridge にする**
である。

理由は次の通り。

1. bless decision と review-note linkage を保ったまま、retained notebook 側への接点だけを narrow に追加できる
2. actual path string / overwrite / retention policy / session lifecycle を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を review session lifecycle threshold へ狭く進めやすい

## minimal retained-ready bless bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref
}
```

### `retained_notebook_ref`

`retained_notebook_ref` は、

- bless-ready / review-linked bridge から後で参照される retained notebook candidate
- ただし actual path string や emitted file id ではなく
  **symbolic retained target ref**

だけを置く field である。

current task では、

- actual filesystem path
- overwrite rule
- retention policy
- emitted notebook artifact id

をここに入れない。
actual retained path policy や emitted artifact management は later reopen の別 carrier に残す。

## なぜ actual path / overwrite rule をまだ入れないか

actual path / overwrite rule を current phase で入れると、

- detached validation loop の path policy
- retained artifact management
- actual emitted notebook artifact

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず retained notebook への symbolic linkage を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-ready bless bridge

```text
proof_notebook_bridge_retained_sketch = {
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
  retained_notebook_ref = retained_notebook:e12.latest
}
```

この段階で必要なのは retained notebook への linkage までであり、
actual file path や overwrite rule ではない。

### example B — `try` / `atomic_cut` retained-ready bless bridge

```text
proof_notebook_bridge_retained_sketch = {
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
  retained_notebook_ref = retained_notebook:e6.followup
}
```

ここでも current docs-only cut は retained notebook への linkage までで十分であり、
actual emitted notebook file や session actor はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. review-linked bless bridge の次段では、retained-notebook ref までは足してよい
2. minimal retained-ready bless bridge は `bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs + bless_decision_state + review_note_refs + retained_notebook_ref` に留める
3. review session id / actor / timestamp / lifecycle は still 後段に残す
4. actual retained path / overwrite / retention policy はさらに後段に残す
5. next later reopen は、retained-ready bless bridge に review session lifecycle metadata をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- review session id / actor / timestamp / lifecycle state
- retained notebook actual path / overwrite / retention rule
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
