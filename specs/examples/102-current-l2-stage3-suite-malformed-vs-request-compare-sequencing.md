# 102 — current L2 stage 3 suite malformed vs request compare sequencing

## 目的

この文書は、`specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md` までで
request-local `require:` / `ensure:` の fixed two-slot suite bridge first tranche が actualize 済みになったことを前提に、
**次段として malformed/source family extension と fixture-side full request contract compare のどちらを先に扱うべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも final diagnostics contract でもない。
固定するのは、stage 3 later branch の **next-step sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge first tranche は helper-local / test-only actual evidence として actualize 済みである。
- current bridge は `require_fragment_text` / `ensure_fragment_text` slot carrier に留まり、request head full parse をまだ導入していない。
- public parser API 化や fixture-side full request contract node compare は still later stage に残している。

## current issue

現在残っている主要 gap は 2 系統ある。

1. malformed/source family extension
   - duplicate `ensure`
   - missing multiline `ensure:` block
   - unsupported direct child line
   - wording の fail-closed tightening
2. fixture-side full request contract compare
   - request head kind
   - request-local contract slot array / ordering
   - fixture-side `PerformOn` / `PerformVia` subset との direct compare

次段では、この 2 系統のどちらを先に比較 / actualize するかを決める必要がある。

## 比較する 3 案

### 案 1. malformed/source family extension を先に比較する

fixed two-slot suite bridge を current floor として据えたまま、
helper-local malformed/source family をもう 1 段広げる。

たとえば次のような question を先に切る。

```text
perform write_profile on profile_doc
  ensure owner_is(session_user)
  ensure owner_is(session_user)
```

```text
perform write_profile on profile_doc
  require write
  note delegated
```

- duplicate `ensure` を current stage で malformed として固定するか
- unsupported direct child line を generic continuation と誤読させず reject するか
- missing `ensure:` block wording を request-local suite bridge familyへ昇格させるか

#### 利点

- current bridge が暗黙に持っている fail-closed path を source-backed に可視化できる。
- full request compare より narrow であり、request head parse pressure を増やさない。
- helper-local diagnostics family を整理してから structural compare を太らせられる。

#### 欠点

- fixture-side structure compare 自体はすぐには増えない。

### 案 2. fixture-side full request contract compare を先に比較する

suite bridge の次段として、
request-local contract subset を fixture-side `PerformOn` / `PerformVia` contract node と直接 compare する line を先に比べる。

#### 利点

- request-local suite compare と fixture-side machine-readable structure が近づく。
- bridge から full request compare への道筋は見えやすい。

#### 欠点

- request head parse / accepted cluster widening の pressure が強い。
- helper-local malformed family の hidden behavior を未整理のまま structural compare を太らせやすい。
- current Phase 3 の narrow line を越えやすい。

### 案 3. malformed extension と request compare を同時に開く

#### 利点

- stage 3 later branch を一気に進めているように見える。

#### 欠点

- diagnostics family と structural compare family が再び混ざる。
- current repo の narrow progression に反する。

## 比較

### current bridge との連続性

- fixed two-slot suite bridge が actualize 済みになった直後の gap は、helper-local fail-closed family の source-backed tightening 側にある。
- full request compare を先に開くと、bridge を request node compare の前処理として既成事実化しやすい。

### parser / checker pressure

- malformed/source family extension は helper-local wording / guard の比較で止まる。
- fixture-side full request compare は request head parse、contract row ownership、accepted cluster widening と近接しやすい。
- current phase では前者の方が pressure が小さい。

### hidden behavior の抑制

- current helper は two-slot carrier と structural fail-closed を already 持っているため、duplicate `ensure` や unsupported direct child line の line を docs-first で揃える価値が高い。
- これを飛ばして full request compare へ行くと、helper-local reject path が repo memory に十分固定されないまま前進しやすい。

## current judgment

current repo の next narrow step としては、
**案 1. malformed/source family extension を先に比較する**
のが最も自然である。

つまり、

1. fixed two-slot suite bridge first tranche は current floor として据え置く
2. 次は request-local suite bridge family の helper-local malformed/source pair をどこから actualize するかを docs-only で比較する
3. fixture-side full request contract compare はその後段に残す

## なぜこれが最小か

- request head full parse を still later stage に残せる。
- bridge first tranche が既に持つ fail-closed path を hidden behavior のまま放置しない。
- malformed family と structural compare family を同時に膨らませずに済む。

## current stage でまだやらないこと

- fixture-side full request contract subset compare
- request head kind / target carrier の parse
- public parser API 化
- typed diagnostics carrier

## next narrow step

次は、
**request-local suite bridge family の helper-local malformed/source pair をどこから actualize するか**
を docs-only で整理するのが自然である。
