# 119 — current L2 reconnect freeze threshold

## 目的

この文書は、`specs/examples/117-current-l2-stage2-contrast-vs-stage1-summary-preserving-widening.md` と
`specs/examples/118-current-l2-stage1-summary-preserving-widening-actualization.md` を前提に、
**`e19-malformed-target-mismatch` を stage 1 reconnect summary へさらに混ぜるべきか、
それとも stage 2 `E21` / `E22` runtime contrast を parser-side reconnect に mirror するべきか、
あるいは reconnect subline 自体を current tranche で freeze するべきか**
を比較する。

ここで固定するのは final checker API や final parser grammar ではない。
固定するのは、

- current reconnect helper contract をここで広げる価値があるか
- `e19` と `E21` / `E22` のどちらが current reconnect line の自然な次段か
- それとも current evidence で一区切りとして freeze する方が理論的に整っているか

という current threshold judgment だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 1 reconnect line は `Stage1ReconnectClusters` の
  - `same_lineage_floor`
  - `missing_option_structure_floor`
  - `capability_strengthening_floor`
  の 3-bool summary に留める。
- stage 2 reconnect line は parser-side private summary と fixture-side
  `checked_try_rollback_structural_*` compare に留める。
- `e19-malformed-target-mismatch` は、reconnect line とは別に
  `expected_static.checked_reason_codes` / detached static gate `reason_codes`
  を通る **declared target edge pair family** の source-backed actual anchor を already 持つ。
- `E21` / `E22` は runtime representative として already strong evidence を持つが、
  current parser-side reconnect line では nested `place`、`place_anchor == current_place`、
  restore scope を direct に mirror していない。

## 比較する 3 案

### 案 1. stage 1 reconnect summary を `e19` まで widening する

`Stage1ReconnectClusters` に
`declared_target_mismatch_floor` を足すか、あるいは既存 bool のどれかへ弱く畳んで
`e19` を current reconnect line に混ぜる。

#### 利点

- stage 1 parser-side reconnect line の coverage は厚くなる。
- `e19` は static malformed fixture なので runtime boundary を持ち込まない。

#### 欠点

- `e19` は **already typed static reason family で actualize 済み** であり、
  reconnect line に再度抱え込むと evidence が二重化する。
- `declared_target_mismatch` は
  same-lineage / missing-option / capability とは別 family なので、
  current 3-bool summary contract を自然には保てない。
- helper-local reconnect summary の meaning が「parser-side structural floor representative」
  から「typed static reason family の再掲」へ寄りやすい。

### 案 2. stage 2 reconnect line を `E21` / `E22` runtime contrast まで widening する

parser-side private helper を widening し、
`E21` / `E22` の contrast を reconnect evidence として mirror する。

#### 利点

- stage 2 line の contrast evidence は厚くなる。
- `try` / rollback locality の runtime representative へ closer になる。

#### 欠点

- `E21` / `E22` の差の本体は
  - nested `place`
  - `place_anchor == current_place`
  - restore scope
  にあり、current parser-side reconnect line より runtime / proof boundary に近い。
- current `checked_try_rollback_structural_*` compare surface は malformed pair と
  `no_findings` smoke に留まっており、contrast を素直に受け止める dedicated carrier をまだ持たない。
- narrow reconnect helper というより、runtime representative の mirror helper へ寄りやすい。

### 案 3. reconnect subline を current tranche で freeze する

- stage 1 reconnect line は `e4` / `e13` / `e16` / `e18` / `e20` で止める
- stage 2 reconnect line は `e23` / `e24` + valid one-shot `atomic_cut` smoke で止める
- `e19` は typed static reason family 側に残す
- `E21` / `E22` は runtime / proof boundary 側に残す

#### 利点

- helper contract を増やさずに current evidence line を clean に閉じられる。
- `e19` と `E21` / `E22` を、それぞれ already stronger な carrier に委ねられる。
- parser-side reconnect line の責務を
  「current private staged spike で structural floor representative を narrow に持つ」
  ところに留められる。

#### 欠点

- reconnect line 自体の coverage widening はここで止まる。
- `e19` や `E21` / `E22` を parser-side reconnect に mirror したい場合は、
  将来別比較で reopen する必要がある。

## current judgment

current repo の next threshold judgment として最も自然なのは、
**案 3. reconnect subline を current tranche で freeze する**
ことである。

理由は次の通り。

1. `e19` は already source-backed な declared target edge pair family の actual anchor を持っており、
   reconnect summary へ重ねると helper contract widening に対して前進量が薄い。
2. `E21` / `E22` は runtime / proof boundary に近く、
   current `checked_try_rollback_structural_*` compare surface を widen しない限り
   parser-side reconnect line に自然に乗らない。
3. current reconnect line は、
   - stage 1 側で same-lineage / missing-option / capability の representative widening
   - stage 2 側で malformed pair + valid one-shot `atomic_cut` smoke
   まで揃っており、private staged spike としては十分に source-backed である。

## current meaning

- current reconnect subline は **一区切りとして freeze 済み** とみなしてよい。
- `e19` は typed static reason family の anchor として扱い、
  reconnect summary contract には混ぜない。
- `E21` / `E22` は runtime / proof boundary の representative として扱い、
  current parser-side reconnect line には混ぜない。

## current stage でまだやらないこと

- `Stage1ReconnectClusters` に `declared_target_mismatch_floor` を足すこと
- `checked_try_rollback_structural_*` に runtime contrast carrier を足すこと
- nested `place` / restore scope / `place_anchor` gate の parser-side mirror
- public checker API 化

## next narrow step

current phase の next step として自然なのは、
**Phase 3 closeout sweep と current state summary を行い、parser boundary / first checker cut の private staged spike line を一旦閉じること**
である。
