# Report 0246 — review stage1 parser spike placement and compare surface

- Date: 2026-04-05T22:38:25.184927Z
- Author / agent: Codex
- Scope: review of the uncommitted task introducing `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` and its mirror updates
- Decision levels touched: L2 / review only

## 1. Objective

Review the uncommitted docs-only task for semantic coherence with `specs/examples/31`, `37`, `74`, `75`, `76`, `77`; verify that the proposed private placement and compare surface preserve current helper boundaries; check mirror / traceability completeness in the scoped mirror files; and assess report `0245` hygiene.

## 2. Scope and assumptions

- Scope limited to:
  - `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
  - mirror updates in `Documentation.md`, `specs/00-document-map.md`, `plan/11-roadmap-near-term.md`, `plan/12-open-problems-and-risks.md`, `plan/90-source-traceability.md`, `progress.md`
  - `docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- Assumed review target is the current uncommitted worktree before any follow-up fixes.
- No broader refactor proposals were considered.

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
- `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md`

## 4. Actions taken

1. Read the required repo-level context and the stage-1 parser example chain in order.
2. Inspected the uncommitted diff for the new example, mirror updates, and report `0245`.
3. Checked the new placement / compare-surface judgment against the existing detached-helper boundary established by `specs/examples/31` and `specs/examples/37`.
4. Verified whether the scoped mirrors mention `78` consistently and whether `plan/90` stays traceable to material actually present in the reviewed task.
5. Recorded review findings in this report.

Files changed:
- Added `docs/reports/0246-review-stage1-parser-spike-placement-and-compare-surface.md`

Commands run:
- `git status --short`
- `git diff -- Documentation.md specs/00-document-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md progress.md specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `sed -n '1,220p' ...` / `nl -ba ... | sed -n '...'` over the scoped files

- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Evidence / outputs / test results

Findings:
1. `plan/90-source-traceability.md:516-525` treats `docs/reports/0246-review-stage1-parser-spike-placement-and-compare-surface.md` as a principal source for the task’s mirror updates, but that review report is not part of the uncommitted task being introduced. At review time the file does not exist, so the traceability addendum points at a nonexistent source and overstates mirror completeness.
2. `docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md:14-37` and `docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md:65-76` do not meet the repository’s report hygiene expectations cleanly. The required `Scope and assumptions` section is missing as a dedicated section, and the section titled `Commands run and exact outputs` records paraphrases rather than exact command output for the `sed` reads.

Non-findings within the requested scope:
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md:85-173` is semantically coherent with `specs/examples/74` through `77`.
- The choice of `crates/mir-ast/tests/support/` as a private non-production placement and lowered fixture-subset compare as the surface is consistent with the helper-boundary discipline described in `specs/examples/31` and `specs/examples/37`, because it avoids widening `lib.rs` / `harness.rs` APIs and keeps the bridge test-only.

Test results:
- No executable tests were run. This was a docs-only review.

## 6. What changed in understanding

- The main semantic decision in `78` is sound within the current stage-1 parser chain.
- The problems are limited to traceability accuracy and report-format hygiene, not to the proposed private parser-spike boundary itself.

## 7. Open questions

- None beyond the two concrete findings above.

## 8. Suggested next prompt

Address the two review findings for `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`: remove or defer the premature `0246` traceability reference in `plan/90-source-traceability.md`, and bring `docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md` into the repository’s required report structure and evidence style.
