# 309 — current L2 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready phase6-reserve-formal-tool-binding-inventory comparison

## 目的

`specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
と
`specs/examples/308-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold.md`
で parser second tranche first package を actualize した次段として、

- theorem-first / model-check concrete binding をどの reserve inventory で保持するか
- tool-neutral formal hook を current entry criteria のまま維持しつつ、non-selected formal line をどう mirror するか
- parser-side mainline と formal-side reserve path の境界をどこまで narrow に切るか

を比較する。

ここで固定するのは
**current L2 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready phase6-reserve-formal-tool-binding-inventory comparison**
であり、

- concrete theorem prover binding の actual workflow
- concrete model-check tool binding の actual workflow
- dual tool binding の同時 actualization
- parser-side follow-up package 自体の selected cut

はまだ固定しない。

## scope

- Phase 6 front-half compile-ready checkpoint close と parser second tranche first package actualization を fixed entry criteria とする。
- formal-side の current reserve line は theorem-first と model-check side に限定して読む。
- theorem-first line は Phase 5 theorem-side consumer pressure / notebook pressure の docs-first ladderを根拠に読む。
- model-check line は Phase 5 proof / protocol / runtime-policy inventory と `specs/examples/30` checker cut boundary を根拠に読む。

## current 前提

current repo では次が成立している。

1. `specs/examples/297...298` により、Phase 5 closeout は checker-side verifier handoff surface、theorem retained bridge stop line、boundary inventory、retained-later lineを含む narrow closeout bundleとして fixed 済みである。
2. `specs/examples/303...304` により、compile-ready formal hook は theorem-line整合の tool-neutral emitted hook row core と selected verification gate まで actualize 済みであり、concrete theorem / model-check tool binding は retained-later に残っている。
3. `specs/examples/305...306` により、next reopen sequencing は parser second tranche first に fixed 済みであり、theorem-first concrete tool binding と model-check line は deferred reserve path に残っている。
4. `specs/examples/307...308` により、parser second tranche first package は `mir_ast::current_l2` の stage3 declaration-side admit attached slot と shared isolated predicate fragment まで actualize 済みである。
5. theorem-side には `specs/examples/130...138`、`151...155`、`200...205` などの consumer pressure / notebook pressure ladder があり、model-check side には `specs/examples/30` と `297...298` の inventory / retained-later line がある。

したがって current 問いは、
**formal side を immediate line に戻さずに、next reopen 時の reserve inventory をどこまで narrow に mirror すれば十分か**
である。

## 比較観点

1. tool-neutral formal hook を current entry criteria として維持できるか
2. theorem-first と model-check side の reserve priority を narrow に明示できるか
3. parser-side follow-up package と concrete tool choice を同じ immediate line に混ぜないか
4. public checker / runtime surface への逆流を抑えられるか

## 比較対象

### 案 1. theorem-first を first reserve、model-check を second reserve に置く

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

- theorem-side には existing consumer pressure / notebook pressure ladder があり、model-check side より source-backed に reserve wording を組みやすい。
- model-check side を second reserve に置くことで、protocol / finite-state property 側の later reopen を消さずに残せる。
- current mainline を parser-side follow-up sequencing へ返しやすい。

#### 欠点

- concrete tool 自体はまだ選ばないので、formal workflow の actual friction は残る。

### 案 2. theorem-first / model-check を順序づけず、orderless reserve inventory にする

#### shape

```text
phase6_reserve_formal_tool_binding_inventory = {
  reserve_refs = [
    theorem_concrete_tool_binding,
    model_check_concrete_tool_binding
  ]
}
```

#### 利点

- 見た目は軽い。

#### 欠点

- theorem-side に既存 ladder があることと、model-check side が still thinner であることが読めない。
- next reopen 時の priority が曖昧になり、parser-side follow-up package との順序付けも弱くなる。

### 案 3. model-check 先行または dual-first の reserve inventory にする

#### 利点

- protocol / verifier side を強く意識できる。

#### 欠点

- current repo では model-check side の concrete consumer pressure が theorem-side より薄い。
- dual-first は concrete tool choice を premature に太らせやすい。
- parser-side follow-up package と同時に reopen しやすくなる。

## current judgment

current L2 で最も自然なのは、
**案 1. theorem-first を first reserve、model-check を second reserve に置く**
である。

理由は次の通り。

1. theorem-side には Phase 5 の consumer pressure / notebook pressure ladder が既にあり、reserve wording を source-backed に書きやすい。
2. model-check side は `specs/examples/30` と proof / protocol / runtime-policy inventory までは強いが、concrete tool first cut は theorem-side より薄い。
3. tool-neutral formal hook は compile-ready gate として already fixed なので、formal side を immediate line に戻す必要はない。
4. parser-side current mainline を止めずに、formal side の non-selected line だけを reserve inventory に落とす方が narrow reopen discipline に合う。

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

current reserve inventory judgment が示すのは、次の 5 点だけである。

1. theorem-first concrete binding は first reserve line に置く
2. model-check side は second reserve line に置く
3. tool-neutral formal hook は current entry criteria に維持する
4. parser-side follow-up package を current promoted line に戻す
5. concrete tool name / CI gate / public checker migration はまだ固定しない

## next promoted line

next promoted line は、
**phase6-reserve-formal-tool-binding-inventory-ready phase6-parser-side-follow-up-package-sequencing comparison**
に置く。

## open questions

- theorem-first reserve line を notebook pressure 以降のどの wording まで snapshot に残すか
- parser-side follow-up package で shared single attachment frame を immediate line に含めるか
- sample/program corpus と static / theorem / model-check verification staging をどの phase gate で mainline に接続するか
