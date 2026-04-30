# 1017 order-handoff source-wording / emitted-artifact boundary inventory

## Objective

Make the current order-handoff source-wording / emitted-artifact boundary explicit in repository memory, so the surface-first floor, source-wording route-first floor, coupled-later candidates, and final-public seam residual keep remain separated from any final public order-handoff contract.

This package is docs-first research / maintenance only. It does not promote any final public order-handoff seam and does not change runtime behavior.

## Scope and assumptions

- The primary repository-memory home for this package is `plan/09-helper-stack-and-responsibility-map.md`.
- The package only summarizes and sharpens existing order-handoff cuts that are already reflected in `specs/10`, `specs/11`, and `plan/18`.
- No new reader-facing doc is required if `progress.md` and `tasks.md` mirror the remaining gate explicitly enough.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
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
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

- Re-read the normative order-handoff open-question ledger and the helper responsibility memory to identify the current source-wording / emitted-artifact split.
- Added a dedicated order-handoff source-wording / emitted-artifact boundary inventory to `plan/09`.
- Synced the `progress.md` finite-index / order-handoff row and the `tasks.md` research-discovery line / current status bullet.
- Re-ran focused Sugoroku and clean near-end validation to confirm publish-before-handoff and emitted-artifact stop lines still match the documented boundary.

## Files changed

- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1017-order-handoff-source-wording-emitted-artifact-boundary-inventory.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the package |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `c65d273 Inventory witness/provider public-shape boundary` |
| `rg -n "source-wording\|source wording\|emitted-artifact\|emitted artifact\|emitted-handoff\|handoff wording\|artifact schema\|principal surface" specs/10-open-questions.md specs/11-roadmap-and-workstreams.md plan progress.md tasks.md docs/research_abstract docs/hands_on \| sed -n '1,260p'` | pass | confirmed `specs/10` / `specs/11` already fix the source-wording route, emitted-artifact candidate, and later-gate split and that `plan/09` / `plan/18` already hold the helper-side memory |
| `nl -ba plan/09-helper-stack-and-responsibility-map.md \| sed -n '334,440p'` | pass | confirmed current order-handoff surface actual-adoption, source-wording route actual-adoption, and source-wording / emitted-artifact coupled-later cuts are already adjacent in helper memory |
| `nl -ba plan/18-type-proof-modelcheck-and-ordering-research-program.md \| sed -n '675,734p'` | pass | confirmed the same package ladder is already preserved as long-range research memory and later mixed-gate reopen band |
| `nl -ba specs/11-roadmap-and-workstreams.md \| sed -n '177,218p'` | pass | confirmed the fixed ladder: surface/artifact threshold, surface actual-adoption, source-wording / emitted-artifact coupled-later, and source-wording route actual-adoption |
| `nl -ba specs/10-open-questions.md \| sed -n '423,440p'` | pass | confirmed unresolved items remain final source-surface handoff wording, final emitted-artifact schema, final emitted-handoff contract, final public witness/provider/artifact contract, exact low-level wording, and final modal/fairness connection |
| `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --format json` | pass | returned `terminal_outcome = success`, `published_witness = draw_pub#1`, `publication_order`, `witness_order`, `scoped_happens_before`, and `roll_is_published_before_handoff` evidence |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 03_handoff_before_publication_rejected --format json` | pass | returned `static_verdict = malformed`, `reason_family = handoff_before_publication`, and failed constraint `publish(draw) must precede handoff that requires witness(draw_pub)` |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | returned canonical order-handoff inventory including `01_authorized_roll_publish_handoff`, `03_handoff_before_publication_rejected`, `source_wording`-adjacent proof samples, and report-local visualization / telemetry inventory |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 17:41 JST` |

## What changed in understanding

- The current order-handoff problem is not just “final wording later.” The repo already distinguishes:
  - surface-first actual-adoption memory
  - source-wording route-first memory
  - emitted-artifact schema candidate memory
  - final-public seam residual keep
- The clean negative sample and Sugoroku positive sample together are enough to keep the publish/witness/handoff ordering line honest while the public wording and artifact schema stay explicitly later.
- `plan/09` is the best working home because it already stores the concrete helper-local lanes and references, while `specs/10` / `specs/11` remain the normative ladder and `plan/18` remains reopen-band memory.

## Open questions

- When this lane reopens, should the first public-seam package freeze final source-surface handoff wording first, emitted-artifact schema first, or a coupled final-public seam together with witness/provider residuals?

## Suggested next prompt

Continue with the next safe docs-first package: inventory the residual order-handoff / witness-provider final public seam compression line more explicitly, or switch to another maintenance lane such as storage guardrail or dashboard freshness.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/09`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No full validation floor was rerun because this package only clarified order-handoff repository memory and snapshot mirrors; focused Sugoroku / clean near-end validation covered the touched surface.
- Reader-facing docs were intentionally left unchanged because `progress.md` / `tasks.md` were sufficient mirrors and the current sample-oriented docs already leave final wording / artifact schema unresolved.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after final diff inspection.

## Sub-agent session close status

- `Pasteur` (`docs_researcher`) completed a parallel read and confirmed that `plan/09` is the right primary memory home, `plan/18` stays reopen-band memory, and no new reader-facing doc is needed if `tasks.md` is sharpened. Its findings were incorporated into the package close, and the session was then closed.
- `Curie` (`reviewer`) first reported four traceability / mirror issues in the report and snapshot docs, then confirmed in a narrow re-check that those findings were resolved. No broader semantic overclaim was reported against the added `plan/09` inventory, and the session was then closed.
