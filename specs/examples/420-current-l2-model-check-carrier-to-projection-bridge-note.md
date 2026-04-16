# 420 — current L2 model-check carrier to projection bridge note

## 目的

`specs/examples/415-current-l2-model-check-projection-and-property-family-reserve-inventory.md`
で fixed した reserve inventory を前提に、

- row-local carrier からどの grain で projection へ橋をかけるか
- theorem line と protocol line をどこで分けるか
- current bridge bundle に何を含めるか

を docs-first に整理する。

## source-backed floor

- current floor は row-local machine-facing `model_check_concrete_carriers` である。
- source-sample emitted artifact wiring は carrier line まで fan-out 済みである。
- sample-facing theorem/model-check summary は already docs / README / regression flow に反映済みである。

## bridge grain comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| row-to-row alias only | row-local carrier をそのまま reserve とみなす | 最も軽い | projection planning が進まない |
| small-cluster semantic projection | 同一 subject family に属する row-local carrier 群を small semantic cluster として束ねる | theorem line と room protocol line の中間 bridge になる | scope を広げすぎると protocol line と混線する |
| room protocol projection | replay / provider receipt / fairness まで含む protocol projection | long-run property familyに近い | package 4 line を premature に吸い込む |

## current judgment

current L2 で最も自然なのは、
**small-cluster semantic projection を bridge grain に置く**
ことである。

## bridge bundle floor

current bridge note が持つ minimum は次である。

```text
model_check_projection_bridge = {
  projection_subject_kind,
  projection_subject_ref,
  member_row_refs[],
  projected_relation_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

## guard

- row-local carrier を current floor に維持する
- theorem notebook / review unit lineを model-check bridge の principal source にしない
- replay / provider receipt / fairness / room seriality family を current bridge bundle に入れない
- concrete model-check tool binding を still later に残す

## next promoted line

next promoted line は、
**sample-visible property summary wording**
に置く。

## what is not decided here

- concrete model-check tool brand
- first settled property language
- room protocol / fairness projection timing
- production checker / runtime-policy contract
