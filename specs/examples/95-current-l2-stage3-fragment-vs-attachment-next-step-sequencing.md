# 95 — current L2 stage 3 fragment vs attachment next-step sequencing

## 目的

この文書は、`specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md` までで

- shared isolated predicate fragment helper
- `e2` / `e3` / `e10` / `e11` anchor の success-side compare

が揃ったことを前提に、
**次段として predicate fragment helper の malformed-source pair と request head + clause attachment multiline shape のどちらを先に比較すべきか**
を narrow に整理する。

ここで固定するのは final parser grammar でも final diagnostics surface でもない。
固定するのは、Phase 3 mainline を unnecessary にねじらせない **next-step sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semanticsは変更しない。
- stage 3 helper は private / test-only のまま維持する。
- shared isolated predicate fragment helper は current phase で success-side compare だけを持つ。
- request head + clause attachment multiline shape はまだ docs-only でも actual compare でも開いていない。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` では、
  - request-local / option-local clause attachment
  - minimal predicate fragment well-formedness
  を別 cluster として切っている。

## current source anchor

- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - clause attachment と predicate fragment は別 cluster
- `specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md`
  - predicate fragment boundary は shared isolated helper から reopen する
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
  - success-side compare まで actualize 済み
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `admit:` / `require:` / `ensure:` の header + indented predicate block 候補

current issue は、次の narrow step を

1. helper-local predicate malformed-source pair
2. request head + clause attachment multiline shape

のどちらから開くかである。

## 比較する 3 案

### 案 1. predicate fragment helper の malformed-source pair を先に actualize する

たとえば次のような fragment-local malformed pair を先に切る。

```text
(
  owner_is(session_user)
  and
)
```

```text
not owner_is(session_user)
```

#### 利点

- current isolated helper の内側で閉じた比較なので、実装は最も narrow に見える。
- helper-local diagnostics wording を先に持てる。

#### 欠点

- request head / clause attachment との bridge をまだ作らない。
- fragment malformed を先に増やすと、program surface 側の structural questionより先に helper-local wording が太りやすい。
- `specs/examples/30` の cluster 順では、predicate fragment と clause attachment の橋渡しがまだ見えない。

### 案 2. request head + clause attachment multiline shape を先に docs-only comparison する

たとえば次のような surface shape を、まだ full parse せずに comparison する。

```text
perform write_profile via profile_ref
  require:
    (
      owner_is(session_user)
      and well_formed(profile_draft)
    )
```

```text
option owner_writer on profile_doc capability write lease live
  admit:
    (
      owner_is(session_user)
      and well_formed(profile_draft)
    )
```

#### 利点

- isolated fragment helper と current program surface の bridge を先に整理できる。
- request-local / option-local clause attachment cluster を先に扱うので、`specs/examples/30` の staged line に合う。
- malformed wording を増やす前に、どの structural carrier が必要かを見極められる。

#### 欠点

- success-side fragment helper 自体の malformed diagnostics はまだ増えない。
- docs-only comparison の段階では code evidence は増えない。

### 案 3. multiline shape と fragment malformed を同時に比較する

#### 利点

- stage 3 later branch を一気に進められるように見える。

#### 欠点

- structural attachment と fragment diagnostics が同時に混ざる。
- request head / clause attachment の橋渡しと helper-local wording の責務分離が崩れやすい。
- current phase の narrow progression に合わない。

## 比較

### checker-led staging との整合

- `specs/examples/30` では clause attachment と minimal predicate fragment は別 cluster である。
- current repo では fragment helper の success-side compare が先に立ったので、次に必要なのは fragment helper を deeper diagnostics にすることより、surface attachment との bridge を narrow に決めることである。
- この観点では案 2 が最も自然である。

### hidden diagnostics pressure の回避

- 案 1 は helper-local malformed wording を先に増やすため、program surface が見えないまま diagnostics contract が先行しやすい。
- 案 2 は structure comparison を先に行うので、malformed wording をまだ薄く保てる。
- 案 3 は最も危険である。

### declaration-side / request-local の shared line

- 案 1 は helper 内部に閉じるため、declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment を同じ structural floor で扱えない。
- 案 2 は header + indented predicate block という shared structural questionを先に扱える。
- この観点でも案 2 が最も素直である。

## current judgment

current repo の next narrow step としては、
**案 2. request head + clause attachment multiline shape を先に docs-only comparison する**
のが最も自然である。

つまり、

1. predicate fragment helper の malformed-source pair は still later stage に残す
2. 先に
   - request-local `require:` / `ensure:`
   - declaration-side `admit:`
   の multiline attachment shape をどこで shared structural floor として切るかを比較する
3. その comparison を通してから、必要なら fragment malformed diagnostics を narrow に actualize する

## なぜこれが最小か

- current success-side fragment helper は already source-backed なので、次の不足は helper 内部の expression family よりも surface attachment bridge にある。
- clause attachment は first checker cut inventory でも独立 cluster なので、fragment malformed wording より先に structural floor を見る方が staged line に合う。
- declaration-side と request-local の両方に効く bridge を先に決める方が、後で malformed diagnostics を入れても責務がねじれにくい。

## current stage でまだやらないこと

- fragment malformed-source pair の actualization
- request head + clause attachment multiline shape の actual parser 実装
- fixture-side full request / option node compare
- public parser API 化

## next narrow step

次は
**request head + clause attachment multiline shape を、declaration-side `admit:` と request-local `require:` / `ensure:` の shared structural floor としてどこまで比較するか**
を docs-only で整理するのが自然である。
