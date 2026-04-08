# 96 — current L2 stage 3 multiline attachment shape comparison

## 目的

この文書は、`specs/examples/95-current-l2-stage3-fragment-vs-attachment-next-step-sequencing.md` で
predicate fragment helper の malformed-source pair より先に
**declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment shape を比較する**
と整理したことを前提に、
**current stage でどこまで shared structural floor を切るのが最小か**
を narrow に比較する。

ここで固定するのは final parser grammar でも final checker API でもない。
固定するのは、isolated predicate fragment helper へ渡す前段として

- 何を attachment frame として共有してよいか
- 何を request-local clause suite / option-local metadata role に残すべきか

という **stage 3 multiline attachment cut** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- shared isolated predicate fragment helper は success-side compare まで actualize 済みである。
- request head / option declaration の full parser actualization はまだ行わない。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` では、
  - request-local / option-local clause attachment
  - minimal predicate fragment well-formedness
  を別 cluster として扱っている。
- `specs/examples/01-current-l2-surface-syntax-candidates.md` では、
  - request-local `require:` / `ensure:`
  - option-local `admit:`
  がいずれも header line + deeper predicate block を current companion 候補にしている。

## current source anchor

- `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `admit:` / `require:` / `ensure:` の multiline candidate
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - clause attachment と predicate fragment の cluster 分離
- `specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md`
  - shared isolated predicate fragment helper を reopen cut に採る
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
  - isolated fragment helper first tranche は actualize 済み
- `specs/examples/95-current-l2-stage3-fragment-vs-attachment-next-step-sequencing.md`
  - next narrow step は multiline attachment shape comparison

current issue は、
declaration-side `admit:` と request-local `require:` / `ensure:` に対して
次のどこまでを shared floor として扱うのが最小かである。

## 比較する 3 案

### 案 1. shared single attachment frame を切る

`admit:` / `require:` / `ensure:` のどれであっても、
current stage では次の **single attachment frame** だけを共有する。

```text
<clause-head>:
  <predicate-line-1>
  <predicate-line-2>
```

ここで共有するのは次だけである。

- header line は clause marker + `:`
- 直下に 1 段深い predicate block を持つ
- predicate block は isolated fragment string へ切り出す source carrier である
- predicate block 内に blank line は置かない

一方で、次は共有しない。

- `perform` が clause suite owner であること
- `option` が option-local metadata owner であること
- `require` と `ensure` の suite ordering
- `admit` が at-most-one attachment であること

#### 利点

- clause attachment cluster と predicate fragment cluster の境界を保ちながら、shared structural floor だけを切れる。
- declaration-side / request-local の両 branch が、同じ isolated fragment helper input formation を使える。
- request head full parse や option body full parse をまだ導入しない。

#### 欠点

- shared floor といっても clause suite 全体は共有しないため、role separation を文書で明示しないと generic block parser のように誤読されうる。

### 案 2. request-local clause suite だけを multiline attachment family として扱う

`require:` / `ensure:` だけに multiline attachment rule を先に与え、
`admit:` は declaration-side option metadata として別 branch に残す。

#### 利点

- request-local branch だけ見れば `perform` owner の clause suite と一体で説明しやすい。
- `admit` 側の option body continuation を still later stage に残せる。

#### 欠点

- declaration-side と request-local が shared floor を持たない。
- `admit:` multiline の reopen が later branch へずれ、stage 3 line が再び片寄る。
- shared isolated predicate fragment helper と接続する source carrier が 2 系統に分かれやすい。

### 案 3. generic statement continuation block を切る

`perform` でも `option` でも、
「header line の下に indented continuation block が来る」という generic rule を先に切る。

#### 利点

- 見た目上は最も統一的に見える。

#### 欠点

- clause attachment / option metadata / chain continuation / predicate block continuation が generic continuation として混ざりやすい。
- current stage で必要なのは generic continuation rule ではなく、shared isolated fragment helper に渡す source carrier だけである。
- stage 3 narrow progression を外れやすい。

## 比較

### shared helper input との整合

- 案 1 は declaration-side `admit:` と request-local `require:` / `ensure:` のどちらも、`header + deeper predicate block` から isolated fragment string を切り出せる。
- 案 2 は request-local branch に偏る。
- 案 3 は helper input 形成より generic continuation rule が前面に出る。

### cluster 分離の維持

- `specs/examples/30...` の line では、clause attachment と predicate fragment は別 cluster である。
- 案 1 は shared floor を attachment frame だけに留めるため、この分離と最も整合する。
- 案 2 も分離自体は保てるが、declaration-side branch の reopen が遅れる。
- 案 3 は continuation generic 化により cluster 境界をにじませやすい。

### hidden parser pressure の回避

- 案 1 は request head / option body の full parse を deferred にしたまま、attachment frame だけを切れる。
- 案 2 も request-local branch では narrow だが、later branch に option-local multiline reopen を積み残す。
- 案 3 は generic indentation parser への pressure を最も増やす。

## current judgment

current repo の next narrow step としては、
**案 1. shared single attachment frame を切る**
のが最も自然である。

つまり current stage では、

1. declaration-side `admit:`
2. request-local `require:`
3. request-local `ensure:`

の 3 つに共通するものとして、
**`<clause-head>:` + 直下 1 段深い predicate block**
だけを shared structural floor にする。

ただし shared にするのは **attachment frame** だけであって、

- request-local clause suite owner が `perform` であること
- declaration-side metadata owner が `option` であること
- clause suite ordering / multiplicity / suite termination

までは generic 化しない。

## shared floor に含めるもの

- header line は clause marker + `:`
- predicate block は header より 1 段深い
- predicate block の本体は isolated fragment string source として読む
- predicate block 内の blank line は current stage では不可
- predicate block から header indent へ戻ること自体は許す

## shared floor に含めないもの

- `perform` head の parse
- `option` declaration full body parse
- request-local clause suite の exact multiplicity / ordering / dedent rule の completion
- `admit` を clause suite member と読むこと
- generic statement continuation rule

## なぜこれが最小か

- declaration-side と request-local の両方を同じ isolated fragment helper input へ接続できる。
- clause attachment と predicate fragment の cluster 分離を壊さない。
- request head / option body / generic continuation parser を hidden に導入しない。
- later stage で
  - request-local clause suite
  - option-local metadata continuation
  - fragment malformed diagnostics
  を別々に actualize する余地を残せる。

## current stage でまだやらないこと

- request head + clause attachment の full parser actualization
- option declaration body continuation の full parser actualization
- `require` / `ensure` suite ordering / multiplicity の finalization
- fragment malformed-source pair の actualization
- generic indentation-sensitive continuation rule の導入

## next narrow step

次は、
**shared single attachment frame を helper-local / test-only actual evidence にどこまで actualize するか**
を narrow に比較するのが自然である。

その actualization では、

- declaration-side `admit:`
- request-local `require:`
- request-local `ensure:`

の multiline block extraction だけを helper-local に切り出し、
isolated predicate fragment helper へ渡す source carrier compare に留めるのが最小候補である。
