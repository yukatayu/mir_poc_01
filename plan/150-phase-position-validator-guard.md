# plan/150 - phase-position validator guard

## status

LAB repository-memory / validator guard hardening.

This note records the package that made the current phase-position reading from
`plan/149-current-phase-position-reading.md` mechanically visible to
`scripts/validate_docs.py`. It does not edit canon and does not move any gate,
phase, OBL, proof, conformance, runtime, sample, or workflow status.

## purpose

`plan/149` gives the short answer to "where are we in the whole plan?":
canonically the project is still `T0/G0 rebaseline`; by human stage count this
is phase 1 of 9; within T0 it is late pre-exit, but G0 exit is unclaimed.

That answer is important enough that the repo-wide snapshot documents should
not silently drift away from it. This package therefore adds a docs validator
guard for `progress.md` and `tasks.md`.

## guard shape

`scripts/validate_docs.py` now requires both `progress.md` and `tasks.md` to
retain the following phrases:

- `plan/149-current-phase-position-reading.md`
- `T0/G0 rebaseline`
- `phase 1 of 9`
- `late pre-exit`
- `G0 exit`

The phrases are intentionally small. The guard checks that the snapshot still
carries the key reading; it does not validate a full semantic interpretation of
the project phase.

## TDD evidence

The package added failing tests before production code:

- `test_main_rejects_progress_missing_phase_position_guard`
- `test_main_rejects_tasks_missing_phase_position_guard`

The RED run showed both tests failing because `validate_docs.main()` still
returned `0` when the phase-position phrases were missing. The GREEN run passed
after adding the guard.

P104 added a narrow follow-up RED/GREEN pair for the `late pre-exit` phrase:

- `test_main_rejects_progress_missing_late_pre_exit_guard`
- `test_main_rejects_tasks_missing_late_pre_exit_guard`

The RED run showed both tests failing because `validate_docs.main()` still
returned `0` when the rest of the phase-position phrases were present but
`late pre-exit` was absent. The GREEN run passed after adding `late pre-exit`
to the required snapshot phrases and updating the valid test scaffold.

## non-claims

This note does not:

- edit canon;
- claim G0 exit, T1 entry, G1 exit, or any I-phase entry;
- turn percentage into a gate metric;
- select OBL-020 or OBL-001 review-facing extraction;
- choose requested OBL status;
- move the canon ledger;
- discharge proof obligations;
- claim C-static / C-runtime / C-distributed conformance;
- change runtime, sample, product, or workflow readiness;
- freeze public grammar, ABI, SDK, transport, storage, or distribution status.

## next use

Use this note when updating `scripts/validate_docs.py`, `progress.md`, or
`tasks.md` so the phase-position guard remains a management check rather than a
new canon decision.
