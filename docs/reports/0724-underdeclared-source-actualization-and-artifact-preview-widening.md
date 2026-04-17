# Report 0724 — underdeclared source actualization and artifact preview widening

- Date: 2026-04-17T04:41:50.962328Z
- Author / agent: Codex
- Scope: current L2 source-authored underdeclared omission family (`e5` / `e12`) actualization, helper-local verifier preview widening, snapshot synchronization
- Decision levels touched: L2 current design proposal, L3 helper-local convenience cut

## 1. Objective

fixture-side source of truth を already 持っていた underdeclared family のうち、

- lineage assertion omission
- declared target omission

を current source parser / lowerer convenience cut で受け、
`samples/current-l2/` authored corpus と helper-local `verification_preview` / `artifact_preview` 上で比較できる状態まで引き上げる。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/90-source-traceability.md`
- `docs/reports/0722-sample-verification-preview-and-prototype-second-tranche.md`
- `docs/reports/0723-sample-artifact-preview-third-tranche.md`

## 3. Actions taken

1. `samples/current-l2/` に underdeclared omission family を source-authored sample として追加した。
   - `e5-underdeclared-lineage.txt`
   - `e12-underdeclared-target-missing.txt`
2. `samples/not_implemented/current-l2-underdeclared/` の preservation files を削除し、current source convenience cut に actualize 済みであることへ読み替えた。
3. `crates/mir-ast/src/current_l2.rs` を widened し、
   - `fallback successor` の lineage assertion omission
   - `option name on capability read lease live` の declared target omission
   を current parser convenience cut で受けるようにした。
4. `crates/mir-runtime/src/current_l2.rs` を widened し、
   - accepted sample list
   - source lowerer
   - stage1 reconnect summary
   を `e5` / `e12` 対応へ更新した。
5. `crates/mir-runtime/src/current_l2_cli.rs` を widened し、`verification_preview` / `artifact_preview` で `underdeclared` static cluster を `fixture_static_cluster` route reached として表示するようにした。
6. runtime / AST / regression helper / sample docs を authored sixteen inventory へ同期した。
7. `specs/examples/454` を helper-local `artifact_preview` 専用 note に整理し直し、新規 `specs/examples/455` で underdeclared source actualization judgment を切り出した。
8. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`faq_005.md` を current snapshot に同期した。

## 4. Files changed

- Added:
  - `samples/current-l2/e5-underdeclared-lineage.txt`
  - `samples/current-l2/e12-underdeclared-target-missing.txt`
  - `specs/examples/455-current-l2-underdeclared-source-actualization-and-static-artifact-widening.md`
- Deleted:
  - `samples/not_implemented/current-l2-underdeclared/u01-missing-lineage-assertion.txt`
  - `samples/not_implemented/current-l2-underdeclared/u02-missing-declared-target.txt`
- Modified:
  - `crates/mir-ast/src/current_l2.rs`
  - `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
  - `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
  - `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
  - `crates/mir-runtime/src/current_l2.rs`
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  - `crates/mir-runtime/tests/current_l2_source_lowering.rs`
  - `crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs`
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  - `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
  - `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
  - `scripts/current_l2_source_sample_regression.py`
  - `scripts/tests/test_current_l2_source_sample_regression.py`
  - `samples/current-l2/README.md`
  - `samples/not_implemented/README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/08-representative-programs-and-fixtures.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/90-source-traceability.md`
  - `faq_005.md`
  - `specs/00-document-map.md`
  - `specs/examples/454-current-l2-artifact-preview-and-underdeclared-source-gap-note.md`

## 5. Commands run and exact outputs

- Resource check:
  - `df -h .`
    - free space は約 `17G`
  - `free -h`
    - low RAM / ample swap を確認
- RED / parser-side:
  - `cargo test -p mir-ast --test current_l2_stage1_parser_spike -- --nocapture`
    - `16 passed`
- GREEN / runtime-side:
  - `cargo test -p mir-runtime --test current_l2_source_lowering -- --nocapture`
    - `18 passed`
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner -- --nocapture`
    - `20 passed`
  - `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring -- --nocapture`
    - `9 passed`
  - `cargo test -p mir-runtime --test current_l2_operational_cli -- --nocapture`
    - `8 passed`
  - `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
    - `13 tests OK`
  - `python3 scripts/current_l2_source_sample_regression.py inventory`
    - authored sixteen inventory を表示
  - `python3 scripts/current_l2_source_sample_regression.py regression --run-label underdeclared-src --artifact-root target/current-l2-source-sample-regression-underdeclared`
    - `all regression commands passed`
    - new smoke commands:
      - `[11/20] static formal hook smoke for e5-underdeclared-lineage`
      - `[12/20] static formal hook smoke for e12-underdeclared-target-missing`
  - `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder -- --nocapture`
    - `16 passed`
  - `cargo test -p mir-ast --test current_l2_stage1_parser_spike -- --nocapture`
    - `18 passed`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 723 numbered report(s).`
  - `git diff --check`
    - no output
  - focused reviewer pass + narrow re-review
    - first pass found 4 issues
    - follow-up fixes: ladder table drift, prototype tranche wording drift, `plan/90` stale pointers, AST fixture-parity coverage for `e5/e12`

## 6. Evidence / findings

- `e5` は source-authored omission form でも static gate verdict `underdeclared` へ落ちる。
- `e12` も source-authored omission form で same underdeclared route へ落ちる。
- helper-local `verification_preview` / `artifact_preview` では、underdeclared static rows を `fixture_static_cluster` reached として表示できる。
- current first cut では、underdeclared static cluster の obligation family は malformed static cluster と同じ
  - `canonical_normalization_law`
  - `no_re_promotion`
  に揃えてよい。
- omission form widening は current parser / lowerer convenience cut であり、final grammar adoption ではない。

## 7. Changes in understanding

- underdeclared family は source-form omission を helper-local comparison corridor に十分引き上げられる。
- previous note で保持していた `samples/not_implemented/current-l2-underdeclared/` は current snapshot では stale になった。
- `specs/examples/454` に underdeclared gap を混ぜるより、
  - `454` = helper-local `artifact_preview` cut
  - `455` = underdeclared source actualization judgment
  と分けた方が reader が current fact と former gap を誤読しにくい。
- reader-facing sample table と traceability memory は、actualization package のたびに reviewer 観点で stale wording を洗う必要がある。

## 8. Open questions

- omission form convenience cut を final public syntax に昇格させるかは未決である。
- underdeclared dedicated artifact schema を final public contract として切るかは未決である。
- stronger typed surface / theorem discharge / model-check property language との接続は mixed gate に残る。

## 9. Suggested next prompt

`samples/prototype/` で typed/theorem/model-check sample-visible corrected prototype tranche を進め、helper-local verification/artifact preview を使って current bridge floor の usability を比較してください。
