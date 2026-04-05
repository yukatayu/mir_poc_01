# 38 — current L2 static reason codes authoring assist

## 目的

この文書は、current L2 parser-free PoC で detached static gate artifact の
helper-local / reference-only `reason_codes` を
**display-only assist** として見る最小 cut を整理する。

ここで固定するのは final typed checker API でも final fixture schema でもない。
固定するのは、

- suggestion source を detached static gate artifact の `detached_noncore.reason_codes` に限り
- fixture JSON に typed field を足したことにせず
- future typed carrier 候補 row を stdout へ表示するだけに留める

という current L2 の narrow authoring aid である。

## current 問題

`checked_reasons` assist は current fixture-side machine-check bridge 候補を
display-only で示せる。
しかし `checked_reasons` の次段として typed reason code に進めてよい stable cluster は別にあり、
author が

- detached static gate artifact にどういう `reason_codes` row が出るか
- duplicate cluster が current helper cut でどう落ちるか
- future typed carrier 候補としてどこまで stable cluster が揃っているか

を確認したい場面がある。

一方で current fixture schema には fixture-side typed reason code carrier がまだ無い。
したがって helper が `reason_codes` を fixture JSON へ auto-fill したり、
fixture-side field があるかのように振る舞うのは current judgment と衝突する。

## 比較対象

### 案 1. assist を入れない

author が detached static gate artifact を開き、
`detached_noncore.reason_codes` を手で読む。

#### 利点

- helper が schema を先取りしない
- `reason_codes` mirror の reference-only 性を最も保守的に守れる

#### 欠点

- stable cluster inventory を実地確認するときの反復が重い
- future typed carrier の row 候補を review しづらい

### 案 2. display-only reason code suggestion

helper は detached static gate artifact を読み、
future typed carrier 候補 row を stdout に表示する。
fixture JSON は書き換えず、current fixture schema に field がないことも明示する。

#### 利点

- `reason_codes` mirror を first-class carrier に昇格させずに authoring aid を足せる
- stable cluster と duplicate cluster の cut を実地確認しやすい
- hidden elaboration を起こしにくい

#### 欠点

- fixture へ貼り付ける current field は存在しない
- typed carrier の final schema が決まったわけではない

### 案 3. typed carrier 候補として fixture JSON を直接更新する

helper が future typed field を仮置きで fixture JSON へ書く。

#### 利点

- authoring は最も速い

#### 欠点

- current fixture schema を黙って拡張したことに近い
- helper-local / reference-only `reason_codes` mirror を premature に first-class 化しやすい
- hidden acceptance / hidden elaboration と緊張が大きい

## current judgment

current L2 の next narrow step として自然なのは **案 2. display-only reason code suggestion** である。

理由は次の通り。

1. `detached_noncore.reason_codes` は helper-local / reference-only mirror に留まる
2. future typed carrier 候補 row を実地で review できる
3. current fixture schema に typed field を足したことにしない
4. duplicate cluster を stable cluster と混ぜずに current cut のまま観察できる

## current code anchor

current cut では次を置いてよい。

```text
scripts/current_l2_reason_codes_assist.py
scripts/current_l2_detached_loop.py
scripts/tests/test_current_l2_reason_codes_assist.py
scripts/tests/test_current_l2_static_gate_loop.py
```

wrapper の call chain は次である。

```text
suggest-reason-codes
  -> emit-static-gate
    -> current_l2_emit_static_gate
  -> current_l2_reason_codes_assist.py
```

## minimal behavior

display-only assist は最低でも次を行ってよい。

1. fixture path を表示する
2. static gate artifact path を表示する
3. current fixture-side typed reason code carrier は absent だと明示する
4. `detached_noncore.reason_codes_scope` を表示してよい
5. `detached_noncore.reason_codes` が非空なときだけ、future typed carrier 候補 row を表示する

### fail-closed boundary

helper は fixture JSON を読んだうえで、
`expected_static.reason_codes` のような unsupported fixture-side typed field を見つけたら
fail-closed で止まってよい。

これは current fixture schema に typed carrier をまだ導入していない以上、
「absent」と誤表示して境界 violation を見逃さないためである。

### current snippet shape

current helper は、fixture JSON へ貼り付ける current field ではなく、
**future typed carrier 候補 row**
として次のような array を表示してよい。

```json
[
  {
    "kind": "declared_target_missing",
    "predecessor": "writer",
    "successor": "readonly"
  }
]
```

これは reference-only suggestion である。
current fixture schema に `reason_codes` field が存在することを意味しない。

## no-suggestion policy

次の場合、helper は suggestion を出さなくてよい。

- `detached_noncore` が absent である
- `detached_noncore.reason_codes` が空である

この cut により、

- stable cluster が mirror される case
- duplicate cluster のように mirror されない case

の両方を current helper boundary のまま観察できる。

## `checked_reasons` assist との関係

2 つの assist は source と役割が違う。

- `suggest-checked-reasons`
  - source は `checker_core.reasons`
  - current fixture-side machine-check bridge の候補を出す
- `suggest-reason-codes`
  - source は `detached_noncore.reason_codes`
  - future typed carrier 候補 row を reference-only で出す

したがって `suggest-reason-codes` は
`checked_reasons` assist の代替でも、typed carrier actualization そのものでもない。

## current non-goal

次は current assist の目的にしない。

- fixture JSON の自動更新
- unsupported fixture-side typed field を見逃すこと
- current fixture schema への typed field の仮追加
- `detached_noncore.reason_codes` を machine-check core へ昇格させること
- duplicate declaration cluster を current stable code inventory に押し込むこと
- theorem prover / checker contract の final typed source を決めること

## OPEN に残すもの

- future typed carrier を fixture-side / detached-side のどちらへ first-class に置くか
- final typed row schema の exact serialization
- `checked_reasons` と typed carrier の migration timing
- duplicate declaration cluster をいつ code inventory に上げるか

## current judgment

- `reason_codes` authoring assist を入れるなら display-only が最小である
- assist source は detached static gate artifact の `detached_noncore.reason_codes` に限る
- current fixture schema に typed field がないことを helper が明示してよい
- helper は fixture JSON を書き換えない
- `reason_codes` は引き続き helper-local / reference-only mirror であり、first-class typed source ではない
