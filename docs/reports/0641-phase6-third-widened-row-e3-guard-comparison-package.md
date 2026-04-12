# 0641 — Phase 6 third widened row `e3` guard comparison package

## Objective

Close the Phase 6 package that compares how the deferred third widened row `e3-option-admit-chain` should stay guarded at the theorem-side / formal-hook boundary after `e21` actualization.

## Scope and assumptions

- Entry criteria are the fixed authored-row widen sequencing in `specs/examples/329...330`, bridge-sketch reopen ordering in `specs/examples/331...332`, and authored row actualizations in `specs/examples/333...336`.
- This package is docs-first only.
- No source sample, runner, formal-hook helper, or proof-notebook helper behavior is widened here.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md`
- `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md`
- `specs/examples/329-current-l2-theorem-first-concrete-tool-pilot-ready-deferred-authored-row-widen-sequencing-comparison.md`
- `specs/examples/330-current-l2-deferred-authored-row-widen-sequencing-ready-minimal-deferred-authored-row-widen-sequencing-threshold.md`
- `specs/examples/331-current-l2-deferred-authored-row-widen-sequencing-ready-proof-notebook-bridge-sketch-reopen-ordering-comparison.md`
- `specs/examples/332-current-l2-proof-notebook-bridge-sketch-reopen-ordering-ready-minimal-proof-notebook-bridge-sketch-reopen-ordering-threshold.md`
- `specs/examples/333-current-l2-proof-notebook-bridge-sketch-reopen-ordering-ready-first-widened-authored-row-e1-actualization-comparison.md`
- `specs/examples/334-current-l2-first-widened-authored-row-e1-actualization-ready-minimal-first-widened-authored-row-e1-threshold.md`
- `specs/examples/335-current-l2-first-widened-authored-row-e1-actualization-ready-second-widened-authored-row-e21-actualization-comparison.md`
- `specs/examples/336-current-l2-second-widened-authored-row-e21-actualization-ready-minimal-second-widened-authored-row-e21-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- explorer notes from agent `Banach`

## Actions taken

1. Added `specs/examples/337...338` to compare and threshold the deferred `e3` theorem-side / formal-hook guard line.
2. Fixed the current first choice to keep `e3` out of the authored-row inventory, keep the current theorem-side consumer at row-local `proof_notebook_review_unit`, and keep `runtime_try_cut_cluster` as the current formal-hook top.
3. Moved the repository current line from `e3` guard comparison to theorem-side plain bridge sketch actualization.
4. Updated snapshot / mirror documents to keep `e3` as a deferred row and to point the next reopened line at plain docs-only bridge sketch actualization.
5. `plan/08-representative-programs-and-fixtures.md` was intentionally left unchanged because this package did not alter the representative / fixture / source mapping matrix or authored source corpus shape.

## Files changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/examples/337-current-l2-second-widened-authored-row-e21-actualization-ready-third-widened-row-e3-theorem-side-formal-hook-guard-comparison.md`
- `specs/examples/338-current-l2-third-widened-row-e3-theorem-side-formal-hook-guard-comparison-ready-minimal-third-widened-row-e3-guard-threshold.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`

## Commands run

- `cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `git status --short`

## Evidence / outputs / test results

- `cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support`
  - 4 tests passed; the row-local theorem-side review-unit cut stayed green.
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - 5 tests passed; the current formal-hook artifact family stayed green.
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 640 numbered report(s).`
- `git diff --check`
  - no output.
- `git status --short`
  - only intended docs-first package files were dirty before commit.

## What changed in understanding

- The repository no longer needs to treat `e3` guard comparison itself as an open question.
- The natural next line is not `e3` actualization but theorem-side plain bridge sketch actualization.
- `e3` should remain a deferred row until the theorem-side plain bridge sketch line is actually reopened.

## Open questions

- Whether `e3` actualization should reopen immediately after plain bridge sketch actualization or after compare-ready bridge sketch second reopen.
- Whether a later `e3` line should widen the theorem-side bridge only, or both theorem-side and formal-hook family together.
- Whether `E21` / `E22` contrast should reopen before any concrete `e3` authored-row work.

## Suggested next prompt

Close the theorem-side plain bridge sketch actualization package, then re-run a full mirror sweep so the repository current line advances to compare-ready bridge sketch second reopen.
