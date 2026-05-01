# Report 1099 — Alpha-0 sample coverage review

- Date: 2026-05-02 06:20:30 JST
- Author / agent: Codex
- Scope: Review Alpha-0 sample coverage completeness across `sub-agent-pro/alpha-0/08-sample-matrix.md`, `sub-agent-pro/alpha-0/09-repository-docs-and-file-plan.md`, `samples/alpha/`, `samples_progress.md`, and touched `plan/` / `specs/` docs that define required sample families.
- Decision levels touched: none; review only

## Objective

Review whether the Alpha-0 sample scaffold matches the declared sample matrix and sample-family docs, with emphasis on missing families, missing sidecars, status overclaim, and implied-but-absent sample IDs.

## Scope and assumptions

- Review-only task; no normative edits requested.
- `sub-agent-pro/` documents are treated as handoff/planning inputs, not normative sources.
- Sample completeness is judged against the current package claim of planned skeletons plus expected-verdict sidecars, not against future runnable implementation.

## Start state / dirty state

- Repository was already dirty on entry.
- Existing modified/untracked Alpha-0 package files were not changed by this review except for this new report.

## Documents consulted

- `sub-agent-pro/alpha-0/08-sample-matrix.md`
- `sub-agent-pro/alpha-0/09-repository-docs-and-file-plan.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `samples/alpha/*/README.md`
- `samples_progress.md`

## Actions taken

- Read the required handoff docs first, then the repository baseline docs in AGENTS order.
- Enumerated all files under `samples/alpha/`.
- Checked `.mir` to `.expected.json` pairing for every populated Alpha-0 family.
- Compared declared sample families in the handoff/package docs against `samples_progress.md` coverage rows.
- Compared required sample references in `specs/13..17` and `plan/39..43` against the scaffold inventory.

## Files changed

- `docs/reports/1099-alpha0-sample-coverage-review.md`

## Commands run

- `git status --short`
- `sed -n '1,240p' sub-agent-pro/alpha-0/08-sample-matrix.md`
- `sed -n '1,260p' sub-agent-pro/alpha-0/09-repository-docs-and-file-plan.md`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,220p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,220p' specs/02-system-overview.md`
- `sed -n '1,220p' specs/03-layer-model.md`
- `sed -n '1,220p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' plan/00-index.md`
- `sed -n '1,260p' samples_progress.md`
- `sed -n '1,260p' samples/README.md`
- `rg -n "required sample|sample families|samples/alpha|LIF-|VAR-|CUT-|NET-|HP-|AV-|VIS-|E2E-" specs plan samples_progress.md Documentation.md README.md sub-agent-pro/alpha-0 -g '!docs/reports/**'`
- `find samples/alpha -maxdepth 3 -type f | sort`
- `for f in samples/alpha/*/README.md; do sed -n '1,220p' "$f"; done`
- `python3 - <<'PY' ...` to verify `.mir` / `.expected.json` pairing counts by family
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `date '+%Y-%m-%d %H:%M:%S %Z'`

## Evidence / outputs / test results

- Populated Alpha-0 families all have complete `.mir` / `.expected.json` pairing:
  - `lifetime-fallback` 15/15
  - `contract-variance` 15/15
  - `cut-save-load` 13/13
  - `network-docker` 10/10
  - `hotplug-runtime` 14/14
  - `avatar-runtime` 10/10
  - `visualization` 12/12
  - `e2e` 10/10
- `local-runtime/` and `layer-insertion/` contain only `README.md`; they have zero sample files and zero sidecars.
- Required sample references explicitly named in `specs/13..16` are present in the scaffold.
- The review findings are about dashboard/model completeness, not missing sidecars in populated families.
- `python3 scripts/validate_docs.py` passed with `Documentation scaffold looks complete.`
- `git diff --check` passed.

## What changed in understanding

- The Alpha-0 scaffold is more complete than the grouped `samples_progress.md` summary suggests.
- The main gap is not missing sample files in the populated families; it is that `local-runtime/` and `layer-insertion/` are modeled as sample families/directories without corresponding sample IDs, and the dashboard does not represent that boundary accurately.

## Open questions

- Should `local-runtime/` and `layer-insertion/` remain reserved phase directories only, or should the package assign concrete Alpha-0 sample IDs to them now?
- If they remain directory-only, should `samples_progress.md` represent them as separate `1%`/directory-scaffold rows instead of grouping or omitting them?

## Suggested next prompt

Audit and fix the Alpha-0 dashboard/model mismatch by deciding whether `samples/alpha/local-runtime/` and `samples/alpha/layer-insertion/` should receive concrete sample IDs plus sidecars now, or be downgraded to explicit reserve directories in `samples/alpha/README.md`, `samples_progress.md`, and the Alpha-0 handoff docs.

## Plan update status

`plan/` 更新不要:

## Documentation.md update status

`Documentation.md` 更新不要:

## progress.md update status

`progress.md` 更新不要:

## tasks.md update status

`tasks.md` 更新不要:

## samples_progress.md update status

`samples_progress.md` 更新不要:

## Reviewer findings and follow-up

- Finding 1: `samples_progress.md` does not represent the `layer-insertion/` Alpha-0 family at all even though `samples/alpha/README.md` and the file-plan handoff list it as a current family/directory. This leaves the dashboard unable to show that the family is only a directory scaffold and has no sample IDs yet.
- Finding 2: `samples_progress.md` groups `local-runtime/` with `hotplug-runtime/` and `avatar-runtime/` at `10%`, but `local-runtime/` has no sample files or sidecars. That grouping overstates `local-runtime` relative to the legend (`10%` = spec/sample skeleton exists) and hides that only the other two families actually have per-sample scaffold rows.

## Skipped validations and reasons

- Did not run `python3 scripts/check_source_hierarchy.py` because this task was a review of sample coverage claims rather than a source-hierarchy package closeout.
- Did not run any Alpha runner/checker commands because the reviewed Alpha-0 rows explicitly claim skeleton-only status and no dedicated runners exist yet.

## Commit / push status

Not performed. Review-only task; no commit requested.

## Sub-agent session close status

No sub-agent session was started.
