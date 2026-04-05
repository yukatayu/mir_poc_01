# 81 — current L2 stage 1 parser spike malformed-source comparison

## 目的

この文書は、`specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
で current first tranche が actualize 済みであることを前提に、
**stage 1 parser spike が「受けない source」を helper 自身でどこまで fail-closed に smoke すべきか**
を narrow に比較する。

ここで固定するのは final parser error API ではない。
固定するのは、current private / non-production helper に
**最初に足してよい malformed-source smoke の working set**
だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 parser spike は declaration / chain structural floor に留める。
- declaration-side guard slot は opaque attached slot であり、predicate fragment parse は later stage に残す。
- compare surface の主線は依然として lowered fixture-subset compare である。
- malformed-source smoke は helper-local error surface に留め、public parser error API を既成事実化しない。

## current source anchor

- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
  - stage 1 accepted parse cluster と non-goal
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
  - `e4` / `e7` の working set
- `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
  - current actualized first tranche
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
  - current private parser spike helper

current first tranche は success-side structural smoke だけを持ち、
malformed source を parser helper 自身がどこまで exact に reject するかはまだ固定していない。

## 比較する 3 案

### 案 1. malformed-source smoke をまだ足さない

current helper は success-side smoke だけに留め、
parser helper 自身の malformed-source smoke は later stage へ送る。

#### 利点

- current first tranche の scope を最も狭く保てる。
- helper-local error wording を早く固定しない。

#### 欠点

- stage 1 accepted cluster の boundary を fail-closed で実証できない。
- later stage syntax が current helper に紛れ込んでも、test で即座に見えにくい。

### 案 2. accepted-cluster malformed だけを足す

stage 1 が本来受ける cluster の中で、
required structural companion が欠けた source だけを malformed-source smoke にする。

例:

- `fallback mirror`
  - `@ lineage(...)` が欠けている

#### 利点

- accepted parse cluster の必須構造を fail-closed で見られる。
- later stage spillover と混線しにくい。

#### 欠点

- current helper が stage 2 / stage 3 syntax を accidentally 受けないことまでは見られない。
- accepted cluster boundary と later-stage boundary の両方を一度には固定できない。

### 案 3. accepted-cluster malformed と later-stage spillover を 1 件ずつ足す

first tranche の malformed-source smoke を 2 本だけに絞り、

1. accepted cluster の required structure 欠落
2. later-stage syntax の spillover

を 1 件ずつ見る。

候補例:

- `fallback mirror`
  - edge-local lineage metadata 欠落
- `option owner_writer on profile_doc capability write lease live admit owner_is(session_user)`
  - option-local `admit` spillover

#### 利点

- stage 1 helper が
  - 受けるべき cluster の必須構造
  - まだ受けてはいけない later-stage syntax
  の両方で fail-closed であることを最小本数で示せる。
- `perform via` や request clause より declaration boundary に近い spillover を選べる。

#### 欠点

- helper-local error wording を 2 件分だけ narrow に固定する必要がある。
- malformed-source smoke が success-side subset compare 以外の compare line を持つ。

## error surface の比較

### 案 A. generic `unsupported ...` 文字列だけをそのまま使う

#### 利点

- 実装は最短。

#### 欠点

- どの boundary を守って reject したのかが読み取りにくい。
- later に wording drift が起きても、何を守りたい test なのかがぼやける。

### 案 B. helper-local な exact wording fragment を 2 件だけ固定する

例:

- `missing edge-local lineage metadata`
- `option-local admit is outside stage 1 accepted cluster`

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

1. malformed-source smoke は **案 3** を採る
2. first malformed pair は次の 2 本に留める
   - `fallback ...` row に `@ lineage(...)` が無い accepted-cluster malformed
   - option declaration に option-local `admit` が spill した later-stage malformed
3. error surface は **案 B** を採り、
   helper-local exact wording fragment の substring compare に留める

## なぜこれが最小か

- `fallback` row の malformed は、stage 1 が explicit edge-row family と edge-local lineage metadata を accepted cluster に含める current judgmentを直接守る。
- option-local `admit` spillover は、`perform` や request clause より declaration boundary に近く、stage 1 non-goal を最小に確認できる。
- typed error carrier へ進まず、helper-local wording fragment だけに留めることで、private / non-production boundary を壊さない。

## current stage でまだやらないこと

- `perform on` / `perform via` line の malformed-source smoke
- request-local `require` / `ensure` spillover の malformed-source smoke
- typed parser error carrier
- public parser diagnostics API
- dedicated malformed text fixture file

## next narrow step

この comparison を actualize するなら、
`crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
に 2 本の malformed-source smoke を足し、
`crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
の helper-local error wording を narrow に整えるのが最小である。
