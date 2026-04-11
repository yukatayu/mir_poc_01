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

### 3. checker / runtime first tranche の cut

`specs/examples/301...302` により、Phase 6 front-half actual checker / runtime first tranche の current first choice は、

- `mir-semantics` に `static_gate_program_detailed`、`DirectStyleEvaluator::from_program`、`FixtureHostStub::run_program` を足す
- `mir-runtime` に `mir_runtime::current_l2` thin orchestrator を足す
- parser side evidence は stage 1 reconnect summary と stage 2 try/rollback structural summary の optional bridge input に留める
- bridge input と semantic `Program` が食い違うときは fail-closed に止める

というものに固定した。

### 4. compile-ready verification / formal hook の cut

`specs/examples/303...304` により、Phase 6 front-half compile-ready verification / formal hook の current first choice は、

- selected gate を `cargo test -p mir-ast`、`cargo test -p mir-runtime`、`cargo test -p mir-semantics --test current_l2_*` selected suite、Python detached-loop suiteに留める
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs` と support helper で tool-neutral emitted formal hook を actualize する
- emitted hook の envelope / row core は Phase 5 theorem-line existing cut と同じ `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` に揃える
- source artifact の `schema_version` / `artifact_kind` mismatch は fail-closed に止める

というものに固定した。

## まだ fixed していないこと

- stage 3 request / predicate reconnect
- concrete theorem / model-check tool binding
- perform head final public parser API
- span-rich diagnostics と final grammar

## この phase の evidence

- `specs/examples/287...290`
- `specs/examples/299...300`
- `specs/examples/301...302`
- `specs/examples/303...304`
- `docs/reports/0611-phase6-actual-parser-ast-carrier-first-tranche-package.md`
- `docs/reports/0612-phase6-actual-checker-runtime-skeleton-first-tranche-package.md`
- `docs/reports/0613-phase6-compile-ready-verification-and-formal-hook-package.md`
- `docs/reports/0614-phase6-compile-ready-checkpoint-drift-suppression-and-mirror-sweep.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `cargo test -p mir-ast`
- `cargo test -p mir-runtime`
- `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop`

## 次の mainline

`specs/examples/305...306` により、
compile-ready checkpoint close 後の next reopen sequencing は parser second tranche first に fixed 済みである。

current mainline は、
**Phase 6 parser second tranche attached-slot / predicate-fragment first package**
である。

ここでは fixed entry criteria を崩さずに、

- admit-slot / predicate route の最小 code anchor
- theorem-first / model-check concrete binding の reserve wording
- parser-side follow-up package の stop line

を揃えるのが主眼になる。
