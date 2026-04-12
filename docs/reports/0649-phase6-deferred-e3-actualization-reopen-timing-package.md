# Report 0649 — phase6 deferred e3 actualization reopen timing package

- Date: 2026-04-12T12:52:53.448772Z
- Author / agent: Codex (GPT-5)
- Scope: Close the Phase 6 docs-only package that fixes when deferred `e3-option-admit-chain` actualization reopens after compare-ready bridge sketch second reopen, then sync snapshot / plan / abstract mirrors.
- Decision levels touched: Read-only on normative semantics. Updated current-L2 snapshot / roadmap / traceability wording and added new example-package docs.

## 1. Objective

Close the current Phase 6 timing package that decides `e3` actualization should reopen before the first concrete theorem/model-check pilot, while keeping the current theorem-side consumer and formal-hook top unchanged.

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
- `specs/examples/327-current-l2-source-sample-authoring-bless-regression-policy-ready-theorem-first-concrete-tool-pilot-comparison.md`
- `specs/examples/328-current-l2-theorem-first-concrete-tool-pilot-ready-minimal-theorem-first-concrete-tool-pilot-threshold.md`
- `specs/examples/337-current-l2-second-widened-authored-row-e21-actualization-ready-third-widened-row-e3-theorem-side-formal-hook-guard-comparison.md`
- `specs/examples/338-current-l2-third-widened-row-e3-theorem-side-formal-hook-guard-comparison-ready-minimal-third-widened-row-e3-guard-threshold.md`
- `specs/examples/341-current-l2-minimal-plain-proof-notebook-bridge-sketch-ready-compare-ready-bridge-sketch-second-reopen-comparison.md`
- `specs/examples/342-current-l2-compare-ready-bridge-sketch-second-reopen-ready-minimal-compare-ready-bridge-sketch-threshold.md`
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
- explorer notes from agents `Jason` and `McClintock`

## 3. Actions taken

1. Added `specs/examples/343...344` to compare and threshold the deferred `e3` actualization reopen timing line.
2. Fixed the current first choice to reopen actual `e3` authored-row work immediately after compare-ready bridge sketch second reopen, before the first concrete theorem/model-check pilot.
3. Kept the current theorem-side consumer at row-local `proof_notebook_review_unit` and the current formal-hook top at `runtime_try_cut_cluster` / `fixture_static_cluster`.
4. Advanced the repository current line from deferred `e3` actualization reopen timing to actual `e3` authored-row reopen.
5. Updated snapshot / plan / abstract mirrors and source traceability to reflect that timing judgment is now fixed entry criteria.

## 4. Files changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/examples/343-current-l2-minimal-compare-ready-bridge-sketch-ready-deferred-e3-actualization-reopen-timing-comparison.md`
- `specs/examples/344-current-l2-deferred-e3-actualization-reopen-timing-ready-minimal-deferred-e3-actualization-reopen-threshold.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`

## 5. Commands run and exact outputs

Representative commands:

```text
$ cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support
running 4 tests
test proof_notebook_review_unit_support_emits_static_row_local_units ... ok
test proof_notebook_review_unit_support_emits_runtime_review_unit ... ok
test proof_notebook_review_unit_support_rejects_unsupported_pairs_and_empty_evidence ... ok
test proof_notebook_review_unit_support_rejects_wrong_schema_or_artifact_kind ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test -p mir-semantics --test current_l2_formal_hook_support
running 5 tests
test formal_hook_support_emits_static_cluster_subject_and_row_refs ... ok
test formal_hook_support_emits_runtime_try_cut_subject_and_row_refs ... ok
test formal_hook_support_rejects_runtime_artifact_outside_try_cut_cluster ... ok
test formal_hook_support_rejects_runtime_artifact_with_wrong_schema_or_kind ... ok
test formal_hook_support_rejects_static_artifact_with_wrong_schema_or_kind ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 648 numbered report(s).

$ git diff --check
(no output)
```

## 6. Evidence / findings

- The natural next reopen is actual `e3` authored-row work, not a first concrete theorem/model-check pilot.
- The current formal-hook guard still matters: `e3` actualization should not force a new formal-hook family into the same package.
- `cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support`
  - 4 tests passed; the current theorem-side row-local consumer stayed green.
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - 5 tests passed; the current formal-hook top stayed green, including the reject path for runtime artifacts outside `runtime_try_cut_cluster`.
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 648 numbered report(s).`
- `git diff --check`
  - no output.

## 7. Changes in understanding

- The repository no longer needs to keep `e3` timing itself as an open question.
- The correct next package is now the actual `e3` authored-row reopen, but under the existing theorem-side / formal-hook guard.
- The first concrete theorem/model-check pilot should remain after `e3` authored-row work, not before it.

## 8. Open questions

- How to represent `e3` in the authored verification ladder while preserving the current formal-hook guard.
- Whether the actual `e3` package should keep formal hook as `not reached (guarded)` or use a more specific guard wording.
- How soon the post-`e3` line should reopen second source-sample cluster sequencing versus the first concrete theorem/model-check pilot.

## 9. Suggested next prompt

Close the actual `e3` authored-row reopen with TDD-style source-sample / runner / regression updates, while preserving the current theorem-side consumer and current formal-hook top.
