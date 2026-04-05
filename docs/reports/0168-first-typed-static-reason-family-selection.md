# Report 0168 — first typed static reason family selection

- Date: 2026-04-05T14:55:00Z
- Author / agent: Codex
- Scope: `expected_static.checked_reason_codes` の first family 実装から続く stable tranche を actual corpus / machine-check / readiness scan / docs mirror まで揃え、current stable cluster 8 kind を source-backed に actualize する
- Decision levels touched: L2

## 1. Objective

lineage edge pair family の first actualization を起点に、

- second tranche の declared target edge pair family
- remaining stable tranche の capability / missing-option family

を `expected_static.checked_reason_codes` に narrow に広げ、
current stable cluster inventory 8 kind が fixture corpus / `run_bundle()` / detached readiness scan で一致する状態まで進める。

## 2. Scope and assumptions

- current L2 semantics、failure family、parser grammar、machine-check policy は変えない。
- `checked_reason_codes` は fixture-side additive optional carrier に留める。
- `detached_noncore.reason_codes` は helper-local / reference-only mirror のままにする。
- duplicate declaration cluster (`e14` / `e15`) は current cut では `checked_reason_codes` に昇格させない。
- `checked_reasons` explanatory bridge は current corpus で維持し、今回の task では coexistence policy そのものは最終化しない。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
- `specs/examples/41-current-l2-first-typed-static-reason-family-carrier-cut.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `scripts/current_l2_reason_codes_assist.py`
- `scripts/current_l2_reason_code_readiness.py`
- `scripts/tests/test_current_l2_reason_codes_assist.py`
- `scripts/tests/test_current_l2_reason_code_readiness.py`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. current code anchor を再読し、`StaticReasonCodeRow` enum と `checked_reason_codes` compare bridge がすでに lineage first tranche まで入っていることを確認した。
2. `e12` / `e19` に declared target edge pair family の `checked_reason_codes` row を追加し、`run_bundle_accepts_matching_checked_reason_codes_for_target_pair_family` を RED から追加した。
3. `is_supported_checked_reason_code()` を lineage-only から stable cluster 8 kind 全体へ広げ、target pair family の GREEN を取った。
4. `e13` / `e16` / `e17` / `e18` に capability / missing-option family の `checked_reason_codes` row を追加し、remaining stable tranche 用 targeted test を RED から追加した。
5. `run_bundle_accepts_matching_checked_reason_codes_for_remaining_stable_families` と corpus expectation test を通し、stable cluster 8 kind の actual carrier support を GREEN にした。
6. `scripts/current_l2_reason_codes_assist.py`、`scripts/current_l2_reason_code_readiness.py`、その tests を更新し、`checked_reason_codes` adoption 8 / suggestion 8 / duplicate 2 absent の readiness summary が current state を反映するよう揃えた。
7. `specs/examples/42...`、`43...` を追加し、second tranche actualization と remaining stable tranche completion の docs-only judgment を明記した。
8. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を mirror 更新した。

## 5. Files changed

- 追加:
  - `specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
  - `specs/examples/41-current-l2-first-typed-static-reason-family-carrier-cut.md`
  - `specs/examples/42-current-l2-second-typed-static-reason-family-actualization.md`
  - `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md`
  - `docs/reports/0168-first-typed-static-reason-family-selection.md`
- 更新:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
  - `scripts/current_l2_reason_codes_assist.py`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_codes_assist.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/90-source-traceability.md`
  - `progress.md`
  - current fixture corpus:
    - `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`

## 6. Commands run

- `cargo test -p mir-semantics run_bundle_accepts_matching_checked_reason_codes_for_target_pair_family -- --exact`
- `cargo test -p mir-semantics run_bundle_accepts_matching_checked_reason_codes_for_remaining_stable_families -- --exact`
- `cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact`
- `python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label full-stable-readiness --overwrite`
- `python3 -m unittest scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_reason_code_readiness scripts.tests.test_current_l2_static_gate_loop`
- `cargo test -p mir-semantics`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 7. Evidence / outputs / test results

- RED 1:
  - `run_bundle_accepts_matching_checked_reason_codes_for_target_pair_family` は当初
    `bundle static checked_reason_codes kind not yet supported for e12_underdeclared_target_missing`
    で失敗した。
- GREEN 1:
  - `is_supported_checked_reason_code()` を expanded した後、target pair family targeted test は pass した。
- RED 2:
  - `run_bundle_accepts_matching_checked_reason_codes_for_remaining_stable_families` は当初
    `CapabilityStrengthens { from_capability: "read", to_capability: "write" }`
    unsupported で失敗した。
- GREEN 2:
  - capability / missing-option family support 追加後、remaining stable tranche targeted test は pass した。
- final verification:
  - `python3 -m unittest ...` は `Ran 12 tests in 0.021s` / `OK`
  - `cargo test -p mir-semantics` は unit 2、support integration 2 + 2 + 8、main integration 44、doc-tests 0 が pass
  - `python3 scripts/validate_docs.py` は `Documentation scaffold looks complete.` / `Found 168 numbered report(s).`
  - `git diff --check` は無出力
- real readiness smoke:
  - `fixtures with checked_reasons: 8`
  - `fixtures with reason_codes suggestions: 8`
  - `fixtures with checked_reason_codes: 8`
  - kinds は
    `missing_lineage_assertion`、
    `lineage_assertion_edge_mismatch`、
    `declared_target_missing`、
    `declared_target_mismatch`、
    `capability_strengthens`、
    `missing_chain_head_option`、
    `missing_predecessor_option`、
    `missing_successor_option`
  - `e14` / `e15` は引き続き no-suggestion / no-adoption

## 8. What changed in understanding

- first typed family selection は docs-only 比較で終わらず、current stable cluster inventory 8 kind を additive に `checked_reason_codes` へ actualize しても helper boundary は壊れないところまで前進できた。
- current open issue は「次にどの family を actualize するか」ではなく、`checked_reasons` と `checked_reason_codes` をどこまで併存させ、いつ shrink するかになった。
- duplicate declaration cluster は current repo では stable typed carrier に上げない方が自然であり、`checked_reason_codes` actualization 完了後も `checker_core.reasons` / focused smoke / explanation 側に残すのが妥当である。

## 9. Open questions

- `checked_reasons` と `checked_reason_codes` の coexistence / shrink policy をどう narrow に決めるか
- detached static gate artifact の `reason_codes` mirror を helper-local / reference-only のままに保つための guardrail をどこまで強めるか
- duplicate declaration cluster を長期的に typed 化しないままにするか、それとも別 family / explanation-only cluster として明示するか

## 10. Suggested next prompt

`checked_reasons` と `checked_reason_codes` が current stable cluster 8 kind で併存している現状を前提に、coexistence / shrink / deprecation policy を docs-first で narrow に比較し、必要なら assist / readiness helper / fixture authoring template の最小更新まで進めてください。
