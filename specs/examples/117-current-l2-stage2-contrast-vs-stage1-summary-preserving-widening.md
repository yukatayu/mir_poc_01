# 117 — current L2 stage2 contrast vs stage1 summary-preserving widening

## 目的

この文書は、`specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md` で
stage 2 reconnect first tranche を actualize 済みとしたことを前提に、
**次の narrow step を stage 2 runtime-contrast widening に置くか、それとも stage 1 reconnect family の summary-preserving widening に戻るか**
を比較する。

ここで固定するのは final parser / checker bridge ではない。
固定するのは、

- current helper-local compare surface を増やさずに evidence を厚くできるか
- runtime / proof boundary に属する contrast case を parser-side reconnect へ早く持ち込まないか
- `e19` のような summary redesign pressure を still later に残せるか

という sequencing judgment だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semanticsは変更しない。
- stage 2 reconnect first tranche は parser-side private summary と
  `checked_try_rollback_structural_*` exact compare まで actualize 済みである。
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md` の
  runtime / proof boundary も維持する。
- stage 1 reconnect first tranche は `e4` / `e13` / `e16` と
  `Stage1ReconnectClusters` summary まで actualize 済みである。

## 比較する 3 案

### 案 1. stage 2 reconnect family を `E21` / `E22` runtime contrast まで widening する

#### 利点

- current stage 2 line を続けられる。
- `AtomicCut` ordinary statement と nested-place mismatch gate の contrast へ近づく。

#### 欠点

- `E21` / `E22` の差は current `checked_try_rollback_structural_*` compare surface では表れにくい。
- contrast の本体は
  - `place_anchor == current_place`
  - restore scope
  という runtime / proof boundary にあり、parser-side reconnect へ早く持ち込む圧がある。
- no-finding valid case を増やすだけなら、new checker reconnect evidence としての前進量は薄い。

### 案 2. stage 1 reconnect family を summary-preserving に widening する

- `e18-malformed-missing-successor-option`
- `e20-malformed-late-capability-strengthening`

を current `Stage1ReconnectClusters` summary のままで追加する。

#### 利点

- helper contract を広げずに fixture compare evidence を厚くできる。
- `e18` は `missing_option_structure_floor`、`e20` は `capability_strengthening_floor` の widening として素直である。
- current parser-side helper は already generic enough であり、helper 実装変更なしに actual evidence を増やせる。

#### 欠点

- new checker cluster を増やすわけではない。
- `e19` は still later に残る。

### 案 3. stage 1 remaining widening を full に進める

- `e18`
- `e19`
- `e20`

を一括で reconnect する。

#### 利点

- stage 1 family coverage は最も厚くなる。

#### 欠点

- `e19` が current `Stage1ReconnectClusters` summary に自然に乗らない。
- `declared_target_mismatch_floor` を summary contract に足すかどうかという別比較を同時に背負う。

## current judgment

current repo の next narrow step として最も自然なのは、
**案 2. stage 1 reconnect family を summary-preserving に widening する**
ことである。

理由は次の通り。

1. stage 2 `E21` / `E22` contrast は current structural compare surface より runtime / proof boundary へ近い。
2. `e18` / `e20` は current `Stage1ReconnectClusters` summary を増やさずに actual evidence を厚くできる。
3. `e19` を先に入れると summary redesign pressure が発生するため、next narrow step としては still heavy である。

## current meaning

- next reconnect widening は stage 2 runtime-contrast より先に stage 1 summary-preserving widening を取る。
- その widening は `e18` / `e20` に留め、`e19` は still later に残す。
- `E21` / `E22` contrast は、parser-side reconnect へすぐ mirror するより、runtime / proof boundary をどう narrow に扱うかが見えてから比べる方が自然である。

## next narrow step

次段では、
**stage 1 reconnect family の summary-preserving widening (`e18` / `e20`) を helper-local / test-only actual evidence にどこまで actualize してよいか**
を narrow に切るのが自然である。
