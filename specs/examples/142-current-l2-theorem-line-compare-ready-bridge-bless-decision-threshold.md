# 142 — current L2 theorem line compare-ready bridge bless-decision threshold

## 目的

`specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`、
`specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md`、
`specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md`
までを前提に、

- compare-ready bridge sketch に bless decision state をどこまで足すか
- reviewer notes / retained notebook path / review session lifecycle をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の compare-ready-bridge bless-decision threshold** であり、

- reviewer notes lifecycle
- retained notebook path / overwrite / retention policy
- review session id / actor / timestamp carrier
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current compare-ready bridge sketch を起点にする。
- current named review unit bundle は維持する。
- detached validation loop の bless / retention policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. concrete notebook workflow pressure の first threshold は human review checklist / walkthrough pressure に置く。
2. review unit の current first cut は docs-only named review unit bundle に置く。
3. named review unit の次段は docs-only notebook bridge sketch に置く。
4. bridge sketch の次段では compare basis refs までは足してよい。
5. reviewer notes / retained notebook path / review session metadata は still 後段に残している。

したがって current 問いは、
**compare-ready bridge sketch に bless decision state だけを先に足してよいか、それとも bless pressure も still bridge 外へ残すべきか**
である。

## 比較観点

1. current compare-ready bridge sketch の最小性を壊さないか
2. compare basis と bless decision を、review session lifecycle から分離できるか
3. detached validation loop の bless / retention line を premature に混ぜないか
4. later reopen で reviewer notes / retained path へ narrow に進めるか
5. `proof_assistant_adapter` line より先に進みすぎないか

## 比較対象

### 案 1. bless decision も bridge 外へ残し、compare-ready bridge sketch を terminal cut にする

#### 読み

bridge sketch には

- `bridge_subject_ref`
- `review_units`
- `bridge_goal_text`
- `comparison_basis_refs`

だけを置き、bless decision は review prose / external note に残す。

#### 利点

- current compare-ready bridge sketch を最も動かさない
- reviewer notes / retained path / session lifecycle と混ざる余地をさらに減らせる

#### 欠点

- compare の次段として最も自然な bless pressure まで prose 依存に残りやすい
- bridge sketch の narrow reopening point が弱い

### 案 2. bless decision state だけを持つ bless-ready bridge sketch にする

#### 読み

compare-ready bridge sketch に、
reviewer notes や retained path を入れずに
**bless decision state**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_bless_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state
}
```

#### 利点

- compare basis と bless decision を narrow に bridge-level へ引き上げられる
- reviewer notes / retained path / session lifecycle を still 後段に残せる
- later reopen を review-session metadata threshold へ狭く進めやすい
- detached validation loop の bless/update line と theorem-line bridge の接点を最小に保てる

#### 欠点

- bridge-level field が 1 つ増える
- `bless_decision_state` の symbolic vocabulary を final enum と誤読されない説明が要る

### 案 3. bless decision と review session metadata をまとめて入れる

#### 読み

bridge sketch に

- `bless_decision_state`
- `reviewer_notes`
- `review_session_ref`
- `retained_notebook_path`

をまとめて足し、review session bundle に近い shape へ進める。

#### 利点

- compare / bless-like flow から retained notebook handoff まで一直線につながる
- review session contract を早く語れる

#### 欠点

- detached validation loop の bless / retention / overwrite policy を premature に混ぜやすい
- review session lifecycle を theorem-line bridge に持ち込みすぎる
- actual emitted notebook artifact や retained path pressure と近づきすぎる

## current judgment

current L2 で最も自然なのは、
**案 2. bless decision state だけを持つ bless-ready bridge sketch にする**
である。

理由は次の通り。

1. current bridge sketch の docs-only disciplineを保ったまま、compare の次段に最も近い bless pressureだけを narrow に受け止められる
2. reviewer notes / retained path / review session lifecycle を still 後段に残せる
3. `proof_notebook` first bridge の current phase に必要な pressure を最も小さく切れる
4. next later reopen を reviewer notes / retained path threshold へ狭く進めやすい

## minimal bless-ready bridge sketch

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_bless_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state
}
```

### `bless_decision_state`

`bless_decision_state` は、

- compare-ready bridge sketch の current read に対して
- bless / accept / revise-request のような
  **short symbolic decision state**

だけを置く field である。

current task では、

- final closed enum
- reviewer actor
- timestamp
- retained notebook path

をここに入れない。
`accepted` / `revise_requested` のような label は example 用の短い symbolic tag であり、final closed enum としてはまだ固定しない。

## なぜ reviewer notes / retained path をまだ入れないか

reviewer notes / retained path を current phase で入れると、

- detached validation loop の bless / update policy
- notebook retention / overwrite rule
- review session lifecycle

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず bless decision state を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain bless-ready bridge

```text
proof_notebook_bridge_bless_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の normalization family を walkthrough する",
  comparison_basis_refs = [
    bridge_sketch:e12.previous
  ],
  bless_decision_state = accepted
}
```

この段階で必要なのは bless decision state までであり、
review session id や retained notebook path ではない。

### example B — `try` / `atomic_cut` bless-ready bridge

```text
proof_notebook_bridge_bless_sketch = {
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
  bless_decision_state = revise_requested
}
```

ここでも current docs-only cut は bless decision state までで十分であり、
reviewer note blob や retained notebook file はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. compare-ready bridge sketch の次段では bless decision state までは足してよい
2. minimal bless-ready bridge sketch は `bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs + bless_decision_state` に留める
3. reviewer notes / retained notebook path / review session metadata は second step に残す
4. actual emitted notebook artifact はさらに後段に残す
5. next later reopen は、bless-ready bridge sketch に review-session metadata をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- reviewer notes lifecycle
- retained notebook path / overwrite / retention rule
- review session id / actor / timestamp
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
