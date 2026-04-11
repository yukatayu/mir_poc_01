# 307 — current L2 phase6-next-reopen-sequencing-ready phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package comparison

## 目的

`specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
と
`specs/examples/306-current-l2-phase6-next-reopen-sequencing-ready-minimal-phase6-next-reopen-sequencing-threshold.md`
で parser second tranche first を fixed した次段として、

- parser-side first execution package をどの cut で actualize するか
- attached-slot / minimal predicate fragment をどこまで `mir_ast::current_l2` 本体へ持ち上げるか
- shared single attachment frame / request clause suite / perform head / richer diagnostics をどこまで retained-later に残すか

を比較する。

ここで固定するのは
**current L2 phase6-next-reopen-sequencing-ready phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package comparison**
であり、

- shared single attachment frame
- request clause suite
- perform head final public parser API
- span-rich diagnostics
- final grammar
- theorem/model-check concrete binding

はまだ固定しない。

## scope

- parser second tranche first package として、`mir-ast` current L2 non-production carrier の narrow widening だけを扱う。
- root source は `specs/examples/83`、`85`、`92`、`94`、`299...300`、`305...306` とする。
- code anchor は `crates/mir-ast/src/current_l2.rs`、stage3 admit-slot / predicate-fragment / multiline-attachment / request-clause-suite tests を主に扱う。
- parser-to-`Program` lowering や checker/runtime side actualization には進まない。

## current 前提

current repo では次が成立している。

1. `specs/examples/299...300` により、Phase 6 parser first tranche は stage 1 / stage 2 carrier に留まり、stage 3 family は retained-later である。
2. `specs/examples/305...306` により、next reopen sequencing は parser second tranche first に fixed 済みであり、first package は `perform head` 直行ではなく attached-slot / predicate route に置く判断がある。
3. tests/support には stage3 admit-slot helper と shared isolated predicate fragment helper の actual evidence があり、multiline attachment / request clause suite tests もその predicate fragment parser に依存する。
4. `perform head` / clause suite / richer diagnostics 一括 widen は current phase では重い later line として残している。

したがって current 問いは、
**attached-slot / predicate route をどの narrow carrier cut で actualize すれば、parser second tranche first package と読めるか**
である。

## 比較観点

1. parser-side first package として十分な code anchor を持てるか
2. stage3 attached-slot と shared predicate fragment を同じ package に束ねられるか
3. multiline attachment / request suite / perform head を premature に library 化しないか
4. existing support-based tests を crate surface reuse へ自然に寄せられるか
5. next mainline を formal-side reserve inventory へ自然に送れるか

## 比較対象

### 案 1. stage3 helper は tests/support private のまま維持し、spec だけ更新する

#### 利点

- library surface は増えない
- stage3 later family を accidental に public-ish 化しない

#### 欠点

- parser second tranche first package を actual code anchor として閉じられない
- stage3 attached-slot / predicate route を source-backed execution package にできない
- multiline attachment / suite tests も crate parser surface reuse へ寄せられない

### 案 2. declaration-side admit attached slot と shared isolated predicate fragment だけを `mir_ast::current_l2` へ actualize する

#### shape

```text
phase6_parser_second_tranche_first_package = {
  carrier_kind = current_l2_nonproduction_parser_second_tranche_carrier,
  accepted_surface_refs = [
    stage3_decl_admit_slot_surface,
    stage3_minimal_predicate_fragment_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage3_admit_slot_tests,
    stage3_predicate_fragment_tests,
    stage3_multiline_and_suite_tests_reusing_fragment_parser
  ],
  retained_later_refs = [
    shared_single_attachment_frame,
    request_clause_suite_publicization,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar,
    theorem_model_check_concrete_binding
  ]
}
```

#### 利点

- attached-slot / predicate route の first package を actual code anchor へ落とせる
- existing stage3 tests/support を crate parser surface reuse に寄せられる
- multiline attachment / request suite 自体は still support-local に留められる
- `perform head` / richer diagnostics を still later に残せる

#### 欠点

- `mir_ast::current_l2` surface は stage 3 へ少し広がる
- stage3 support helper との責務分離 wording を明示する必要がある

### 案 3. shared single attachment frame / request clause suite / perform head まで同時に library 化する

#### 利点

- parser-side line は大きく進んで見える

#### 欠点

- attached-slot / predicate route の narrow reopen を越えて widen しすぎる
- request head / clause suite / richer diagnostics bulk widen に近づく
- current sequencing guard と衝突する

## current judgment

current L2 で最も自然なのは、
**案 2. declaration-side admit attached slot と shared isolated predicate fragment だけを `mir_ast::current_l2` へ actualize する**
である。

理由は次の通り。

1. `specs/examples/305...306` で選んだ first package は attached-slot / predicate route であり、最小 actualization もそこに揃えるのが自然である。
2. stage3 admit-slot helper と predicate-fragment helper には既存 actual evidence があり、crate surface reuse へ寄せやすい。
3. multiline attachment / request clause suite tests は fragment parser reuse による compile/test gate を得つつ、support-local structural helper のまま retained-later に残せる。
4. `perform head` / request suite publicization / richer diagnostics / final grammar を premature に library 化しなくて済む。

## current first choice shape

```text
phase6_parser_second_tranche_first_package = {
  carrier_kind = current_l2_nonproduction_parser_second_tranche_carrier,
  accepted_surface_refs = [
    stage3_decl_admit_slot_surface,
    stage3_minimal_predicate_fragment_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage3_admit_slot_tests,
    stage3_predicate_fragment_tests,
    stage3_multiline_and_suite_tests_reusing_fragment_parser
  ],
  retained_later_refs = [
    shared_single_attachment_frame,
    request_clause_suite_publicization,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar,
    theorem_model_check_concrete_binding
  ]
}
```

## practical reading

current parser second tranche first package が示すのは、次の 5 点だけである。

1. `mir_ast::current_l2` に declaration-side admit attached slot parser を足す
2. `mir_ast::current_l2` に shared isolated predicate fragment parser を足す
3. stage3 admit-slot / predicate-fragment tests は crate import に寄せる
4. multiline attachment / request clause suite は support-local helper のまま retained-later structural family に留める
5. perform head / richer diagnostics / final grammar / formal tool binding は still later に残す

## next promoted line

next promoted line は、
**phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready phase6-reserve-formal-tool-binding-inventory comparison**
に置く。

## open questions

- shared single attachment frame を next parser-side package に置くか
- request clause suite publicization を crate surface へどの timing で上げるか
- theorem-first reserve line を notebook-pressure wording までどう mirror するか
