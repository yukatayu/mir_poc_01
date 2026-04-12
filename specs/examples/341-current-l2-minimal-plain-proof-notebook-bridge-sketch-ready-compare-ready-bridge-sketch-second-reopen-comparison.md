# 341 — current L2 minimal-plain-proof-notebook-bridge-sketch-ready compare-ready bridge-sketch second reopen comparison

## 目的

`specs/examples/340-current-l2-plain-proof-notebook-bridge-sketch-actualization-ready-minimal-plain-proof-notebook-bridge-sketch-threshold.md`
で plain proof-notebook bridge sketch actualization の minimum を fixed した次段として、

- compare-ready bridge sketch を current Phase 6 line でどこまで reopen するか
- `specs/examples/141` で切った compare basis-only threshold をそのまま current cut に持ち直すか
- bless / review-session / retained-path / helper-emitter / concrete tool binding をどこまで still later に残すか

を比較する。

ここで固定するのは
**current L2 minimal-plain-proof-notebook-bridge-sketch-ready compare-ready bridge-sketch second reopen comparison**
であり、

- bless decision state
- review-session metadata
- helper / emitter code anchor
- concrete theorem / model-check tool binding
- deferred `e3` actualization reopen timing

はまだ固定しない。

## scope

- entry criteria は `specs/examples/327...340` で fixed 済みの theorem-first review-unit pilot、bridge-sketch reopen ordering、`e3` guard comparison、plain bridge sketch actualizationに置く。
- current theorem-side actualization は docs-first に留める。
- compare-ready bridge sketch は `comparison_basis_refs` だけを扱い、bless/session/path metadata は扱わない。

## current 前提

current repo では次が成立している。

1. theorem-side current cut は row-local `proof_notebook_review_unit` を entry にする docs-only bridge sketch まで actualize 済みである。
2. bridge-sketch reopen ordering は plain docs-only bridge sketch first / compare-ready bridge sketch second に fixed 済みである。
3. old theorem-line `specs/examples/141` では compare-ready bridge sketch の最小 shape を `comparison_basis_refs` だけに留める current cut を already 持っている。
4. current immediate next line は compare-ready bridge sketch second reopen であり、その後に deferred `e3` actualization reopen timing を置く current reading に揃っている。

したがって current 問いは、
**Phase 6 current line として、`specs/examples/141` の compare basis-only bridge sketch shape をそのまま second reopen package に落とすのが自然か**
である。

## 比較観点

1. plain docs-only bridge sketch current cut を壊さないか
2. compare need と bless / review-session / retained-path pressure を分離できるか
3. helper / emitter と concrete tool binding を premature に混ぜないか
4. deferred `e3` actualization reopen timing を still later に残せるか

## 比較対象

### 案 1. `comparison_basis_refs` だけを足した compare-ready bridge sketch を second reopen にする

#### shape

```text
proof_notebook_bridge_compare_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs
}
```

#### `comparison_basis_refs`

`comparison_basis_refs` は、

- prior bridge sketch
- prior review-unit family
- detached aggregate / static-gate / runtime evidence family

のような **compare の基準として辿る typed symbolic refs**
に留める。

#### 利点

- `specs/examples/141` の current theorem-line cut を Phase 6 current lineへそのまま持ち直せる。
- compare need だけを narrow に受け止め、bless/session/path metadata を still later に残せる。
- deferred `e3` actualization reopen timing を compare-ready line の次段に保ちやすい。

#### 欠点

- bridge-level field が 1 つ増える。

### 案 2. plain bridge sketch を terminal cut に保ち、compare basis も still bridge 外へ残す

#### 利点

- current docs-only bridge sketch を最も動かさない。

#### 欠点

- compare-ready second reopen を current mainline に置く意味が弱くなる。
- later `e3` timing line との間に narrow reopen point を repository memory として残しにくい。

### 案 3. compare-ready bridge sketch に bless / review-session metadata も一緒に入れる

#### 利点

- compare / bless-like review flow へ直接つながる。

#### 欠点

- detached validation loop の bless / retention / overwrite policy を premature に混ぜやすい。
- helper / emitter / concrete tool binding とも距離が近すぎる。

## current judgment

current L2 で最も自然なのは、
**案 1. `comparison_basis_refs` だけを足した compare-ready bridge sketch を second reopen にする**
である。

理由は次の通り。

1. `specs/examples/141` の compare basis-only cut を、そのまま Phase 6 current theorem-side lineへ持ち直せる。
2. current plain bridge sketch の docs-only discipline を保ったまま、compare need だけを narrow に actualize できる。
3. bless / review-session / retained-path / helper-emitter / concrete tool binding を still later に残せる。

## current first choice details

- second reopen package は docs-only compare-ready bridge sketch shape に留める。
- `comparison_basis_refs` は typed symbolic refs に留め、actual retained artifact id や review-session id は持ち込まない。
- bless decision state、review-session metadata、helper / emitter code anchor、concrete theorem / model-check tool binding は still later に残す。
- deferred `e3` actualization reopen timing は compare-ready package の次段に残す。

## next promoted line

next promoted line は、
**compare-ready-bridge-sketch-second-reopen-ready deferred-e3-actualization-reopen-timing**
に置く。

## open questions

- deferred `e3` actualization reopen timing を compare-ready bridge sketch の直後に置く current cutをどこまで minimum に残すか
- `comparison_basis_refs` の first practical current family を prior bridge / review-unit refs と detached evidence refs のどこまでに留めるか
- later helper / emitter code anchor を compare-ready package にも still 入れない current cutを保つか
