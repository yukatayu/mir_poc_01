# 339 — current L2 minimal-third-widened-row-e3-guard-ready plain proof-notebook bridge-sketch actualization comparison

## 目的

`specs/examples/338-current-l2-third-widened-row-e3-theorem-side-formal-hook-guard-comparison-ready-minimal-third-widened-row-e3-guard-threshold.md`
で third widened row `e3` theorem-side / formal-hook guard comparison の minimum を fixed した次段として、

- theorem-side plain bridge sketch を current Phase 6 line でどこまで actualize するか
- `specs/examples/140` で切った docs-only bridge sketch をそのまま current cut に再利用するか
- compare-ready metadata / helper / concrete tool binding をどこまで still later に残すか

を比較する。

ここで固定するのは
**current L2 minimal-third-widened-row-e3-guard-ready plain proof-notebook bridge-sketch actualization comparison**
であり、

- compare-ready bridge sketch
- compare-bless metadata
- helper / emitter code anchor
- concrete theorem / model-check tool binding
- deferred `e3` actualization reopen

はまだ固定しない。

## scope

- entry criteria は `specs/examples/327...338` で fixed 済みの theorem-first review-unit pilot、bridge-sketch reopen ordering、`e3` guard comparisonに置く。
- current theorem-side actualization は docs-first に留める。
- old theorem-line `specs/examples/140` の bridge sketch shape を Phase 6 current cut へ持ち直すことだけを扱う。

## current 前提

current repo では次が成立している。

1. theorem-side current cut は row-local `proof_notebook_review_unit` に留まる。
2. bridge-sketch reopen ordering は plain docs-only bridge sketch first / compare-ready bridge sketch second に fixed 済みである。
3. `e3` guard comparison も fixed 済みであり、`e3` actualization を急がず theorem-side plain bridge sketch line を先に reopen してよい。
4. old theorem-line では `specs/examples/140` が `bridge_subject_ref + review_units + bridge_goal_text` の plain docs-only bridge sketch を already cut している。

したがって current 問いは、
**Phase 6 current line として、`specs/examples/140` の plain docs-only bridge sketch shape をそのまま actual package に落とすのが自然か**
である。

## 比較観点

1. row-local `proof_notebook_review_unit` current cut を壊さないか
2. compare-ready bridge sketch second reopen を premature に混ぜないか
3. helper / emitter や concrete tool binding を premature に混ぜないか
4. later `e3` actualization reopen timing を still later に残せるか

## 比較対象

### 案 1. `specs/examples/140` の plain docs-only bridge sketch shape をそのまま actualize する

#### shape

```text
proof_notebook_bridge_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text
}
```

#### 利点

- row-local review unit と compare-ready bridge sketch の間に docs-only bridge を 1 段だけ actualize できる。
- old theorem-line `140` と連続的である。
- helper / emitter / compare-ready metadata / concrete tool binding を later に押し分けやすい。

#### 欠点

- docs-only actualization のため、repo 内の code anchor はまだ増えない。

### 案 2. plain bridge sketch を飛ばし、compare-ready bridge sketch を first actualization にする

#### 利点

- compare needs まで一気に見える。

#### 欠点

- `140 -> 141` の順序を壊す。
- compare metadata を premature に混ぜやすい。

### 案 3. plain bridge sketch と同時に helper / emitter code anchor も足す

#### 利点

- later code path は見えやすい。

#### 欠点

- docs-first actualization の cut を超えており、concrete tool binding とも距離が近すぎる。

## current judgment

current L2 で最も自然なのは、
**案 1. `specs/examples/140` の plain docs-only bridge sketch shape をそのまま actualize する**
である。

理由は次の通り。

1. bridge-sketch reopen ordering で fixed 済みの plain-first / compare-ready-second をそのまま実体化できる。
2. row-local `proof_notebook_review_unit` current cut を壊さずに theorem-side bridge を 1 段だけ actualize できる。
3. helper / emitter / concrete tool binding を still later に残せる。

## current first choice details

- actual package は docs-only bridge sketch shape に留める。
- `bridge_subject_ref + review_units + bridge_goal_text` を current plain bridge sketch actual shape として再利用する。
- compare-ready `comparison_basis_refs` は second reopen に残す。
- helper / emitter code anchor はまだ足さない。

## next promoted line

next promoted line は、
**plain-proof-notebook-bridge-sketch-actualization-ready compare-ready-bridge-sketch-second-reopen**
に置く。

## open questions

- compare-ready bridge sketch second reopen の minimum に `comparison_basis_refs` 以外をどこまで入れるか
- deferred `e3` actualization reopen timing を compare-ready bridge sketch second reopen の直後に置く current cutを保ったまま、その後の tool-cut line とどう接続するか
- later helper / emitter code anchor を plain bridge sketch ではなく compare-ready package 側に寄せるべきか
