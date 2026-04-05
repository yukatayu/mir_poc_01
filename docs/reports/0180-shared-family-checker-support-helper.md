# Report 0180 — shared family checker support helper

- Date: 2026-04-06T01:34:00+09:00
- Author / agent: Codex
- Scope: same-lineage / missing-option / capability の 3 checker spike が共有する compare contract を non-production support helper へ薄く寄せ、family facade script と detached loop wrapper command 名を維持する
- Decision levels touched: L2

## 1. Objective

- 3 family checker spike の duplicated core contract を 1 箇所へ寄せる
- family facade script と `smoke-*` wrapper command 名は維持する
- generic checker-side shared CLI や public checker API へは進まない

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_same_lineage_checker.py`
- `scripts/tests/test_current_l2_missing_option_checker.py`
- `scripts/tests/test_current_l2_capability_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`

## 3. Actions taken

- `specs/examples/49-current-l2-shared-family-checker-support-helper.md` を追加し、family-specific facade 維持 + shared support helper 導入の current judgment を固定した
- `scripts/current_l2_family_checker_support.py` を追加し、parser / filter / status / stdout rendering の共通 contract を実装した
- 3 family facade script を shared support helper 呼び出しへ薄く寄せた
- `scripts/tests/test_current_l2_family_checker_support.py` を追加し、generic contract を test-first で固定した
- docs / plan / progress / traceability を shared support helper actualization に追従させた

## 4. Files changed

- 追加:
  - `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
  - `scripts/current_l2_family_checker_support.py`
  - `scripts/tests/test_current_l2_family_checker_support.py`
  - `docs/reports/0180-shared-family-checker-support-helper.md`
  - `docs/reports/0181-review-shared-family-checker-support-helper.md`
- 変更:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/90-source-traceability.md`
  - `progress.md`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support`
  - 初回: `ModuleNotFoundError: No module named 'current_l2_family_checker_support'`
- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_capability_checker`
  - `Ran 12 tests in 0.012s`
  - `OK`
- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 20 tests in 0.017s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py smoke-same-lineage-checker crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --run-label shared-family-smoke --overwrite`
  - `status: matched`
- `python3 scripts/current_l2_detached_loop.py smoke-missing-option-checker crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json --run-label shared-family-smoke --overwrite`
  - `status: matched`
- `python3 scripts/current_l2_detached_loop.py smoke-capability-checker crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json --run-label shared-family-smoke --overwrite`
  - `status: matched`
- `cargo test -p mir-semantics`
  - `test result: ok. 44 passed; 0 failed`（integration）
  - `test result: ok. 8 passed; 0 failed`（static gate support）
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 181 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

- 3 family checker spike は cluster 名、kind set、missing status 名だけが family-specific であり、parser / filter / status / stdout contract は narrow に共通化できる
- shared support helper を `scripts/` 内の non-production module に留めれば、facade script と detached loop wrapper command 名をそのまま残せる
- generic checker-side shared CLI を切らなくても duplicated contract の drift source は減らせる

## 7. Changes in understanding

- current phase では「generic shared entry を切る」より「facade 維持 + shared support helper」から始める方が自然である
- same-lineage / missing-option / capability の 3 spike は current detached validation loop の family-specific readability を保ったまま refactor できる
- 次の narrow question は、generic checker-side shared family compare entry が本当に要るか、それとも facade 維持で十分かである

## 8. Open questions

- generic checker-side shared family compare entry を別 helper として切るべきか
- family facade script をどこまで長く残すべきか
- final public checker API をどこへ置くか

## 9. Suggested next prompt

`same-lineage / missing-option / capability の 3 family facade を維持した current cut の上で、generic checker-side shared family compare entry を追加する価値があるかを narrow に比較してください。`
