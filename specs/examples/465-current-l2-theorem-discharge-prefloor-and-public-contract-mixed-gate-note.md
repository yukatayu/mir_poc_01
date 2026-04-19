# 465 — current L2 theorem discharge pre-floor and public-contract mixed-gate note

## 目的

`specs/examples/440`、`446`、`463` を前提に、

- `proof_notebook_review_unit`
- abstract discharge-entry reserve
- transport seam
- public-contract seam

を、final adoption に上げずに helper-local compare floor としてどこまで concrete に見せてよいかを整理する。

ここで fixed するのは、
**sample-local theorem discharge pre-floor を test/support helper に置き、current mixed-gate comparison を 1 段厚くする current judgment**
であり、

- actual discharge transport adoption
- public theorem contract adoption
- concrete theorem prover binding
- proof object public schema

は still later に残す。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. row-local `proof_notebook_review_unit`
2. sample-facing `verification_preview` / `artifact_preview`
3. sample-local preview-aligned typed artifact route
4. abstract discharge-entry reserve line
5. transport seam と public-contract seam を coupled だが distinct に保つ later-gate framing

したがって、
**row-local review unit から mixed-gate seam までを compare する helper-local floor**
を置くこと自体は current repo memory と矛盾しない。

## comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| docs-only reserve only | discharge reserve / seam は prose comparison だけに留める | public widening risk が最も低い | representative corpus で compare が回らない |
| helper-local theorem discharge pre-floor | sample-local helper が review-unit refs / discharge-entry reserve / transport seam / public-contract seam を同時に返す | runtime/static/guarded/prototype corpus で compare を回しやすい | public theorem contract に誤読すると widen しやすい |
| transport-first adoption | actual discharge transport shape を current helper shape へ寄せる | next adoption を想像しやすい | public-contract seam を hidden adoption に変えやすい |

## current judgment

current L2 で最も自然なのは、
**helper-local theorem discharge pre-floor**
である。

これは次を意味する。

1. principal source は引き続き row-local `proof_notebook_review_unit` に置く。
2. abstract discharge-entry reserve は helper-local compare floor に留める。
3. transport seam と public-contract seam は distinct refs として並べてよい。
4. concrete theorem prover binding と proof object public schema は kept-later marker に留める。

## pre-floor shape

current package が持つ minimum は次である。

```text
theorem_discharge_prefloor = {
  discharge_subject_kind,
  discharge_subject_ref,
  principal_review_unit_refs[],
  discharge_entry_reserve_refs[],
  transport_seam_refs[],
  public_contract_seam_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

### current reading

- `principal_review_unit_refs[]`
  - row-local `proof_notebook_review_unit` の current principal floor
- `discharge_entry_reserve_refs[]`
  - abstract discharge-entry reserve の helper-local compare floor
- `transport_seam_refs[]`
  - actual discharge transport を still later に残す distinct seam
- `public_contract_seam_refs[]`
  - public theorem contract を still later に残す distinct seam
- `guard_refs[]`
  - reached / guarded-not-reached を current compare floor で読める最小 guard
- `kept_later_refs[]`
  - concrete theorem prover binding と proof object public schema を later line に残す marker

## adequacy corpus

current helper-local compare floor では、少なくとも次を representative corpus にしてよい。

- `e5-underdeclared-lineage`
  - static reached
- `p05-dice-owner-guarded-chain`
  - guarded not reached
- `p06-typed-proof-owner-handoff`
  - typed/theorem/model-check runtime reached
- `p07-dice-late-join-visible-history`
  - order/handoff runtime reached
- `p08-dice-stale-reconnect-refresh`
  - stale reconnect runtime reached

## keep

- row-local review unit floor を principal source に維持する
- abstract discharge-entry reserve は helper-local compare floor に留める
- transport seam と public-contract seam を distinct に保つ
- guarded-not-reached sample を empty-success に読み替えない
- concrete theorem prover binding と proof object schema を kept-later marker に留める

## drop from current package

- actual discharge transport adoption
- public theorem contract adoption
- concrete theorem prover brand / schema adoption
- proof object public schema adoption
- helper-local pre-floor を final public verifier contract と読むこと

## next promoted line

next promoted line は、
**stronger typed-surface actual adoption**
または
**theorem discharge transport / public-contract actual adoption**
の mixed gate に置く。

## what is not decided here

- actual discharge transport format
- public theorem contract
- concrete theorem prover brand / binding
- proof object public schema
- helper-local theorem discharge pre-floor を public layer に昇格するかどうか
