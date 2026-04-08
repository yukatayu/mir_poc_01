# 107 — current L2 stage 3 suite reopen conditions

## 目的

この文書は、`specs/examples/106-current-l2-stage3-missing-ensure-first-tranche-actualization.md` までで
fixed two-slot suite bridge family の hidden fail-closed path を

- duplicate `require`
- duplicate `ensure`
- `require` after `ensure`
- clause-between blank line
- unsupported direct child line
- missing multiline `ensure:` block

まで surfaced 済みにしたことを前提に、
**remaining suite malformed wording family をまだ追うべきか、それとも fixture-side full request contract compare を narrow に reopen してよいか**
を比較する。

ここで固定するのは final parser grammar でも public diagnostics carrier でもない。
固定するのは、stage 3 later branch の **reopen 条件** と **next-step sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge helper は private / test-only に留める。
- current helper output は `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` の two-slot carrier に留める。
- fixture-side full request contract compare はまだ actualize していない。
- shared single attachment frame helper では、少なくとも `missing multiline predicate block after require:` と `blank line is not allowed inside multiline predicate block after admit:` が source-backed に surfaced 済みであり、attachment-floor 側の hidden path を suite helper とは別 family として already 持っている。
- declaration-side admit-slot branch では、bare request-local `require` / `ensure` spillover line の reject が already source-backed に surfaced 済みである。

## current issue

current helper が still 持っている hidden path のうち、次段で問題になるのは主に次である。

1. `missing multiline predicate block after require:`
2. `missing predicate fragment after \`require\``
3. `missing predicate fragment after \`ensure\``
4. `blank line is not allowed inside multiline predicate block after require:`
5. `blank line is not allowed inside multiline predicate block after ensure:`
6. `unexpected nested continuation outside request-local clause block`

ここで決めたいのは、

- この remaining family を suite helper の中でまだ surfaced する価値があるか
- それとも current phase では already enough とみなし、fixture-side full request contract compare の narrow reopen へ進むべきか

である。

## 比較する 3 案

### 案 1. remaining suite malformed wording family をもう 1 段 actualize する

たとえば次を stage 3 suite helper 側でさらに surfaced する。

- `missing multiline predicate block after require:`
- `missing predicate fragment after \`require\`` / `ensure`
- block-inside blank line wording

#### 利点

- suite helper が still 持つ hidden wording をさらに減らせる。
- `require` / `ensure` symmetry をもう 1 段補える。

#### 欠点

- `missing multiline predicate block after require:` は shared single attachment frame helper で already source-backed である。
- request-local block-inside blank line wording は current suite helper ではまだ hidden だが、これは suite owner / ordering / termination floor というより multiline block-internal policy に寄る。
- bare clause payload 欠落は clause-token family の問題であり、suite owner / ordering / termination floor 固有の前進量が弱い。
- `unexpected nested continuation outside request-local clause block` まで追い始めると generic continuation / diagnostics widening に寄りやすい。

### 案 2. fixture-side full request contract compare を narrow に reopen する

remaining hidden path のうち suite 固有の前進量が高いものは already surfaced 済みだとみなし、
次は fixture-side `contract.require` / `contract.ensure` subset compare を narrow に比較する。

ただし reopen は次の条件つきに留める。

- source-side helper output は still `Stage3RequestClauseSuite` に留める
- request head parse は導入しない
- fixture-side compare は `PerformOn` / `PerformVia` の contract subset だけに限定する
- public parser API / typed diagnostics carrier は still later に残す

#### 利点

- suite helper family を unnecessary に wording 追従へ広げずに済む。
- current two-slot bridge を fixture-side machine-readable carrier へ近づけられる。
- request head parse pressure を抑えたまま、Phase 3 の structural compare line を 1 段前へ出せる。

#### 欠点

- request compare の first cut を慎重に絞らないと、request head kind / accepted cluster widening と混ざりやすい。

### 案 3. remaining malformed wording と full request compare を同時に開く

#### 利点

- stage 3 later branch が一見早く進む。

#### 欠点

- diagnostics family と fixture-side structural compare family が再び混ざる。
- current repo の narrow progression に反する。

## 比較

### suite 固有の前進量

- duplicate symmetry、unsupported direct child、suite-level missing `ensure:` block までは、fixed two-slot suite bridge family 固有の hidden path として価値があった。
- 一方で `missing multiline predicate block after require:` は shared attachment-floor helper で already source-backed であり、request-local block-inside blank line wording も multiline block-internal policy と意味論的に近く、suite bridge family 固有の前進量が弱い。
- bare clause payload 欠落も clause-token / fragment floor に近く、suite owner / ordering / termination の問いではない。

### helper family の重複回避

- current repo では stage 3 later branch を
  - admit-slot spillover
  - isolated predicate fragment
  - shared single attachment frame
  - fixed two-slot suite bridge
  の family に分けて進めている。
- remaining hidden wording を suite helper だけで追い続けると、attachment helper や clause-token family と evidence が重複しやすい。
- current phase では、family ごとの責務分離を維持する方が理論的にきれいである。

### full request compare の reopen 条件

fixture-side full request contract compare を reopen してよいのは、少なくとも次を守る場合に限る。

1. source-side accepted cluster を still 広げない
2. request head kind / target parse を still 導入しない
3. compare carrier は fixed two-slot suite bridge の two-slot shape を超えない
4. fixture-side compare は contract subset だけに限定する
5. public diagnostics carrier を増やさない

この条件なら、request compare は helper-local malformed family と衝突せず、
Phase 3 の structural compare line として reopen してよい。

## current judgment

current repo の next narrow step としては、
**案 2. fixture-side full request contract compare を narrow に reopen する**
のが最も自然である。

ただし、その reopen は

- request head parse を still later に残し
- source-side helper output を `Stage3RequestClauseSuite` の two-slot carrierに留め
- fixture-side `contract.require` / `contract.ensure` subset compare だけを first cut にする

という条件つきで読む。

## なぜこれが最小か

- suite helper family 固有の hidden path は、current phase で意味のある範囲まで already surfaced 済みである。
- remaining hidden wordingの多くは shared attachment helper か clause-token family と重複しやすい。
- ここで request compare を reopen しても、request head parse と public diagnostics widening を still later に残せる。

## current stage でまだやらないこと

- request head kind parse
- `PerformOn` / `PerformVia` source-side reconstruction
- public parser API 化
- typed diagnostics carrier
- generic continuation family の widening

## next narrow step

次は、
**fixed two-slot suite bridge を fixture-side full request contract subset compare へどこまで actualize してよいか**
を narrow に比較するのが自然である。
