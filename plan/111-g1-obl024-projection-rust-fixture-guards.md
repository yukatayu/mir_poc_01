# plan/111 - G1 OBL-024 projection Rust fixture guard hardening

## Purpose

This file records a narrow test-only hardening package for the LAB-only OBL-024
`diagnostic_soundness_projection` carrier introduced in `plan/110`.

The package adds Rust-side guard coverage for the current projection-bearing
Surface E-ROW sample fixtures:

- `ELAB-04`
- `ELAB-07`
- `ELAB-10`
- `ELAB-13`
- `ELAB-14`
- `ELAB-15`
- `ELAB-16`

The package does not edit production emission logic, expected JSON, canon, Lean
statements, or repair semantics.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- OBL-024 Lean statement draft:
  `plan/109-g1-obl024-lean-statement-draft.md`
- OBL-024 executable projection carrier:
  `plan/110-g1-obl024-executable-projection-carrier.md`
- Rust test evidence:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`

If this LAB evidence conflicts with canon, canon wins.

## What changed

The Rust helper `assert_obl024_diagnostic_soundness_projection` now checks that:

- skipped internal fields such as `association_key` and
  `associated_request_count` are not serialized in LAB JSON;
- every reported binding in the projection matches the enclosing
  `request_context` / `failure_row_context`;
- trace-local replay fields stay aligned with the local failed premise and
  missing evidence;
- `projection_non_final`, `lab_non_final`, and `replay_non_final` remain true.

Additional fixture-backed Rust tests now load the actual sample files for:

- `ELAB-04`, asserting the projection exists and `suggested_repair` remains
  absent;
- `ELAB-07`, asserting the projection exists and the repair remains the exact
  non-final `set_insertion` item;
- `ELAB-10`, asserting the projection exists and the repair remains the
  singleton `VisibilityDenied` row-addition item.

The existing fixture loop for `ELAB-13..16` now also applies the strengthened
projection helper to each non-visibility singleton fixture.

## Relation to plan/110

`plan/110` added the executable carrier. This package does not change that
carrier's serialized shape. It only makes Rust tests fail if current fixtures
stop carrying internally consistent projection evidence.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-024 proof or completion.
- No final Diagnostic JSON / request ID / association-key ABI / replay ABI.
- No production behavior change.
- No expected JSON change.
- No repair widening.
- No `ELAB-04` repair payload.
- No OBL-025 completion claim.
- No conformance or G1 exit claim.

## Validation anchors

```bash
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all
```

## Suggested next packages

1. Keep the projection guard as LAB evidence until final Diagnostic / replay ABI
   work is intentionally designed.
2. If future OBL-024 work needs proof-level replay, refine the statement and
   proof vocabulary first rather than treating these fixture guards as proof.
3. If future OBL-025 repair work resumes, keep it separate from this OBL-024
   projection guard line.
