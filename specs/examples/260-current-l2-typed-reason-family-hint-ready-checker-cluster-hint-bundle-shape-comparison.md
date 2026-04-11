# 260 — current L2 typed-reason-family-hint-ready checker-cluster-hint-bundle-shape comparison

## 目的

`specs/examples/259-current-l2-checker-cluster-fixture-evidence-attachment-ready-typed-reason-family-hint-threshold.md`
で `typed_reason_family_hint` を optional row attachment として許す cut を固定した次段として、

- `typed_reason_family_hint` payload を single ref / multi-ref / richer bundle のどこまでに留めるのが自然か
- stable typed static reason actualization が cluster と 1 対 1 で対応しない場合をどう扱うか
- current checker-cluster line を actual checker payload と誤読させずに proximity だけを見せるにはどの shape が最小か

を比較する。

ここで固定するのは **checker-cluster hint bundle shape の current comparison** であり、

- actual checker payload family
- actual supported kind set row
- public checker API

はまだ固定しない。

## scope

- current L2 first checker cut の checker-cluster matrix を扱う。
- `typed_reason_family_hint` payload shape の比較に限る。
- current stable typed static reason actualization は source evidence として参照する。
- actual helper / fixture / checker schema の変更は行わない。

## current 前提

current repo では次が成立している。

1. `typed_reason_family_hint` は row core ではなく optional row attachment として許してよい。
2. stable typed static reason actualization は少なくとも
   - lineage edge pair family
   - declared target edge pair family
   - missing option family
   - capability strengthening singleton
   まで source-backed に揃っている。
3. checker-cluster row は 6 cluster であり、cluster と typed static reason family の対応は 1 対 1 ではない。
   - `same_lineage_static_evidence_floor` はほぼ 1 family
   - `malformed_underdeclared_rejection` は複数 family にまたがる
   - request / predicate / `try` structural floor は current では family hint absent が自然

したがって current 問いは、
**typed_reason_family_hint を present にする row で、single symbolic ref で足りるのか、それとも複数 family を束ねる minimal bundle が必要か**
である。

## 比較観点

1. cluster-to-family の non-1:1 対応を誤魔化さずに表現できるか
2. `checked_reason_codes` actualization との proximity だけを示し、actual kind row を premature に出さないか
3. row attachment として十分軽いか
4. 次段で coverage state / supported kind summary を narrow に比較できるか

## 比較対象

### 案 1. single symbolic family ref に留める

```text
typed_reason_family_hint = lineage_edge_pair_family
```

#### 利点

- 最も軽い
- same-lineage floor のような 1 family row には自然である

#### 欠点

- `malformed_underdeclared_rejection` のように複数 stable family をまたぐ row を表しにくい
- future family actualization が増えたときに single ref 前提が崩れやすい

### 案 2. `family_refs[]` を持つ minimal bundle にする

```text
typed_reason_family_hint = {
  family_refs = [...]
}
```

#### 利点

- 1 family row と multi-family row の両方を同じ shape で表せる
- actual kind row / carrier detail をまだ出さずに済む
- row attachment として still 軽い
- 次段で coverage state や supported kind summary だけを追加比較しやすい

#### 欠点

- single ref より 1 段だけ bundle 化が進む
- family name の docs-only symbolic discipline が要る

### 案 3. `family_refs + carrier_ref + supported_kind_refs` まで current line に入れる

```text
typed_reason_family_hint = {
  family_refs = [...],
  carrier_ref = expected_static_checked_reason_codes,
  supported_kind_refs = [...]
}
```

#### 利点

- `checked_reason_codes` line との接点が最も具体的に見える
- cluster ごとの typed readiness を詳しく説明できる

#### 欠点

- docs-only checker-cluster matrix を actual checker payload と誤読させやすい
- `fixture_evidence_refs` attachment と責務が重なりやすい
- current phase では details を出しすぎる

## current judgment

current L2 で最も自然なのは、
**案 2. `family_refs[]` を持つ minimal bundle**
である。

理由は次の通り。

1. same-lineage row のような 1 family case も、malformed / underdeclared row のような multi-family case も同じ shape で扱える
2. actual kind row や `carrier_ref` を still later に残せる
3. current checker-cluster line に欲しいのは typed static reason actualization との **近さ** であって、actual payload detail ではない

## current reading

current docs-only では、`typed_reason_family_hint` は次の minimal bundle でよい。

```text
typed_reason_family_hint = {
  family_refs = [...]
}
```

### practical example A — same-lineage floor

```text
typed_reason_family_hint = {
  family_refs = [lineage_edge_pair_family]
}
```

### practical example B — malformed / underdeclared row

```text
typed_reason_family_hint = {
  family_refs = [
    declared_target_edge_pair_family,
    missing_option_family
  ]
}
```

この bundle は、
duplicate declaration cluster を typed family に昇格させたことを意味しない。
current repo では duplicate declaration cluster は still out-of-scope である。

## next promoted line

next promoted line は、
**checker-cluster-hint-bundle-shape-ready typed-family-coverage-state threshold**
に置く。

ここで比べるのは、
multi-family row や partially-typed row に対して
coverage state を current checker-cluster line に足すべきかどうかである。

## what is decided here

### decided

- `typed_reason_family_hint` payload は single ref ではなく minimal bundle にしてよい
- current minimal bundle は `family_refs[]` まででよい
- `carrier_ref` や actual kind row は still later に残す

### not decided

- coverage state を current line に足すか
- `family_refs` の exact symbolic namespace
- actual `checked_reason_codes` payload との bridge shape

## open questions

- `malformed_underdeclared_rejection` のような row に coverage state を足さないと、family hint が過剰に広く見えないか
- capability singleton を `family_refs` の 1 要素として扱う naming で十分か
- future parser-side / request-side typed family が出たときも同じ bundle shape を保てるか
