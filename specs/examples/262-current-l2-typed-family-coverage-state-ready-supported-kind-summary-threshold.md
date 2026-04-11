# 262 — current L2 typed-family-coverage-state-ready supported-kind-summary threshold

## 目的

`specs/examples/261-current-l2-checker-cluster-hint-bundle-shape-ready-typed-family-coverage-state-threshold.md`
で `typed_reason_family_hint` bundle を

- `family_refs[]`
- `coverage_state = full_cluster | partial_cluster`

までに留める cut を固定した次段として、

- `supported_kind_refs[]` のような actual kind summary を current checker-side line に足すべきか
- `coverage_state` の次段として kind summary を docs-only matrix に持ち込むのが自然か
- それとも cluster matrix での説明はここで止め、actual checker payload family 側へ pressure を送る方が自然か

を比較する。

ここで固定するのは **supported-kind summary の current threshold** であり、

- actual checker payload family
- public checker API
- final type system / final checker schema

はまだ固定しない。

## scope

- current L2 first checker cut の checker-cluster matrix を扱う。
- `typed_reason_family_hint` bundle に `supported kind` summary を足すかどうかだけを比較する。
- stable typed static reason actualization の source evidence は参照するが、actual schema actualization には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster row core は
   `cluster_kind + checker_subject + decidable_class + external_handoff`
   に留める cut が固定済みである。
2. `fixture_evidence_refs` は attachment として足す cut が固定済みである。
3. `typed_reason_family_hint` も row core ではなく optional attachment に留める cut が固定済みである。
4. hint payload は `family_refs[]` minimal bundle に留める cut が固定済みである。
5. multi-family row の over-read を抑えるため、lightweight `coverage_state` を足す cut が固定済みである。

したがって current 問いは、
**`coverage_state` の次段として actual kind summary を current checker-cluster matrix に持ち込むべきか**
である。

## 比較観点

1. current checker-cluster matrix を actual checker payload に近づけすぎないか
2. partial coverage の誤読を十分に抑えられるか
3. stable typed family actualization の proximity と actual kind inventory を混ぜすぎないか
4. next 段で actual checker payload family 側へ自然に接続できるか

## 比較対象

### 案 1. `supported kind` summary はまだ足さない

#### 読み

current checker-side line は

```text
typed_reason_family_hint = {
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster
}
```

までで止める。

#### 利点

- current checker-cluster matrix を docs-first inventory として保てる
- actual checker payload / actual kind carrier との混線を抑えられる
- `supported kind` detail は actual checker payload family 側へ移しやすい

#### 欠点

- partial の具体的中身は prose 参照に残る
- row 単体で kind-level explanation を完結できない

### 案 2. `supported_kind_count` のような弱い summary だけを足す

```text
typed_reason_family_hint = {
  family_refs = [...],
  coverage_state = partial_cluster,
  supported_kind_count = 2
}
```

#### 利点

- partial coverage の粗い量感は 1 段だけ出せる
- `supported_kind_refs[]` よりは軽い

#### 欠点

- count だけでは何を support しているか分からない
- docs-only cluster matrix に人工的な中間 state を増やすだけになりやすい
- actual checker payload family への bridge としては still 弱い

### 案 3. `supported_kind_refs[]` を current checker-side line に足す

```text
typed_reason_family_hint = {
  family_refs = [...],
  coverage_state = partial_cluster,
  supported_kind_refs = [...]
}
```

#### 利点

- partial coverage の具体的な kind inventory を row 上で見せられる
- current stable typed actualization と kind set の接点を最も直接に示せる

#### 欠点

- current checker-cluster matrix を actual checker payload に近づけすぎる
- `fixture_evidence_refs` attachment と役割が重なりやすい
- stable cluster inventory の current corpus state を docs-only row attachment へ premature に焼き込みやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `supported kind` summary はまだ足さない**
ことである。

理由は次の通り。

1. `coverage_state` までで current checker-cluster matrix の docs-first 目的は十分達成できる
2. `supported_kind_count` は説明力が弱い一方で line を複雑にする
3. `supported_kind_refs[]` は actual checker payload family と kind inventory を premature に current matrix へ持ち込みすぎる
4. current next pressure は kind summary を cluster matrix に足すことより、actual checker payload family 側へ narrow に接続することにある

## current reading

current checker-side line は、

```text
typed_reason_family_hint = {
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster
}
```

までで止める。

### practical example A — same-lineage floor

```text
typed_reason_family_hint = {
  family_refs = [lineage_edge_pair_family],
  coverage_state = full_cluster
}
```

ここでは `supported_kind_refs[]` を足しても新しい情報はほぼ増えない。

### practical example B — malformed / underdeclared row

```text
typed_reason_family_hint = {
  family_refs = [
    declared_target_edge_pair_family,
    missing_option_family
  ],
  coverage_state = partial_cluster
}
```

ここで partial の具体的中身を cluster matrix へ直接書くより、
actual checker payload family 側で `checked_reason_codes` / detached static gate `reason_codes` の接点として扱う方が自然である。

## next promoted line

next promoted line は、
**supported-kind-summary-ready actual-checker-payload-family comparison**
に置く。

ここで比べるのは、

1. checker-cluster matrix line を current package close として止める
2. actual checker payload family を docs-first comparison として別 line に切る
3. public checker API / public checker artifact へ直接進む

のどれが自然かである。

## what is decided here

### decided

- `supported kind` summary は current checker-cluster matrix にはまだ足さない
- `coverage_state` までで current checker-side line を止める
- next pressure は actual checker payload family 側へ送る

### not decided

- actual checker payload family の minimal shape
- `supported_kind_refs[]` を later checker payload side にどう置くか
- final public checker API / final checker schema

## open questions

- `supported_kind_refs[]` を actual checker payload family 側の minimal bundle に足すべきか
- `coverage_state` の 2-state が future typed family expansion でも十分か
- checker-cluster line と actual checker payload family line の橋渡しをどの field で切るべきか
