# 435 — current L2 tool-binding stop-line refresh

## 目的

`specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
と
`specs/examples/427-current-l2-sample-visible-theorem-model-check-property-summary-wording.md`
で fixed した

- row-local machine-facing carrier first
- small-cluster semantic projection reserve
- sample-facing summary wording floor

を前提に、
model-check line の **tool-binding stop line** を refresh する。

ここで fixed するのは non-goal と stop line であり、
concrete model-check tool binding は fixed しない。

## source-backed floor

- current machine-facing floor は row-local `model_check_concrete_carriers` である。
- source-sample emitted artifact wiring は carrier line まで fan-out 済みである。
- sample-facing summary は presence / guard / reached wording に留まる。
- replay / provider receipt / fairness / room protocol family は order / handoff side に残る。

## refresh comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| explicit stop-line refresh | current floor と non-goal を明記する | premature binding を防ぎやすい | concrete next step が dry に見える |
| tool placeholder promotion | tool slot だけ先に public に出す | later integration を想像しやすい | brand / schema / contract を hidden promotion しやすい |
| projection-first promotion | small-cluster projection を current floor に繰り上げる | property-language pressure を前へ出せる | row-local carrier floor を崩しやすい |

## current judgment

current L2 で最も自然なのは、
**explicit stop-line refresh**
である。

## refreshed stop line

current package が持つ minimum は次である。

```text
model_check_tool_binding_stop_line = {
  principal_machine_carrier_ref,
  projection_reserve_ref,
  sample_summary_ref,
  guard_refs[],
  kept_later_refs[]
}
```

### guard

- row-local machine-facing carrier を principal floor に維持する
- small-cluster projection は reserve に留める
- sample summary を settled property language と呼ばない
- concrete tool brand / tool schema / runtime-policy contract を still later に残す

## next promoted line

next promoted line は、
**model-check small-cluster projection keep/drop refresh**
に置く。

## what is not decided here

- concrete tool brand / binding
- first settled property language
- room-protocol / fairness projection timing
- full protocol family
- production checker / runtime-policy contract
