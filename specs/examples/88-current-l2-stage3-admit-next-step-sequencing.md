# 88 — current L2 stage 3 admit next-step sequencing

## 目的

この文書は、`specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md`
までで

- declaration-side `admit` attached slot の success-side first tranche
- malformed-source first tranche

が揃ったことを前提に、
**次の narrow step として `request-local clause spillover` と `fixture-side OptionDecl.admit handoff` のどちらを先に比較すべきか**
を整理する。

ここで固定するのは final parser grammar でも final parser API でもない。
固定するのは、stage 3 line の lexical pressure を unnecessary に増やさず、
stage 1 handoff line と整合する **sequencing judgment** だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 3 first tranche は declaration-side `admit` attached slot に留める。
- `PerformVia` / request-local `require` / `ensure` / predicate fragment parse は still later stage に残す。
- fixture-side `OptionDecl.admit` predicate node は current parser-free AST fixture schema の already elaborated carrier である。
- stage 1 line では parser-side opaque slot carrier と fixture-side compatibility anchor を thin lowering で分ける judgment がある。

## current source anchor

- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
  - stage 1 は parser-side opaque slot carrier と fixture-side `OptionDecl.lease` を thin lowering で接続する
- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
  - stage 3 の最初の sub-cut は declaration-side `admit` attached slot
- `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
  - fixture-side `OptionDecl.admit` への direct lowering は current first tranche でしない
- `specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md`
  - malformed-source first tranche は request head spillover まで actualize 済み

current stage で未着手なのは、主に次の 2 本である。

1. request-local `require` / `ensure` spillover
2. fixture-side `OptionDecl.admit` node handoff comparison

## 比較する 3 案

### 案 1. request-local clause spillover を先に比較する

`require` / `ensure` line を next docs-only step として先に扱う。

#### 利点

- stage 3 request / admissibility cluster の request 側へ早く近づける。
- `PerformVia` spillover に続く later-stage boundary を細かく見られる。

#### 欠点

- request head / request-local clause attachment の前提が必要で、stage 3 attach rule を早く持ち込みやすい。
- declaration-side `admit` branch 自体の handoff 境界をまだ整理しないまま request cluster へ進むことになる。
- lexical freeze pressure が高い。

### 案 2. fixture-side `OptionDecl.admit` handoff comparison を先に比較する

request cluster へ進む前に、
declaration-side `admit` attached slot と fixture-side `OptionDecl.admit` predicate node の間を
どこまで、どの形で later stage に残すかを比較する。

#### 利点

- declaration-side branch の中で閉じた比較なので、request attachment rule をまだ持ち込まない。
- stage 1 handoff line と対称に、opaque slot と compatibility anchor の切り分けを先に整理できる。
- `PerformVia` / `require` / `ensure` を still later stage に残す current judgmentと整合する。

#### 欠点

- request cluster 自体の前進は直接は増えない。
- fixture-side `OptionDecl.admit` が already elaborated predicate node なので、比較の切り方を間違えると hidden predicate parse へ寄りやすい。

### 案 3. request-local clause spillover と handoff を同時に比較する

stage 3 remaining pieces を一度にまとめて扱う。

#### 利点

- stage 3 line の残件を一気に見通せる。

#### 欠点

- declaration-side branch と request-side branch の論点が混ざる。
- hidden parse / hidden attachment / hidden diagnostics pressure が最も高い。
- current phase の narrow progression に合わない。

## 比較

### stage order の維持

- 案 1 は request-local clause を先に扱うぶん、`PerformVia` / request head 以降の later-stage attachment を早く意識させる。
- 案 2 は declaration-side branch の内部 comparison なので、stage order を崩しにくい。
- 案 3 は最も崩れやすい。

### stage 1 handoff line との対称性

- 案 2 だけが、stage 1 の opaque slot handoff line と直接比較可能である。
- 案 1 は request cluster に寄るため、stage 1 line と連続比較しにくい。

### hidden parse / hidden attachment の回避

- 案 1 は request attachment を hidden に先食いしやすい。
- 案 2 は declaration-side handoff だけを比較する限り、request attachment を避けやすい。
- 案 3 は両方の hidden pressure を同時に持つ。

### next actualization への接続

- 案 1 は later-stage malformed smoke へ寄るが、actualization すると helper wording が request cluster に広がりやすい。
- 案 2 は docs-only handoff comparison に留めやすく、必要なら stage 1 と同じく thin handoff line を later step で actualize できる。
- 案 3 は actualization の cut が曖昧である。

## current judgment

current repo の next narrow step としては、
**案 2. fixture-side `OptionDecl.admit` handoff comparison を先に比較する**
のが最も自然である。

つまり、

1. stage 3 declaration-side `admit` branch の次段は、request-local clause spillover ではなく handoff comparison を先に扱う
2. request-local `require` / `ensure` spillover は still later stage に残す
3. stage 1 handoff line との対称性を保ちながら、fixture-side `OptionDecl.admit` predicate node を hidden parse せずにどう later stage へ渡すかを先に整理する

## なぜこれが最小か

- current malformed-source first tranche ですでに request head spillover までは fail-closed に示せている。
- その次に request-local clause malformed へ進むと、attachment rule を暗黙に背負いやすい。
- 一方 handoff comparison は declaration-side branch の内部に閉じており、stage 1 line からの類推が効く。

## current stage でまだやらないこと

- request-local `require` / `ensure` spillover の actualization
- `PerformVia` parse の拡張
- fixture-side `OptionDecl.admit` predicate node への direct lowering
- predicate fragment canonicalization

## next narrow step

次は
`specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
として、
fixture-side `OptionDecl.admit` node handoff の最小 comparison を docs-only で切るのが自然である。
