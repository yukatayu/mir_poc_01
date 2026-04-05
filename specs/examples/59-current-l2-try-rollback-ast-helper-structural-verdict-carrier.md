# 59 — current L2 try/rollback AST helper structural verdict carrier

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の compare contract、
expected field 名、detached-loop insertion judgment を前提に、
**future dedicated AST structural helper の structural verdict carrier / name をどこまで narrow に切るか**
を整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- structural verdict を fixture-side expected field に持つべきか
- 持つなら field 名をどう切るか
- verdict 値を findings list や full static gate verdict とどう分けるか

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- dedicated AST structural helper は future option であり、current phase では actualize しない
- compare contract は helper-local dedicated contract から始める
- optional expected finding field 名は `expected_static.checked_try_rollback_structural_findings` が最小候補である
- focused compare shape は `subject_kind` / `finding_kind` の helper-local row list に留める
- future dedicated helper を detached validation loop に載せるとしても、bundle-first runtime path ではなく static gate artifact loop の helper-local smoke family に留める
- `specs/examples/56` では helper が `structural verdict` と `structural finding rows` を返す最小 contract を採っているが、verdict carrier / name はまだ OPEN である

したがって current 問いは、
**finding rows と別に structural verdict carrier を持たせるなら、その最小 naming family と value shape をどこまで narrow に決めてよいか**
である。

## 比較観点

1. helper-local dedicated contract を full static gate verdict と混同しないか
2. findings list の有無だけに hidden meaning を乗せないか
3. existing `checked_*` naming family と衝突しないか
4. future detached artifact shared carrier や public checker API を premature に固定しないか
5. fixture authoring template に future field 候補として安全に書けるか

## carrier placement の比較

### 案 1. findings list だけを持ち、verdict は empty/non-empty から推論する

#### 利点

- field 数が最小で済む

#### 欠点

- compare contract の meaning が implicit になる
- future helper が `out_of_scope` や `unsupported` のような status を持ちたくなったときに shape を壊しやすい
- row list が「evidence」なのか「status そのもの」なのかが曖昧になる

### 案 2. dedicated verdict field を別に持つ

#### 利点

- compare contract が explicit になる
- findings list と verdict の責務を分けられる
- helper-local compare contract の future拡張余地を少しだけ残せる

#### 欠点

- field が 1 つ増える

### 案 3. `expected_static.verdict` を流用する

#### 利点

- top-level expected field を増やさなくて済む

#### 欠点

- full static gate verdict と dedicated helper-local structural verdict が混ざる
- `valid` / `malformed` / `underdeclared` との意味衝突が強い
- helper-local dedicated contract から外れてしまう

## current judgment: carrier placement

current L2 の next narrow step として最も自然なのは、
**案 2. dedicated verdict field を別に持つ**
である。

理由は次の通り。

1. `specs/examples/56` の最小 contract には verdict と finding rows の 2 要素がある
2. findings list だけに hidden meaning を持たせると compare contract が暗黙化する
3. `expected_static.verdict` を流用すると full static gate judgment と dedicated helper-local judgment を混同しやすい

## field 名候補の比較

### 案 A. `expected_static.checked_try_rollback_structural_verdict`

#### 利点

- `checked_` prefix により helper actual output を fail-closed compare する field だと分かる
- `try_rollback` により dedicated helper-local family だと分かる
- `structural_verdict` により findings list とは別 carrier だと明示できる

#### 欠点

- field 名としては長い

### 案 B. `expected_static.try_rollback_structural_verdict`

#### 利点

- 案 A より短い

#### 欠点

- `checked_` が無いので compare field と explanatory note の境界が弱い

### 案 C. `expected_static.checked_structural_verdict`

#### 利点

- 判別したいのが structural verdict だとは読める

#### 欠点

- `TryFallback` / `AtomicCut` dedicated helper なのか generic structural checker なのかが曖昧になる
- future genericization pressure を強める

### 案 D. `expected_static.verdict`

#### 利点

- 追加 field が無い

#### 欠点

- carrier placement comparison で退けた通り、full static gate verdict と混同する

## current judgment: field 名

current L2 の next narrow step として最も自然なのは、
**案 A. `expected_static.checked_try_rollback_structural_verdict`**
である。

## verdict 値 shape の比較

### 案 i. bool `has_findings`

例:

```json
{
  "checked_try_rollback_structural_verdict": true
}
```

#### 利点

- 最も小さい

#### 欠点

- `true` / `false` が何を意味するかを prose で補わないといけない
- field 名そのものに meaning を埋め込む pressure が強い

### 案 ii. string enum `valid` / `malformed`

#### 利点

- human-readable ではある

#### 欠点

- full static gate verdict と衝突しやすい
- helper-local dedicated contract より大きい taxonomy に見える

### 案 iii. helper-local string enum `no_findings` / `findings_present`

#### 利点

- findings list との関係が直接分かる
- full static gate verdict や reason family と衝突しない
- helper-local dedicated contract の narrow さを保てる

#### 欠点

- final API の語彙としてはまだ rough でありうる

## current judgment: verdict 値 shape

current docs-only minimum として最も自然なのは、
**案 iii. helper-local string enum `no_findings` / `findings_present`**
である。

理由は次の通り。

1. dedicated helper が扱うのは AST structural findings の有無であり、full static gate verdict ではない
2. bool だけだと meaning が prose 依存になる
3. `valid` / `malformed` は static gate verdict と混線しやすい

## minimum future shape

current docs-only minimum は次である。

```json
{
  "expected_static": {
    "checked_try_rollback_structural_verdict": "findings_present",
    "checked_try_rollback_structural_findings": [
      { "subject_kind": "TryFallback", "finding_kind": "missing_fallback_body" }
    ]
  }
}
```

または finding が無い場合は次である。

```json
{
  "expected_static": {
    "checked_try_rollback_structural_verdict": "no_findings",
    "checked_try_rollback_structural_findings": []
  }
}
```

ただしこれは **future dedicated AST structural helper を actualize した場合の最小候補** であり、
current fixture schema の actual field ではない。

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual実装
- current fixture schema への actual field 追加
- detached artifact shared carrier 追加
- `subject_path` や source span の固定
- full static gate verdict との統合

## current guidance

current fixture authoring と helper stack では、次を守る。

1. `expected_static.checked_try_rollback_structural_verdict` は **future field 候補** であり、current fixture JSON にまだ書かない
2. dedicated helper verdict は `expected_static.verdict` と別に扱う
3. verdict 値も helper-local string enum に留め、full static gate taxonomy へ先回りしない

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper を detached artifact shared carrier へ上げる閾値をどこに置くか**
を narrow に比較することである。

## OPEN に残すもの

- detached artifact shared carrier へ上げる閾値
- actual subcommand 名をいつ決めるか
- malformed static family を actual corpus に増やす必要が本当にあるか
- future generic structural checker family とどこで合流させるか
