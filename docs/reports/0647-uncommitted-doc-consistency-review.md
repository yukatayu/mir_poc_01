# Report 0647 — uncommitted doc consistency review

- Date: 2026-04-12 21:08:57 JST
- Author / agent: Codex
- Scope: Review the current uncommitted documentation changes for doc consistency, policy consistency, and misleading statements only. No code changes.
- Decision levels touched: None. Read-only review of snapshot/policy/report wording.

## 1. Objective

Review the scoped uncommitted diffs in:

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `.docs/continuous-task-policy.md`
- `.docs/progress-task-axes.md`
- `faq_003.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/reports/0644...0646`

Focus only on contradictions, stale wording, overclaims, AGENTS-policy mismatch, progress/tasks vs plan mismatch, and statements that exceed current evidence.

## 2. Scope and assumptions

- Followed repo read order: `README.md`, `Documentation.md`, `specs/00...03`, `specs/09`, then status/planning documents relevant to roadmap/progress review.
- Read `progress.md`, `.docs/progress-task-axes.md`, `plan/00-index.md`, and the scoped `plan/` files because this review concerns status/roadmap/task-map rewrites.
- Treated `specs/` and current repo policy files as the normative baseline for consistency checks.
- Did not edit scoped target files; only this report was added per repo policy.

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `faq_003.md`
- `docs/reports/0644-phase-recut-faq003-and-snapshot-rewrite.md`
- `docs/reports/0645-rust-python-implementation-split.md`
- `docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md`

## 4. Actions taken

- Read the required baseline docs and policy files.
- Inspected the current uncommitted diffs for the scoped files.
- Cross-checked new snapshot wording against AGENTS policy and the new `.docs` guidance.
- Compared `progress.md`, `tasks.md`, and relevant `plan/` files for promoted-line and macro-phase consistency.
- Recorded only substantive findings; skipped style-only comments.

## 5. Files changed

- Added `docs/reports/0647-uncommitted-doc-consistency-review.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 6. Evidence / outputs / test results

Representative commands and outputs:

```text
$ git status --short
 M .docs/continuous-task-policy.md
 M AGENTS.md
 M Documentation.md
 M README.md
 M plan/01-status-at-a-glance.md
 M plan/10-roadmap-overall.md
 M plan/11-roadmap-near-term.md
 M plan/17-research-phases-and-autonomy-gates.md
 M progress.md
 M tasks.md
?? .docs/progress-task-axes.md
?? docs/reports/0644-phase-recut-faq003-and-snapshot-rewrite.md
?? docs/reports/0645-rust-python-implementation-split.md
?? docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md
?? faq_003.md

$ date '+%Y-%m-%d %H:%M:%S %Z'
2026-04-12 21:08:57 JST
```

Findings:

1. `docs/reports/0644-phase-recut-faq003-and-snapshot-rewrite.md` is still an empty template, but `progress.md` cites `0644` as one of the source reports for the phase-recut rewrite.
2. `faq_003.md` overstates the ladder status of the authored source-sample quintet by implying all five rows currently have `interpreter` in the ladder.
3. `docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md` says it intentionally skipped `specs/00` and `specs/01`, which conflicts with current AGENTS read-order policy unless an explicit higher-priority user override existed and was documented.

## 7. What changed in understanding

- The new macro-phase / feature-maturity rewrite is mostly internally consistent across `progress.md`, `tasks.md`, and the scoped `plan/` files.
- The remaining problems are concentrated in evidence/report hygiene and one FAQ overclaim, not in the core phase recut itself.

## 8. Open questions

- OPEN QUESTION: was `docs/reports/0644...` intentionally left as a placeholder pending fill-in, or was it meant to be the actual evidence record for the snapshot rewrite?
- OPEN QUESTION: did the task behind report `0646` include an explicit user override allowing the baseline read-order exception, or is the report text simply too broad?

## 9. Suggested next prompt

Address the remaining doc-review findings only: fill in report `0644`, tighten the ladder wording in `faq_003.md` so static-only rows are not read as interpreter-backed, and make report `0646` consistent with AGENTS read-order policy or explicitly document the override basis.
