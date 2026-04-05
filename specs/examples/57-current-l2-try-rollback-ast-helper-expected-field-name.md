# 57 — current L2 try/rollback AST helper expected field name

## 目的

この文書は、current L2 parser-free PoC、`TryFallback` / `AtomicCut` dedicated AST structural helper の
entry criteria、malformed source placement、helper-local compare contract を前提に、
**future dedicated AST structural helper を actualize する場合の expected field 名と focused compare shape をどこまで narrow に切るか**
を整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- fixture-side optional expected field を将来足すなら、どの naming family が最小か
- helper-local focused compare を最小で保つ row shape は何か
- current phase でまだ決めないものは何か

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- `TryFallback` / `AtomicCut` structural floor は existing reason-row family helper の fourth spike には actualize しない
- future dedicated AST structural helper は AST-only floor を読み、dynamic gate / restore scope を non-goal にする
- malformed source placement は parser / loader / static gate に分け、semantic structural malformed は static gate / dedicated helper に置く
- malformed static family は current phase ではまだ actual corpus に増やさない
- compare contract は helper-local dedicated contract から始め、shared detached carrier や public API へはまだ上げない

したがって current 問いは、
**helper-local dedicated compare contract を前提にしたとき、fixture-side optional expected field と focused compare row をどこまで narrow に決めてよいか**
である。

## 比較観点

1. existing `checked_reasons` / `checked_reason_codes` naming family と不必要に衝突しないか
2. `TryFallback` / `AtomicCut` 専用 helper だと一目で分かるか
3. current helper-local compare に必要な情報だけへ絞れているか
4. future detached artifact shared carrier や public checker API を premature に固定しないか
5. fixture authoring template に「まだ使ってはいけない future field 候補」として安全に書けるか

## field 名候補の比較

### 案 1. `expected_static.checked_try_rollback_structural_findings`

#### 利点

- `expected_static` 配下の additive optional expected field であることが明確である
- `checked_` prefix により、helper の actual output を fail-closed compare する field だと読める
- `try_rollback` により helper-local dedicated contract であることが明確である
- `findings` により row-family code ではなく focused structural finding だと分かる

#### 欠点

- field 名としてはやや長い

### 案 2. `expected_static.try_rollback_structural_findings`

#### 利点

- 案 1 より短い
- dedicated helper family であることはまだ読める

#### 欠点

- `checked_` が無いので、explanatory prose なのか machine-check compare field なのかが少し曖昧になる
- current `checked_reasons` / `checked_reason_codes` family との naming discipline も弱くなる

### 案 3. `expected_static.structural_findings`

#### 利点

- 最も短い

#### 欠点

- `TryFallback` / `AtomicCut` 専用 helper なのか、generic structural checker family なのかが読めない
- future parser / static gate / other checker family まで一括で generic 化する pressure が強く、current helper-local cut を壊しやすい

### 案 4. `expected_static.checked_try_rollback_structural_rows`

#### 利点

- row shape を直接表せる

#### 欠点

- `rows` は internal carrier 寄りで、user-facing fixture field 名としては意味が薄い
- `findings` よりも low-level で、future row shape の微修正まで field 名に引きずられやすい

## current judgment: field 名

current L2 の next narrow step として最も自然なのは、
**案 1. `expected_static.checked_try_rollback_structural_findings`**
である。

理由は次の通り。

1. `checked_` prefix により helper actual output との fail-closed compare field だと分かる
2. `try_rollback` で dedicated helper-local family だと明示できる
3. `findings` は row-family code や generic structural carrier より narrow で、focused compare contract と整合する
4. current phase では exact field 名だけを docs-only で先に決め、final schema や shared carrier をまだ固定しない cut と相性が良い

## focused compare shape の比較

### 案 A. string list

例:

```json
[
  "missing_fallback_body",
  "disallowed_atomic_cut_nesting"
]
```

#### 利点

- 最も軽い

#### 欠点

- `TryFallback` と `AtomicCut` のどちらに対する finding かを row 側で持てない
- same helper が複数 node family を扱うと explanation 依存が増える

### 案 B. helper-local row list

例:

```json
[
  { "subject_kind": "TryFallback", "finding_kind": "missing_fallback_body" },
  { "subject_kind": "AtomicCut", "finding_kind": "disallowed_nesting" }
]
```

#### 利点

- focused compare に必要な subject/finding の最小情報だけを持つ
- generic reason-row family や detached shared carrier と衝突しない
- future dedicated helper が複数 structural family を扱っても compare shape を保ちやすい

#### 欠点

- string list よりは少し重い

### 案 C. generic structural carrier

例:

```json
[
  {
    "checker_family": "structural",
    "subject_kind": "TryFallback",
    "finding_kind": "missing_fallback_body",
    "subject_path": ["program", 0, "body", 1]
  }
]
```

#### 利点

- future generic structural checker に広げやすい

#### 欠点

- current dedicated helper-local cut と比べて generic すぎる
- `subject_path`、`checker_family` のような field が early API pressure を生みやすい
- malformed family actualization も detached artifact shared carrier も未actualizeの current phase では premature である

## current judgment: focused compare shape

current L2 の next narrow step として最も自然なのは、
**案 B. helper-local row list**
である。

最小 row shape は、少なくとも次で足りる。

```json
[
  { "subject_kind": "TryFallback", "finding_kind": "missing_fallback_body" },
  { "subject_kind": "AtomicCut", "finding_kind": "disallowed_nesting" }
]
```

ここで重要なのは、

- `subject_kind`
- `finding_kind`

の 2 要素だけを current docs-only minimum とすることである。

`subject_path`、source span、checker family label、long-form note は current phase ではまだ要らない。

## field と shape の関係

current docs-only minimum は、次の組み合わせで読むのが自然である。

```json
{
  "expected_static": {
    "checked_try_rollback_structural_findings": [
      { "subject_kind": "TryFallback", "finding_kind": "missing_fallback_body" },
      { "subject_kind": "AtomicCut", "finding_kind": "disallowed_nesting" }
    ]
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
- generic structural checker family の actualization
- `subject_path` / source span / long-form note の固定

## current guidance

current fixture authoring と helper stack では、次を守る。

1. `expected_static.checked_try_rollback_structural_findings` は **future field 候補** であり、current fixture JSON にまだ書かない
2. dedicated helper を actualize するまでは、`TryFallback` / `AtomicCut` は runtime representative と docs-only judgment で扱う
3. focused compare shape は helper-local に留め、shared detached carrier や public checker API に先回りしない

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper を detached validation loop のどこへ差し込むのが最小か**
を narrow に比較することである。

## OPEN に残すもの

- dedicated helper を actualize した後、detached artifact shared carrier へ上げる閾値
- `subject_path` や source span を compare shape に入れるか
- dedicated helper を `scripts/current_l2_detached_loop.py` のどこへ差し込むか
- malformed static family を actual corpus に増やす必要が本当にあるか
