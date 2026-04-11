# Report 0606 — phase3 parser-to-checker reconnect freeze package

- Date: 2026-04-11 21:12 JST
- Author / agent: Codex
- Scope: Phase 3 reopen continuation through parser-to-checker reconnect freeze package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the ordered mainline after the minimal parser subset freeze.
- Determine what the minimal parser-to-checker reconnect bridge should be for Phase 6 front-half planning.
- Update the snapshots so the next promoted line moves from reconnect freeze to Phase 1 semantics / invariants / notation closeout.

## 2. Scope and assumptions

- Keep the staged parser order from `specs/examples/73...`.
- Keep stage 1 / stage 2 / stage 3 helper implementations private / test-only.
- Treat stage 1 summary and stage 2 structural floor as the only current bridge candidates.
- Leave stage 3 request / predicate reconnect, `e19` redesign, `E21` / `E22` runtime contrast outside the current freeze.
- `plan/13-heavy-future-workstreams.md` はこの package では current reconnect freeze の外にある later widen を扱う文書なので **更新不要** とした。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
- `specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `scripts/tests/test_current_l2_family_checker_support.py`
- `scripts/tests/test_current_l2_same_lineage_checker.py`
- `scripts/tests/test_current_l2_missing_option_checker.py`
- `scripts/tests/test_current_l2_capability_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`

## 4. Actions taken

- Re-checked the parser-side staged spike evidence and the first-checker-cut docs before fixing the reconnect freeze.
- Added:
  - `specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
  - `specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
- Fixed the current first choice as:
  - bridge = stage 1 reconnect summary + stage 2 try/rollback structural contract
  - retained-later line = stage 3 request / predicate reconnect + `e19` redesign + `E21` / `E22` runtime contrast
- Updated the mirrors so `Documentation.md` / `progress.md` / `tasks.md` / `plan/07` / `plan/09` / `plan/10` / `plan/11` / `plan/12` / `plan/17` / `plan/90` / research abstract all point at the Phase 1 closeout sweep as the next line.

## 5. Files changed

- `specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
- `specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0606-phase3-parser-to-checker-reconnect-freeze-package.md`

## 6. Commands run

- `cargo test -p mir-ast --test current_l2_stage1_parser_spike`
- `cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike`
- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `git status --short`

## 7. Evidence / outputs / test results

- `cargo test -p mir-ast --test current_l2_stage1_parser_spike`
  - `running 14 tests`
  - `test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike`
  - `running 3 tests`
  - `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `python3 -m unittest ...`
  - `Ran 21 tests in 0.017s`
  - `OK`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 605 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - ` M Documentation.md`
  - ` M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - ` M plan/07-parser-free-poc-stack.md`
  - ` M plan/09-helper-stack-and-responsibility-map.md`
  - ` M plan/10-roadmap-overall.md`
  - ` M plan/11-roadmap-near-term.md`
  - ` M plan/12-open-problems-and-risks.md`
  - ` M plan/17-research-phases-and-autonomy-gates.md`
  - ` M plan/90-source-traceability.md`
  - ` M progress.md`
  - ` M specs/00-document-map.md`
  - ` M tasks.md`
  - `?? docs/reports/0606-phase3-parser-to-checker-reconnect-freeze-package.md`
  - `?? specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
  - `?? specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`

## 8. What changed in understanding

- The minimal reconnect bridge should not widen into stage 3 request / predicate reconnect yet; stage 1 summary plus stage 2 structural contract is enough to define the entry criteria.
- `e19` direct target mismatch and `E21` / `E22` runtime contrast are real follow-up pressures, but they are better treated as later reopen lines than as current freeze content.
- Once the reconnect freeze is fixed, the repository mainline should return to Phase 1 / 2 / 4 / 5 closeout instead of staying on the reserve path.

## 9. Open questions

- How to unify stage 1 summary and stage 2 structural contract in the first compile-ready crate-local bridge
- Whether stage 3 reconnect should reopen from declaration-side `admit` or request clause subset first
- Which tool-neutral shape should carry the first runtime / proof contrast once Phase 6 actualization starts

## 10. Suggested next prompt

- `Phase 1 semantics / invariants / notation final sweep を進め、closeout package を docs / plan / progress / tasks / report まで含めて固定してください。`
