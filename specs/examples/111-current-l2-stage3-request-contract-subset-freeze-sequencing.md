# 111 — current L2 stage 3 request contract subset freeze sequencing

## 目的

この文書は、`specs/examples/110-current-l2-stage3-request-contract-subset-widening-guard.md` で
request contract subset family を still `0-or-1` guard に留めると整理したことを前提に、
**この family をなお続けて source-side suite bridge widening entry criteria を先に比べるべきか、それとも一旦 freeze して別 Phase 3 subline に戻るべきか**
を比較する。

ここで固定するのは final parser grammar でも request head parse でもない。
固定するのは、Phase 3 主線における **sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- request contract subset family では、
  - reopen judgment
  - first-cut judgment
  - first tranche actualization
  - widening guard
  まで source-backed に揃っている。
- current fixture corpus には multi-row clause array anchor がない。
- request head kind / op / target / chain_ref parse は still later stage に残っている。
- Phase 3 全体では、parser boundary staging と first checker cut の接点を docs-only に再棚卸しする余地が still 残っている。

## current issue

request contract subset family の next candidate は一応まだ 2 つある。

1. source-side suite bridge widening の entry criteria を先に比較する
2. この family は current tranche で freeze し、別 Phase 3 subline に戻る

ここで決めたいのは、**Phase 3 主線としてどちらが前進量 / 理論の明晰さ / source-backed 性のバランスがよいか**である。

## 比較する 3 案

### 案 1. source-side suite bridge widening entry criteria をこのまま先に比較する

この案では、row-list widening 自体はまだ採らないが、
future widening を許す条件だけを先に比べる。

#### 利点

- request contract subset family の出口条件を先に整理できる。
- future widening を再開するときの足場が明示になる。

#### 欠点

- current corpus に multi-row anchor が無い段階で、future widening 条件だけを先に積むと speculative ratio が高い。
- family 自体の current source-backed closure は already 十分であり、直近の前進量が小さい。

### 案 2. request contract subset family を current tranche で freeze し、別 Phase 3 subline に戻る

この案では、current family は

- `Stage3RequestContractSubset`
- `0-or-1` guard
- request head metadata still deferred

の line で一旦 freeze し、次は parser boundary staging と first checker cut 接点の docs-only re-sweep に戻る。

#### 利点

- source-backed closure がある family を unnecessary に speculative widening しないで済む。
- Phase 3 全体として still 残っている parser boundary / checker boundary の整理へ作業を戻せる。
- request contract subset family の current shape を stable comparison checkpoint として扱える。

#### 欠点

- request contract subset family 自体の next widening は後ろへ回る。

### 案 3. request head metadata 方向へもう 1 段進める

#### 利点

- 見かけ上は request compare family が前に進む。

#### 欠点

- spec107 / 108 / 109 / 110 で deliberately 残した `request head still later` line に逆行する。
- request head parse pressure と compare surface widening pressure を同時に上げる。
- current staged line に反する。

## 比較

### source-backed 性

- 案 1 は完全に speculative ではないが、future widening 条件の比較が current corpus より先に出る。
- 案 2 は already source-backed に閉じた current family を checkpoint 化するので、最も conservative である。
- 案 3 は current source-backed line を越える。

### Phase 3 全体の前進量

- request contract subset family は current state でも reopen / first-cut / actualization / guard まで揃っている。
- 一方で Phase 3 全体には、parser boundary staging と first checker cut 接点の re-sweep という、なお broad だが current mainline に近い比較が残っている。
- したがって、family 内の speculative next-step を詰めるより、Phase 3 主線の broader connection に戻る方が前進量が大きい。

### 理論の明晰さ

- 案 2 は「contract-only compare family はここで一旦閉じる」「request head metadata は still later」と line をきれいに保てる。
- 案 1 は future widening を意識し始めるぶん、family freeze line が曖昧になる。

## current judgment

current repo の next narrow step としては、
**案 2. request contract subset family を current tranche で freeze し、別 Phase 3 subline に戻る**
のが最も自然である。

戻り先の first choice は、
**parser boundary staging と first checker cut 接点の docs-only re-sweep**
である。

## なぜこれが最小か

- request contract subset family は current source-backed evidence で十分に checkpoint 化できる。
- widening guard の先を speculative に積むより、Phase 3 全体の remaining connection を整理する方が前進量が大きい。
- request head metadata still deferred という current line を崩さずに済む。

## current stage でまだやらないこと

- source-side suite bridge の row-list widening
- request head metadata parse
- fixture-side full request node compare
- public parser API 化

## next narrow step

次段では、
**parser boundary staging と first checker cut 接点を current Phase 3 主線としてどう再棚卸しするか**
を narrow に比較するのが自然である。
