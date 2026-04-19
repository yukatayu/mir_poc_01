# 464 — current L2 model-check projection pre-floor and property/tool-seam mixed-gate note

## 目的

`specs/examples/420`、`441`、`447`、`463` を前提に、

- row-local machine-facing `model_check_concrete_carriers`
- same-subject stage-local small-cluster projection reserve
- property-language seam
- tool-binding seam

を、public adoption に上げずに helper-local compare floor としてどこまで concrete に見せてよいかを整理する。

ここで fixed するのは、
**sample-local projection pre-floor を test/support helper に置き、current mixed-gate comparison を 1 段厚くする current judgment**
であり、

- first settled property language adoption
- concrete model-check tool binding
- production checker / runtime-policy contract
- final public verifier contract

は still later に残す。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. row-local machine-facing `model_check_concrete_carriers`
2. sample-facing `verification_preview` / `artifact_preview`
3. sample-local preview-aligned typed artifact route
4. small-cluster semantic projection keep line
5. property-language seam と tool-binding seam を adjacent だが distinct に保つ later-gate framing

したがって、
**row-local carrier から mixed-gate seam までを compare する helper-local floor**
を置くこと自体は current repo memory と矛盾しない。

## comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| docs-only reserve only | projection / seam は prose comparison だけに留める | public widening risk が最も低い | representative corpus で compare が回らない |
| helper-local projection pre-floor | sample-local helper が row-local carrier / small-cluster projection / property/tool seam refs を同時に返す | runtime/static/guarded/prototype corpus で compare を回しやすい | public contract に誤読すると widen しやすい |
| property-language-first adoption | settled property language を current helper shape に寄せて先に見せる | next adoption を想像しやすい | later-gate framing を hidden adoption に変えやすい |

## current judgment

current L2 で最も自然なのは、
**helper-local projection pre-floor**
である。

これは次を意味する。

1. principal source は引き続き row-local machine-facing carrier に置く。
2. same-subject stage-local small-cluster projection は helper-local compare floor に留める。
3. property-language seam と tool-binding seam は distinct refs として並べてよい。
4. typed reserve / theorem discharge / room protocol projection は excluded family に残してよい。

## pre-floor shape

current package が持つ minimum は次である。

```text
model_check_projection_prefloor = {
  projection_subject_kind,
  projection_subject_ref,
  principal_machine_carrier_refs[],
  small_cluster_projection_refs[],
  property_language_seam_refs[],
  tool_binding_seam_refs[],
  guard_refs[],
  excluded_family_refs[],
  kept_later_refs[]
}
```

### current reading

- `principal_machine_carrier_refs[]`
  - row-local `model_check_concrete_carriers` の current principal floor
- `small_cluster_projection_refs[]`
  - same-subject stage-local small-cluster reserve の helper-local compare floor
- `property_language_seam_refs[]`
  - first settled property language を still later に残す distinct seam
- `tool_binding_seam_refs[]`
  - concrete tool brand / binding を still later に残す distinct seam
- `guard_refs[]`
  - reached / guarded-not-reached を current compare floor で読める最小 guard
- `excluded_family_refs[]`
  - typed reserve / theorem discharge / room protocol projection を current model-check principal へ混ぜない explicit marker
- `kept_later_refs[]`
  - fairness / replay operational profile と production checker/runtime-policy contract を later line に残す marker

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

- row-local carrier floor を principal source に維持する
- small-cluster projection は helper-local compare floor に留める
- property-language seam と tool-binding seam を distinct に保つ
- guarded-not-reached sample を empty-success に読み替えない
- typed / theorem / room-protocol family を excluded として明示する

## drop from current package

- settled property language adoption
- concrete tool brand / schema adoption
- production checker / runtime-policy contract の固定
- room protocol / fairness / replay family の current 昇格
- helper-local pre-floor を final public verifier contract と読むこと

## next promoted line

next promoted line は、
**theorem discharge transport / public-contract concretization**
または
**model-check property-language / tool-seam actual adoption**
の mixed gate に置く。

## what is not decided here

- first settled property language
- concrete model-check tool brand / schema
- production checker / runtime-policy contract
- room protocol / fairness / replay projection timing
- helper-local projection pre-floor を public layer に昇格するかどうか
