# 116 — current L2 stage2 try/rollback reconnect first tranche actualization

## 目的

この文書は、`specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md` で
next reconnect step を stage 2 try/rollback structural floor に置くと整理したことを前提に、
**その first tranche を helper-local / test-only actual evidence としてどこまで actualize してよいか**
を比較し、current cut を固定する。

ここで固定するのは final parser / checker bridge ではない。
固定するのは、

- parser-side private helper をどの compare surface に合わせるか
- malformed pair `e23` / `e24` と valid no-finding smoke をどう束ねるか
- runtime / proof boundary を still later に残したまま reconnect を切る最小 shape

だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md` の
  runtime / proof split は維持する。
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md` の
  dedicated helper-local contract は維持する。
- current stage 2 reconnect は `mir-ast` test-only / private helper に置き、
  `mir-ast/src/lib.rs` の public parser API には上げない。

## current code anchor

current repo では、次の code anchor が stage 2 reconnect first tranche の正本である。

- `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`
- `crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json`
- `scripts/current_l2_try_rollback_structural_checker.py`

## 比較する 3 案

### 案 1. parser-side private summary を fixture-side `checked_try_rollback_structural_*` に直接合わせる

- inline source から minimal `try { ... } fallback { ... }` shape を parse する
- helper-local summary として
  - `checked_try_rollback_structural_verdict`
  - `checked_try_rollback_structural_findings`
  に対応する carrier を返す
- `e23` / `e24` の fixture-side expected field と exact compare する
- 追加で valid one-shot を `no_findings` smoke として持つ

#### 利点

- existing dedicated helper-local contract に最も近い。
- `scripts/current_l2_try_rollback_structural_checker.py` と meaning をそろえやすい。
- detached artifact や public checker API へ prematurely coupling しない。

#### 欠点

- compare surface は finding row summary に留まり、full AST subset compare にはならない。

### 案 2. parser-side output を detached/static-gate helper へ直接 handoff する

#### 利点

- reconnect 感は強い。

#### 欠点

- parser-side private helper と saved-artifact / checker-side exact compare が混ざる。
- current phase では too early である。

### 案 3. parser-side full AST subset compare を切る

- body / fallback body の statement subtree を fixture subset と広く比較する

#### 利点

- parser-side fidelity は高い。

#### 欠点

- `perform` contract や nested place shape まで stage 2 reconnect に持ち込みやすい。
- current first tranche の目的である structural floor reconnect を超える。

## current judgment

current repo の next narrow step として最も自然なのは、
**案 1. parser-side private summary を fixture-side `checked_try_rollback_structural_*` に直接合わせる**
ことである。

## current actualization

current task で actualize してよいのは次である。

### private support helper

- file:
  - `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`
- current role:
  - inline source から stage 2 minimal block form を parse する
  - helper-local `Stage2TryRollbackStructuralSummary` を返す
  - fixture-side `checked_try_rollback_structural_*` carrier を load する

### focused tests

- file:
  - `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- current first-tranche coverage:
  - `e23` malformed pair
  - `e24` malformed pair
  - valid `atomic_cut` in try body の `no_findings` smoke

### current accepted surface

first tranche で accepted とみなすのは、次に限る。

- single `try { ... } fallback { ... }` block
- line head としての `atomic_cut`
- その他の statement line は opaque `Other` として保持

ここで current first tranche では、次をまだ accepted cluster に入れない。

- nested `place`
- request-local clause attachment
- restore scope 読み
- `place_anchor == current_place` gate
- runtime representative `E21` / `E22` の full contrast parse

### current compare surface

current first tranche の compare surface は次である。

- `verdict`
  - `no_findings`
  - `findings_present`
- finding rows
  - `TryFallback` / `missing_fallback_body`
  - `AtomicCut` / `disallowed_fallback_placement`

これは `specs/examples/68...` の dedicated helper-local contract と一致させる。

## なぜこれが最小か

1. `e23` / `e24` の fixture-side expected field を parser-side private helper へ reconnect できる。
2. valid one-shot `atomic_cut` in try body を `no_findings` で通すことで、
   `atomic_cut` が ordinary statement として try body に置ける structural floor を narrow に示せる。
3. nested-place mismatch gate や restore scope を still later に残せる。
4. parser-side helper を detached artifact / public checker API へ直接結びつけない。

## current meaning

- stage 2 try/rollback reconnect first tranche は actualize 済みとみなしてよい。
- ただし current actualization は helper-local / test-only summary と focused tests に留まる。
- `scripts/current_l2_try_rollback_structural_checker.py` や detached loop smoke は existing code anchor として維持するが、
  current tranche では parser-side private helper から直接は呼ばない。

## current stage でまだやらないこと

- `E21` / `E22` runtime contrast の full parser-side reconnect
- nested place / dynamic gate / restore scope の parser-side carrier actualization
- detached artifact static-gate checker への direct handoff
- public checker API 化
- generic parser family support helper への昇格

## next narrow step

次段では、次の 2 案を narrow に比較するのが自然である。

1. stage 2 reconnect family を `E21` / `E22` contrast まで widening する
2. stage 1 reconnect family の remaining widening (`e18` / `e19` / `e20`) に戻る
