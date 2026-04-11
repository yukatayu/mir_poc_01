# 299 — current L2 phase5-proof-protocol-runtime-policy-handoff-closeout-ready phase6-actual-parser-ast-carrier-first-tranche comparison

## 目的

`specs/examples/297-current-l2-phase4-shared-space-self-driven-closeout-ready-phase5-proof-protocol-runtime-policy-handoff-closeout-comparison.md`
と
`specs/examples/298-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-minimal-phase5-proof-protocol-runtime-policy-handoff-closeout-threshold.md`
で Phase 5 closeout を fixed した次段として、

- Phase 6 front-half の actual parser / AST carrier first tranche をどの cut で actualize するか
- Phase 3 freeze で固定した stage 1 / stage 2 structural floor をどこまで `mir-ast` crate 本体へ持ち上げるか
- stage 3 request / predicate cluster、span-rich diagnostics、final grammar をどこまで retained-later に残すか

を比較する。

ここで固定するのは
**current L2 phase5-proof-protocol-runtime-policy-handoff-closeout-ready phase6-actual-parser-ast-carrier-first-tranche comparison**
であり、

- stage 3 admit / request clause / predicate fragment の library actualization
- perform head の final public parser API
- span-rich diagnostics
- final grammar

はまだ固定しない。

## scope

- Phase 6 front-half の first tranche として、`mir-ast` non-production carrier だけを扱う。
- accepted floor source は `specs/examples/73`、`79`、`287...288` とする。
- reconnect bridge 自体の closeout wording は `specs/examples/289...290` を前提とするが、本 task では checker/runtime bridge actualization へは進まない。
- code anchor は `crates/mir-ast/src/current_l2.rs` と `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` / `current_l2_stage2_try_rollback_spike.rs` を主に扱う。

## current 前提

current repo では次が成立している。

1. `specs/examples/287...288` により、Phase 6 front-half parser gate の accepted cluster は stage 1 + stage 2 structural floor に fixed 済みである。
2. `specs/examples/289...290` により、first checker reconnect bridge は stage 1 summary + stage 2 try/rollback structural contract に fixed 済みである。
3. `mir-semantics` には parser-free current L2 interpreter がある一方、`mir-ast/src/lib.rs` は placeholder skeleton に留まり、actual parser evidence は stage 1 / stage 2 integration test support helper に閉じている。

したがって current 問いは、
**fixed 済みの parser freeze を壊さずに、どの carrier cut なら `mir-ast` first tranche を compile-ready と読めるか**
である。

## 比較観点

1. stage 1 / stage 2 freeze を code anchor へ自然に持ち上げられるか
2. stage 3 spillover を public-ish contract に混ぜずに済むか
3. `mir-ast` を final parser API へ premature に誤読させないか
4. next line を Phase 6 actual checker / runtime skeleton first tranche へ自然に送れるか

## 比較対象

### 案 1. parser spike を tests/support に留め、crate 本体は placeholder のまま維持する

#### 利点

- library surface は増えない
- stage 3 retained-later line を accidental に library 化しない

#### 欠点

- Phase 6 front-half actual parser first tranche を code anchor として閉じられない
- compile-ready checkpoint の parser carrier が tests/support private helper のままになる
- next line の checker/runtime actualization が library entry を欠いたままになる

### 案 2. stage 1 / stage 2 structural floor だけを `mir-ast` non-production carrier として昇格する

#### shape

```text
mir_ast_current_l2_first_tranche = {
  module_ref = mir_ast::current_l2,
  accepted_surface_refs = [
    stage1_option_decl_chain_surface,
    stage2_try_fallback_structural_surface
  ],
  retained_later_refs = [
    stage3_admit_slot_surface,
    stage3_request_clause_suite,
    stage3_predicate_fragment,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar
  ]
}
```

#### 利点

- phase freeze で accepted floor に置いた stage 1 / stage 2 を code anchor に持ち上げられる
- existing stage 1 / stage 2 spike tests を crate API import へ寄せられる
- stage 3 helper を retained-later evidence として切り分け続けられる
- next line を checker/runtime bridge actualization に送れる

#### 欠点

- `mir-ast` は placeholder ではなくなる
- final parser API と誤読されないよう non-production wording を明示する必要がある

### 案 3. stage 3 request / predicate cluster や richer diagnostics まで同時に昇格する

#### 利点

- parser surface は大きく見える

#### 欠点

- `specs/examples/287...288` の minimal parser freeze を超えて widen しやすい
- stage 3 retained-later line が Phase 6 first tranche に逆流する
- checker/runtime bridge より前に parser contract を重くしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. stage 1 / stage 2 structural floor だけを `mir-ast` non-production carrier として昇格する**
である。

理由は次の通り。

1. Phase 3 / 6 docs で fixed 済みなのは stage 1 / stage 2 accepted floor であり、actual parser first tranche に必要なのもそこまでである。
2. stage 3 helper は evidence としては厚いが、Phase 6 first tranche の public-ish contract へ上げるにはまだ重い。
3. `mir-ast` crate 本体へ narrow carrier を持ち上げれば、placeholder から先へ進めつつ final parser API を still later に保てる。
4. next mainline を `mir-semantics` / `mir-runtime` checker/runtime skeleton first tranche へ送れる。

## current first choice shape

```text
mir_ast_current_l2_first_tranche = {
  module_ref = mir_ast::current_l2,
  accepted_surface_refs = [
    stage1_option_decl_chain_surface,
    stage2_try_fallback_structural_surface
  ],
  retained_later_refs = [
    stage3_admit_slot_surface,
    stage3_request_clause_suite,
    stage3_predicate_fragment,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar
  ]
}
```

## practical reading

current Phase 6 parser first tranche が示すのは、次の 4 点だけである。

1. `mir-ast` crate 本体に stage 1 option/chain surface と stage 2 try/fallback structural surface を non-production carrier として置く
2. stage 1 / stage 2 spike tests は crate API import に寄せる
3. stage 3 request / predicate cluster は引き続き tests/support retained-later evidence に留める
4. final parser grammar、richer diagnostics、final public parser API は still later に残す

## next promoted line

next promoted line は、
**phase6-actual-parser-ast-carrier-first-tranche-ready phase6-actual-checker-runtime-skeleton-first-tranche comparison**
に置く。

## open questions

- perform head の first actual parse を checker/runtime first tranche に含めるか
- stage 3 request / predicate reconnect を compile-ready checkpoint 後の second tranche に切り出すか
- parser carrier と final public parser API の naming をどの時点で分けるか
