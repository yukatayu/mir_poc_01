# 115 — current L2 stage1 widening vs stage2 try/rollback reconnect

## 目的

この文書は、`specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md` で
stage 1 reconnect first tranche を actualize 済みとしたことを前提に、
**次の narrow step を stage 1 widening と stage 2 try/rollback reconnect のどちらに置くのが自然か**
を比較する。

ここで固定するのは final checker API ではない。
固定するのは、

- 現在の parser-side actual evidence をどの checker family に次に reconnect するか
- helper-local / test-only cut を保ったまま新しい evidence を増やせるか
- reconnect summary の contract を不必要に太らせないか

という sequencing judgment だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` の
  first checker cut inventory は維持する。
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` の
  same-lineage / missing-option / capability floor baseline は維持する。
- `specs/examples/73-current-l2-first-parser-spike-staging.md` の
  checker-led staged spike judgment も維持する。
- stage 1 reconnect first tranche は `e4` / `e13` / `e16` と
  `Stage1ReconnectClusters` summary まで actualize 済みである。
- `try` / rollback family は、`specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
  で helper-local dedicated contract と `e23` / `e24` malformed pair が source-backed に整理済みである。

## 比較する 3 案

### 案 1. stage 1 reconnect family を `e18` / `e19` / `e20` まで widening する

#### 利点

- stage 1 parser-side evidence を厚くできる。
- `e18` / `e20` は existing `missing_option_structure_floor` / `capability_strengthening_floor`
  の延長として読みやすい。

#### 欠点

- `e19-malformed-target-mismatch` は current `Stage1ReconnectClusters`
  の 3 bool summary に自然には乗らない。
- `e19` を入れるには
  - summary contract を `declared_target_mismatch_floor` 方向へ広げるか、
  - same-lineage floor の中へ弱く畳むか、
  のどちらかを選ぶ必要があり、first reconnect summary の設計圧が上がる。
- `e18` / `e20` は既存 family の coverage を厚くするが、
  **new checker cluster を 1 つ増やす** 効果は弱い。

### 案 2. stage 2 try/rollback structural floor reconnect へ進む

#### 利点

- `specs/examples/73...` の stage order に素直である。
- `specs/examples/30...` の first checker cut inventory にある
  `try` / rollback locality の structural floor を新たに reconnect できる。
- `specs/examples/68...` の dedicated helper-local contract、
  `e23` / `e24` fixture pair、`scripts/current_l2_try_rollback_structural_checker.py`
  という code anchor が already ある。
- stage 1 widening のように existing reconnect summary contract を広げなくて済む。

#### 欠点

- parser-side private helper を新設する必要がある。
- nested place mismatch gate や restore scope など、runtime / proof boundary を
  まだ parser-side reconnect には持ち込めない。

### 案 3. stage 1 widening と stage 2 reconnect を同時に進める

#### 利点

- parser-side evidence は一見最も増える。

#### 欠点

- first checker reconnect の next step としては scope が太い。
- stage 1 summary redesign と stage 2 new helper design が同時に走り、
  review scope と mirror drift risk が大きい。

## 比較

### first checker cut inventory への前進量

- 案 1:
  same-lineage / missing-option / capability の **既存 3 family の coverage widening** が主になる。
- 案 2:
  `try` / rollback locality という **別 cluster** を新たに reconnect できる。
- 案 3:
  前進量は大きいが、narrow step ではない。

### reconnect summary contract の安定性

- 案 1:
  `e19` が current `Stage1ReconnectClusters` に自然に乗らず、summary contract の増築圧がある。
- 案 2:
  stage 2 側は既存の `checked_try_rollback_structural_*` family を parser-side summary に再利用できる。
- 案 3:
  2 本の contract change を同時に抱える。

### parser-side evidence の source-backed thickness

- 案 1:
  stage 1 parser spike 自体は already thick だが、next widening の contract はまだ未整理である。
- 案 2:
  parser-side private helper は未 actualize だが、
  static gate / fixture-side expected field / dedicated compare helper / detached loop smoke が already そろっている。
- 案 3:
  source はあるが、task 粒度としては広い。

### syntax pressure

- 案 1:
  declaration / chain familyのままなので low だが、summary meaning を拡張しやすい。
- 案 2:
  block form `try { ... } fallback { ... }` と `atomic_cut` に限るなら still low である。
- 案 3:
  両 family の pressure を同時に受ける。

## current judgment

current repo の next narrow step として最も自然なのは、
**案 2. stage 2 try/rollback structural floor reconnect へ進む**
ことである。

理由は次の通り。

1. stage 1 first tranche は already 3 family representative を source-backed に通しており、
   次に欲しいのは same family widening より **新しい checker cluster との reconnect** である。
2. stage 1 widening では `e19` が current reconnect summary contract を押し広げやすい。
3. stage 2 側には、`specs/examples/51` / `68`、`e23` / `e24`、
   `scripts/current_l2_try_rollback_structural_checker.py`
   という dedicated code/doc anchor が既にある。
4. `specs/examples/73...` の checker-led staged spike でも、
   stage 1 の次は stage 2 を取る順序が最も自然である。

## current meaning

- next reconnect family widening は stage 1 より先に stage 2 を取る。
- ただし current task でも、restore scope や nested-place runtime gate を parser-side reconnect に持ち込まない。
- next task では、
  **stage 2 try/rollback structural floor family の first reconnect tranche を helper-local / test-only にどこまで actualize してよいか**
  を narrow に切るのが自然である。

## current stage でまだやらないこと

- stage 1 reconnect summary へ `declared_target_mismatch_floor` を足すこと
- stage 1 `e18` / `e19` / `e20` widening の actualization
- runtime representative `E21` / `E22` の full parser-side reconnect
- public checker API 化
- detached artifact shared carrier への昇格

## next narrow step

次段では、
**stage 2 try/rollback structural floor family を parser-side private helper と focused tests にどこまで reconnect するのが最小か**
を narrow に比較するのが自然である。
