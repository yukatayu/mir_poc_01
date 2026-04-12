# Report 0667 — phase6 stable malformed broader followup inventory package

- Date: 2026-04-12T23:50:49.307192Z
- Author / agent: Codex
- Scope: Phase 6 docs-only close for `stable malformed broader follow-up inventory`, including normative package `369...370` and snapshot / plan mirror updates.
- Decision levels touched: current L2 package; docs-only sequencing / inventory close only.

## 1. Objective

- Close `stable malformed broader follow-up inventory` after the `e4/e19` stable-static edge-pair close.
- Fix the next stable malformed reopen order without widening source samples, fixtures, or helper code.
- Hand off the repo-level current line cleanly to `public operational CLI / final public contract later gate`.

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
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
- `specs/examples/353...354`
- `specs/examples/361...368`
- `docs/reports/0661-phase6-stable-static-edge-pair-first-reopen-package.md`
- `docs/reports/0666-phase6-post-model-check-gate-document-consistency-audit.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 3. Actions taken

1. Re-read the stable malformed post-contrast sequencing, stable-static edge-pair reopen, missing-option checker spike, capability checker spike, and try/rollback malformed-static comparison to keep the existing split intact.
2. Wrote `specs/examples/369...370` to fix the docs-only broader follow-up order:
   - missing-option family first,
   - capability family second,
   - duplicate cluster kept outside stable inventory,
   - `TryFallback` / `AtomicCut` malformed-static family kept later.
3. Updated `Documentation.md`, `progress.md`, `tasks.md`, `specs/00-document-map.md`, relevant `plan/`, and the Phase 6 abstract so the repo-level current line advances to `public operational CLI / final public contract later gate`.
4. Left code, fixtures, sample corpus, sample README, `.docs` authoring policy, and runtime/helper behavior untouched because this package is sequencing-only.

## 4. Files changed

- `Documentation.md`
- `docs/reports/0667-phase6-stable-malformed-broader-followup-inventory-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/369-current-l2-model-check-concrete-carrier-first-actualization-gate-ready-stable-malformed-broader-follow-up-inventory-comparison.md`
- `specs/examples/370-current-l2-stable-malformed-broader-follow-up-inventory-ready-minimal-stable-malformed-broader-follow-up-inventory-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 668 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- The current stable malformed line is now sequenced as:
  - missing-option family first (`e16/e17/e18`)
  - capability family second (`e13/e20`)
  - duplicate cluster later
  - `TryFallback` / `AtomicCut` malformed-static family later
- The existing helper-local evidence from `specs/examples/47` and `48` is enough to justify that docs-only ordering without widening the source-backed sample line yet.
- The repo-level current line can move to public operational later-gate work without losing the Macro 4 next reopen point.

## 7. Changes in understanding

- The edge-pair close was not the end of the malformed line; it was the point where the broader stable malformed order became narrow enough to fix.
- The next malformed widening should not jump to duplicate cluster or try/rollback malformed-static family.
- `missing-option first / capability second` is the cleanest continuation because it reuses existing checker-spike evidence and preserves the stable-vs-duplicate split.

## 8. Open questions

- Should the missing-option first reopen start as another docs-only actualization comparison, or go directly to source-backed widening?
- After missing-option, should capability follow immediately on the same track?
- When should duplicate cluster re-enter the mainline, if at all, instead of staying a side later gate?
- How should the try/rollback malformed-static family reconnect to the dedicated AST structural helper line later?

## 9. Suggested next prompt

- Continue with `public operational CLI / final public contract later gate`, then refresh snapshot docs before moving to `shared-space admission / compile-time visibility reopen`.
