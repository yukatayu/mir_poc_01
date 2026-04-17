# 446 — current L2 theorem discharge transport and public-contract later-gate framing note

## 目的

`specs/examples/419-current-l2-first-theorem-lemma-family-wording-hardening.md`、
`434-current-l2-admissible-evidence-contraction-note.md`、
`440-current-l2-notebook-consumer-threshold-and-discharge-reserve-note.md`
で fixed した

- semantic-core theorem pilot
- admissible evidence contraction
- notebook-first threshold and abstract discharge-entry reserve

を前提に、
**actual discharge transport と public theorem contract をどの later gate に残すか**
を docs-first に整理する。

ここで fixed するのは later-gate framing であり、

- concrete prover brand
- actual proof object transport
- final public theorem contract

は still later に残す。

## source-backed floor

- theorem pilot の principal line は semantic-core lemma family である。
- notebook-first consumer threshold は abstract discharge-entry reserve row に置かれている。
- admissible theorem evidence は symbolic refs only に contraction 済みである。
- `proof_notebook_review_unit` は theorem discharge result ではなく row-local review artifact である。

## later-gate comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| transport-first framing | discharge transport seam だけを先に fixed する | proof-object movement を考えやすい | public contract seam が曖昧に残る |
| public-contract-first framing | public theorem contract seam だけを先に fixed する | public API pressure を見やすい | transport / receipt / proof-object boundary が collapse しやすい |
| coupled later-gate framing | transport seam と public-contract seam を adjacent だが distinct な later gate として同時に framing する | current repo memory と最も整合する | two-gate distinctionを落とすと誤読されやすい |

## current judgment

current L2 で最も自然なのは、
**coupled later-gate framing**
である。

ただし coupling は “同時採用” ではなく、
**transport seam と public-contract seam を distinct に保ったまま、どちらも current package の外へ送る**
という意味である。

## later-gate floor

```text
theorem_discharge_later_gate = {
  discharge_entry_reserve_refs[],
  transport_seam_refs[],
  public_contract_seam_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

## keep

- notebook-first consumer threshold を維持する
- abstract discharge-entry reserve row を維持する
- admissible evidence floor は symbolic refs only に維持する
- transport seam と public-contract seam を distinct に保つ

## drop from current package

- concrete prover-facing payload
- proof object public schema
- transport を theorem result adoption と同一視すること
- final public theorem contract を fixed すること

## next promoted line

next promoted line は、
**model-check property-language / tool-binding later-gate framing note**
に置く。

## what is not decided here

- concrete theorem prover brand
- proof object schema
- theorem discharge transport format
- public theorem contract
- public checker migration timing
