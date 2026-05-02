# Report 1121 — P-A0-18 runtime_mirror helper boundary review

- Date: 2026-05-02 18:54:34 +0900
- Author / agent: Codex
- Scope: Review-only pass on the planned `runtime_mirror` helper/test boundary for `P-A0-18`, limited to the requested helper support files, tests, and `LI-01/03/04` plus `VAR-08/11/13` sidecars.
- Decision levels touched: none; review only

## Objective

Evaluate whether a planned `runtime_mirror` helper API can be added without overclaiming beyond the current Alpha-0 helper/runtime boundary.

## Scope and assumptions

- Read order followed through `README.md`, `Documentation.md`, `specs/00..03`, `specs/09`, `plan/00-index.md`, and helper/runtime-relevant Alpha-0 specs and plan docs.
- Review focus stayed on:
  - carrier separation from `reason_codes` and `acceptance_rows`
  - confinement to `alpha-runtime-mirror-floor`
  - verification of source `status` / `claims` / `current_validation` / `mirrors` / `expected_runtime`
  - rejection behavior for unsupported kinds and missing source evidence
- No implementation changes were made.

## Start state / dirty state

- Git worktree was clean at review time.
- No pre-existing diff was present in the requested files.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/README.md`
- `samples/alpha/contract-variance/README.md`
- `samples/alpha/layer-insertion/README.md`

## Actions taken

- Read the requested helper support scripts and tests.
- Read the requested `LI-01/03/04` and `VAR-08/11/13` sidecars.
- Cross-checked those files against Alpha-0 spec and plan wording for helper-local acceptance versus runtime-backed evidence.
- Ran the targeted existing unit tests to confirm current helper behavior before writing findings.

## Files changed

- `docs/reports/1121-review-p-a0-18-runtime-mirror-helper-boundary.md`

## Commands run

- `rg --files README.md Documentation.md specs plan samples/alpha scripts`
- `ls specs`
- `find samples/alpha -maxdepth 2 -type f | sort`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,220p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,220p' specs/02-system-overview.md`
- `sed -n '1,220p' specs/03-layer-model.md`
- `sed -n '1,240p' specs/09-invariants-and-constraints.md`
- `sed -n '1,240p' plan/00-index.md`
- `sed -n '1,240p' plan/09-helper-stack-and-responsibility-map.md`
- `rg -n "runtime_mirror|P-A0-18|alpha-runtime-mirror-floor|expected_runtime|acceptance_rows|reason_codes" specs plan Documentation.md scripts samples/alpha -g '!target'`
- `sed -n '1,260p' specs/14-contract-subtyping-layer-compatibility.md`
- `sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md`
- `sed -n '1,260p' specs/17-mirrorea-spaces-alpha-scope.md`
- `sed -n '1,220p' samples/alpha/README.md`
- `sed -n '1,220p' samples/alpha/layer-insertion/README.md`
- `sed -n '1,220p' samples/alpha/contract-variance/README.md`
- `sed -n '1,260p' scripts/current_l2_family_checker_support.py`
- `sed -n '1,320p' scripts/current_l2_family_acceptance_support.py`
- `sed -n '1,320p' scripts/alpha_contract_variance_checker.py`
- `sed -n '1,360p' scripts/alpha_contract_variance_acceptance.py`
- `sed -n '1,280p' scripts/tests/test_current_l2_family_acceptance_support.py`
- `sed -n '1,320p' scripts/tests/test_alpha_contract_variance_acceptance.py`
- `sed -n '1,240p'` on the six requested sidecars
- `git status --short`
- `git diff -- <requested files>`
- `python3 -m unittest scripts.tests.test_current_l2_family_acceptance_support scripts.tests.test_alpha_contract_variance_acceptance`
- `date '+%Y-%m-%d %H:%M:%S %z'`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_current_l2_family_acceptance_support scripts.tests.test_alpha_contract_variance_acceptance`
  - `Ran 11 tests in 0.012s`
  - `OK`
- Existing acceptance helper behavior is confirmed to:
  - ignore `reason_codes` when comparing `acceptance_rows`
  - treat unsupported filtered rows as `out_of_scope`
  - enforce only same-carrier scope matching for the acceptance floor

## What changed in understanding

- The current Alpha-0 evidence split is stricter than a simple “add more positive kinds” widening.
- `VAR-08/11/13` are not merely missing from the acceptance kind allowlist; they are intentionally kept as planned/sample-mirror rows until runtime-backed source evidence is consulted from `LI-04/01/03`.
- A safe `runtime_mirror` helper therefore needs cross-sidecar source verification rather than a shallow reuse of the acceptance helper template.

## Open questions

- Should `runtime_mirror` consume `VAR-*` sidecars as the public-facing fixture and then resolve `LI-*` sidecars as mandatory source evidence, or should the source `LI-*` sidecar itself be the checked fixture?
- What exact runtime-mirror row body is narrow enough to avoid overclaim while still proving useful beyond `sample_id + source_ref + outcome`?

## Suggested next prompt

Review a concrete `runtime_mirror` schema and helper contract for `P-A0-18`, including exact row fields, source-sidecar lookup rules, unsupported-kind rejection semantics, and the tests for missing `expected_runtime`, missing mirror backlinks, non-runtime `status`, and skeleton-only validation.

## Plan update status

`plan/` 更新不要:
review-only task; no repository-memory change was justified by the evidence.

## Documentation.md update status

`Documentation.md` 更新不要:
review-only task; no snapshot wording changed.

## progress.md update status

`progress.md` 更新不要:
this task produced review findings only and did not change repo progress state.

## tasks.md update status

`tasks.md` 更新不要:
the queue wording already states that `P-A0-18` is still at review stage.

## samples_progress.md update status

`samples_progress.md` 更新不要:
no runnable sample status changed.

## Reviewer findings and follow-up

- High: a `runtime_mirror` helper must reject direct runtime claims sourced only from `VAR-08/11/13` sidecars, because those sidecars still declare planned-skeleton status and skeleton-only validation. Runtime-backed authority currently lives in `LI-01/03/04`.
- Medium: reusing current acceptance-helper semantics would silently pass unsupported runtime-mirror kinds as `out_of_scope`, which is too permissive for the requested reject-on-unsupported boundary.
- Medium: current tests do not cover any source-evidence verification path for `status`, `claims`, `current_validation`, `mirrors`, or `expected_runtime`; dedicated tests are required before promoting `P-A0-18`.

## Skipped validations and reasons

- No new `runtime_mirror` tests were run because no such helper or test target exists yet in the reviewed files.
- No Rust runtime commands were run because this review stayed on the helper/test boundary and the needed runtime evidence was already encoded in the inspected sidecars and docs.

## Commit / push status

Pending at report write. No commit created in this review turn.

## Sub-agent session close status

No sub-agent used.
