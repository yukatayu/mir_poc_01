# 2315 - Full System V1 evidence-integrity hardening

- Date: 2026-07-21
- Author / agent: Codex with one read-only reviewer sub-agent
- Scope: bounded LAB Full System V1 helper and release-check maintenance
- Decision levels touched: no L0/L1/L2/L3 semantic decision; implementation
  evidence integrity only

## Objective

Audit the bounded Full System V1 release path and correct concrete cases where
the validation result could overstate evidence integrity or workflow readiness.

## Scope and assumptions

Canon remains authoritative. This package changes only LAB helper/release-check
behavior and its reader-facing classification. It does not claim conformance,
real transport, multi-process execution, proof discharge, a Gate/Phase exit,
or final product readiness.

## Start state / dirty state

Started clean at `5c083216` (`docs: record post-checkpoint candidate triage`).
The preceding package had no active new L3 candidate and had recorded a passing
bounded Full System V1 release-check.

## Documents consulted

- `mirrorea_canon/README.md`, `MAP.md`, `spec/06-conformance.md`, and
  `plan/01-phases.md`.
- `AGENTS.md`, `plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`,
  `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and `scripts/README.md`.
- Full System V1 release helper, provider/renderer helpers, projection helper,
  their unit tests, committed generated reports, and release output bundles.

## Actions taken

- Audited the release orchestration and its bounded helper surfaces with local
  source tracing and an independent read-only review.
- Made checker/runtime rows require the exit code implied by their accepted or
  rejected payload while retaining expected rejection exit code 2.
- Changed provider and renderer helpers to validate committed generated reports
  as read-only evidence, including nested renderer provider-admission reports;
  they no longer write into tracked sample roots.
- Made missing generated evidence a validation error and made invalid matrices
  fail before running samples. Standalone failed `run`/`check-all` now returns
  exit status 2.
- Aligned projection helper `workflow_ready` with the dashboard's
  evidence-closed classification.
- Added explicit release-bundle non-claims for C-distributed conformance and
  real transport / multi-process distributed execution.
- Updated the bounded runnable-baseline memory and reader-facing status views.

## Files changed

- `scripts/full_system_v1_samples.py`
- `scripts/provider_admission_samples.py`
- `scripts/renderer_pose_backend_samples.py`
- `scripts/projection_v1_samples.py`
- `scripts/full_system_v1_release_check.py`
- `scripts/tests/test_full_system_v1_samples.py`
- `scripts/tests/test_provider_admission_samples.py`
- `scripts/tests/test_renderer_pose_backend_samples.py`
- `scripts/tests/test_projection_v1_samples.py`
- `scripts/tests/test_full_system_v1_release_check.py`
- `scripts/README.md`
- `plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`
- `docs/project-status.md`
- `progress.md`
- `samples_progress.md`
- `docs/reports/2315-full-system-v1-evidence-integrity-hardening.md`

## Commands run

- Read-only audit with `rg`, `sed`, `nl`, `git diff`, `ps`, and `pstree`.
- Resource checks: `df -h .`, `free -h`, `lsblk -f`, `findmnt -T .`, and
  `du -sh . target .git .cargo .lake`.
- Test-first focused runs, then
  `python3 -m unittest scripts.tests.test_provider_admission_samples scripts.tests.test_renderer_pose_backend_samples scripts.tests.test_projection_v1_samples scripts.tests.test_full_system_v1_release_check`.
- `python3 -m unittest scripts.tests.test_full_system_v1_samples`.
- `python3 scripts/full_system_v1_samples.py check-all --format json`.
- `python3 scripts/provider_admission_samples.py check-all --format json`.
- `python3 scripts/renderer_pose_backend_samples.py check-all --format json`.
- `python3 scripts/projection_v1_samples.py check-all --format json`.
- `python3 -m py_compile` over the changed Full System V1 Python helpers.
- `python3 scripts/validate_docs.py`.
- Two Full System V1 release-check runs into separate disposable `/tmp` output
  roots; the first exposed a stale `progress.md` timestamp header and the
  second was the accepted confirmation run.

## Evidence / outputs / test results

- The focused regression suite passed 52 tests. The existing Full System V1
  helper suite passed 23 tests.
- Real provider admission passed 5/5 rows, renderer pose passed 3/3 rows,
  projection passed 6/6 rows, and the aggregate helper passed 41/41 rows with
  no validation errors.
- The first full release-check reached 26 passed commands but failed the
  standalone docs validator because `progress.md` had a stale header after its
  new timestamped log. The header was synchronized; the failure did not alter
  source evidence.
- The second release-check at
  `/tmp/mirrorea-full-v1-release-20260721-integrity-r2` was accepted with all
  29 planned commands passed, compatibility floor preserved, and no failed
  commands. Its bundle now states both new non-claims explicitly.
- The provider and renderer release reports were accepted at 5 and 3 passed
  rows respectively. `git diff -- samples/full-system-v1/provider-adapter`
  was empty after the accepted release-check, so tracked generated evidence was
  not rewritten.
- Resource preflight showed 19 GiB free on `/`; `target/` was about 1.8 GiB,
  `.git` about 70 MiB, no external workdir was mounted, and the accepted
  release output used about 123 MiB under `/tmp`.

## What changed in understanding

Expected JSON alone is insufficient when a runner publishes generated bridge
evidence or when a subprocess exit status carries an accepted/rejected contract.
The bounded release-check is now stronger evidence of its stated LAB surface,
but it remains neither C-distributed conformance nor actual real-transport or
multi-process execution.

## Open questions

- Explicit operator workflow for intentionally regenerating a committed bridge
  evidence baseline remains a later maintenance decision; ordinary validation
  must stay read-only.
- C-distributed conformance still requires the canon-defined two-OS-process and
  real-transport scenario suite.
- Final packet/FFI semantics, arbitrary provider execution, distributed durable
  save/load, final ABI/SDK, and public product readiness remain deferred.

## Suggested next prompt

Continue autonomous work with a new scoped research candidate or another
evidence-integrity audit; do not widen the bounded LAB release-check into a
conformance or distributed-runtime claim.

## Plan update status

更新済み: `plan/161` now records the fail-closed bounded release baseline and
its explicit non-claims.

## Documentation.md update status

更新不要: the concise project entry already classifies Full System V1 as
bounded runnable LAB evidence; this maintenance package does not change that
classification.

## docs/project-status.md update status

更新済み: the current LAB view records read-only generated-evidence validation
and the explicit distributed non-claims.

## progress.md update status

更新済み: added a timestamped integrity-hardening log and synchronized the
snapshot header.

## tasks.md update status

更新不要: the finding was repaired within this maintenance package; it creates
no current blocker or new autonomous research task.

## samples_progress.md update status

更新済み: aligned the projection readiness field with its evidence-closed
reading and recorded provider/renderer generated-evidence validation behavior.

## Reviewer findings and follow-up

One read-only reviewer found four concrete issues: tracked generated reports
were overwritten during validation; provider/renderer helpers returned zero on
failed validation; projection exposed conflicting readiness; and the release
bundle did not explicitly exclude C-distributed/real-transport readiness. All
four were addressed with focused regression coverage. The reviewer made no
edits.

## Skipped validations and reasons

No Canon, Lean theorem, or transport/deployment validation was added because
this package does not change those boundaries. The release-check itself
explicitly reports real transport and C-distributed conformance as non-claims.

## Commit / push status

Pending at report write. This package is committed with `--no-gpg-sign`,
validated, and pushed before close.

## Sub-agent session close status

The read-only reviewer completed, its findings were independently verified,
and the session was closed. No Oracle consult was needed for these local
implementation-integrity findings.
