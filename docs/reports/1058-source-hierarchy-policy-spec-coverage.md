# 1058 - Source hierarchy policy/spec coverage

## Objective

Expand structural/docs scaffold presence checks so current policy files and the complete ordered normative spec set are guarded by the validation front door.

## Scope and assumptions

- Scope is path-presence validation only.
- This package does not add semantic linting, stale-wording linting, sample execution, Cargo validation, or report content validation.
- `check_source_hierarchy.py` should cover repo source hierarchy front-door paths, including policy docs and sample/script taxonomy docs.
- `validate_docs.py` should cover the docs/spec scaffold, including `specs/12` and `.docs/*` policy files.
- Stop line: passing these checks means required paths exist; it does not prove theoretical consistency or current wording correctness.

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
- `samples/README.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Ran red inline checks against `check_source_hierarchy.py` and `validate_docs.py` required path lists.
- Added missing subsystem spec paths `specs/04..08`, `specs/10`, and `specs/12` to `check_source_hierarchy.py`.
- Added `.docs/progress-task-axes.md`, `.docs/continuous-task-policy.md`, `.docs/current-l2-source-sample-authoring-policy.md`, and `samples/README.md` to `check_source_hierarchy.py`.
- Added `specs/12-decision-register.md` and the three `.docs/*` policy files to `validate_docs.py`.
- Updated `progress.md` recent log.

## Files changed

- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `progress.md`
- `docs/reports/1058-source-hierarchy-policy-spec-coverage.md`

## Evidence / outputs / test results

- Package start:
  - `git status --short` -> clean.
  - `git branch --show-current` -> `main`.
  - `git log -1 --oneline` -> `d0a7a03 Guard report template headings`.
  - `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-01 09:44 JST`.
- Red checks:
  - inline check over `scripts.check_source_hierarchy.REQUIRED_PATHS` -> failed before patch, missing `.docs/*`, `samples/README.md`, and specs `04..08`, `10`, `12`.
  - inline check over `scripts.validate_docs.REQUIRED` -> failed before patch, missing `.docs/*` and `specs/12-decision-register.md`.
- Green checks after patch:
  - inline required-path check over `scripts.check_source_hierarchy.REQUIRED_PATHS` -> pass.
  - inline required-path check over `scripts.validate_docs.REQUIRED` -> pass.
  - `python3 scripts/check_source_hierarchy.py` -> pass; required 35 / present 35 / missing 0.
  - `python3 scripts/validate_docs.py` -> pass; documentation scaffold complete, 1056 numbered reports found.
  - `make docs` -> pass; source hierarchy required 35 / present 35 / missing 0, docs scaffold complete.
  - `git diff --check` -> pass.
  - `git diff --cached --check` -> pass after staging.

## What changed in understanding

- `validate_docs.py` already covered most ordered specs, but missed `specs/12`.
- `check_source_hierarchy.py` was intentionally lighter, but now that report/template and script taxonomy guards have tightened, guarding policy docs and the full spec front door is a low-risk structural improvement.

## Open questions

- Actual `U1` commitment remains user-facing and open.
- Whether `plan/18..38` should be structurally required remains open; this package avoids requiring broad plan slices because relevant-plan selection is task-dependent.

## Suggested next prompt

Continue autonomous maintenance with a narrow validation rerun or docs freshness audit after this path-coverage expansion.

## Plan update status

`plan/` 更新不要。No roadmap, repository memory, boundary, or sequencing interpretation changed.

## progress.md update status

`progress.md` 更新済み。Recent log records this path-coverage guard expansion.

## tasks.md update status

`tasks.md` 更新不要。The current task map already covers source hierarchy / docs scaffold validation maintenance.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample path, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only structural/docs scaffold required-path lists and documentation.
- Cargo tests and sample closeouts were not rerun because no Rust code, sample source, or runner behavior changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after post-report validation.

## Sub-agent session close status

No sub-agent was spawned for this narrow path-coverage package.
