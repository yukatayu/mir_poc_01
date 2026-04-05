# Report 0248 — review stage1 parser spike input surface and helper naming

- Date: 2026-04-05T22:52:59.615523Z
- Author / agent: Codex
- Scope: review of the uncommitted task introducing `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md` and its mirror updates
- Decision levels touched: L2 / review only

## 1. Objective

Review the uncommitted docs-only task for semantic coherence with `specs/examples/74`, `75`, `76`, `78`; verify that the chosen input surface, internal carrier, and helper naming preserve the private non-production stage-1 parser boundary without over-freezing the final grammar or parser API; check mirror / traceability completeness in the scoped mirror files; and assess report `0247` hygiene.

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0247-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`

## 3. Actions taken

1. Read the required repo-level documents and the stage-1 parser comparison chain in order.
2. Inspected the uncommitted diff for `79`, the scoped mirror updates, and report `0247`.
3. Checked whether the new judgment stays coherent with the earlier decision that the lowering bridge should remain option-level rather than program-level.
4. Verified the scoped mirrors for completeness and traceability.
5. Recorded the review findings in this report.

## 4. Files changed

- Added `docs/reports/0248-review-stage1-parser-spike-input-surface-and-helper-naming.md`
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

- `git status --short`
- `git diff -- Documentation.md specs/00-document-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md progress.md specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md docs/reports/0247-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
- `sed -n '1,260p' ...` / `nl -ba ... | sed -n '...'` over the scoped files

## 6. Evidence / findings

1. `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md:237-247` introduces `lower_stage1_program_to_fixture_subset(...)` as part of the selected helper surface, but `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md:152-166` explicitly rejects a program-level bridge as too broad and `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md:204-216` settles on an option-level structural transfer. This is a semantic regression in the comparison chain: even if kept private, the chosen helper name makes the stage-1 bridge look program-wide again, weakening the earlier non-freeze boundary around the handoff contract.
2. `docs/reports/0247-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md:111-117` does not match the repository’s required report ordering cleanly. `Suggested next prompt` appears as section 8 and a non-required `plan/ update note` is promoted to section 9, even though `AGENTS.md` requires section 9 to be `Suggested next prompt`. The content is mostly present, but the section structure is not fully compliant.

Non-findings within the requested scope:
- The judgments about inline test text, a dedicated wrapper with owned `surface_text`, and `current_l2_stage1_parser_spike_support` are coherent with `specs/examples/74`, `75`, `76`, and `78`.
- The scoped mirror updates in `Documentation.md`, `specs/00-document-map.md`, `plan/11-roadmap-near-term.md`, `plan/12-open-problems-and-risks.md`, `plan/90-source-traceability.md`, and `progress.md` are complete for the decision actually stated in `79`.

## 7. Changes in understanding

- The substantive risk is narrower than the user-visible headline judgment. The input surface and helper-family choice are fine; the inconsistency is the newly named program-level lowering surface inside `79`.
- The mirror set is in good shape. The remaining documentation issue is report-format hygiene, not traceability coverage.

## 8. Open questions

- None beyond the two concrete findings above.

## 9. Suggested next prompt

Address the two review findings for `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`: align the selected helper surface with the option-level bridge boundary from `specs/examples/76`, and reorder `docs/reports/0247-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md` so section 9 is the suggested next prompt and any plan note is kept unnumbered or folded into evidence.
