# 114 — current L2 stage1 first checker reconnect first tranche actualization

## 目的

この文書は、`specs/examples/113-current-l2-first-checker-reconnect-family-selection.md` で
stage 1 chain / declaration structural floor family を first reconnect family にすると整理したことを前提に、
**その first tranche を helper-local / test-only actual evidence としてどこまで actualize してよいか**
を比較し、current cut を固定する。

ここで固定するのは final checker bridge ではない。
固定するのは、

- stage 1 parser-side evidence をどの checker family representative まで広げるか
- helper-local reconnect summary をどこまで持つか
- まだ later に残す malformed family をどこで切るか

という最小 actualization cut だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 1 parser spike は `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
  と `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` に private / test-only で置く。
- compare surface は lowered fixture-subset compare に留める。
- current stage 1 surface では fallback edge の structural predecessor は chain progression で決まり、
  arbitrary predecessor rewrite は受けない。

## current source anchor

### already actualized

- `e4-malformed-lineage`
- `e7-write-fallback-after-expiry`

は stage 1 parser spike の current working set として already actualize 済みである。

### newly relevant checker families

- capability floor representative:
  - `e13-malformed-capability-strengthening`
- missing-option floor representative:
  - `e16-malformed-missing-chain-head-option`

### still later candidates

- `e18-malformed-missing-successor-option`
- `e19-malformed-target-mismatch`
- `e20-malformed-late-capability-strengthening`

### current stage 1 surface でまだ awkward なもの

- `e17-malformed-missing-predecessor-option`

これは current stage 1 surface では、fallback edge の structural predecessor が
chain progression で固定されるため、first tranche で直接は reconnect しない。

## 比較する 3 案

### 案 1. one-representative-per-new-family tranche

- existing `e4` / `e7` は維持する
- capability floor representative として `e13` を足す
- missing-option floor representative として `e16` を足す
- helper-local reconnect summary を追加し、stage 1 surface が
  `same_lineage_floor` / `capability_strengthening_floor` / `missing_option_structure_floor`
  のどれに触れているかだけを narrow に示す

#### 利点

- stage 1 parser-side evidence が 3 checker family に触れていることを最小で示せる。
- still later family を一気に混ぜずに済む。
- public checker API や detached static gate artifact との direct coupling を避けられる。

#### 欠点

- same-lineage / capability / missing-option family の full coverage までは still later に残る。

### 案 2. all stage1-representable anchors を一気に入れる

- `e13`
- `e16`
- `e18`
- `e19`
- `e20`

まで含める。

#### 利点

- stage 1 surface が表せる範囲を広く押さえられる。

#### 欠点

- first tranche としては広い。
- stage 1 reconnect first cut の意図が「代表 anchor を通す」から「family coverage を一気に厚くする」へ変わりやすい。

### 案 3. parser-side output を checker helper / static gate artifact へ直接渡す

#### 利点

- reconnect の見え方は強い。

#### 欠点

- parser-side reconnect summary と checker-side exact compare の責務が混ざる。
- current private parser helper を早く detached artifact / checker helper へ結びすぎる。

## current judgment

current repo の next narrow step として最も自然なのは、
**案 1. one-representative-per-new-family tranche**
である。

## current actualization

current task で actualize してよいのは次である。

- `Stage1ReconnectClusters`
  - `same_lineage_floor`
  - `missing_option_structure_floor`
  - `capability_strengthening_floor`
- `summarize_stage1_reconnect_clusters(&Stage1FixtureSubset)`
  - stage 1 parser-side lowered subset が、どの first checker family representative に触れているかを helper-local / test-only に要約する
- focused smoke
  - `e13-malformed-capability-strengthening`
  - `e16-malformed-missing-chain-head-option`
  - existing `e4-malformed-lineage`

この cut では次をまだ行わない。

- `e18` / `e19` / `e20` までの widening
- `e17` の direct reconnect
- checker helper / static gate artifact への direct handoff
- public checker API 化

## なぜこれが最小か

- existing `e4` / `e7` working set を壊さずに capability / missing-option family を足せる。
- reconnect helper は summary に留まり、checker-side exact compare へ prematurely coupling しない。
- `e17` のような current stage 1 surface で awkward な malformed family を first tranche に混ぜなくて済む。

## current meaning

- stage 1 chain / declaration structural floor family の first reconnect tranche は actualize 済みとみなしてよい。
- ただし current actualization は helper-local / test-only summary と representative fixture compare に留まる。
- full family coverage も checker-side direct handoff も still later に残す。

## next narrow step

次段では、次の 2 案を narrow に比較するのが自然である。

1. stage 1 reconnect family を `e18` / `e19` / `e20` まで widening する
2. stage 2 の `try` / rollback structural floor reconnect へ移る
