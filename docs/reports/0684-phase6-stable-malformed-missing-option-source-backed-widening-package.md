# Report 0684 — Phase 6 stable malformed missing-option source-backed widening package

- Date: 2026-04-13T10:15:43Z
- Author / agent: Codex
- Scope: final public parser/checker/runtime later-gate actualization comparison close 後の次段として、missing-option family の source-backed widening first cut を `e16/e18` actual row と `e17` staged guard に絞って actualize し、repo-level current line を public operational CLI second later gate へ進める。
- Decision levels touched: L2

## 1. Objective

- `e16/e17/e18` triplet family judgmentを保ったまま、current source surfaceで素直に actualize できる最小 widening cut を固定する。
- `e16` / `e18` を source-authored static-stop pair として sample / runner / ladder / regression helper に接続する。
- `e17` は silent drop せず staged same-family guard として残し、next line を public operational CLI second later gate に送る。

## 2. Scope and assumptions

- current package は missing-option family の first source-backed widening actualization と snapshot 同期に留める。
- `run_current_l2_source_sample` の public/report shape や runtime-led thin facade first later cut は巻き戻さない。
- `e17` source-authored actualization、capability second reopen、duplicate cluster / `TryFallback` / `AtomicCut` malformed-static broader promotion は still later に残す。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/369...370`
- `specs/examples/371...372`
- `specs/examples/387...388`
- `specs/examples/389...390`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`

## 4. Actions taken

- `specs/examples/391...392` を追加し、missing-option family の current first source-backed widening cut を `e16/e18` actual row + `e17` staged guard に固定した。
- `samples/current-l2/` に `e16-malformed-missing-chain-head-option.txt` と `e18-malformed-missing-successor-option.txt` を追加した。
- `mir-runtime` current L2 accepted sample set と source lowerer / runner / verification ladder tests を `e16/e18` まで widen した。
- `scripts/current_l2_source_sample_regression.py` inventory / regression bundle とその unit tests を decet + static smoke widening に追随させた。
- `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、Phase 6 abstract、FAQ、sample docs、`.docs` を current snapshot に同期し、repo-level current line を `public operational CLI second later gate actualization comparison` に進めた。

## 5. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0684-phase6-stable-malformed-missing-option-source-backed-widening-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `samples/current-l2/e16-malformed-missing-chain-head-option.txt`
- `samples/current-l2/e18-malformed-missing-successor-option.txt`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `specs/00-document-map.md`
- `specs/examples/391-current-l2-final-public-parser-checker-runtime-first-later-gate-actualization-ready-stable-malformed-missing-option-first-source-backed-widening-actualization-comparison.md`
- `specs/examples/392-current-l2-stable-malformed-missing-option-first-source-backed-widening-actualization-ready-minimal-stable-malformed-missing-option-first-source-backed-widening-threshold.md`
- `tasks.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`

## 6. Evidence / outputs / test results

- `cargo test -p mir-runtime --test current_l2_source_lowering`
  - `10 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `12 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder`
  - `10 passed`
- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `Ran 13 tests`
  - `OK`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - current L2 fixed-subset authored inventory は authored decet を列挙し、`e16` / `e18` を `fixture_static_cluster` static-stop row として含めた。
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label pkg0684 --artifact-root target/current-l2-source-sample-regression-pkg0684`
  - current decet の lowering / runner / ladder bundle と static formal-hook smoke widening が通った。
- `date -u '+%Y-%m-%dT%H:%M:%SZ'`
  - `2026-04-13T10:15:43Z`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-13 19:15 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 683 numbered report(s).`
- `git diff --check`
  - 出力なし

## 7. What changed in understanding

- missing-option family の first source-backed widening は `e16/e18` static-stop pair までで十分に sample-facing evidence を厚くでき、`e17` のために current parser bridge を歪める必要はない。
- current source lowerer では predecessor が edge progression から導かれるため、missing predecessor row は same-family staged guard として明示的に残す方が drift が少ない。
- malformed-side widening を current line で閉じることで、repo-level mainline は Macro 7 public operational CLI second gate へ clean に handoff できる。

## 8. Open questions

- `e17` を source-authored row に上げるための最小 bridge cut をどこで reopen するか。
- capability second reopen を CLI second gate / thin-facade later support のどこに差し込むか。
- final public parser/checker/runtime thin-facade later support を public operational CLI second gate の前後どちらで閉じるか。

## 9. Suggested next prompt

- `tasks.md` の current line どおり、public operational CLI second later gate actualization comparison を進めてください。runtime-led thin facade first later cut を巻き戻さず、public operational CLI と repo-local Python helper / example surface の境界を narrow に整理してください。
