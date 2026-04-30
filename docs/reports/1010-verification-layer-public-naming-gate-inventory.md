# 1010 verification-layer public naming gate inventory

## Objective

Make the current `VerificationLayer` naming gate explicit in repository memory, so emitted row names, evidence-carrier names, and downstream-consumer names are not silently conflated before any future public contract decision.

This package is docs-first research / maintenance only. It does not change semantics, runtime behavior, or any executable path.

## Scope and assumptions

- The target is repository memory plus snapshot logs only: `plan/29`, `plan/14`, `progress.md`, `tasks.md`, and this report.
- The current front-door docs already present the correct stop line, so this package does not widen or restate them.
- This package must not claim final public `VerificationLayer` naming, a final public verifier contract, or a hidden builtin.

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
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/29-verification-layer-widening-threshold.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/verification_layer_widening_threshold_01.md`

## Actions taken

- Re-read the current `VerificationLayer` wording to separate emitted row names from evidence-carrier and downstream-consumer naming.
- Added a dedicated naming-gate inventory to `plan/29` so the current naming split is explicit alongside the law-family inventory.
- Mirrored that split into `plan/14` so glossary readers see the same naming boundary without jumping through the whole widening matrix.
- Updated `progress.md` and `tasks.md` to record the package close without promoting any implementation or public-freeze work.

## Files changed

- `plan/29-verification-layer-widening-threshold.md`
- `plan/14-glossary-and-boundary-rules.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1010-verification-layer-public-naming-gate-inventory.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the package |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `d832f77 Inventory VerificationLayer law surface` |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 16:05 JST` |
| `rg -n "VerificationLayer|public naming|public name|naming|verifier contract|verification_summary|model_check_summary|verification_handoff_witness|verification_model_check" plan/29-verification-layer-widening-threshold.md plan/14-glossary-and-boundary-rules.md plan/27-public-api-parser-gate-roadmap.md specs/10-open-questions.md progress.md tasks.md docs/research_abstract/mirrorea_future_axis_01.md docs/hands_on/verification_layer_widening_threshold_01.md \| sed -n '1,260p'` | pass | isolated the unresolved naming statements in `specs/10`, `plan/29`, `plan/14`, and the current mirrors |
| `sed -n '1,220p' plan/29-verification-layer-widening-threshold.md` | pass | confirmed the widening matrix already distinguished emitted rows from evidence/downstream lanes, but did not yet summarize naming as its own gate |
| `sed -n '1,140p' plan/27-public-api-parser-gate-roadmap.md` | pass | confirmed `P18` still treats `VerificationLayer` as typed explanation / evidence carrier rather than public verifier contract |
| `sed -n '135,155p' specs/10-open-questions.md` | pass | confirmed `public naming` remains explicitly unresolved for theorem bridge / runtime policy / visualization lane widening |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |

## What changed in understanding

- The current naming gate is narrower than the overall `VerificationLayer` composition question:
  - active emitted row names are only `verification_handoff_witness` and `verification_model_check`
  - `verification_preview`, `verification_summary`, `model_check_summary`, theorem bridge, and runtime policy preview are intentionally not active emitted row names
- The unresolved question is not just “what will the public names be,” but also whether any current emitted names survive unchanged into a future public verifier surface and how emitted-row names relate to downstream consumer names.
- This distinction belongs in repository memory before any public-freeze work, because otherwise helper/runtime names can look more settled than they are.

## Open questions

- Should a future widening package treat public/shared naming for theorem bridge / runtime policy / visualization as one coordinated gate, or as separate gates that can advance independently?

## Suggested next prompt

Continue with the next safe docs-first research package: inventory `FAIRY-05` reopen criteria or projection-equivalence evidence boundaries, keeping the same rule that repository memory may narrow stop lines without promoting implementation.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/29`, `plan/14`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No sample/Cargo floor was rerun because this package only clarified naming boundaries in repository memory and snapshot docs.
- Reader-facing docs were intentionally left unchanged because their current stop line already matches the refined repository-memory reading.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the docs-focused floor is rerun.

## Sub-agent session close status

- `Planck` (`docs_researcher`) was started for a parallel naming-gate read but was closed before completion once local evidence proved sufficient. No sub-agent findings were incorporated into this package.
