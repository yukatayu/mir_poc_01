# 1009 verification-layer law surface inventory

## Objective

Refresh the repository-memory inventory for the current `VerificationLayer` law surface, so the emitted-law floor is explicit without overpromoting it into a final public verifier contract.

This package is docs-first research / maintenance only. It does not change runtime behavior, sample behavior, or any normative semantic rule.

## Scope and assumptions

- The target is the current non-normative inventory only: `plan/29` as the main repository-memory home, `plan/14` as the glossary mirror, and snapshot logs in `progress.md` / `tasks.md`.
- Front-door docs (`README.md`, `Documentation.md`, reader-facing summaries, hands-on landing pages) already state the correct stop line and do not need more detail unless the current snapshot judgment changes.
- This package must not claim a final public `VerificationLayer` law schema, hidden verifier builtin, or public verifier contract.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/29-verification-layer-widening-threshold.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/verification_layer_widening_threshold_01.md`
- `docs/reports/1008-stale-wording-lint-feasibility-audit.md`
- sub-agent result from `Dewey` (`docs_researcher`)

## Actions taken

- Re-read the current `VerificationLayer` normative and repository-memory sources to separate fixed current reading from unresolved public-contract questions.
- Confirmed that the current emitted floor is still exactly two rows: helper `verification_handoff_witness` and runtime `verification_model_check`.
- Added an explicit law-family inventory to `plan/29` so the currently emitted law set is visible without forcing readers to infer it from row detail.
- Mirrored that inventory narrowly into `plan/14` so the glossary keeps the same stop line and widening-candidate distinction.
- Updated `progress.md` and `tasks.md` to record the package close without promoting any new implementation line.
- Closed the `Dewey` sub-agent session after incorporating its read-only inventory findings.

## Files changed

- `plan/29-verification-layer-widening-threshold.md`
- `plan/14-glossary-and-boundary-rules.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1009-verification-layer-law-surface-inventory.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the package |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `0b76f78 Record stale-wording lint feasibility audit` |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 16:02 JST` |
| `rg -n "VerificationLayer|verification layer|law surface|law inventory|law" specs/07-typed-effects-wiring-platform.md specs/08-cross-system-relations.md specs/10-open-questions.md specs/11-roadmap-and-workstreams.md plan progress.md tasks.md docs/research_abstract docs/hands_on \| sed -n '1,260p'` | pass | isolated the current fixed reading in `specs/10`, `plan/29`, `plan/14`, `plan/12`, and reader-facing mirrors |
| `sed -n '1,220p' plan/29-verification-layer-widening-threshold.md` | pass | confirmed row detail already encoded the emitted laws but lacked a dedicated law-family inventory |
| `sed -n '58,80p' plan/14-glossary-and-boundary-rules.md` | pass | confirmed `VerificationLayer` glossary wording was the right narrow mirror point |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |

## What changed in understanding

- The repo had already fixed the current `VerificationLayer` emitted floor, but the law-family picture was implicit inside row detail rather than explicit as a small inventory.
- The stable current emitted-law reading is narrow:
  - shared emitted laws: `evidence_preservation`, `residual_obligations_are_explicit`
  - helper-only emitted law: `no_hidden_handoff_without_witness`
- Theorem bridge, runtime policy, and visualization / telemetry law families are still widening candidates, not current emitted laws and not public-contract commitments.
- The best place for this detail is repository memory (`plan/29` and `plan/14`), not front-door docs.

## Open questions

- When a future widening package revisits `VerificationLayer`, should theorem bridge, runtime policy, and visualization lanes be promoted one family at a time, or only after a shared public law vocabulary is fixed?

## Suggested next prompt

Continue with the next safe docs-first research package: inventory another open research-discovery item such as the exact `VerificationLayer` public naming gate, `FAIRY-05` reopen criteria, or projection-equivalence evidence boundaries without promoting implementation.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/29`, `plan/14`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No sample/Cargo floor was rerun because this package only clarified repository-memory inventory and did not touch executable paths.
- Reader-facing docs were intentionally left unchanged because the current front-door summary and stop line were already accurate.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the docs-focused floor is rerun.

## Sub-agent session close status

- `Dewey` (`docs_researcher`) completed a read-only `VerificationLayer` inventory pass and was closed in this package.
