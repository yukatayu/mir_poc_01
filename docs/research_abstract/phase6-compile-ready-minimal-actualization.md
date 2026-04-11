# Phase 6 — compile-ready minimal actualization

## この phase の主眼

Phase 6 は、Phase 1〜5 で fixed した semantics / parser-free PoC / proof boundary を entry criteria としつつ、
`mir-ast` / `mir-semantics` / `mir-runtime` の actual code path を **non-production minimal cut** で compile-ready にする phase である。

この phase は production parser / runtime を確定する段階ではない。
主眼は、

- actual parser carrier
- checker / runtime skeleton bridge
- compile-ready gate と formal hook

を narrow に揃えることである。

## ここまでに fixed したこと

### 1. parser first tranche の cut

`specs/examples/299...300` により、Phase 6 front-half actual parser / AST carrier first tranche の current first choiceは、

- `mir-ast` crate 本体へ `mir_ast::current_l2` を追加する
- accepted floor を stage 1 option/chain と stage 2 try/fallback structural surface に留める
- stage 3 admit / request clause / predicate fragment、perform head final public API、span-rich diagnostics、final grammar は retained-later に残す

というものに固定した。

### 2. code anchor

parser first tranche の actual code anchor は次である。

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`

ここでは stage 1 / stage 2 spike tests を crate API import へ寄せ、TDD の red/green を通して narrow carrier を昇格した。

## まだ fixed していないこと

- `mir-semantics` / `mir-runtime` をまたぐ actual checker / runtime skeleton first tranche
- compile-ready checkpoint の selected cargo / smoke gate
- theorem / model-check first tranche の concrete tool binding
- stage 3 request / predicate reconnect
- perform head final public parser API
- span-rich diagnostics と final grammar

## この phase の evidence

- `specs/examples/287...290`
- `specs/examples/299...300`
- `docs/reports/0611-phase6-actual-parser-ast-carrier-first-tranche-package.md`
- `crates/mir-ast/src/current_l2.rs`
- `cargo test -p mir-ast`

## 次の mainline

next mainline は、
**Phase 6 front-half actual checker / runtime skeleton first tranche**
である。

ここでは parser carrier を前提に、

- `mir-semantics` の program-level checker / eval entry
- `mir-runtime` thin orchestrator
- parser-first-tranche と parser-free interpreter の boundary wording

を揃えるのが主眼になる。
