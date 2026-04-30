# 1012 projection equivalence evidence boundary inventory

## Objective

Make the current projection/codegen `equivalence` claim explicit in repository memory, so `P15` stays constrained to committed generated bridge evidence and review-category alignment instead of drifting toward checker / emitted-program overclaim.

This package is docs-first research / maintenance only. It does not change sample runner behavior, manifest contents, or any normative `specs/` statement.

## Scope and assumptions

- The target is repository memory and snapshot mirrors only: `plan/20`, `progress.md`, `tasks.md`, and this report.
- The current claim must stay at `committed generated bridge evidence only`.
- Helper/report-local preview and committed generated manifest evidence must not be rewritten as final emitted place programs, final public projection API, or a completed cross-place equivalence checker.
- Reader-facing projection docs already describe the stop line well enough, so this package does not widen `docs/research_abstract/` or `docs/hands_on/`.

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
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/20-projection-and-placement-roadmap.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `samples/generated/projection-placement/manifest.json`

## Actions taken

- Re-read the current projection / placement roadmap, reader-facing mirrors, and generated manifest to separate fixed current evidence from later checker / ABI work.
- Added a dedicated `current equivalence-evidence boundary` section to `plan/20`.
- Mirrored the closeout into `progress.md` and `tasks.md` without reopening implementation.
- After reviewer feedback, widened the durable `progress.md` snapshot wording so the stricter stop line does not live only in `recent log`.
- Ran focused projection validation against the committed manifest and both live anchor families.

## Files changed

- `plan/20-projection-and-placement-roadmap.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1012-projection-equivalence-evidence-boundary-inventory.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the package |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `faf70e0 Inventory FAIRY-05 reopen criteria` |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 16:13 JST` |
| `rg -n "projection|equivalence|generated bridge evidence|generated bridge|committed generated|emitted-program|stop line|validity" specs/10-open-questions.md specs/11-roadmap-and-workstreams.md plan progress.md tasks.md docs/research_abstract docs/hands_on samples/README.md \| sed -n '1,260p'` | pass | confirmed `plan/20` is the right repository-memory anchor and that `specs/10` / `specs/11` already keep final projection IR / checker / ABI unresolved |
| `nl -ba plan/20-projection-and-placement-roadmap.md \| sed -n '1,260p'` | pass | confirmed current `P15` first cut, generated reserve policy, validity report minimum, and stop line are already concentrated in one file |
| `nl -ba samples/generated/projection-placement/manifest.json \| sed -n '1,220p'` | pass | confirmed `projection_scope = generated_reserve_bridge_evidence`, `artifact_boundary = committed manifest bridge evidence only`, `P15-GEN-01..04`, and the 10 current `equivalence_review_categories` |
| `nl -ba docs/research_abstract/projection_placement_plan_01.md \| sed -n '1,220p'` | pass | confirmed reader-facing abstract already states the generated-bridge-only boundary accurately |
| `nl -ba docs/hands_on/projection_placement_views_01.md \| sed -n '1,220p'` | pass | confirmed hands-on landing already frames the current floor as helper/report-local preview plus committed manifest bridge evidence |
| `python3 scripts/projection_codegen_samples.py closeout --format json` | pass | returned `artifact_count = 4`, `projection_scope = generated_reserve_bridge_evidence`, `artifact_boundary = committed manifest bridge evidence only`, and the expected `equivalence_review_categories` / `kept_later_gates` |
| `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json` | pass | returned `terminal_outcome = success` and `projection_view` with `SugorokuWorldSource#1`, server/game/participant place split, `local_queue`, observer views, and membership frontier |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | returned `terminal_outcome = success` and `cross_place_projection` showing distinct authority place, provider place, and witness-preserving provider-boundary transport |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |

## What changed in understanding

- The current projection/codegen `equivalence` claim is narrower than the word may suggest: it is a manifest-centered review-category alignment inventory, not a semantic equivalence theorem or checker.
- The committed bridge surface has four stable pieces:
  - reserve-path manifest evidence
  - live-anchor artifact IDs `P15-GEN-01..04`
  - the named closeout categories
  - the kept-later gate list
- The most important future widening question is not only "how to build a checker" but "how any future checker relates to the current review-category inventory and to the final public emitted-program contract."

## Open questions

- If projection equivalence is widened later, should the first next step be a richer committed evidence inventory, a helper-local checker preview, or a public emitted-program contract draft that explains how the checker surface would bind to generated artifacts?

## Suggested next prompt

Continue with the next safe docs-first research package: inventory the runtime-policy / theorem-bridge widening boundary for `VerificationLayer`, or perform another focused maintenance sweep if a stale current-line wording pocket is found.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/20`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No full validation floor was rerun because this package only clarified projection repository memory and snapshot mirrors; focused projection closeout, both anchor families, and docs checks were sufficient for the touched surface.
- `docs/research_abstract/` and `docs/hands_on/` were intentionally left unchanged because they already matched the refined current reading.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after final diff review.

## Sub-agent session close status

- `Kant` (`docs_researcher`) completed a parallel read and its findings were incorporated into this package's final boundary wording.
- `Arendt` (`reviewer`) returned one useful low finding: the stricter stop line initially lived only in `progress.md` `recent log`. That durable snapshot mirror was added before package close. Its medium report-missing note reflected a pre-report diff and was stale once this report file existed.
