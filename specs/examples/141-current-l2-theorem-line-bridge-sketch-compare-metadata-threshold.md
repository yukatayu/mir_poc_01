# 141 — current L2 theorem line bridge sketch compare metadata threshold

## 目的

`specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`、
`specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`、
`specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md`
までを前提に、

- docs-only notebook bridge sketch に compare-like metadata をどこまで足すか
- bless / retained file / review session policy をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の bridge-sketch compare-metadata threshold** であり、

- bless decision state
- reviewer notes lifecycle
- retained notebook path / overwrite / retention policy
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current docs-only notebook bridge sketch を起点にする。
- current named review unit bundle は維持する。
- detached validation loop の bless / retention policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. concrete notebook workflow pressure の first threshold は human review checklist / walkthrough pressure に置く。
2. review unit の current first cut は docs-only named review unit bundle に置く。
3. named review unit の次段は docs-only notebook bridge sketch に置く。
4. stronger compare / bless-like review flow metadata は still 後段に残している。

したがって current 問いは、
**bridge sketch に compare basis だけを先に足してよいか、それとも bless / review session metadata と一緒にしか導入しない方がよいか**
である。

## 比較観点

1. current docs-only bridge sketch の最小性を壊さないか
2. compare needs と bless / retention policy を分離できるか
3. detached validation loop の bless/update line を premature に混ぜないか
4. later reopen で bless decision state へ narrow に進めるか
5. `proof_assistant_adapter` line より先に進みすぎないか

## 比較対象

### 案 1. bridge sketch は pure docs-only shape のまま維持する

#### 読み

bridge sketch には

- `bridge_subject_ref`
- `review_units`
- `bridge_goal_text`

だけを置き、compare basis も持たせない。

#### 利点

- current bridge sketch を最も動かさない
- bless / retention line と混ざる余地をさらに減らせる

#### 欠点

- compare walkthrough の再利用単位が prose 依存に戻りやすい
- bridge sketch の次段を narrow に切る start point が弱い

### 案 2. compare basis refs だけを持つ compare-ready bridge sketch にする

#### 読み

bridge sketch に、bless decision state を入れずに
**compare basis refs**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_compare_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs
}
```

#### 利点

- compare need と bless policy を分離できる
- row / bridge walkthrough を side-by-side に読む次段の pressure を受け止めやすい
- retained path / overwrite policy / review session lifecycle をまだ固定しなくてよい
- later reopen で bless decision metadata を narrow に足しやすい

#### 欠点

- bridge-level field が 1 つ増える
- compare basis ref の説明を甘くすると actual artifact ref と誤読されやすい

### 案 3. compare / bless-like review flow metadata をまとめて入れる

#### 読み

bridge sketch に

- `comparison_basis_refs`
- `reviewer_notes`
- `bless_decision`

をまとめて足し、review session bundle に近い shape へ進める。

#### 利点

- compare / bless-like flow へ直接つながる
- stronger review contract を早く語れる

#### 欠点

- detached validation loop の bless / retention / overwrite policy を premature に混ぜやすい
- review session lifecycle を current theorem-line docs-only bridge に持ち込みすぎる
- actual emitted artifact や retained notebook path と近づきすぎる

## current judgment

current L2 で最も自然なのは、
**案 2. compare basis refs だけを持つ compare-ready bridge sketch にする**
である。

理由は次の通り。

1. current bridge sketch の docs-only discipline を保ったまま、compare need だけを narrow に受け止められる
2. bless decision / retained path / overwrite policy を still 後段に残せる
3. `proof_notebook` first bridge の current phase に必要な pressure を最も小さく切れる
4. next later reopen を bless decision state / review session metadata threshold へ狭く進めやすい

## minimal compare-ready bridge sketch

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_compare_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs
}
```

### `comparison_basis_refs`

`comparison_basis_refs` は、

- prior bridge sketch
- prior review unit family
- detached aggregate / static gate source evidence

のような **compare の基準として辿る ref**
だけを置く。

ここには

- `bless_decision`
- retained notebook path
- reviewer session id

を入れない。
review-session metadata や retained artifact metadata は later carrier に残す。

## なぜ bless decision state をまだ入れないか

bless decision state を current phase で入れると、

- detached validation loop の bless / update policy
- notebook retention / overwrite rule
- review session lifecycle

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず compare basis を安定に切るところまでで十分である。

## practical examples

### example A — fallback chain compare-ready bridge

```text
proof_notebook_bridge_compare_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の normalization family を walkthrough する",
  comparison_basis_refs = [
    bridge_sketch:e12.previous
  ]
}
```

ここで必要なのは compare basis だけであり、
bless decision や retained notebook file ではない。

### example B — `try` / `atomic_cut` compare-ready bridge

```text
proof_notebook_bridge_compare_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.rollback_cut_non_interference,
    review_unit:e6.rollback_frontier_reading
  ],
  bridge_goal_text = "rollback family を side-by-side で見直す",
  comparison_basis_refs = [
    runtime_cluster:e6,
    static_gate:e6
  ]
}
```

この段階でも compare basis までで十分であり、
review session state はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. bridge sketch の次段では compare basis refs までは足してよい
2. minimal compare-ready bridge sketch は `bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs` に留める
3. bless decision / reviewer notes / retained notebook path は second step に残す
4. actual emitted notebook artifact はさらに後段に残す
5. next later reopen は、compare-ready bridge sketch に bless decision state をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- bless decision state
- reviewer notes lifecycle
- retained notebook path / overwrite / retention rule
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
