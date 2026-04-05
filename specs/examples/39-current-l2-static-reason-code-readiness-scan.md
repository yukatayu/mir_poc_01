# 39 — current L2 static reason code readiness scan

## 目的

この文書は、current L2 parser-free PoC の static-only fixture corpus に対して、
detached static gate artifact の helper-local / reference-only `reason_codes` suggestion を
**batch で横断確認する display-only readiness scan**
の最小 cut を整理する。

ここで固定するのは final typed carrier でも final checker contract でもない。
固定するのは、

- current corpus のうちどの fixture / cluster で suggestion row が実際に出るか
- duplicate cluster のように no-suggestion へ留める fixture をどう見分けるか
- `checked_reasons` adoption と future typed carrier 候補 row を同じ scan で観察してよいか

という authoring / review 補助の narrow boundary だけである。

## current 問題

`suggest-reason-codes` は 1 fixture ごとの display-only assist として十分有効である。
しかし fixture authoring を tranche 単位で進めるときには、

- current static-only corpus で stable cluster がどこまで actualize 済みか
- `checked_reasons` adoption がある fixture とない fixture がどう分かれているか
- `reason_codes` suggestion が出る fixture と duplicate cluster のように出ない fixture がどこにあるか

を **1 fixture ずつ開かずに** 見たい。

一方で current L2 では、

- detached static gate artifact の `checker_core` が exact-compare core である
- `detached_noncore.reason_codes` は helper-local / reference-only mirror に留まる
- `expected_static.reason_codes` のような fixture-side typed field は未導入である

という境界を崩してはいけない。

## 比較対象

### 案 1. 1 fixture ごとの assist だけで運用する

必要な fixture ごとに `suggest-reason-codes` を呼ぶ。

#### 利点

- helper 境界が最も単純である
- detached artifact を必要なときだけ見るので conservative である

#### 欠点

- tranche 単位の fixture authoring では反復が重い
- stable cluster inventory と duplicate cluster の current split を corpus 横断で見づらい

### 案 2. static-only corpus を横断する readiness scan を入れる

static-only fixture だけに対して detached static gate artifact を emit し、
`checked_reasons` の有無、`reason_codes` suggestion の有無、kind count をまとめて表示する。

#### 利点

- future typed carrier actualization を急がずに、authoring 実地反復の現況を見える化できる
- stable cluster と duplicate cluster の split を corpus 横断で確認できる
- `checked_reasons` adoption と `reason_codes` suggestion を同時に観察できる

#### 欠点

- runtime fixture を scan から除外する current cut を明示する必要がある
- batch summary 風の表示に見えても machine-check core ではないことを docs で保つ必要がある

### 案 3. scan 結果を fixture や detached artifact へ書き戻す

scan が suggestion row や readiness flag を fixture / artifact 側へ保存する。

#### 利点

- 後追い比較はしやすい

#### 欠点

- fixture schema と detached artifact schema を premature に拡張しやすい
- display-only / reference-only の current cut を壊しやすい

## current judgment

current L2 の next narrow step として自然なのは **案 2** である。

理由は次の通り。

1. `suggest-reason-codes` を 1 fixture assist のまま保ちつつ、corpus 横断の authoring aid を足せる
2. `reason_codes` を first-class typed carrier に昇格させず、stable cluster inventory を実地で見える化できる
3. duplicate declaration cluster を no-suggestion のまま current cut に残せる

## current code anchor

current cut では次を置いてよい。

```text
scripts/current_l2_reason_code_readiness.py
scripts/current_l2_detached_loop.py
scripts/tests/test_current_l2_reason_code_readiness.py
scripts/tests/test_current_l2_static_gate_loop.py
```

wrapper の call chain は次である。

```text
scan-reason-code-readiness
  -> emit-static-gate (static-only fixtures only)
  -> current_l2_reason_code_readiness.py
```

## minimal behavior

readiness scan は最低でも次を行ってよい。

1. fixture directory と artifact directory を表示する
2. static-only fixtures scanned 数を表示する
3. runtime fixtures skipped 数を表示する
4. `checked_reasons` present fixture 数を表示する
5. `reason_codes` suggestion がある fixture 数を表示する
6. `kind` ごとの count を表示する
7. suggestion あり / なし の fixture list を表示する

### current placement

- scan 対象は current cut では static-only fixture に限る
- runtime fixture は skipped count にだけ入れる
- `reason_codes` suggestion の source は detached static gate artifact の `detached_noncore.reason_codes`
- `checked_reasons` の source は fixture 側 `expected_static.checked_reasons`

## fail-closed boundary

scan helper は fixture JSON を読んだうえで、
unsupported `expected_static.reason_codes` field を見つけたら fail-closed に止まってよい。

これは 1 fixture assist と同じ理由で、
fixture-side typed field の creep を corpus scan が隠さないためである。

## actual corpus での current reading

current corpus に対する smoke では、

- static-only fixtures scanned: `10`
- runtime fixtures skipped: `9`
- fixtures with checked_reasons: `8`
- fixtures with reason_codes suggestions: `8`
- fixtures with checked_reason_codes: `8`

であり、stable cluster と duplicate cluster の split は次のように読める。

### suggestion あり

- `e4`
- `e5`
- `e12`
- `e13`
- `e16`
- `e17`
- `e18`
- `e19`

### suggestion なし

- `e14`
- `e15`

この split は current docs で stable cluster として扱う inventory と整合する。
また current code / fixture corpus では、stable cluster 8 件が `checked_reason_codes` まで
actualize 済みであり、duplicate cluster 2 件だけが absent に留まる。

## non-goal

次は current scan の目的にしない。

- fixture JSON の自動更新
- detached artifact への readiness flag 永続化
- typed carrier の actualization
- duplicate cluster を stable code inventory に押し上げること
- runtime fixture の static gate readiness を batch core に混ぜること

## OPEN に残すもの

- future first-class typed carrier をどの cluster から actualize するか
- scan 結果を detached aggregate 側へどこまで寄せるか
- runtime fixture の static gate readiness を別 helper で扱うか
- final typed row schema の exact serialization

## current judgment

- readiness scan を入れるなら static-only corpus 横断の display-only helper が最小である
- `reason_codes` suggestion は引き続き helper-local / reference-only である
- `checked_reasons` adoption と `reason_codes` suggestion を同じ scan に並べてよい
- duplicate cluster は no-suggestion の current cut に留める
