# 86 — current L2 stage 3 admit-slot malformed-source comparison

## 目的

この文書は、`specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
で stage 3 admit-slot branch の success-side first tranche が actualize 済みであることを前提に、
**stage 3 helper 自身が「受けない source」をどこまで fail-closed に smoke すべきか**
を narrow に比較する。

ここで固定するのは final parser error API ではない。
固定するのは、current private / non-production helper に
**最初に足してよい malformed-source smoke の working set**
だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 3 first tranche は declaration-side `admit` attached slot に留める。
- fixture-side `OptionDecl.admit` predicate node への direct lowering はまだ行わない。
- `PerformVia` / request-local `require` / `ensure` / predicate fragment parse は still later stage に残す。
- malformed-source smoke は helper-local error surface に留め、public parser diagnostics API を既成事実化しない。

## current source anchor

- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
  - stage 3 の最初の sub-cutは declaration-side `admit` attached slot
- `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
  - `decl_admit_slot` naming と compare surface
- `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
  - current success-side first tranche
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
  - `admit` と `PerformVia` / `require` が同居する contrast anchor
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
  - current private parser spike helper

current first tranche は success-side compare と slot retention だけを持ち、
malformed source を helper 自身がどこまで exact に reject するかはまだ固定していない。

## 比較する 3 案

### 案 1. malformed-source smoke をまだ足さない

current helper は success-side first tranche に留め、
malformed-source smoke は later stage へ送る。

#### 利点

- current first tranche の scope を最も狭く保てる。
- helper-local wording をまだ固定しない。

#### 欠点

- stage 3 admit-slot branch の accepted cluster boundary を fail-closed で示せない。
- `perform` / request cluster が accidental に紛れ込んでも、helper-local test で即座に見えにくい。

### 案 2. accepted-cluster malformed と request-head spillover を 1 件ずつ足す

first malformed pair を 2 本だけに絞り、

1. declaration-side `admit` attached slot の required payload 欠落
2. later-stage request head の spillover

を 1 件ずつ見る。

候補例:

- `option owner_writer on profile_doc capability write lease live admit`
  - `decl_admit_slot` payload が無い
- `perform write_profile via profile_ref`
  - stage 3 first tranche の accepted cluster 外にある request head

#### 利点

- stage 3 helper が
  - 受けるべき cluster の required structure
  - まだ受けてはいけない request cluster
  の両方で fail-closed であることを最小本数で示せる。
- `e3` anchor で later-stage contrast として最も近い `PerformVia` を選べる。

#### 欠点

- helper-local wording fragment を 2 件だけ narrow に固定する必要がある。

### 案 3. accepted-cluster malformed と request-local clause spillover を 1 件ずつ足す

later-stage spillover を `require` / `ensure` line で代表させる。

#### 利点

- request-local clause を直接 non-goal として示せる。

#### 欠点

- `require` / `ensure` は request head と attachment して初めて意味が出るため、
  stage 3 first tranche の malformed-source smoke としては文脈依存が一段強い。
- `PerformVia` spillover より boundary が読み取りにくい。

## error surface の比較

### 案 A. generic `unsupported ...` wording のままにする

#### 利点

- 実装は最短。

#### 欠点

- どの boundary を守って reject したかが読み取りにくい。
- later に wording drift が起きたとき、何を守りたい smoke なのかがぼやける。

### 案 B. helper-local exact wording fragment を 2 件だけ固定する

候補例:

- `missing declaration-side admit slot payload`
- `request head is outside stage 3 admit-slot first tranche`

#### 利点

- public typed carrier を作らずに boundary を明示できる。
- test は substring compare に留められる。
- current phase の helper-local error surface としては十分 narrow である。

#### 欠点

- wording を 2 件だけでも管理する必要がある。

### 案 C. typed error enum / structured row を入れる

#### 利点

- 将来の parser diagnostics へ接続しやすい。

#### 欠点

- current private helper には重すぎる。
- public parser error API の既成事実化に近づく。

## current judgment

current repo の next narrow step としては、次が最も自然である。

1. malformed-source smoke は **案 2** を採る
2. first malformed pair は次の 2 本に留める
   - declaration-side `admit` attached slot の payload 欠落
   - `PerformVia` line の request-head spillover
3. error surface は **案 B** を採り、
   helper-local exact wording fragment の substring compare に留める

## なぜこれが最小か

- `admit` payload 欠落は、stage 3 first tranche が declaration-side attached slot を accepted cluster に含める current judgmentを直接守る。
- `PerformVia` spillover は、`e3` contrast の later-stage 部分として最も source-backed に近く、
  request-local clause 単体より boundary が読みやすい。
- typed error carrier に進まず、helper-local wording fragment 2 件だけに留めることで、
  private / non-production boundary を壊さない。

## current stage でまだやらないこと

- request-local `require` / `ensure` spillover の malformed-source smoke
- fixture-side `OptionDecl.admit` node との direct compare
- typed parser error carrier
- public parser diagnostics API
- dedicated malformed text fixture file

## next narrow step

この comparison を actualize するなら、
`crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
に 2 本の malformed-source smoke を足し、
`crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
の helper-local wording を narrow に整えるのが最小である。
