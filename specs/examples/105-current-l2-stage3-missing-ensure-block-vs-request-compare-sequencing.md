# 105 — current L2 stage 3 missing ensure block vs request compare sequencing

## 目的

この文書は、`specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md` までで
request-local suite bridge family の first malformed/source pair が surfaced 済みになったことを前提に、
**次段として `missing multiline ensure block` family と fixture-side full request contract compare のどちらを先に開くべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも final request contract node carrier でもない。
固定するのは、stage 3 later branch の **next-step sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge first tranche は actualize 済みである。
- `duplicate ensure` と unsupported direct child line の first pair は helper-local / test-only actual evidence として surfaced 済みである。
- helper は private / test-only のまま維持する。

## current issue

first pair actualization の後に残る next-step 候補は、大きく 2 系統ある。

1. **helper-local missing multiline block family**
   - `ensure:` に続く predicate block が空
   - current multiline attachment bridge family と suite bridge family の接点を narrow に固める
2. **fixture-side full request contract compare**
   - request head / contract slot array / ordering / ownership を fixture-side node compare に開く

次段では、この 2 系統のどちらを先に扱うかを決める必要がある。

## 比較する 3 案

### 案 1. `missing multiline ensure block` family を先に actualize する

```text
perform write_profile on profile_doc
  ensure:
```

#### 利点

- current helper が already 持つ hidden fail-closed path をもう 1 本 narrow に surfaced できる。
- request head parse pressure を増やさずに、suite bridge family の残りを 1 段閉じられる。
- multiline attachment bridge family と suite bridge family の接点を source-backed に確認できる。

#### 欠点

- fixture-side full request compare 自体は still later に残る。

### 案 2. fixture-side full request contract compare を先に比較 / actualize する

#### 利点

- suite bridge family から full request compare への道筋は見えやすい。
- request-local suite compare と fixture-side machine-readable structure が近づく。

#### 欠点

- request head parse / accepted cluster widening の pressure が強い。
- helper-local malformed family を still hidden のまま structural compare を太らせやすい。
- current stage の narrow line を越えやすい。

### 案 3. `missing multiline ensure block` と full request compare を同時に開く

#### 利点

- stage 3 later branch を一気に進めているように見える。

#### 欠点

- helper-local malformed family と fixture-side structure compare family が again 混ざる。
- current repo の narrow progression に反する。

## 比較

### current helper family との連続性

- first pair actualization までで、suite bridge family は still helper-local hidden path の surfacing を主目的として進んでいる。
- `missing multiline ensure block` はこの line の延長にあり、full request compare は別 family である。

### parser / checker pressure

- `missing multiline ensure block` family は current helper / test-only boundary の中で閉じる。
- full request compare は request head kind、target carrier、contract slot ordering まで pressure を広げやすい。
- current Phase 3 では前者の方が narrow である。

### existing hidden behavior の明示

- current helper は `ensure:` multiline payload 欠落に対して already fail-closed path を持っている。
- これを source-backed に surfaced してから full request compare へ進む方が、later-stage structural compare を hidden behavior の上に積まないで済む。

## current judgment

current repo の next narrow step としては、
**案 1. `missing multiline ensure block` family を先に actualize する**
のが最も自然である。

つまり、

1. next tranche でも helper-local hidden fail-closed path の surfacing を続ける
2. `missing multiline ensure block` を focused smoke に上げる
3. fixture-side full request contract compare はその後段に残す

## なぜこれが最小か

- request head parse / contract node compare を still later に残せる。
- multiline attachment bridge family と suite bridge family の接点を current helper family の中で確認できる。
- diagnostics family と structural compare family を同時に太らせずに済む。

## current stage でまだやらないこと

- fixture-side full request contract compare
- request head kind / target carrier parse
- public parser API 化
- typed diagnostics carrier

## next narrow step

次は、
**`missing multiline ensure block` family を helper-local / test-only actual evidence に上げる**
のが自然である。
