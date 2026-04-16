# 440 — current L2 notebook-consumer threshold and discharge reserve note

## 目的

`specs/examples/434-current-l2-admissible-evidence-contraction-note.md`
で fixed した

- symbolic-ref-only admissible theorem evidence floor
- review prose / `goal_text` / walkthrough prose の explicit exclusion

を前提に、
theorem-side pilot で
**notebook-first consumer の threshold をどこに置き、どこからを discharge reserve として残すか**
を docs-first に整理する。

ここで fixed するのは threshold / reserve cut であり、
concrete prover brand、actual discharge transport、public theorem contract は fixed しない。

## source-backed floor

- theorem pilot は semantic-core lemma family に anchored している。
- current lemma order は
  `canonical_normalization_law -> no_re_promotion -> rollback_cut_non_interference`
  である。
- `proof_notebook_review_unit` は row-local human-review artifact であり、proof object でも theorem discharge result でもない。
- admissible theorem evidence は
  `fixture_ref + subject_ref + contract_row_refs[] + optional_structural_refs[]`
  に contraction 済みである。
- review checklist prose、`goal_text`、walkthrough prose、concrete prover payload、public discharge receipt は admissible evidence に入らない。

## threshold comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| review-only threshold | review unit + admissible evidence が揃えば notebook consumer ready とみなし、discharge は完全に later に送る | safest | consumer-ready の cut が弱い |
| abstract discharge-entry row | review unit と admissible evidence の外側に、tool-neutral な discharge-entry reserve row だけを置く | threshold と reserve を分けやすい | row naming を早く固定しすぎる恐れ |
| abstract discharge-result family | tool-neutral discharge result / receipt family を current cut に置く | end-to-end story は見やすい | public contract を premature に固定しやすい |
| prose-assisted threshold | `goal_text` や walkthrough prose を threshold 判定へ混ぜる | human workflow には近い | review と discharge を collapse する |
| concrete prover-facing payload | concrete payload / serialized handoff を threshold に入れる | later transport に近い | concrete brand と transport contract を早く固定しすぎる |

## current judgment

current L2 で最も自然なのは、
**abstract discharge-entry row**
を current first choice に置くことである。

理由は次の通り。

1. review-only threshold だけでは、notebook consumer ready と actual discharge reserve の境界が薄い。
2. discharge-result family や concrete payload を current cut にすると、proof object public contract や tool brand を早く固定しすぎる。
3. abstract discharge-entry row なら、
   notebook-first line を維持したまま
   threshold と later discharge reserve を分けられる。

## current threshold / reserve split

```text
notebook_consumer_threshold = {
  review_unit_refs[],
  admissible_evidence_refs[],
  discharge_entry_reserve_refs[],
  threshold_guard_refs[],
  kept_later_refs[]
}

abstract_discharge_entry_reserve = {
  obligation_kind_refs[],
  subject_ref,
  evidence_refs[],
  contract_row_refs[],
  optional_consumer_hint_refs[]
}
```

### keep

- notebook-first consumer を current first concrete line に維持する
- admissible evidence floor は symbolic refs only に維持する
- abstract discharge-entry row は reserve に留める
- concrete discharge result / receipt / transport は later に残す

### drop from current package

- review prose や `goal_text` を admissible evidence に戻すこと
- notebook consumer を theorem discharge result と書くこと
- concrete prover-facing payload を threshold に入れること
- public theorem contract を fixed すること

## next promoted line

next promoted line は、
**model-check small-cluster projection keep/drop refresh**
に置く。

## what is not decided here

- exact discharge-entry row namespace
- theorem discharge result family
- concrete theorem prover brand
- actual discharge transport
- public theorem contract
- public checker migration timing
