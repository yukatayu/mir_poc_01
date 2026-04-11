# 261 — current L2 checker-cluster-hint-bundle-shape-ready typed-family-coverage-state threshold

## 目的

`specs/examples/260-current-l2-typed-reason-family-hint-ready-checker-cluster-hint-bundle-shape-comparison.md`
で `typed_reason_family_hint` payload を `family_refs[]` minimal bundle に留める cut を固定した次段として、

- multi-family row や partially-typed row の誤読を抑えるために coverage state を足すべきか
- `coverage_state` と actual supported kind summary のどちらを current checker-side line に入れるのが最小か
- current checker-cluster matrix を actual checker payload と誤読させずに typed-family proximity を説明するには何が必要十分か

を比較する。

ここで固定するのは **typed-family coverage state の current threshold** であり、

- actual supported kind row
- checker payload family
- public checker API

はまだ固定しない。

## scope

- current L2 first checker cut の checker-cluster matrix を扱う。
- `typed_reason_family_hint` bundle に足す coverage state の比較に限る。
- stable typed static reason actualization の actual source は参照するが、schema actualization には進まない。

## current 前提

current repo では次が成立している。

1. `typed_reason_family_hint` は optional attachment であり、payload は `family_refs[]` minimal bundle に留めてよい。
2. `same_lineage_static_evidence_floor` のように typed family actualization が cluster とほぼ 1 対 1 の row もある。
3. `malformed_underdeclared_rejection` のように
   - `declared_target_edge_pair_family`
   - `missing_option_family`
   を含む一方、duplicate declaration cluster を still 含まない row もある。

したがって current 問いは、
**`family_refs[]` だけでは over-read になりうる row に対して、coverage state を 1 段だけ足すべきか**
である。

## 比較観点

1. partial coverage を actual kind row まで行かずに説明できるか
2. bundle を重くしすぎないか
3. stable typed family proximity と actual checker payload detail を混ぜないか
4. 次段で supported kind summary を narrow に比較できるか

## 比較対象

### 案 1. `coverage_state` は足さない

#### 利点

- bundle は最も軽い
- `family_refs[]` だけで single-family row は十分読める

#### 欠点

- multi-family row が cluster 全体を fully cover しているように誤読されやすい
- duplicate declaration cluster の non-promotion が row 単位では見えにくい

### 案 2. lightweight `coverage_state` だけを足す

```text
typed_reason_family_hint = {
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster
}
```

#### 利点

- actual kind row を still later に残しつつ、partial / full の読みを 1 段だけ見せられる
- same-lineage row と malformed / underdeclared row を同じ attachment family で扱える
- next 段で supported kind summary を追加すべきかを narrow に比較しやすい

#### 欠点

- state 値の naming discipline が要る
- partial の理由は still prose に残る

### 案 3. `supported_kind_refs[]` まで current line に入れる

```text
typed_reason_family_hint = {
  family_refs = [...],
  supported_kind_refs = [...]
}
```

#### 利点

- partial coverage の理由を最も具体的に見せられる

#### 欠点

- current checker-side line を actual checker payload に近づけすぎる
- `fixture_evidence_refs` attachment と責務が重なりやすい
- Phase 5 current line としては still early である

## current judgment

current L2 で最も自然なのは、
**案 2. lightweight `coverage_state` だけを足す**
ことである。

理由は次の通り。

1. `family_refs[]` だけだと multi-family / partial row の読みが still 粗い
2. しかし `supported_kind_refs[]` を current line に入れるのは actual payload に近づきすぎる
3. `coverage_state` なら current line を docs-first に保ったまま partial / full の違いだけを見せられる

## current reading

current docs-only では、typed-family hint は次の minimal shape まででよい。

```text
typed_reason_family_hint = {
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster
}
```

### practical example A — same-lineage floor

```text
typed_reason_family_hint = {
  family_refs = [lineage_edge_pair_family],
  coverage_state = full_cluster
}
```

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

この `partial_cluster` は、
duplicate declaration cluster が current typed family actualization に入っていないことを反映する。

## next promoted line

next promoted line は、
**typed-family-coverage-state-ready supported-kind-summary threshold**
に置く。

ここで比べるのは、
`supported_kind_refs[]` のような actual kind summary を current checker-side line に足すべきか、それとも coverage state で十分かである。

## what is decided here

### decided

- `family_refs[]` minimal bundle の次段として `coverage_state` は足してよい
- `coverage_state` は `full_cluster | partial_cluster` の lightweight distinction に留める
- `supported_kind_refs[]` は still later に残す

### not decided

- `coverage_state` の exact naming を将来も固定するか
- `supported_kind_refs[]` を current checker-side line に入れるべきか
- actual checker payload への ratchet

## open questions

- `partial_cluster` の理由を prose に残すだけで十分か
- future typed family が request / predicate / `try` cluster に広がったとき、この 2-state で足りるか
- `coverage_state` を row-level field として持つか hint subfield に留めるか
