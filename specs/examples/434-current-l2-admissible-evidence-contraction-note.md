# 434 — current L2 admissible evidence contraction note

## 目的

`specs/examples/419-current-l2-first-theorem-lemma-family-wording-hardening.md`
と
`specs/examples/426-current-l2-proof-artifact-and-bridge-stop-line-refresh.md`
で fixed した

- semantic-core lemma wording floor
- formal hook / review unit / bridge sketch / theorem discharge split

を前提に、
theorem-side pilot で **admissible evidence** と呼んでよいものを contraction する。

ここで fixed するのは admissible evidence floor であり、
concrete prover brand や proof-object transport は fixed しない。

## source-backed floor

- first theorem pilot order は
  `canonical_normalization_law -> no_re_promotion -> rollback_cut_non_interference`
  である。
- tool-neutral formal hook は actual source-backed anchor を持つ。
- `proof_notebook_review_unit` は row-local human-review artifact である。
- review checklist / `goal_text` / walkthrough prose は theorem discharge ではない。

## contraction comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| symbolic-ref-only floor | `fixture_ref + subject_ref + contract_row_refs[] + optional structural refs` のみを admissible evidence とする | theorem pilot の stop line が明確になる | first consumer には dry に見える |
| symbolic refs + review prose | review unit の `goal_text` / checklist prose を admissible evidence に含める | human workflow に近く見える | review artifact と theorem discharge を collapse する |
| serialized artifact first | concrete external payload schema を admissible floor にする | long-run transport には近い | concrete prover / public contract を premature に固定する |

## current judgment

current L2 で最も自然なのは、
**symbolic-ref-only floor を admissible evidence contraction として採る**
ことである。

理由は次の通り。

1. theorem pilot は semantic law の wording floor を固める段階であり、review prose を theorem evidence へ混ぜると stop line が崩れる。
2. concrete serialized artifact schema を first floor にすると concrete prover brand と transport contract を早く固定しすぎる。
3. symbolic refs だけでも notebook-first review pressure は残せる。

## contracted floor

current package で admissible と呼ぶ minimum は次である。

```text
admissible_theorem_evidence = {
  fixture_ref,
  subject_ref,
  contract_row_refs[],
  optional_structural_refs[]
}
```

## explicit exclusions

- review checklist prose
- `goal_text`
- walkthrough note
- narrative explanation
- concrete prover payload
- public discharge receipt

## next promoted line

next promoted line は、
**notebook-consumer threshold and discharge reserve note**
に置く。

## what is not decided here

- exact contraction cut beyond the minimum refs
- concrete theorem prover brand
- proof-object public contract
- actual theorem discharge transport
- public checker migration timing
- final review workflow
