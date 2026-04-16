# 441 — current L2 model-check small-cluster projection keep/drop refresh

## 目的

`specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
までで fixed した

- row-local machine-facing carrier floor
- small-cluster semantic projection bridge grain
- sample-facing summary wording
- tool-binding stop line

を前提に、
**small-cluster projection で何を keep し、何を drop するか**
を docs-first に refresh する。

ここで fixed するのは current bridge cut であり、

- final property language
- concrete model-check tool binding
- production checker / runtime-policy contract

は still later に残す。

## source-backed floor

- current floor は row-local machine-facing `model_check_concrete_carriers` である。
- source-sample emitted artifact wiring は carrier line まで fan-out 済みである。
- sample-facing theorem/model-check summary は current bless / regression flow に反映済みである。
- theorem review / discharge line と typed reserve cluster は current principal source ではない。

## keep / drop comparison

| candidate | reading | keep value | current drop reason |
|---|---|---|---|
| row-alias bundle only | row-local carrier 群を alias のまま並べる | 最も narrow | semantic projection planning が進まない |
| same-subject stage-local small cluster | 同じ `projection_subject_kind` / `projection_subject_ref` に属し、同じ stage / local guard family を共有する cluster | current bridge cut として十分 | room protocol まで吸い込むと scope が広がる |
| mixed typed / theorem / handoff cluster | typed reserve、review/discharge、handoff reserve を 1 cluster に束ねる | one-bundle で見やすい | theorem / typed / order-handoff line が混線する |
| room protocol cluster | replay / fairness / provider receipt まで含む cluster | long-run property family に近い | protocol line を premature に吸い込む |

## current judgment

current L2 で最も自然なのは、
**same-subject stage-local small cluster**
を small-cluster projection の keep line に置くことである。

## reserve bundle floor

current package が持つ minimum は次である。

```text
small_cluster_projection_reserve = {
  projection_subject_kind,
  projection_subject_ref,
  stage_cluster_family_refs[],
  member_row_refs[],
  projected_relation_refs[],
  cluster_guard_refs[],
  excluded_family_refs[],
  kept_later_refs[]
}
```

## keep

- same `projection_subject_kind` / `projection_subject_ref`
- same stage-sequence family に属する member row 群
- row-local carrier から直接導ける projected relation refs
- cluster 単位で必要な local guard refs

## drop from current package

- request / predicate / `try` reserve cluster
- theorem review unit / discharge reserve row
- order / handoff replay / provider receipt / fairness family
- room-level protocol projection
- concrete tool schema

## guard

- row-local carrier floor を principal source に維持する
- theorem notebook line を model-check projection principal にしない
- typed reserve cluster を small-cluster projection へ混ぜない
- room protocol / fairness / replay family を still later に残す

## next promoted line

next promoted line は、
**order / handoff source-surface wording reserve note**
に置く。

## what is not decided here

- final property language
- concrete model-check tool brand
- room protocol projection timing
- production checker / runtime-policy contract
