# 263 — current L2 supported-kind-summary-ready actual-checker-payload-family comparison

## 目的

`specs/examples/262-current-l2-typed-family-coverage-state-ready-supported-kind-summary-threshold.md`
で `supported_kind_refs[]` のような kind summary を current checker-cluster matrix へは足さない cut を固定した次段として、

- checker-side line をここで package close として止めるべきか
- actual checker payload family を docs-first に 1 段切るべきか
- public checker artifact / public checker API へ直接進むべきか

を比較する。

ここで固定するのは **actual checker payload family comparison の current reading** であり、

- actual checker schema
- final public checker API
- final type system

はまだ固定しない。

## scope

- current L2 first checker cut の checker-side line だけを扱う。
- `checked_reason_codes` と detached static gate `reason_codes` の actual source は evidence として参照する。
- public API / actual checker implementation には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix line は row core、evidence attachment、typed family hint、`coverage_state` まで docs-first に切れている。
2. `supported kind` summary は current matrix には足さない方が自然である。
3. current actual machine-check source としては、
   - fixture-side `expected_static.checked_reason_codes`
   - detached static gate `reason_codes`
   が already ある。
4. public checker API や actual checker payload family はまだ固定していない。

したがって current 問いは、
**checker-cluster matrix line の次段として、actual checker payload family を docs-first に 1 段切るべきか**
である。

## 比較観点

1. checker-cluster matrix と actual machine-check source の橋渡しを見せられるか
2. public checker API を premature に固定しないか
3. `checked_reason_codes` / `reason_codes` mirror をそのまま final contract 化しないか
4. next 段で minimal checker payload family shape を narrow に比較できるか

## 比較対象

### 案 1. checker-side line はここで package close とし、actual checker payload family は still implicit に残す

#### 利点

- current checker-cluster matrix line を最小に保てる
- public checker side の premature actualization をさらに避けられる

#### 欠点

- current repo に already ある actual machine-check source との接点が prose 依存に残る
- `coverage_state` で止めた理由が「actual payload side に送るため」であることを docs 上で見せにくい

### 案 2. actual checker payload family を docs-first comparison として 1 段切る

#### 利点

- checker-cluster matrix line と actual machine-check source の間に 1 段の橋を置ける
- public checker API へは still 進まずに済む
- next 段で minimal checker payload family shape を narrow に比較しやすい

#### 欠点

- Phase 5 checker-side line が 1 段長くなる
- family naming discipline が要る

### 案 3. public checker artifact / public checker API へ直接進む

#### 利点

- actual checker-facing surface を早く見せられる

#### 欠点

- current phase では still early である
- docs-first line を actual implementation contract と誤読させやすい
- final type system / parser / public checker boundary の blocker と衝突しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual checker payload family を docs-first comparison として 1 段切る**
ことである。

理由は次の通り。

1. checker-cluster matrix line は docs-first inventory として十分切れている
2. `checked_reason_codes` / `reason_codes` の actual source は already あり、これを still implicit に残すより 1 段 bridge を置く方が user-facing に自然である
3. ただし public checker API へ直接進むのは blocker 6 に早く触れすぎる

## current reading

current next step では、
checker-cluster matrix line の次段として
**actual checker payload family**
を docs-first bridge として置いてよい。

ここで言う family は、

- current machine-check source を指す symbolic family
- public checker API ではない
- actual implementation schema でもない

という 3 条件を守る。

## practical example

### example — same-lineage floor

checker-cluster matrix 側では、

```text
typed_reason_family_hint = {
  family_refs = [lineage_edge_pair_family],
  coverage_state = full_cluster
}
```

までで止める。

その次段として actual checker payload family 側では、
`expected_static.checked_reason_codes` と detached static gate `reason_codes`
の current machine-check source を同じ family に属するものとして扱う comparison が自然になる。

## next promoted line

next promoted line は、
**actual-checker-payload-family-ready minimal-checker-payload-family threshold**
に置く。

ここで比べるのは、

1. family kind だけの marker
2. family kind + source refs を持つ minimal bundle
3. supported kind refs まで current family に入れる richer bundle

のどこまでを current first choice にするかである。

## what is decided here

### decided

- checker-cluster matrix line の次段として actual checker payload family を docs-first に切ってよい
- public checker API へはまだ進まない
- `checked_reason_codes` / `reason_codes` の actual source は still evidence だが、family bridge comparison の対象にしてよい

### not decided

- actual checker payload family の minimal field set
- `supported kind` summary を family 側にどこまで入れるか
- final public checker artifact / public checker API

## open questions

- actual checker payload family の minimal source ref setはどこまで必要か
- detached static gate `reason_codes` を family bridge の current first source に含めるべきか
- future parser-side checker payload family と同じ naming を維持できるか
