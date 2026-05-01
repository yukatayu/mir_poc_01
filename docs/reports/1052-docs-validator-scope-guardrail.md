# 1052 - Docs validator scope guardrail

## Objective

Clarify the current scope of the docs-focused validator scripts so future maintenance does not over-read structural checks as semantic or stale-wording validation.

## Scope and assumptions

- Scope is documentation only.
- `scripts/check_source_hierarchy.py` behavior is changed only to require `scripts/README.md` as the script taxonomy front door.
- `scripts/validate_docs.py` behavior is intentionally unchanged.
- This package does not introduce lexical linting, active-current wording scans, sample execution, or Cargo validation into the docs scaffold floor.
- Stop line: this package does not claim theoretical consistency, stale wording absence, sample success, or implementation correctness from docs scaffold checks alone.

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
- `plan/18-public-surface-and-u1-gate.md`
- `plan/19-repository-map-and-taxonomy.md`
- `samples/README.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/new_report.py`
- `Makefile`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Read the docs-focused validator scripts and their front-door documentation.
- Confirmed `check_source_hierarchy.py` checks required path existence and reports counts.
- Confirmed `validate_docs.py` checks required documentation scaffold files and at least one numbered report.
- Added `scripts/README.md` to `check_source_hierarchy.py` required paths so the script taxonomy front door is guarded by the source hierarchy check.
- Left `validate_docs.py` behavior unchanged to avoid duplicating `check_source_hierarchy.py` or broadening validation semantics without a separate design decision.
- Added short scope / non-scope notes for both validators in `scripts/README.md`.
- Updated `progress.md` recent log for this package.

## Files changed

- `scripts/check_source_hierarchy.py`
- `scripts/README.md`
- `progress.md`
- `docs/reports/1052-docs-validator-scope-guardrail.md`

## Evidence / outputs / test results

- `git status --short` at package start was clean.
- `git branch --show-current` -> `main`
- `git log -1 --oneline` -> `1d4751d Audit dashboard freshness`
- `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-01 09:20 JST`
- Pre-patch script inspection:
  - `scripts/check_source_hierarchy.py` builds a required-path presence status and exits non-zero only when required paths are missing.
  - `scripts/validate_docs.py` checks required scaffold paths and the existence of numbered reports.
- Red check before patch:
  `python3 - <<'PY' ... assert 'scripts/README.md' in paths ... PY` -> failed with `AssertionError: scripts/README.md is not required by source hierarchy check`.
- Code-mapper sub-agent result: add `scripts/README.md` to `check_source_hierarchy.py` required paths as the one narrow high-value maintenance improvement; keep everything else report-only and do not add naive stale-wording lint.
- Green check after patch:
  `python3 - <<'PY' ... assert 'scripts/README.md' in paths ... PY` -> pass; printed `scripts/README.md required`.
- `python3 scripts/check_source_hierarchy.py` -> pass; required 24 / present 24 / missing 0.
- `python3 scripts/validate_docs.py` -> pass; documentation scaffold complete, 1050 numbered reports found.
- `git diff --check` -> pass.
- `git diff --cached --check` -> pass after staging.

## What changed in understanding

- The docs floor is correctly lightweight, but `scripts/README.md` did not say clearly enough that these checks do not prove stale-wording absence or semantic consistency.
- `scripts/README.md` is a script taxonomy front door and should be guarded by source hierarchy presence checks alongside the `scripts/` directory itself.
- A scope clarification plus one presence-check addition is lower-risk than widening validator behavior while `U1` and public-surface decisions remain open.

## Open questions

- Whether to add a future active-current-doc lexical lint remains open. Prior evidence shows naive global scans produce false positives over `docs/reports/`, `specs/examples/`, `sub-agent-pro/`, and `progress.md` recent log.
- Actual `U1` commitment remains open and user-facing.

## Suggested next prompt

Continue autonomous maintenance with a focused docs-floor rerun, or inspect whether a future active-current-doc-only lint design should be captured as a separate docs-first proposal.

## Plan update status

`plan/` 更新不要。No roadmap, repository memory, boundary, or sequencing statement changed.

## progress.md update status

`progress.md` 更新済み。Recent log records this guardrail clarification.

## tasks.md update status

`tasks.md` 更新不要。The current maintenance lane already includes docs freshness and explicitly rejects naive banned-phrase scans over historical / report-heavy paths.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only docs wording, source-hierarchy required-path configuration, and report evidence.
- Cargo tests and sample closeouts were not rerun because no Rust code, sample source, or executable sample behavior changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after staged diff validation.

## Sub-agent session close status

Code-mapper sub-agent `019de0e6-c891-70f2-955c-7c5d28560a86` (`Peirce`) audited the docs validator surface, recommended adding `scripts/README.md` to `check_source_hierarchy.py` required paths, and was closed after incorporation.
