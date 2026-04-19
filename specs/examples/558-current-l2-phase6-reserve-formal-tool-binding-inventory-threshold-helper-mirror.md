# 558 — current L2 phase6-reserve-formal-tool-binding-inventory threshold helper mirror

## 目的

`specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
と
`specs/examples/310-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-minimal-phase6-reserve-formal-tool-binding-inventory-threshold.md`
で source-backed に残していた
reserve formal tool binding inventory の current first choice を、
crate-level manifest と helper-local operational CLI summary に narrow に mirror する。

ここで actualize するのは
**phase6 reserve formal tool binding inventory threshold helper mirror**
であり、

- concrete theorem prover tool 名
- concrete model-check tool 名
- actual CI / artifact retention policy
- parser-side follow-up package 自体の selected cut
- final public parser / checker / runtime surface

はまだ固定しない。

## helper mirror shape

current cut は、
次の 5 要素だけを inspectable にする。

```text
inventory_kind
fixed_entry_criteria_refs
first_reserve_ref
second_reserve_ref
guard_refs
```

これにより、

1. theorem-first reserve を first line に置く
2. model-check concrete binding を second reserve に置く
3. tool-neutral formal hook を current entry criteria に維持する
4. parser-side mainline を止めない
5. dual tool choice / public checker-runtime backpressure を避ける

という minimum を code と CLI summary から読める。

## actualization cut

- crate anchor:
  `CurrentL2Phase6ReserveFormalToolBindingInventoryManifest`
- runtime getter:
  `current_l2_phase6_reserve_formal_tool_binding_inventory_manifest()`
- test anchor:
  `crates/mir-runtime/tests/current_l2_reserve_formal_tool_binding_inventory_manifest.rs`
- helper preview:
  `actual_phase6_reserve_formal_tool_binding_inventory_threshold`

helper preview では、
`p07 / p08 / p09` representative shared-space trio が
`actual_phase6_next_reopen_sequencing_threshold`
reached 後にだけ
reserve inventory threshold を `reached` として見せる。

## practical reading

- theorem-first reserve は actual binding ではなく reserve inventory である。
- model-check side も捨てず、second reserve として helper-local summary に残す。
- parser-side の次 package はこの package で実装せず、
  **phase6-parser-side-follow-up-package-sequencing**
  として次段へ返す。

## stop line

- concrete theorem prover tool name
- concrete model-check tool name
- actual CI / artifact retention policy
- parser-side follow-up package selection
- final public parser / checker / runtime surface

## next promoted line

next promoted line は、
**phase6-parser-side-follow-up-package-sequencing ratchet**
に置く。
