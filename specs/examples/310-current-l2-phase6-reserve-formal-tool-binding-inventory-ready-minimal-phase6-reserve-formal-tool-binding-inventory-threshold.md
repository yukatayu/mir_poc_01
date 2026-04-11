# 310 — current L2 phase6-reserve-formal-tool-binding-inventory-ready minimal-phase6-reserve-formal-tool-binding-inventory threshold

## 目的

`specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
で reserve formal tool binding inventory の current first choice を
theorem-first first reserve / model-check second reserve に置く判断を fixed した次段として、

- Phase 6 reserve formal tool binding inventory の minimum をどこまでに留めるか
- fixed entry criteria / reserve priority / guard line を minimum にどう反映するか
- current mainline を parser-side follow-up package sequencing へどう返すか

を比較する。

ここで固定するのは
**current L2 phase6-reserve-formal-tool-binding-inventory-ready minimal-phase6-reserve-formal-tool-binding-inventory threshold**
であり、

- concrete theorem prover tool 名
- concrete model-check tool 名
- actual CI / artifact retention policy
- parser-side follow-up package 自体の selected cut

はまだ固定しない。

## 比較観点

1. reserve inventory であることを minimum に反映できるか
2. theorem-first / model-check の priority を minimum に反映できるか
3. tool-neutral formal hook と parser-side mainline を guard として minimum に残せるか

## 比較対象

### 案 1. `first_reserve_ref` だけを持つ

#### shape

```text
phase6_reserve_formal_tool_binding_inventory = {
  first_reserve_ref =
    theorem_first_notebook_pressure_concrete_tool_binding_route
}
```

#### 利点

- 軽い

#### 欠点

- model-check second reserve が minimum に現れない
- fixed entry criteria と guard line が弱い
- reserve inventory というより theorem-first memo になる

### 案 2. `inventory_kind + fixed_entry_criteria_refs + first_reserve_ref + second_reserve_ref + guard_refs` を持つ

#### shape

```text
phase6_reserve_formal_tool_binding_inventory = {
  inventory_kind = phase6_postclose_formal_reserve_inventory,
  fixed_entry_criteria_refs = [
    phase5_handoff_closeout,
    phase6_compile_ready_formal_hook,
    phase6_parser_second_tranche_first_package
  ],
  first_reserve_ref =
    theorem_first_notebook_pressure_concrete_tool_binding_route,
  second_reserve_ref =
    model_check_protocol_property_concrete_tool_binding_route,
  guard_refs = [
    keep_tool_neutral_formal_hook_as_current_entry_criteria,
    keep_parser_followup_package_as_current_mainline,
    avoid_dual_tool_choice_single_package,
    avoid_public_checker_runtime_surface_backpressure
  ]
}
```

#### 利点

- reserve inventory の性格と current entry criteria を minimum に反映できる
- theorem-first / model-check の priority を minimum に同時に残せる
- current mainline を parser-side follow-up package sequencing へ返す guard が明確になる

#### 欠点

- 案 1 より field は増える

### 案 3. concrete tool 名や CI gate まで minimum に含める

#### 利点

- formal workflow の見た目は具体的になる

#### 欠点

- reserve inventory threshold ではなく actual binding threshold になってしまう
- tool choice を premature に固定しやすい
- parser-side mainline と独立させにくい

## current judgment

current L2 で最も自然なのは、
**案 2. `inventory_kind + fixed_entry_criteria_refs + first_reserve_ref + second_reserve_ref + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は actual binding ではなく reserve inventory threshold を固定する task である。
2. theorem-first / model-check の priority を minimum に両方置かないと、next reopen 時の friction が減らない。
3. tool-neutral formal hook と parser-side mainline を guard に残す必要がある。

## current first choice shape

```text
phase6_reserve_formal_tool_binding_inventory = {
  inventory_kind = phase6_postclose_formal_reserve_inventory,
  fixed_entry_criteria_refs = [
    phase5_handoff_closeout,
    phase6_compile_ready_formal_hook,
    phase6_parser_second_tranche_first_package
  ],
  first_reserve_ref =
    theorem_first_notebook_pressure_concrete_tool_binding_route,
  second_reserve_ref =
    model_check_protocol_property_concrete_tool_binding_route,
  guard_refs = [
    keep_tool_neutral_formal_hook_as_current_entry_criteria,
    keep_parser_followup_package_as_current_mainline,
    avoid_dual_tool_choice_single_package,
    avoid_public_checker_runtime_surface_backpressure
  ]
}
```

## practical reading

current minimal reserve formal tool binding inventory が示すのは、

- theorem-first concrete binding を first reserve に置く
- model-check concrete binding を second reserve に置く
- tool-neutral formal hook を current entry criteria に維持する
- parser-side follow-up package sequencing を current mainline に戻す
- concrete tool 名 / actual gate / public surface への逆流はまだ止める

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-reserve-formal-tool-binding-inventory-ready phase6-parser-side-follow-up-package-sequencing comparison**
に置く。

## open questions

- theorem-first reserve line を notebook pressure のどの stop line まで mirror するか
- parser-side follow-up package で shared single attachment frame をどこまで current package に含めるか
- fixed-subset sample/program corpus を Phase 6 reserve line へいつ接続するか
