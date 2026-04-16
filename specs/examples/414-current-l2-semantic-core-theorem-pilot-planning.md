# 414 — current L2 semantic-core theorem pilot planning

## 目的

current theorem-side pilot を、

- first lemma order
- carrier / subject
- admissible evidence floor
- review / discharge stop line

で docs-first に固定する。

ここで固定するのは **concrete prover binding** ではなく、
semantic-core theorem pilot をどこから始めるかの plan である。

## source-backed floor

- tool-neutral formal hook は `fixture_static_cluster` と `runtime_try_cut_cluster` を subject に持つ。
- contract row の current floor は
  - `canonical_normalization_law`
  - `no_re_promotion`
  - `rollback_cut_non_interference`
  である。
- `proof_notebook_review_unit` は row-local review artifact として actual code anchor を持つ。
- source-sample emitted artifact wiring は formal hook と review-unit line を already 接続している。

## first lemma order

| order | lemma family | subject kind | admissible evidence floor | current role |
|---|---|---|---|---|
| 1 | `canonical_normalization_law` | `fixture_static_cluster` | fixture ref + static gate artifact ref | static malformed cluster の first semantic law |
| 2 | `no_re_promotion` | `fixture_static_cluster` | fixture ref | same-lineage floor を theorem-side に保つ law |
| 3 | `rollback_cut_non_interference` | `runtime_try_cut_cluster` | fixture ref + runtime cluster ref | local rollback / cut interaction の first runtime-side law |

## review vs discharge

| layer | current reading |
|---|---|
| formal hook | tool-neutral obligation row bundle |
| proof notebook review unit | human-reviewable row-local unit |
| theorem discharge | still later |

current `proof_notebook_review_unit` は **proof object** ではない。
`goal_text` と checklist は review artifact であり、
machine-checked theorem discharge result を current package で要求しない。

## current judgment

1. theorem pilot の first line は source syntax 全体ではなく **semantic-core relation family** から始める。
2. current first concrete consumer は notebook-first line に維持する。
3. tool-neutral formal hook を theorem-side entry に保ち、review unit を first concrete pilot に置く。
4. theorem pilot package は lemma order / subject / evidence floor / stop line を fixed したところで閉じる。

## current first choice details

- theorem-side carrier は `subject_kind + subject_ref + contract_rows[]` の tool-neutral row bundleを entry にする。
- row-local review unit は current first concrete pilot に保つ。
- discharge 側へ送る前に必要なのは、
  - obligation_kind
  - evidence_refs
  - subject family
  の整合であり、concrete theorem prover brand ではない。

## what is not decided here

- concrete theorem prover brand
- proof object public contract
- final review workflow
- actual theorem discharge transport
- public checker migration timing
