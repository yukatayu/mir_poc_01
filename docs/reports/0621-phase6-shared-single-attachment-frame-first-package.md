# Report 0621 — phase6 shared single attachment frame first package

- Date: 2026-04-12T02:12:20.117631Z
- Author / agent: Codex
- Scope: Phase 6 shared single attachment frame first package actualization、parser-side code promotion、snapshot / roadmap / traceability mirror 更新
- Decision levels touched: L1 / L2

## 1. Objective

- `tasks.md` 先頭 task だった Phase 6 parser-side follow-up package actualization を、shared single attachment frame first package として source-backed に閉じる。
- multiline extraction bridge を `mir_ast::current_l2` へ最小昇格し、request clause suite / perform head / richer diagnostics を still later に残す。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、research abstract、traceability mirror を source-sample corpus scope / file layout mainline へ切り替える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md`
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/311-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-phase6-parser-side-follow-up-package-sequencing-comparison.md`
- `specs/examples/312-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-minimal-phase6-parser-side-follow-up-package-sequencing-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`

## 3. Actions taken

- TDD で multiline attachment spike test を helper-local import から crate import へ切り替え、crate surface に extraction helper が無い red を先に確認した。
- `crates/mir-ast/src/current_l2.rs` に
  - option-local `admit:` multiline extraction
  - request-local `require:` / `ensure:` multiline extraction
  - immediate-child header search
  - blank-line fail-closed
  - dedented fragment text extraction
  の narrow bridge を追加した。
- multiline attachment spike test を crate import 前提で green に戻し、obsolete になった support helper を削除した。
- `crates/mir-ast/src/lib.rs` の crate-level note を更新し、shared single attachment frame extraction bridge が current carrier に入ったことを mirror した。
- `specs/examples/313...314` を追加し、shared single attachment frame first package の comparison / threshold を fixed した。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、Phase 6 abstract を更新し、current mainline を `fixed-subset source-sample corpus scope / file layout` に切り替えた。

## 4. Files changed

- `Documentation.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
- `docs/reports/0621-phase6-shared-single-attachment-frame-first-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
- `specs/examples/314-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug phase6-shared-single-attachment-frame-first-package`
  - `docs/reports/0621-phase6-shared-single-attachment-frame-first-package.md`
- `cargo test -p mir-ast --test current_l2_stage3_multiline_attachment_spike`
  - red:
    - `error[E0432]: unresolved imports 'mir_ast::current_l2::extract_stage3_option_admit_multiline_fragment_text', 'mir_ast::current_l2::extract_stage3_request_clause_multiline_fragment_text'`
  - green:
    - `running 8 tests`
    - `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-ast`
  - `test result: ok.`
  - stage 1 15 passed / stage 2 4 passed / stage3 admit 6 passed / stage3 multiline 8 passed / predicate fragment 7 passed / request clause suite 11 passed
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 620 numbered report(s).`
- `git diff --check`
  - no output
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-12 11:17 JST`

## 6. Evidence / findings

- shared single attachment frame first package は、multiline extraction bridge だけを crate surface へ上げる cut で十分閉じられる。
- actualized helper は existing predicate fragment parser reuse に留まり、request clause suite / perform head / richer diagnostics を巻き込んでいない。
- support helper 削除後も multiline attachment spike 8 tests は green であり、crate surface reuse への移行は source-backed に確認できた。
- request clause suite support helper は依然として later layer の duplicated extraction logic を持つが、current package の blocker ではない。

## 7. Changes in understanding

- shared single attachment frame first package の本質は「multiline attachment 全般を library 化すること」ではなく、「dedented fragment text extraction を existing predicate fragment parser へ橋渡しすること」にある。
- source-sample corpus scope / file layout は、この parser-side follow-up actualization が閉じて初めて current immediate line として自然に扱える。

## 8. Open questions

- request clause suite helper 側の duplicated extraction logic を later package でどう寄せるか
- shared single attachment frame extraction bridge を public parser API と誤読させない wording をどこまで残すか
- source-sample corpus の directory / extension / sample ID policy をどう固定するか

## 9. Suggested next prompt

- `tasks.md` 先頭 task である `fixed-subset source-sample corpus scope / file layout` を docs-first に閉じてください。repo-root corpus か crate-local corpus かを比較し、representative prose / fixture corpus / source sample の 3 層分離と sample ID policy を source-backed に固定してください。
