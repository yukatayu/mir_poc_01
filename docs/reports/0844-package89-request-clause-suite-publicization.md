# 0844 — Package 89 request-clause-suite publicization

## Objective

Package 89 として、
request-local `require` / `ensure` fixed two-slot suite bridge を
test-support only 実装のままにせず、
`mir_ast::current_l2` の crate-local non-production parser carrier に
narrow に actualize する。

## Scope and assumptions

- fixed するのは request clause suite publicization minimum だけである
- perform head final public parser API、span-rich diagnostics、final grammar、
  final public parser / checker / runtime surface は still later に残す
- shared single attachment frame と predicate fragment parser reuse は preserve する
- source-backed floor は `specs/examples/99`、`100`、`101`、`560`、`561` に従う

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
- `specs/examples/560-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold-helper-mirror.md`
- `specs/examples/561-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold-helper-mirror.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

## Actions taken

1. request clause suite line の source-backed floor を再読し、
   helper-local compare floor から crate-local non-production carrier へ上げても
   perform head / diagnostics / final grammar を premature に混ぜない cut を確認した。
2. RED として、
   `current_l2_request_clause_suite_manifest()` を参照する新 test を追加し、
   crate surface に request clause suite carrier が未実装であることを確認した。
3. `crates/mir-ast/src/current_l2.rs` に
   - `CurrentL2RequestClauseSuiteManifest`
   - `current_l2_request_clause_suite_manifest()`
   - `Stage3RequestClauseSuite`
   - `parse_stage3_request_clause_suite_text()`
   を追加した。
4. request clause suite spike test を
   support-local duplicate helper ではなく crate-local carrier を直接使う形へ切り替えた。
5. 使われなくなった
   `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`
   を削除した。
6. Package 89 closeout と next queue を反映する docs / plan / snapshot 更新を行った。

## Evidence / outputs / test results

- red
  - `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact`
  - failure:
    unresolved import `current_l2_request_clause_suite_manifest`
- green
  - `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact`
  - `cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike`
- validation rerun
  - `cargo fmt --all`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- request clause suite line は compare-floor のまま維持するより、
  crate-local non-production carrier に narrow に上げた方が drift が少ない。
- shared single attachment frame と fixed two-slot suite bridge は別 package だが、
  current implementation では
  multiline extraction reuse と predicate fragment parser reuse を
  そのまま保てる。
- Package 89 close 後の current next line は
  perform head final public parser API comparison であり、
  request clause suite 自体をもう compare debt に戻す必要は薄い。
- parser-side closeout package でも、request clause suite carrier と perform head carrier を分けて ratchet した方が
  final grammar / diagnostics / public API を premature に固定しにくい。

## Open questions

- perform head final public parser API をどこまで non-production parser carrier に上げるか
- span-rich diagnostics を structural carrier とどこで分離するか
- final grammar をどの stop line まで deferred に保つか

## Suggested next prompt

Package 90 として、perform head final public parser API comparison を
request clause suite publicization minimum の直後の next parser-side package として進め、
diagnostics / final grammar を premature に混ぜずに current first compare floor を置いてください。
