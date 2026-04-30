# 1016 witness-provider public-shape boundary inventory

## Objective

Make the current witness/provider public-shape boundary explicit in repository memory, so route-first actual adoption, schema candidates, combined-contract candidates, and final emitted-handoff adjacency remain clearly separated from any final public witness/provider contract.

This package is docs-first research / maintenance only. It does not promote any final public witness/provider surface and does not change runtime behavior.

## Scope and assumptions

- The primary repository-memory home for this package is `plan/09-helper-stack-and-responsibility-map.md`.
- The package only summarizes and sharpens existing route/schema/contract cuts that are already reflected in `specs/10`, `specs/11`, and `plan/18`.
- No reader-facing doc widening is required if current sample-oriented docs already keep provider receipt / witness stop lines explicit enough.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

- Re-read the normative witness/provider open-question ledger and the helper responsibility memory to identify the current public-shape split.
- Added a dedicated witness/provider public-shape boundary inventory to `plan/09`.
- Synced the `progress.md` finite-index / order-handoff row and the `tasks.md` research-discovery line / current status bullet.
- Re-ran focused delegated-RNG and clean near-end validation to confirm provider receipt / witness route evidence still matches the documented stop line.

## Files changed

- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1016-witness-provider-public-shape-boundary-inventory.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before the package |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `a06d710 Inventory VerificationLayer composition boundary` |
| `rg -n "witness/provider|provider receipt|witness schema|emitted-handoff|emitted handoff|provider contract|witness contract|provider_receipt|draw_pub|public-shape" specs/10-open-questions.md specs/11-roadmap-and-workstreams.md plan progress.md tasks.md docs/research_abstract docs/hands_on \| sed -n '1,260p'` | pass | confirmed `specs/10` / `specs/11` already fix the route/schema/combined-contract/final-emitted-handoff ladder and that `plan/09` / `plan/18` hold the helper-side memory |
| `nl -ba plan/09-helper-stack-and-responsibility-map.md \| sed -n '320,395p'` | pass | confirmed current witness/provider actual-adoption, public-schema coupled-later, schema-route actual-adoption, and final-public-contract reopen-threshold cuts are already adjacent in helper memory |
| `nl -ba plan/18-type-proof-modelcheck-and-ordering-research-program.md \| sed -n '90,110p;684,708p'` | pass | confirmed the same package ladder is already preserved as long-range research memory |
| `nl -ba specs/10-open-questions.md \| sed -n '417,423p;455,460p'` | pass | confirmed unresolved items remain final witness schema, provider receipt schema, delegated provider attestation, combined provider+witness public contract, and final emitted handoff contract |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | returned `terminal_outcome = success`, `requires witness(provider_receipt)`, `provider_receipt#1`, and provider-boundary route evidence with transport/witness non-collapse |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | returned canonical order-handoff inventory including `05_delegated_rng_service`, `06_auditable_authority_witness`, `transport_provider_boundary`, and report-local witness/provider views |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | documentation scaffold remains complete |
| `git diff --check` | pass | no whitespace or merge-marker issues |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 16:46 JST` |

## What changed in understanding

- The current witness/provider problem is not a single "public contract later" blob. The repo already distinguishes:
  - route-first actual-adoption memory
  - schema candidate memory
  - combined-contract candidate memory
  - final emitted-handoff adjacency
- The delegated RNG sample is enough to keep the route and provider receipt side honest, while the actual public shape stays explicitly later.
- `plan/09` is the best working home because it already stores the concrete helper-local lanes and references, while `specs/10` / `specs/11` remain the normative ladder.

## Open questions

- When this lane reopens, should the first public-shape package freeze the witness/provider schema pair first, the delegated provider attestation + combined contract first, or the final emitted-handoff contract first?

## Suggested next prompt

Continue with the next safe docs-first package: inventory the adjacent order-handoff source-wording / emitted-artifact boundary, or switch to another maintenance lane such as storage guardrail or dashboard freshness.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/09`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No full validation floor was rerun because this package only clarified witness/provider repository memory and snapshot mirrors; focused delegated-RNG and clean near-end validation covered the touched surface.
- Reader-facing docs were intentionally left unchanged because the current sample-oriented docs already state that provider receipt public schema and final runtime/public contracts remain unsettled.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after final diff inspection.

## Sub-agent session close status

- `James` (`docs_researcher`) completed a parallel read and confirmed that `plan/09` is the right primary memory home, `plan/27` is only the broader mixed-gate umbrella, and existing reader-facing summaries already cover the boundary without needing extra edits. Its findings were incorporated into the package close.
