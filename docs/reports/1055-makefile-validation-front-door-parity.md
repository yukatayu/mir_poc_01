# 1055 - Makefile validation front-door parity

## Objective

Align `Makefile` convenience targets with the current docs-focused validation front door so local aliases do not bypass source hierarchy checks.

## Scope and assumptions

- Scope is Makefile validation alias maintenance only.
- No script behavior, sample behavior, Rust code, public API, public ABI, parser grammar, or product surface changes are intended.
- `make docs` should run the same docs-focused structural floor used by closeout packages.
- `make check` should run the docs-focused structural floor before `cargo check`.
- Stop line: this package does not claim test success beyond the commands executed here.

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
- `Makefile`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Inspected `Makefile` and current docs validation command references.
- Ran a red dry-run check showing `make docs` did not include `check_source_hierarchy.py`.
- Updated `make docs` to run:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
- Updated `make check` to run the same docs floor before `cargo check`.
- Updated `make new-report` to use `python3` for consistency with the repo front door.
- Updated `progress.md` recent log.

## Files changed

- `Makefile`
- `progress.md`
- `docs/reports/1055-makefile-validation-front-door-parity.md`

## Evidence / outputs / test results

- Package start:
  - `git status --short` -> clean.
  - `git branch --show-current` -> `main`.
  - `git log -1 --oneline` -> `e2a54cf Suppress target-specific warning noise`.
  - `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-01 09:37 JST`.
- Red check:
  - `make -n docs | rg "check_source_hierarchy.py"` -> failed before patch because the alias only ran `validate_docs.py`.
- Green checks:
  - `make -n docs | rg "check_source_hierarchy.py"` -> pass; printed `python3 scripts/check_source_hierarchy.py`.
  - `make docs` -> pass; `check_source_hierarchy.py` required 24 / present 24 / missing 0, `validate_docs.py` found 1052 numbered reports before this report.
  - `make -n check` -> printed `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, `cargo check`.
  - `make check` -> pass; docs floor passed and `cargo check` finished successfully.
- Post-report docs / diff validation:
  - `python3 scripts/check_source_hierarchy.py` -> pass; required 24 / present 24 / missing 0.
  - `python3 scripts/validate_docs.py` -> pass; documentation scaffold complete, 1053 numbered reports found.
  - `git diff --check` -> pass.

## What changed in understanding

- The main docs floor had moved to source hierarchy plus docs scaffold, while the Makefile alias still represented an older `validate_docs.py`-only front door.
- Keeping the Makefile aligned reduces the chance of local closeout shortcuts silently skipping source hierarchy checks.

## Open questions

- Actual `U1` commitment remains user-facing and open.
- Whether additional Make aliases should be introduced for full validation remains open; this package intentionally does not add a large alias that could encourage unbounded local execution.

## Suggested next prompt

Continue autonomous maintenance with a narrow docs or validation alias audit, or return to active stale wording cleanup if another bounded cluster appears.

## Plan update status

`plan/` 更新不要。No roadmap, repository memory, boundary, or sequencing interpretation changed.

## progress.md update status

`progress.md` 更新済み。Recent log records this Makefile front-door parity fix.

## tasks.md update status

`tasks.md` 更新不要。The current maintenance task map already covers docs freshness and validation reruns; no new implementation queue or blocker changed.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample path, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only Makefile aliases and snapshot/report docs.
- Cargo tests and sample closeouts were not rerun because no Rust implementation, test source, runner behavior, or sample source changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after post-report validation.

## Sub-agent session close status

No sub-agent was spawned for this narrow alias parity package.
