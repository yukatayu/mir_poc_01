# 0637 — Phase 6 proof-notebook bridge-sketch reopen ordering package

## Objective

Close the Phase 6 package that fixes when theorem-side bridge sketch work should reopen after deferred authored-row widen sequencing.

## Scope and assumptions

- Entry criteria are the fixed theorem-first review-unit pilot in `specs/examples/327...328` and the deferred authored-row widen sequencing in `specs/examples/329...330`.
- This package fixes ordering only.
- Actual bridge sketch helper/emitter work, compare-bless metadata, and concrete theorem/model-check binding remain later.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/140-current-l2-proof-notebook-minimal-bridge-sketch-comparison.md`
- `specs/examples/141-current-l2-proof-notebook-compare-ready-bridge-sketch-threshold.md`
- `specs/examples/327-current-l2-source-sample-authoring-bless-regression-policy-ready-theorem-first-concrete-tool-pilot-comparison.md`
- `specs/examples/328-current-l2-theorem-first-concrete-tool-pilot-ready-minimal-theorem-first-concrete-tool-pilot-threshold.md`
- `specs/examples/329-current-l2-theorem-first-concrete-tool-pilot-ready-deferred-authored-row-widen-sequencing-comparison.md`
- `specs/examples/330-current-l2-deferred-authored-row-widen-sequencing-ready-minimal-deferred-authored-row-widen-sequencing-threshold.md`
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
- explorer notes from agents `Ohm` and `Lagrange`

## Actions taken

1. Added `specs/examples/331...332` to compare and threshold the theorem-side bridge-sketch reopen order after authored-row widen sequencing.
2. Fixed the current first choice to:
   - keep authored-row actualization ahead of theorem-side reopen,
   - reopen plain docs-only bridge sketch first,
   - keep compare-ready bridge sketch as the second theorem-side reopen,
   - leave compare-bless metadata and concrete theorem/model-check binding later.
3. Updated snapshot / mirror documents so the current mainline becomes `mirror sweep follow-up maintenance`, followed by `e1` then `e21` authored-row actualization.
4. Added a new traceability addendum in `plan/90-source-traceability.md`.

## Files changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/examples/331-current-l2-deferred-authored-row-widen-sequencing-ready-proof-notebook-bridge-sketch-reopen-ordering-comparison.md`
- `specs/examples/332-current-l2-proof-notebook-bridge-sketch-reopen-ordering-ready-minimal-proof-notebook-bridge-sketch-reopen-ordering-threshold.md`
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
  - 4 tests passed; row-local review-unit runtime/static coverage and fail-closed rejection checks stayed green.
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - 5 tests passed; runtime/static formal-hook artifact family and schema/kind fail-closed checks stayed green.
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 636 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - only intended package files were dirty before commit.

## What changed in understanding

- The theorem-side reopen line does not need to preempt authored-row widening.
- The older docs-first ladder from `140` to `141` can be reused directly as the theorem-side reopen order after widened authored-row actualization.
- The repository-level current line should move to maintenance immediately after this ordering package, rather than jumping straight into plain bridge sketch work.

## Open questions

- Whether plain docs-only bridge sketch actualization should stay spec-only or gain a non-production helper anchor.
- Whether a maintenance pass is still needed between `e1` and `e21` actualization.
- How `e3` theorem-side/formal-hook guard comparison should be ordered against later plain bridge-sketch actualization.

## Suggested next prompt

Close `mirror sweep follow-up maintenance`, then actualize `e1-place-atomic-cut` as the first widened authored row while keeping the current formal-hook and review-unit family fail-closed.
