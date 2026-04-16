# 438 — current L2 malformed duplicate-cluster source-sample widening comparison

## 目的

`specs/examples/430-current-l2-malformed-duplicate-cluster-later-reopen-comparison.md`
で fixed した

- duplicate cluster kept-later family
- duplicate cluster first reopen sequencing

を前提に、
duplicate cluster を source-sample corpus へ widen する **next exact cut**
を comparison する。

ここで fixed するのは next widening cut であり、
duplicate typed / mirrored promotion や broader malformed-static family actualization は fixed しない。

## source-backed floor

- duplicate cluster `e14/e15` は static-only actual corpus として already ある。
- duplicate cluster は current helper cut では `checked_reasons` / detached `reason_codes` に上げていない。
- authored source sample dozen と runner accepted set には `e14/e15` がまだ含まれていない。
- duplicate cluster は `TryFallback` / `AtomicCut` broader malformed-static family より先に compare-first reopen する reading が fixed 済みである。

## comparison family

### 案 1. `e14/e15` を source-authored static-stop pair として同時 widening する

#### 利点

- existing static-only anchor が pair で揃っている。
- missing-option / capability family と同じ pair-first precedent に乗せやすい。
- non-promotion guard を pair 単位で固定しやすい。

#### 欠点

- pair 実装と mirror update が単独 row より広い。

### 案 2. `e14` only を first widening cut にする

#### 利点

- first authored row を最小にできる。

#### 欠点

- duplicate cluster を option-side と chain-side で uneven に開くことになり、keep/drop line が読みづらい。

### 案 3. widening をさらに defer し、static-only corpus に留める

#### 利点

- execution lane の churn を増やさない。

#### 欠点

- duplicate cluster next reopen が長く comparison-only のままになり、sample corpus line の順序が見えにくい。

## current judgment

current L2 で最も自然なのは、
**案 1. `e14/e15` を source-authored static-stop pair として同時 widening する**
ことである。

理由は次の通り。

1. duplicate cluster は already actual static-only pair を持ち、pair-first precedent と整合する。
2. option-side / chain-side を uneven に開くより、pair 単位で non-promotion guard を固定する方が誤読に強い。
3. broader `TryFallback` / `AtomicCut` malformed-static family を先に reopen しないという guard とも整合する。

## keep / drop line

### keep

- duplicate cluster を source-authored static-stop pair として widen する candidate を first に置く
- duplicate cluster を `checked_reasons` / detached `reason_codes` へ昇格させない
- broader malformed-static family は still later に残す

### drop from current package

- duplicate typed / mirrored promotion
- nested-scope / try-fallback duplicate variant actualization
- broader `TryFallback` / `AtomicCut` malformed-static actualization

## next promoted line

next promoted line は、
**malformed duplicate-cluster source-authored static-stop pair actualization comparison**
に置く。

## what is not decided here

- source-authored pair actualization の exact implementation cut
- nested-scope / try-fallback duplicate variants
- duplicate cluster typed / mirrored promotion timing
- broader malformed-static family の reopen timing
