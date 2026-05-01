# 1057 - validate_docs template heading guard

## Objective

Teach `scripts/validate_docs.py` to guard the refreshed report template closeout headings so future template drift is caught by the docs scaffold check.

## Scope and assumptions

- Scope is docs validator scaffold behavior and its script taxonomy description.
- This is not a semantic lint, stale-wording lint, sample runner, Cargo validation, or per-task report compliance proof.
- The validator only checks that `docs/reports/TEMPLATE.md` contains the required closeout headings.
- Stop line: this package does not inspect historical reports or enforce report content quality.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `docs/reports/TEMPLATE.md`
- `scripts/validate_docs.py`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Inspected `validate_docs.py`, the refreshed report template, and `scripts/README.md`.
- Ran a red inline Python check for a missing `missing_template_headings()` helper.
- Added `REQUIRED_TEMPLATE_HEADINGS` and `missing_template_headings()` to `validate_docs.py`.
- Updated `validate_docs.py` main flow to fail if `docs/reports/TEMPLATE.md` is missing required closeout headings.
- Updated `scripts/README.md` to describe the new template-heading scaffold check.
- Updated `progress.md` recent log.

## Files changed

- `scripts/validate_docs.py`
- `scripts/README.md`
- `progress.md`
- `docs/reports/1057-validate-docs-template-heading-guard.md`

## Evidence / outputs / test results

- Package start:
  - `git status --short` -> clean.
  - `git branch --show-current` -> `main`.
  - `git log -1 --oneline` -> `95039af Refresh report template discipline`.
  - `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-01 09:41 JST`.
- Red check:
  - `python3 - <<'PY' ... v.missing_template_headings('## Objective\n') ... PY` -> failed with `AttributeError` before patch.
- Green checks after patch:
  - inline Python check for `missing_template_headings()` -> pass; missing-heading sample detected `## Commit / push status`, current template returned no missing headings.
  - `python3 scripts/validate_docs.py` -> pass; documentation scaffold complete, 1055 numbered reports found.
  - `make docs` -> pass; source hierarchy required 24 / present 24 / missing 0, docs scaffold complete.
  - `git diff --check` -> pass.
  - `git diff --cached --check` -> pass after staging.

## What changed in understanding

- The report template can now be treated as part of docs scaffold correctness, not only as an unchecked helper file.
- Keeping this as heading presence avoids the false-positive problem of semantic or stale-wording lints.

## Open questions

- Actual `U1` commitment remains user-facing and open.
- Whether to validate generated report files from `new_report.py` directly remains a possible future helper test, but this package keeps the validator narrow.

## Suggested next prompt

Continue autonomous maintenance with a small `new_report.py` smoke check or another bounded docs freshness package if a drift cluster appears.

## Plan update status

`plan/` 更新不要。No roadmap, repository memory, boundary, or sequencing interpretation changed.

## progress.md update status

`progress.md` 更新済み。Recent log records this validator guard.

## tasks.md update status

`tasks.md` 更新不要。The current task map already covers docs freshness and validation guardrail maintenance.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample path, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only docs validator scaffold behavior and documentation.
- Cargo tests and sample closeouts were not rerun because no Rust code, sample source, or runner behavior changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after post-report validation.

## Sub-agent session close status

No sub-agent was spawned for this narrow validator guard package.
