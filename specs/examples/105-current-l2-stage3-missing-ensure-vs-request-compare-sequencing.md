# 105 — current L2 stage 3 missing ensure vs request compare sequencing

## 目的

この文書は、`specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md` までで
request-local suite bridge family の first malformed/source pair
`duplicate ensure` + unsupported direct child line が helper-local / test-only actual evidence として
surfaced 済みになったことを前提に、
**次段として `missing multiline ensure block` family と fixture-side full request contract compare のどちらを先に開くべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも public diagnostics carrier でもない。
固定するのは、stage 3 later branch の **next-step sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge first tranche と first malformed/source pair actualization は already 済みである。
- current helper は private / test-only に留める。
- `missing multiline ensure block` は current helper の hidden fail-closed path として already 存在するが、focused smoke にはまだ上がっていない。
- fixture-side full request contract compare は still later stage に残っている。

## current issue

current stage で次に開ける主要 line は 2 系統ある。

1. **`missing multiline ensure block` family**
   - request-local suite bridge family の remaining helper-local malformed/source extension
   - multiline attachment bridge family に近いが、suite helper 側にも hidden fail-closed path がある
2. **fixture-side full request contract compare**
   - request head kind
   - request-local contract slot ordering / ownership
   - `PerformOn` / `PerformVia` subset との direct compare

次段では、この 2 系統のどちらを先に比較 / actualize するかを決める必要がある。

## 比較する 3 案

### 案 1. `missing multiline ensure block` family を先に actualize する

```text
perform write_profile on profile_doc
  ensure:
```

#### 利点

- current helper が already 持つ hidden fail-closed path を narrow に surfaced できる。
- full request compare より pressure が軽く、request head parse widening を still later に残せる。
- suite bridge family の malformed/source extension をもう 1 件だけ閉じられる。

#### 欠点

- fixture-side structure compare 自体は still later のままである。
- `missing multiline ensure block` は multiline attachment bridge family に近いため、suite family 固有の前進量は `unsupported direct child line` より弱い。

### 案 2. fixture-side full request contract compare を先に比較する

#### 利点

- suite bridge family を fixture-side structure compare へ早く接続できる。
- request-local contract subset を machine-readable carrier へ寄せる道筋が見えやすい。

#### 欠点

- request head parse / accepted cluster widening の pressure が強い。
- helper-local malformed family をまだ 1 件残したまま structural compare を太らせやすい。
- current stage では diagnostics family と structural compare family を再び近づけすぎる。

### 案 3. `missing multiline ensure block` family と full request compare を同時に開く

#### 利点

- stage 3 later branch が一見早く進む。

#### 欠点

- malformed/source family と structural compare family が再び混ざる。
- current repo の narrow progression に反する。

## 比較

### current helper pressure

- `missing multiline ensure block` family は current helper / test-only surface の内側で閉じる。
- fixture-side full request compare は request head kind、contract row ownership、fixture-side compare carrier と直結しやすく、pressure が大きい。

### hidden behavior の抑制

- current helper が already 持つ fail-closed path を source-backed に surfaced するという current phase の line に照らすと、`missing multiline ensure block` family は still narrow である。
- これを飛ばして full request compare へ行くと、helper-local malformed family が 1 件 implicit に残る。

### family separation

- `missing multiline ensure block` family は multiline attachment bridge family に近いが、current question は request-local suite bridge family の remaining hidden path をどう扱うかでもある。
- current phase では、family 間の近さより **request head parse pressure をまだ避ける** ことの方が重要である。

## current judgment

current repo の next narrow step としては、
**案 1. `missing multiline ensure block` family を先に actualize する**
のが最も自然である。

つまり、

1. suite bridge family の remaining helper-local malformed/source extension をもう 1 件だけ narrow に surfaced する
2. fixture-side full request contract compare は still later stage に残す
3. request head parse / accepted cluster widening をまだ開かない

## なぜこれが最小か

- current helper が already 持つ hidden fail-closed path を source-backed に固定できる。
- full request compare の pressure を 1 段後ろへ送れる。
- suite bridge family の malformed/source extension を current phase で過不足なく閉じられる。

## current stage でまだやらないこと

- fixture-side full request contract subset compare
- request head kind parse
- public parser API 化
- typed diagnostics carrier

## next narrow step

次は、
**`missing multiline ensure block` family の helper-local / test-only first tranche を actualize する**
のが自然である。
