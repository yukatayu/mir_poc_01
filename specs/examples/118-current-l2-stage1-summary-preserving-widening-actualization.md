# 118 — current L2 stage1 summary-preserving widening actualization

## 目的

この文書は、`specs/examples/117-current-l2-stage2-contrast-vs-stage1-summary-preserving-widening.md` で
next widening step を stage 1 reconnect family の summary-preserving widening に置くと整理したことを前提に、
**その widening tranche を helper-local / test-only actual evidence としてどこまで actualize してよいか**
を整理する。

ここで固定するのは final checker bridge ではない。
固定するのは、

- current `Stage1ReconnectClusters` summary contract を維持したままどこまで fixture compare を足せるか
- helper code 変更が不要な widening をどこで止めるか
- `e19` のような summary redesign pressure を still later に残せるか

という current cut だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- `Stage1ReconnectClusters` summary contract は
  - `same_lineage_floor`
  - `missing_option_structure_floor`
  - `capability_strengthening_floor`
  の 3 bool に留める。
- current stage 1 helper は lowered fixture-subset compare と helper-local summary に留める。

## current actualization

current task で actualize してよいのは次である。

### focused fixture compare

- `e18-malformed-missing-successor-option`
- `e20-malformed-late-capability-strengthening`

を `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` の fixture-subset compare に追加する。

### helper-local summary compare

- `e18`
  - `same_lineage_floor = true`
  - `missing_option_structure_floor = true`
  - `capability_strengthening_floor = false`
- `e20`
  - `same_lineage_floor = true`
  - `missing_option_structure_floor = false`
  - `capability_strengthening_floor = true`

を `Stage1ReconnectClusters` summary の focused smoke として追加する。

## なぜこれが最小か

1. current helper implementation は already generic enough であり、support helper の widening なしに actual evidence を増やせる。
2. `e18` / `e20` は current 3-bool summary contract に自然に乗る。
3. `e19` をまだ混ぜないことで、`declared_target_mismatch_floor` の summary redesign を still later に残せる。

## current stage でまだやらないこと

- `e19-malformed-target-mismatch` の direct reconnect
- `Stage1ReconnectClusters` への new field 追加
- stage 2 `E21` / `E22` runtime contrast の parser-side widening
- public checker API 化

## current meaning

- stage 1 reconnect family の summary-preserving widening tranche は actualize 済みとみなしてよい。
- この widening は helper code redesign を伴わず、focused tests の追加だけで済む narrow cut である。
- `e19` と stage 2 runtime contrast は still later に残す。

## next narrow step

次段では、次の 2 案を narrow に比較するのが自然である。

1. `e19-malformed-target-mismatch` をどういう reconnect summary contract に乗せるかを比較する
2. stage 2 `E21` / `E22` runtime contrast を parser-side reconnect に mirror する threshold を比較する
