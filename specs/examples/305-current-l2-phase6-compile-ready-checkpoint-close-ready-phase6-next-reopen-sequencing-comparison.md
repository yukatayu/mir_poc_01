# 305 — current L2 phase6-compile-ready-checkpoint-close-ready phase6-next-reopen-sequencing comparison

## 目的

`specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
と
`specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
で Phase 6 front-half compile-ready checkpoint close を fixed した次段として、

- parser second tranche widen と concrete formal tool binding のどちらを先に reopen するか
- parser-side reopen を選ぶなら、stage 3 / perform head / richer diagnostics のどこを最小 first line に置くか
- formal side reopen を選ぶなら、theorem-first / model-check-first / dual-first のどこまでを現段階で source-backed に読めるか

を比較する。

ここで固定するのは
**current L2 phase6-compile-ready-checkpoint-close-ready phase6-next-reopen-sequencing comparison**
であり、

- concrete theorem prover binding
- concrete model-check tool binding
- request head / clause suite / richer diagnostics の一括 widen
- final public parser / checker / runtime surface

はまだ固定しない。

## scope

- Phase 6 front-half compile-ready checkpoint close 後の next reopen sequencing だけを扱う。
- fixed entry criteria source は `specs/examples/287...290`、`299...304` とする。
- parser-side retained-later line は stage 3 family と `perform_head_final_public_api` / `span_rich_diagnostics` / `final_grammar` に限定して読む。
- formal-side retained-later line は theorem / model-check concrete tool binding に限定して読む。

## current 前提

current repo では次が成立している。

1. `specs/examples/287...290` により、Phase 3/6 parser-side fixed entry criteria は stage 1 + stage 2 structural floor と first checker reconnect freeze であり、stage 3 request / predicate reconnect は retained-later である。
2. `specs/examples/299...300` により、Phase 6 parser first tranche は `mir_ast::current_l2` stage 1 / stage 2 carrier に留まり、stage 3 / perform head final API / span-rich diagnostics / final grammar は retained-later に残っている。
3. `specs/examples/301...302` により、checker/runtime first tranche は semantic `Program` entry と thin runtime skeleton まで actualize 済みであり、actual parser-to-`Program` lowering は retained-later に残っている。
4. `specs/examples/303...304` により、formal side は theorem-line整合の tool-neutral formal hook と selected cargo / smoke gate まで actualize 済みであり、concrete theorem/model-check tool binding は retained-later に残っている。
5. `tasks.md` と `plan/11` の current recommendation は、checkpoint close 後の immediate question を `parser second tranche widen` と `concrete theorem-first binding` の比較から始めることに置いている。

したがって current 問いは、
**compile-ready checkpoint close を壊さずに、next reopen point をどこへ置くのが最小か**
である。

## 比較観点

1. fixed entry criteria を崩さずに source-backed な first execution package を切れるか
2. parser-side / formal-side の retained-later line を current package へ premature に混ぜないか
3. request head / clause suite / richer diagnostics の一括 widen を回避できるか
4. model-check side を theorem-side と同列の足場がないまま既成事実化しないか
5. non-selected line を reserve path として明示しやすいか

## 比較対象

### 案 1. parser second tranche を先に reopen し、formal side は tool-neutral entry criteria のまま維持する

#### shape

```text
phase6_next_reopen_sequencing = {
  selected_first_reopen_ref =
    phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route,
  retained_entry_criteria_refs = [
    phase6_parser_first_tranche,
    phase6_checker_runtime_first_tranche,
    phase6_compile_ready_formal_hook
  ],
  deferred_reopen_refs = [
    concrete_theorem_tool_binding,
    concrete_model_check_tool_binding
  ]
}
```

#### 利点

- parser-side には stage 3 attached-slot / predicate / attachment / suite family の existing source-backed evidence があり、最小 actual package を選びやすい。
- `perform head` 直行や request head / clause suite / richer diagnostics 一括 widen を避けられる。
- theorem/model-check side は tool-neutral formal hook を entry criteria に維持したまま reserve path に押し戻せる。

#### 欠点

- concrete formal workflow 自体はまだ前進しない。

### 案 2. theorem-first concrete formal tool binding を先に reopen する

#### shape

```text
phase6_next_reopen_sequencing = {
  selected_first_reopen_ref =
    theorem_first_concrete_tool_binding_route,
  retained_entry_criteria_refs = [
    phase6_parser_first_tranche,
    phase6_checker_runtime_first_tranche,
    phase6_compile_ready_formal_hook
  ],
  deferred_reopen_refs = [
    parser_second_tranche_widen,
    concrete_model_check_tool_binding
  ]
}
```

#### 利点

- formal side の next consumer pressure を早く見られる。
- theorem-side consumer ladder には `proof_notebook` first choice が既にある。

#### 欠点

- current repo では theorem-side next cut もまだ docs-first consumer pressure 比較が中心で、code-backed first package は parser-side より薄い。
- tool choice が parser/public surface line に逆流しやすい。
- model-check side はなお足場が薄く、formal side 自体の reserve inventory を同時に整理する必要がある。

### 案 3. parser second tranche と theorem/model-check concrete binding を同時に reopen する

#### 利点

- next workstream を一見まとめて進められる。

#### 欠点

- checkpoint close 後の narrow reopen ではなく broad widen になる。
- parser-side lexical pressure と tool-side external workflow pressure が同時に増える。
- reserve path と immediate line の境界が曖昧になる。

## current judgment

current L2 で最も自然なのは、
**案 1. parser second tranche を先に reopen し、formal side は tool-neutral entry criteria のまま維持する**
である。

理由は次の通り。

1. parser-side retained-later line には stage 3 admit-slot / predicate / attachment / suite family の existing source-backed evidence があり、最小 actual package を選びやすい。
2. `perform head` 直行や request head / clause suite / richer diagnostics 一括 widen は current phase では重く、最小 reopen としては attached-slot / predicate route が自然である。
3. theorem-side concrete binding は theorem-first consumer ladder こそあるが、current cut は still docs-first であり、tool choice を immediate line に置くには足場が薄い。
4. tool-neutral formal hook は既に source-backed compile-ready gate として固定済みなので、formal side は reserve path として整理しても checkpoint close を壊さない。

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

current sequencing judgment が示すのは、次の 5 点だけである。

1. immediate next line は parser second tranche 側に置く
2. parser-side の first package は `perform head` 直行ではなく attached-slot / predicate route に置く
3. tool-neutral formal hook は current entry criteria として維持する
4. theorem-first concrete tool binding は reserve path に残す
5. model-check side はさらに後ろの reserve path に残す

## next promoted line

next promoted line は、
**phase6-next-reopen-sequencing-ready phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package comparison**
に置く。

## open questions

- parser second tranche first package を admit-slot と predicate fragment のどこまで actualize してよいか
- shared single attachment frame を同じ package に入れるか、それとも next parser-side package に残すか
- theorem-first concrete tool binding reserve path をどの consumer pressure wording まで snapshot に残すか
