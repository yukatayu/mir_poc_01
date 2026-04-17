# 447 — current L2 model-check property-language and tool-binding later-gate framing note

## 目的

`specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`、
`435-current-l2-tool-binding-stop-line-refresh.md`、
`441-current-l2-model-check-small-cluster-projection-keep-drop-refresh.md`
で fixed した

- row-local machine-facing carrier
- small-cluster projection grain
- explicit tool-binding stop line

を前提に、
**first settled property language と concrete tool seam をどの later gate に残すか**
を docs-first に整理する。

ここで fixed するのは later-gate framing であり、

- concrete model-check tool binding
- settled property language adoption
- production checker / runtime-policy contract

は still later に残す。

## source-backed floor

- current floor は row-local `model_check_concrete_carriers` である。
- same-subject stage-local small cluster は keep line に入っている。
- typed reserve / theorem discharge / room protocol family は current model-check projection principal ではない。
- tool-binding stop line は already explicit に fixed されている。

## later-gate comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| property-language-first framing | first settled property language を先に具体化し、tool seam は後ろに送る | semantics側の整理がしやすい | tool seam と public contract seam が別 drift しやすい |
| tool-seam-first framing | concrete tool seam だけ先に framing する | implementation pressure は見やすい | property language が hidden placeholder になりやすい |
| adjacent later-gate framing | property language と tool seam を adjacent だが distinct な later gate として同時に framing する | current repo memory と最も整合する | one gate に collapse すると誤読されやすい |

## current judgment

current L2 で最も自然なのは、
**adjacent later-gate framing**
である。

これは、

- first settled property language を current floorに昇格させず、
- concrete tool seam も still later に残しつつ、
- その 2 つが current row-local / small-cluster floor の外側にあることを同時に明示する

ための framing である。

## later-gate floor

```text
model_check_later_gate = {
  principal_machine_carrier_refs[],
  small_cluster_projection_refs[],
  property_language_seam_refs[],
  tool_binding_seam_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

## keep

- row-local carrier floor を principal source に維持する
- same-subject stage-local small cluster を projection keep line に維持する
- property language seam と tool seam を distinct に保つ
- theorem / typed / room protocol family を current model-check principal へ混ぜない

## drop from current package

- settled property language の adoption
- concrete tool brand の決定
- room protocol / fairness / replay family の current昇格
- production checker / runtime-policy contract の固定

## next promoted line

next promoted line は、
**shared-space fairness / replay mixed-gate boundary note**
に置く。

## what is not decided here

- first settled property language
- concrete tool brand / schema
- room protocol projection timing
- production checker / runtime-policy contract
