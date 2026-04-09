# 140 — current L2 theorem line review unit to bridge sketch comparison

## 目的

`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`、
`specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`、
`specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`
までを前提に、

- docs-only named review unit を stronger notebook bridge sketch へどこまで拡張するか
- そのとき compare / bless-like review flow をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の review-unit-to-bridge-sketch threshold** であり、

- compare / bless policy
- reviewer notes lifecycle
- retained notebook path / overwrite policy
- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current named review unit bundle を起点にする。
- theorem-side minimum contract row core は `obligation_kind + evidence_refs` のまま維持する。
- row-local `goal_text` と human-facing `checklist` は維持する。
- detached validation loop の bless / retention policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. `proof_notebook` first bridge は current phase では docs-only derived view に留める。
2. concrete notebook workflow pressure の first threshold は human review checklist / walkthrough pressure に置く。
3. review checklist / walkthrough unit の current first cut は docs-only named review unit bundle に置く。
4. compare / bless-like flow と actual notebook file exchange はまだ後段に残す。

したがって current 問いは、
**named review unit の上に bridge-level の docs-only sketch を切ってよいか、それともまだ review unit 止まりに留めるべきか**
である。

## 比較観点

1. current named review unit bundle の最小性を壊さないか
2. notebook-level の review walkthrough を prose 以上に整理できるか
3. compare / bless / retained file pressure を premature に混ぜないか
4. `proof_assistant_adapter` line より先に進みすぎないか
5. later reopen で stronger review flow に narrow に進められるか

## 比較対象

### 案 1. named review unit を terminal cut とみなし、bridge sketch はまだ切らない

#### 読み

current docs-only named bundle は row-local review unit に留め、
bridge-level の grouping は prose 説明のまま扱う。

#### 利点

- current docs-only bridge を最も動かさない
- new bridge-level shape を増やさない
- compare / bless pressure と混ざる余地をさらに減らせる

#### 欠点

- 複数 review unit をまたぐ notebook walkthrough の再利用単位が still prose 依存になる
- review unit と bridge sketch の差を later reopen で一から説明し直しやすい

### 案 2. docs-only notebook bridge sketch を最小 cut にする

#### 読み

current named review unit bundle を複数束ねる
**docs-only bridge sketch**
を切る。

最小 shape は次に留める。

```text
proof_notebook_bridge_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text
}
```

#### 利点

- human walkthrough を notebook-level の再利用単位として整理できる
- row-local named review unit と compare / bless flow の間に 1 段の橋を置ける
- actual artifact / retained path / review session metadata をまだ混ぜない
- later reopen で stronger review flow や adapter line を narrow に比較しやすい

#### 欠点

- docs-only とはいえ bridge-level 名称と shape が 1 つ増える
- bridge-level `bridge_goal_text` を row-level `goal_text` と誤読させない説明が要る

### 案 3. compare / bless-like review flow を含む stronger bridge bundle に進める

#### 読み

bridge sketch の段階で、
review session / compare / bless decision まで含む stronger bundle に進める。

```text
proof_notebook_bridge = {
  bridge_sketch,
  previous_snapshot,
  reviewer_notes,
  bless_decision
}
```

#### 利点

- compare / bless-like notebook flow に直接つながる
- stable review flow contract を語りやすい

#### 欠点

- detached validation loop の bless / retention / overwrite policy を premature に混ぜやすい
- actual emitted artifact や retained file pressure と近づきすぎる
- current theorem-line の docs-first bridge としては still 強すぎる

## current judgment

current L2 で最も自然なのは、
**案 2. docs-only notebook bridge sketch を最小 cut にする**
である。

理由は次の通り。

1. `specs/examples/139...` で切った row-local named review unit を、そのまま notebook-level bridge へ再利用できる
2. compare / bless-like flow を巻き込まずに、bridge-level の再利用単位だけを導入できる
3. `proof_notebook` first bridge の docs-only disciplineを壊さない
4. next later reopen を compare / bless metadata threshold へ狭く進めやすい

## minimal bridge sketch

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text
}
```

### `bridge_subject_ref`

fixture cluster、room action family など、
bridge-level walkthrough が何を対象にするかを示す ref に留める。

### `review_units`

`review_units` は current named review unit bundle の ordered list であり、
bridge-level で新しい row core を作らない。

### `bridge_goal_text`

`bridge_goal_text` は notebook-level の walkthrough summary であり、
row-level `goal_text` と別に
「この walkthrough 全体で何を確認するか」
だけを書く。

## なぜ compare / bless metadata をまだ入れないか

current Phase 5 の pressure は still docs-first であり、
review session lifecycle や bless state policy までは確定していない。

したがって current bridge sketch に

- `previous_snapshot`
- `reviewer_notes`
- `bless_decision`

を入れると、
detached validation loop の policy line と theorem-line bridge が premature に混ざりやすい。

## practical examples

### example A — fallback chain bridge

```text
proof_notebook_bridge_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の normalization と no re-promotion family をまとめて walkthrough する"
}
```

ここで必要なのは ordered review units と bridge-level summary であり、
bless decision や retained notebook file ではない。

### example B — `try` / `atomic_cut` bridge

```text
proof_notebook_bridge_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.rollback_cut_non_interference,
    review_unit:e6.rollback_frontier_reading
  ],
  bridge_goal_text = "rollback frontier と atomic_cut の local reading をまとめて walkthrough する"
}
```

この case でも bridge sketch は docs-only で十分であり、
compare / bless flow を immediate に入れる必要はない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. current named review unit の次段では、docs-only notebook bridge sketch まで寄せてよい
2. minimal bridge sketch は `bridge_subject_ref + review_units + bridge_goal_text` に留める
3. review session / compare / bless metadata を含む stronger bridge bundleは second step に残す
4. actual emitted notebook artifact はさらに後段に残す
5. next later reopen は、bridge sketch に compare / bless-like review flow metadata をどこまで足すかの comparison に置く

## この段階でまだ固定しないもの

- compare / bless policy
- review session metadata
- retained notebook path / overwrite / retention rule
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
