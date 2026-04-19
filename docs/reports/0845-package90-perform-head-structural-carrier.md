# 0845 — Package 90 perform-head structural carrier

## Objective

Package 90 として、
request clause suite bridge を current entry criteria に保ったまま、
`perform` head の owner / op / target-or-via shape を
`mir_ast::current_l2` の crate-local non-production parser carrier に
narrow に actualize する。

## Scope and assumptions

- fixed するのは perform head structural carrier minimum だけである
- request clause suite bundle attachment、span-rich diagnostics、final grammar、
  final public parser / checker / runtime surface は still later に残す
- request clause suite publicization minimum は prerequisite / entry criteria として preserve する
- source-backed floor は `specs/examples/299`、`305`、`313`、`562` に従う

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
- `specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
- `specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
- `specs/examples/562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`

## Actions taken

1. perform head line の source-backed floor を再読し、
   request clause suite carrier と diagnostics / final grammar を混ぜない narrow cut を確認した。
2. RED として、
   `current_l2_perform_head_manifest()` と `parse_stage3_perform_head_text()` を参照する新 test を追加し、
   crate surface に perform head carrier が未実装であることを確認した。
3. `crates/mir-ast/src/current_l2.rs` に
   - `CurrentL2PerformHeadManifest`
   - `current_l2_perform_head_manifest()`
   - `Stage3PerformTargetRef`
   - `Stage3PerformHead`
   - `parse_stage3_perform_head_text()`
   を追加した。
4. fixture-backed support helper に `load_fixture_perform_head()` を追加し、
   `PerformOn` / `PerformVia` subset と actual carrier を直接比較できるようにした。
5. Package 90 closeout と next queue を反映する docs / plan / snapshot 更新を行った。

## Evidence / outputs / test results

- red
  - `cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact`
  - failure:
    unresolved import `current_l2_perform_head_manifest`
- red
  - `cargo test -p mir-ast --test current_l2_stage3_perform_head_spike`
  - failure:
    unresolved imports `Stage3PerformHead`、`Stage3PerformTargetRef`、`parse_stage3_perform_head_text`
- green
  - `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact`
  - `cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike`
  - `cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact`
  - `cargo test -p mir-ast --test current_l2_stage3_perform_head_spike`
- validation rerun
  - `cargo fmt --all`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- perform head line も compare-floor のまま引き延ばすより、
  owner / op / target-or-via の structural minimum だけを先に crate-local carrier へ上げた方が drift が少ない。
- request clause suite carrier と perform head carrier を分けて ratchet すると、
  bundle attachment と diagnostics / final grammar を premature に固定しにくい。
- Package 90 close 後の current next line は
  perform head と request clause suite の bundle attachment comparison であり、
  final public parser API comparison へ一足飛びに進む必要は薄い。

## Open questions

- perform head と request clause suite をどの combined carrier で narrow に束ねるか
- span-rich diagnostics を structural bundle とどこで分離するか
- final grammar をどの stop line まで deferred に保つか

## Suggested next prompt

Package 91 として、perform head と request clause suite の bundle attachment comparison を
current next parser-side package として進め、
combined carrier の minimum だけを選び、diagnostics / final grammar を premature に混ぜないでください。
