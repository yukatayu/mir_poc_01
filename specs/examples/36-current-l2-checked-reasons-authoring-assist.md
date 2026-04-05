# 36 — current L2 checked reasons authoring assist

## 目的

この文書は、current L2 parser-free PoC で
`expected_static.checked_reasons` を narrow に authoring するときの
**display-only assist** を整理する。

ここで固定するのは final checker API でも final authoring DSL でもない。
固定するのは、

- detached static gate artifact の actual `checker_core.reasons` を source に保ち
- fixture JSON を helper が黙って書き換えず
- copyable suggestion だけを stdout に出す

という current L2 の最小 operational cut である。

## current 問題

`checked_reasons` は current L2 で additive optional bridge であり、
fixture-side explanation を壊さずに actual static gate wording を
bundle machine-check へ narrow transfer したいときだけ使う。

しかし authoring 実務では、特に

- static-only
- malformed / underdeclared
- actual wording が 1 本に安定している

fixture であっても、
author が detached static gate artifact を開いて
`checked_reasons` へ転記する作業がまだ手動である。

一方で helper が fixture JSON を直接補完し始めると、
current L2 の hidden acceptance / hidden elaboration 禁止と衝突しやすい。

## 比較対象

### 案 1. assist を入れない

author が detached static gate artifact を見て完全に手で転記する。

#### 利点

- helper が semantics に触れない
- current narrow boundary を最も保守的に守れる

#### 欠点

- actual wording と fixture-side carrier の転記コストが残る
- authoring 反復の速度が上がらない

### 案 2. display-only suggestion

helper は actual static gate artifact を読み、
`checked_reasons` の copyable snippet だけを stdout に表示する。
fixture JSON は書き換えない。

#### 利点

- hidden acceptance を起こしにくい
- actual wording を narrow に確認しやすい
- scaffold helper の「骨組みだけを作る」という責務を壊さない
- detached static gate artifact loop と fixture-side bridge を素直につなげる

#### 欠点

- 転記自体はまだ人手である
- wording compare の narrow carrier であることを author が理解している必要がある

### 案 3. auto-fill / file rewrite

helper が fixture JSON に `checked_reasons` を直接書き込む。

#### 利点

- authoring は最も速い

#### 欠点

- hidden elaboration に近い
- current L2 で `checked_reasons` を置くべきでない fixture まで広げやすい
- scaffold helper や detached loop wrapper が semantics completion helper と誤読されやすい

## current judgment

current L2 の next narrow step として自然なのは **案 2. display-only suggestion** である。

理由は次の通り。

1. `checked_reasons` の actual source は detached static gate artifact 側にある
2. fixture JSON の direct rewrite は current L2 の hidden acceptance 禁止と緊張が大きい
3. scaffold helper の narrow role を保ったまま authoring 実務だけを少し軽くできる
4. typed reason code への migration を一段飛ばして先取りしない

## current code anchor

current cut では次を置いてよい。

```text
scripts/current_l2_checked_reasons_assist.py
scripts/current_l2_detached_loop.py
scripts/tests/test_current_l2_checked_reasons_assist.py
scripts/tests/test_current_l2_static_gate_loop.py
```

wrapper の call chain は次である。

```text
suggest-checked-reasons
  -> emit-static-gate
    -> current_l2_emit_static_gate
  -> current_l2_checked_reasons_assist.py
```

## minimal behavior

display-only assist は最低でも次を行ってよい。

1. fixture path を読む
2. static gate artifact path を読む
3. current `expected_static.checked_reasons` の有無を表示する
4. actual static gate `checker_core.reasons` を読む
5. actual `reasons` が非空で、current `checked_reasons` と違うときだけ
   copyable snippet を表示する

### snippet の current shape

current helper は、`expected_static` 配下に貼り付ける最小 row として
次の shape を表示してよい。

```json
{
  "checked_reasons": [
    "missing lineage assertion for primary -> mirror"
  ]
}
```

これは display-only であり、helper が fixture JSON を patch することを意味しない。

## no-suggestion policy

次の場合、helper は suggestion を出さなくてよい。

- actual static gate `reasons` が空である
- current `checked_reasons` が既に actual `reasons` と一致している

特に current valid fixture 群では actual static gate `reasons` が空な例が多いので、
`checked_reasons = []` を広く推奨しない current judgment と整合する。

## scaffold helper との関係

`scripts/current_l2_scaffold_fixture.py` は引き続き
**boilerplate だけ** を扱う helper である。

したがって current judgment は次である。

- scaffold helper は `checked_reasons` を自動補完しない
- display-only assist は scaffold 完了後、fixture が static gate を通せる段階で使う
- scaffold helper 自体に semantics inference mode を足さない

## detached static gate artifact との関係

display-only assist の source は
detached static gate artifact の `checker_core.reasons` である。

これは次を意味する。

- actual source は依然として detached artifact 側にある
- `checked_reasons` は fixture-side bridge に留まる
- helper-local / reference-only `detached_noncore.reason_codes` は assist source にしない

したがって display-only assist は、
typed reason code への migration を一段飛ばして進める mechanism ではない。

## current adoption に向く fixture

現時点で display-only assist が自然に効くのは、少なくとも次のような fixture である。

- `e4-malformed-lineage`
- `e5-underdeclared-lineage`

この 2 本では、

- actual static gate wording が 1 本に安定している
- explanatory `reasons` を別に残したい
- `checked_reasons` の narrow adoption が already-safe である

という条件が揃っている。

## current non-goal

次は current assist の目的にしない。

- fixture JSON の自動 rewrite
- `checked_reasons = []` の広域 auto-fill
- runtime fixture での blanket recommendation
- detached non-core `reason_codes` からの typed auto-suggestion
- theorem prover / checker contract の final typed source

## OPEN に残すもの

- display-only assist を detached loop wrapper の subcommand に留めるか、独立 helper に保つか
- suggestion snippet を object row に留めるか、future typed row 候補を併記するか
- runtime fixture でも actual static gate wording が非空になる case をどこまで支援するか
- `checked_reasons` から typed reason code への migration timing
- final authoring DSL / final parser syntax とどう接続するか

## current judgment

- `checked_reasons` authoring assist を入れるなら display-only suggestion が最小である
- fixture JSON の direct rewrite は current L2 では採らない
- scaffold helper は `checked_reasons` を自動補完しない
- detached static gate artifact の actual `checker_core.reasons` を suggestion source に使ってよい
- helper-local / reference-only `reason_codes` は assist source に使わない
