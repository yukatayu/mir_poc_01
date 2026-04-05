# 44 — current L2 checked reasons coexistence and shrink policy

## 目的

この文書は、current stable static reason cluster 8 kind を覆う 9 fixture が
`expected_static.checked_reason_codes` に actualize 済みであることを前提に、
**`checked_reasons` と `checked_reason_codes` の coexistence を current L2 でどう維持し、
どの条件が揃うまで shrink / deprecation を保留するか**
を narrow に整理する。

ここで固定するのは final migration 完了ではない。
固定するのは、

- current cut では additive coexistence を維持すること
- coexistence の健全性を corpus 横断で scan してよいこと
- shrink はまだ実行しないこと

だけである。

## current 前提

current L2 では次が成立している。

- `expected_static.checked_reasons` は actual static gate wording を fixture-side machine-check bridge として持つ
- `expected_static.checked_reason_codes` は stable cluster 8 kind に限って additive optional typed carrier として持つ
- detached static gate artifact の `detached_noncore.reason_codes` は helper-local / reference-only mirror に留まる
- static-only corpus では stable cluster 8 kind を覆う 9 fixture が
  - `checked_reasons` present
  - `checked_reason_codes` present
  - detached `reason_codes` suggestion present
  で揃っている
- duplicate declaration cluster (`e14` / `e15`) は
  - `checked_reasons` absent
  - `checked_reason_codes` absent
  - detached `reason_codes` suggestion absent
  のままに保たれている

したがって current 問いは、
**stable cluster 8 kind を覆う 9 fixture が typed carrier を持てるようになった今、`checked_reasons` を縮退させてよいか**
ではなく、
**どの evidence が揃うまでは additive coexistence を維持するのが自然か**
である。

## 比較対象

### 案 1. current additive coexistence を維持する

- `checked_reasons` を残す
- `checked_reason_codes` を残す
- stable cluster では両者が揃っているかを readiness scan で見る

#### 利点

- wording anchor と typed carrier をまだ切り離さないで済む
- current `run_bundle()` machine-check bridge を崩さない
- duplicate cluster や explanatory prose を typed carrier 側へ無理に押し込まない
- shrink 判断を source-backed に後回しできる

#### 欠点

- fixture authoring では 2 carrier を意識する必要がある
- long term では carrier coexistence が長引く

### 案 2. stable cluster 8 kind を覆う 9 fixture だけ `checked_reasons` を縮退させる

- `checked_reason_codes` がある stable fixture から `checked_reasons` を外す

#### 利点

- typed carrier への migration は速い

#### 欠点

- actual static gate wording compare の bridge を先に失う
- duplicate cluster / explanatory prose と stable cluster の扱いが corpus 内で分裂する
- wording drift を typed row だけで観測する cut を早く固定しすぎる

### 案 3. `checked_reason_codes` を readiness だけに戻す

- typed carrier actualization を後退させる

#### 利点

- carrier 数は 1 つに戻る

#### 欠点

- stable cluster 8 kind / 9 fixture actualization を巻き戻すことになる
- current source-backed progressionを失う

## current judgment

current L2 で最も自然なのは
**案 1. current additive coexistence を維持する**
である。

理由は次の通り。

1. `checked_reasons` は現在も actual wording compare の machine-check bridge として必要である
2. `checked_reason_codes` は stable cluster 8 kind の typed carrier として有益だが、まだ wording bridge を置き換える段階ではない
3. duplicate cluster と explanatory prose を typed carrier 側へ押し込まない current boundary を保ちやすい
4. corpus 横断で coexistence の alignment を scan してから shrink を比較した方が手戻りが少ない

## current helper cut

current task で入れてよい helper cut は次に限る。

- `scan-reason-code-readiness` は
  - `fixtures with stable coexistence anchors`
  - `fixtures with checked_reason_codes but missing checked_reasons`
  - `fixtures with checked_reason_codes mismatching actual suggestion`
  - `fixtures needing coexistence follow-up`
  を display-only summary として出してよい

この helper は次をしてはいけない。

- fixture JSON の自動更新
- `checked_reasons` の自動削除提案
- `checked_reason_codes` を first-class source として `checked_reasons` の意味を置き換えること

## shrink をまだ行わない理由

current L2 では、少なくとも次が揃うまでは shrink を行わない方が自然である。

1. stable cluster 8 kind を覆う 9 fixture の coexistence が corpus 横断で zero follow-up になっていること
2. `checked_reasons` が担っている actual wording compare の代替 boundary を別に固定できていること
3. duplicate cluster / explanation-only cluster をどこへ残すかが整理できていること
4. first checker cut / typed static reason source-of-truth の entry criteria が見えていること

current actual corpus では 1 は満たすが、2〜4 はまだ満たさない。
したがって shrink は still-open であり、current judgment は additive coexistence 維持である。

## current actual corpus reading

current scan では次が source-backed に確認されている。

- static-only fixtures scanned: 11
- fixtures with checked_reasons: 9
- fixtures with reason_codes suggestions: 9
- fixtures with checked_reason_codes: 9
- fixtures with stable coexistence anchors: 9
- fixtures with checked_reason_codes but missing checked_reasons: 0
- fixtures with checked_reason_codes mismatching actual suggestion: 0
- fixtures needing coexistence follow-up: none

このため、
**stable cluster 8 kind を覆う 9 fixture では coexistence が収束している**
とは言ってよい。
ただしそれは immediate shrink を意味しない。

## 未決事項

- `checked_reasons` を最終的に deprecated にするか
- deprecated にするとして、どの helper / harness / detached artifact boundary で置き換えるか
- duplicate declaration cluster を長期的に typed 化しないまま explanation-only に固定するか
- first checker cut と typed source-of-truth の関係をどう設計するか
