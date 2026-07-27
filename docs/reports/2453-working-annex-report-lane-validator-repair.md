# Report 2453 — Working-annex report-lane validator repair

- Date: 2026-07-28 06:41 JST
- Author / agent: Codex
- Scope: Repair the documentation validator so an existing WRK declaration of
  `plan, docs/reports` accepts direct numbered reports only. No theory,
  research result, WRK pre-registration, or public behavior is changed.
- Decision levels touched: LAB validation implementation and tests only.

## Objective

Restore validation of the existing WRK-0024--0027 records without rewriting
their immutable pre-registration material or allowing arbitrary files under
the report directory as evidence.

## Scope and assumptions

`mirrorea_canon/` remains normative. The affected WRK records are L3 and two
are frozen; their declared locations must not be silently rewritten. The
working-annex rule explicitly treats direct numbered Markdown reports as
operational metadata, so the validator must distinguish those reports from
templates, helper files, and nested paths.

## Start state / dirty state

The Plan 200 research-plan package was already uncommitted in the worktree.
The first complete `make docs` run reached `scripts/validate_docs.py` and
failed for WRK-0024--0027 with `Permitted LAB locations must be safe relative
paths`; no project source or Canon theory file was changed by this repair.

## Documents consulted

- `AGENTS.md`, the Canon README/MAP, ADR-0014, and `working/README.md`.
- WRK-0024--0027 and their registration history.
- `scripts/validate_docs.py`, its unit tests, validator history, and Report
  2452.

## Actions taken

1. Reproduced the full-validation failure and extracted the exact rejected
   field from all four affected records.
2. Traced `_permitted_lab_locations` and its allowed-root constants through
   Git history. `docs/reports` was absent from the permit list; the field's
   comma syntax and all other locations were valid.
3. Compared that implementation with the working-annex rule: direct numbered
   reports are permitted operational metadata, while arbitrary helper/source
   files remain excluded.
4. Added a focused regression test before changing validator code and observed
   its expected failure.
5. Added the report root as a declared LAB location but constrained path use
   to the existing direct-numbered-report filename pattern.

## Files changed

- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `docs/reports/2452-oracle-composition-review-and-reanchored-plan.md`
- `docs/reports/2453-working-annex-report-lane-validator-repair.md`

## Commands run

- `make docs` (initial reproduction; failed in the working-annex validator).
- Focused source/history searches and direct Python calls to
  `_permitted_lab_locations`.
- Focused regression test before and after the implementation change.
- Four related Product Alpha / permitted-lane validator tests.

## Evidence / outputs / test results

The pre-change regression test failed because
`_permitted_lab_locations("plan, docs/reports")` returned `None`. The same
call now yields the two declared locations; a direct numbered report is
accepted, while `TEMPLATE.md`, `README.md`, a helper file, and a nested report
path are rejected. The four adjacent existing permitted-lane tests passed.

The full unit suite passed all 88 tests in 5173.270 seconds. A final `make
docs` also passed: Canon index 114/114, source hierarchy 750/750, and
documentation validation accepted all working-annex records.

## What changed in understanding

The failure was not a malformed WRK record or a theory contradiction. It was a
validator/annex mismatch: the records' bounded report metadata could not be
represented by the validator's documented-root list. The repair keeps the
boundary narrow rather than making all report-directory descendants eligible.

## Open questions

None for this repair. The separate Plan 200 research questions remain open and
are not affected.

## Suggested next prompt

Complete the re-anchored R0 source-cut manifest only after the Plan 200 package
and this validator repair have passed their full documentation checks.

## Plan update status

`plan/` 更新不要: this repair changes validation behavior only. The separately
prepared Plan 200 package remains its own pending change.

## Documentation.md update status

`Documentation.md` 更新不要: reader navigation and project-facing claims do not
change.

## docs/project-status.md update status

更新不要: no project maturity, gate, or research status changes.

## progress.md update status

`progress.md` 更新不要: no runtime, theory, or workflow milestone changed.

## tasks.md update status

`tasks.md` 更新不要: this was an internal validator repair, not a new research
package or user decision.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, command, or runnable workflow changed.

## Reviewer findings and follow-up

No independent reviewer was needed for the narrowly reproducible validator
failure. Existing adjacent tests established the location-boundary pattern; the
new regression test covers the direct-report exception and its exclusions.

## Skipped validations and reasons

No applicable validation was skipped. Lean, runtime, parser, and sample runs
are outside this documentation-validator-only change.

## Commit / push status

The adjacent Plan 200 snapshot synchronization was committed and pushed during
the full-suite wait as `28f3c23c9d66401f0c8f0f0855e63ba0321d92bb`. This repair,
its regression test, Plan 200's missing body/registration, and both reports
remain for the immediate follow-up commit; it will be pushed and checked for
`HEAD == origin/main`.

## Sub-agent session close status

No callable sub-agent session was available. No Oracle consultation was needed:
the implementation/history mismatch was locally reproducible and bounded.
