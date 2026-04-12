# 343 — current L2 minimal-compare-ready-bridge-sketch-ready deferred `e3` actualization reopen timing comparison

## 目的

`specs/examples/342-current-l2-compare-ready-bridge-sketch-second-reopen-ready-minimal-compare-ready-bridge-sketch-threshold.md`
で compare-ready bridge sketch second reopen の minimum を fixed した次段として、

- deferred `e3-option-admit-chain` actualization をどこで reopen するか
- actual `e3` source row を concrete theorem / model-check tool pilot より前に戻すか
- current formal-hook top と theorem-side guard をどう保ったまま reopen するか

を比較する。

ここで固定するのは
**current L2 minimal-compare-ready-bridge-sketch-ready deferred `e3` actualization reopen timing comparison**
であり、

- actual `e3` source row
- new formal-hook family
- concrete theorem / model-check tool pilot
- second source-sample cluster sequencing

はまだ固定しない。

## scope

- entry criteria は `specs/examples/327...342` で fixed 済みの theorem-first review-unit pilot、bridge-sketch reopen ordering、`e3` guard comparison、plain / compare-ready bridge actualizationに置く。
- current theorem-side line は docs-only bridge sketch に留める。
- current formal-hook top は `runtime_try_cut_cluster` / `fixture_static_cluster` のまま保つ。
- reopen するのは `e3` actualization の timing judgment だけである。

## current 前提

current repo では次が成立している。

1. `e3-option-admit-chain` は fixture / host-plan / representative target を already 持つが、current authored source row には入っていない。
2. compare-ready bridge sketch second reopen は fixed 済みであり、theorem-side bridge line は `comparison_basis_refs` までを current cut として持つ。
3. current formal-hook support は runtime detached artifact の top を `runtime_try_cut_cluster` に保っており、`e3` runtime artifact は current top の外として reject する。
4. current task ordering では actual `e3` authored-row reopen の後段に proof / model-check first concrete tool pilot を置いている。

したがって current 問いは、
**`e3` actualization を concrete theorem / model-check tool pilot より前に reopen し、current formal-hook top widening は still later に残すのが自然か**
である。

## 比較観点

1. current theorem-side compare-ready bridge line を壊さないか
2. current formal-hook top widening を premature に混ぜないか
3. source-sample authored-row widen と concrete tool pilot を分離できるか
4. package 3 で source row / runner / regression ladder を narrow actualize しやすいか

## 比較対象

### 案 1. compare-ready bridge の直後に `e3` actualization timing を reopen し、concrete tool pilot は後段に残す

#### 読み

`e3` actualization は current next package として reopen するが、

- current theorem-side consumer は row-local `proof_notebook_review_unit` のまま保つ
- current formal-hook top は `runtime_try_cut_cluster` のまま保つ
- `e3` actualization package では source row / runner / authored inventory / regression ladder を主対象にする

という cut に留める。

#### 利点

- current tasks ordering と一致する。
- source-sample path の widening と concrete tool pilot を分離できる。
- `e3` actualization package で formal-hook family widening を要求しなくてよい。

#### 欠点

- `e3` actualization package では formal-hook reached row を still guarded として扱う説明が要る。

### 案 2. concrete theorem / model-check tool pilot を先に reopen し、その後に `e3` actualization を戻す

#### 利点

- theorem-side concrete consumer pressure を先に見られる。

#### 欠点

- authored-row widen sequencing と current task map を逆転させる。
- source sample の widening が unnecessary に遅れる。

### 案 3. `e3` actualization 前に new formal-hook / theorem-side family を先に widen する

#### 利点

- `admit` family を専用 carrier で扱える可能性はある。

#### 欠点

- current formal-hook top と theorem-side bridge line を同時に太らせやすい。
- package 3 の source-row actualization scopeを不必要に重くする。

## current judgment

current L2 で最も自然なのは、
**案 1. compare-ready bridge の直後に `e3` actualization timing を reopen し、concrete tool pilot は後段に残す**
である。

理由は次の通り。

1. current task ordering と widened authored-row sequencing に最も整合する。
2. current compare-ready bridge を fixed entry criteria にしたまま、source-row actualization を narrow に reopen できる。
3. current formal-hook top widening や concrete tool binding を package 3 に混ぜずに済む。

## current first choice details

- actual `e3` authored-row reopen は compare-ready bridge sketch second reopen の直後に置く。
- package 3 では source row / runner / authored inventory / regression ladder を主対象にし、current formal-hook top widening は要求しない。
- proof / model-check first concrete tool pilot は `e3` authored-row reopen の後段に残す。

## next promoted line

next promoted line は、
**deferred-`e3`-actualization-reopen-timing-ready actual-`e3`-authored-row-reopen**
に置く。

## open questions

- actual `e3` authored-row package で formal hook を `not reached (guarded)` とする minimum row をどう表すか
- current regression helper に `e3` を authored row として入れるとき、formal-hook smoke bundle をどう分離するか
- proof / model-check first concrete tool pilot を `e3` actualization の直後にどこまで narrow に reopen するか
