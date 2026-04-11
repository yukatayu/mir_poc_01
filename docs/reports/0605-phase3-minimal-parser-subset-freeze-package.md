# Report 0605 — phase3 minimal parser subset freeze package

- Date: 2026-04-11 20:48 JST
- Author / agent: Codex
- Scope: Phase 3 reopen continuation through minimal parser subset freeze package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the ordered mainline after the verifier handoff gate.
- Determine what the minimal actual parser first tranche should be for Phase 6 front-half planning.
- Update the snapshots so the next promoted line moves from parser subset freeze to parser-to-checker reconnect freeze.

## 2. Scope and assumptions

- Keep the staged parser order from `specs/examples/73...`.
- Keep stage 1 / stage 2 / stage 3 helper implementations private / test-only.
- Distinguish truly malformed first-tranche input from valid-but-retained-later stage 3 input.
- `plan/13-heavy-future-workstreams.md` はこの package では Phase 3 immediate freeze ではなく later widen を扱う文書なので **更新不要** とした。

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
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/285-current-l2-minimal-public-checker-boundary-ready-verifier-handoff-surface-comparison.md`
- `specs/examples/286-current-l2-verifier-handoff-surface-ready-minimal-verifier-handoff-surface-threshold.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

## 4. Actions taken

- Re-checked the staged parser spike evidence and the reconnect-side sequencing docs before fixing the parser subset freeze.
- Added:
  - `specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
  - `specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
- Fixed the current first choice as:
  - accepted cluster = stage 1 + stage 2 structural floor
  - first-tranche reject family = `missing edge-local lineage metadata` / `missing fallback body` / `atomic_cut` fallback placement
  - retained-later floor = stage 3 `admit` / request clause / predicate helper line
- Updated the mirrors so `Documentation.md` / `progress.md` / `tasks.md` / `plan/07` / `plan/09` / `plan/10` / `plan/11` / `plan/12` / `plan/17` / `plan/90` / research abstract all point at the reconnect freeze as the next line.

## 5. Files changed

- `specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
- `specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
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
- `docs/reports/0605-phase3-minimal-parser-subset-freeze-package.md`

## 6. Commands run

- `cargo test -p mir-ast --test current_l2_stage1_parser_spike`
- `cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike`
- `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike`
- `cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike`
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
- `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike`
  - `running 6 tests`
  - `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike`
  - `running 11 tests`
  - `test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 603 numbered report(s).`
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
  - ` M progress.md`
  - ` M specs/00-document-map.md`
  - ` M tasks.md`
  - `?? docs/reports/0605-phase3-minimal-parser-subset-freeze-package.md`
  - `?? specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
  - `?? specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`

## 8. What changed in understanding

- The minimal actual parser first tranche should not stop at stage 1 only; stage 2 try/rollback structural floor is source-backed enough and keeps the future reconnect bridge narrower.
- Stage 3 `admit` / request clause / predicate evidence is real and tested, but it is still better read as retained-later floor than as first-tranche parser acceptance.
- The right split here is accepted structural floor vs malformed structural reject vs retained-later helper line; otherwise stage 3 spillover wording gets over-read as permanent rejection.

## 9. Open questions

- How to represent the reconnect freeze with one narrow bridge across stage 1 summary and stage 2 structural findings
- Whether stage 3 reopen should start from declaration-side `admit` or request clause suite after Phase 6 front-half
- Where the first non-production parser API should live in `mir-ast`

## 10. Suggested next prompt

- `minimal-parser-subset-freeze-ready parser-to-checker-reconnect-freeze comparison を進め、stage1/2 を checker floor へ reconnect する minimal bridge を fixed してください。`
