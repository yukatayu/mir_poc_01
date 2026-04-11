# 306 — current L2 phase6-next-reopen-sequencing-ready minimal-phase6-next-reopen-sequencing threshold

## 目的

`specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
で next reopen sequencing の current first choice を
parser second tranche first に置く判断を fixed した次段として、

- Phase 6 next reopen sequencing の minimum をどこまでに留めるか
- fixed entry criteria / selected first reopen / deferred reserve path / guard line を minimum にどう反映するか
- next mainline を parser-side first execution package へどう渡すか

を比較する。

ここで固定するのは
**current L2 phase6-next-reopen-sequencing-ready minimal-phase6-next-reopen-sequencing threshold**
であり、

- parser second tranche first execution package の actual code anchor
- theorem-first concrete tool binding の actual workflow
- model-check concrete tool binding
- final public parser / checker / runtime surface

はまだ固定しない。

## 比較観点

1. checkpoint close の fixed entry criteria を minimum に反映できるか
2. selected first reopen と deferred reserve path を minimum に同時に置けるか
3. request head / clause suite / richer diagnostics bulk widen と tool-choice bulk reopen を guard として minimum に残せるか

## 比較対象

### 案 1. `selected_first_reopen_ref` だけを持つ

#### shape

```text
phase6_next_reopen_sequencing = {
  selected_first_reopen_ref =
    phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route
}
```

#### 利点

- 軽い

#### 欠点

- compile-ready checkpoint close を何の entry criteria として維持するか minimum に現れない
- theorem/model-check reserve path が minimum に現れない
- guard line が弱い

### 案 2. `sequencing_kind + fixed_entry_criteria_refs + selected_first_reopen_ref + deferred_reopen_refs + guard_refs` を持つ

#### shape

```text
phase6_next_reopen_sequencing = {
  sequencing_kind = phase6_checkpoint_postclose_next_reopen,
  fixed_entry_criteria_refs = [
    phase6_parser_first_tranche,
    phase6_checker_runtime_first_tranche,
    phase6_compile_ready_formal_hook
  ],
  selected_first_reopen_ref =
    phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route,
  deferred_reopen_refs = [
    theorem_first_concrete_tool_binding_route,
    concrete_model_check_tool_binding
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_entry_criteria,
    avoid_request_head_clause_suite_richer_diagnostics_bulk_widen,
    keep_model_check_line_reserve_only
  ]
}
```

#### 利点

- checkpoint close 後の sequencing であることを minimum に反映できる
- parser-side selected line と formal-side reserve line を同時に minimum に反映できる
- bulk widen を guard として minimum に残せる

#### 欠点

- 案 1 より field は増える

### 案 3. parser-side first execution package や theorem-side consumer pressure まで minimum に含める

#### 利点

- sequencing minimum は大きく見える

#### 欠点

- sequencing threshold ではなく execution package threshold になってしまう
- selected line と reserve line の境界を minimum から読みにくくする

## current judgment

current L2 で最も自然なのは、
**案 2. `sequencing_kind + fixed_entry_criteria_refs + selected_first_reopen_ref + deferred_reopen_refs + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は execution package そのものではなく、checkpoint close 後の sequencing threshold を固定する task である。
2. selected parser-side line と deferred formal-side line を minimum に両方置かないと、Task 2 / Task 3 の切り分けが弱い。
3. request head / clause suite / richer diagnostics bulk widen と model-check side の premature reopen を guard に残す必要がある。

## current first choice shape

```text
phase6_next_reopen_sequencing = {
  sequencing_kind = phase6_checkpoint_postclose_next_reopen,
  fixed_entry_criteria_refs = [
    phase6_parser_first_tranche,
    phase6_checker_runtime_first_tranche,
    phase6_compile_ready_formal_hook
  ],
  selected_first_reopen_ref =
    phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route,
  deferred_reopen_refs = [
    theorem_first_concrete_tool_binding_route,
    concrete_model_check_tool_binding
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_entry_criteria,
    avoid_request_head_clause_suite_richer_diagnostics_bulk_widen,
    keep_model_check_line_reserve_only
  ]
}
```

## practical reading

current minimal next reopen sequencing が示すのは、

- stage 1 / stage 2 carrier、checker/runtime thin skeleton、tool-neutral formal hook を fixed entry criteria に維持する
- next first reopen は parser-side attached-slot / predicate route に置く
- theorem-first concrete tool binding と model-check line は reserve path に残す
- bulk widen は guard として止める

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-next-reopen-sequencing-ready phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package comparison**
に置く。

## open questions

- parser-side first package で shared single attachment frame を同梱するか
- theorem-first reserve line を notebook-pressure wording まで mirror するか
