# 112 — current L2 phase 3 resume side selection

## 目的

この文書は、`specs/examples/111-current-l2-stage3-request-contract-subset-freeze-sequencing.md` で
request contract subset family を current tranche で freeze し、
次は **parser boundary staging** と **first checker cut 接点** のどちらから Phase 3 主線を再開するかを比べると整理したことを前提に、
**どちらを先に再棚卸しするのが最小か**
を比較する。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 1 / stage 3 parser boundary helper family は private / test-only tranche まで source-backed に進んでいる。
- first checker cut 側も helper-local compare family と detached static gate loop の基礎がある。
- request contract subset family は current tranche で freeze する。
- final parser grammar と public checker API は still later に残す。

## current issue

Phase 3 の主線を戻す先として、次の 2 つがある。

1. parser boundary staging 側
2. first checker cut connection 側

ここで決めたいのは、**どちらから再開すると syntax pressure を抑えつつ、local / structural floor の整理を一段前へ進めやすいか**である。

## 比較する 3 案

### 案 1. parser boundary staging 側から再開する

stage 1 / stage 3 private parser spike family の accepted cluster と compare surface をもう 1 段整理する。

#### 利点

- current Phase 3 を parser boundary 主線として読み続けやすい。
- current private helper family と直接つながる。

#### 欠点

- syntax / accepted cluster 側の圧力が先に強くなる。
- request contract subset family を freeze した直後に parser-side widening へ戻ると、freeze の意味が薄れやすい。

### 案 2. first checker cut connection 側から再開する

existing parser-boundary evidence を、
どの local / structural judgment familyから first checker cut inventory へ接続し直すかを先に比較する。

#### 利点

- syntax widening ではなく、decidable / structural floor の整理として進められる。
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` が認めた
  first checker cut readiness と整合する。
- `specs/examples/73-current-l2-first-parser-spike-staging.md` の
  checker-led staged spike judgmentと同じく、syntax-first widening より
  checker / proof boundary の handoff を先に整理できる。
- current private parser helper をすぐ public contract へ近づけずに済む。

#### 欠点

- parser boundary 主線から一段 abstraction が上がる。

### 案 3. parser boundary と first checker cut を同時に詰める

#### 利点

- 一見すると全体が速く進む。

#### 欠点

- compare surface と checker-side meaning boundary が再び混ざる。
- current repo の narrow progression に反する。

## 比較

### syntax pressure

- 案 1 は parser-side accepted cluster と compare surface の widening pressure を先に受ける。
- 案 2 は parser-side evidence を input として使うが、主眼を local / structural judgment へ置ける。
- current phase では final parser grammar も public parser API も still later なので、syntax pressure はなるべく遅らせる方がよい。

### proof / checker boundary との整合

- first checker cut は local / structural / decidable 寄りの floor を入れる方向で整理済みである。
- request contract subset family を freeze した今、次に見るべきなのは「どの parser-side evidence family が checker-side inventory へ渡るか」であって、「どの surface syntax をさらに accepted にするか」ではない。

### current source-backed 性

- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` は、
  mainline を first checker cut 側へ戻してよいだけの local / structural floor が
  current corpus にあると整理している。
- `specs/examples/73-current-l2-first-parser-spike-staging.md` は、
  actual parser spike も checker-led staged spike として進める方が自然だと整理している。
- parser boundary family には already source-backed evidence があり、
 その evidence を checker-side inventory とどう接続するかは、まだ narrow に比較できる余地が大きい。
- したがって、Phase 3 の次段としては checker-side connection を先に切る方が前進量が大きい。

## current judgment

current repo の next narrow step としては、
**案 2. first checker cut connection 側から再開する**
のが最も自然である。

つまり次は、
**existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するか**
を narrow に比較する。

## なぜこれが最小か

- request contract subset family freeze の意味を保てる。
- syntax pressure をさらに上げずに Phase 3 を前進できる。
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` の
  local / structural floor と、
  `specs/examples/73-current-l2-first-parser-spike-staging.md` の
  checker-led parser staging judgmentを、同じ direction で再接続できる。

## current stage でまだやらないこと

- final parser grammar 固定
- public parser API 化
- public checker API 化
- parser boundary と checker boundary の同時 actualization

## next narrow step

次段では、
**existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するか**
を narrow に比較するのが自然である。
