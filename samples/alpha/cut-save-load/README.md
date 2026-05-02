# alpha sample family — Cut / Save / Load

- Status: mixed runtime/checker/planned family
- Phase: Phase 2 / 6
- Stage: Stage A -> F bridge
- `scripts/alpha_cut_save_load_samples.py` actualizes `CUT-04` as a local-only save/load runtime bridge over the existing Rust local runtime substrate.
- `scripts/alpha_cut_save_load_checker.py` continues to cover selected negative/deferred rows via sidecar-declared `expected_static.checked_reason_codes`.
- This family is still not an active runnable root, and distributed/durable save/load remains out of scope.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `CUT-01` | `cut-01-local_try_rollback_before_cut.mir` | positive | succeeds |
| `CUT-02` | `cut-02-rollback_across_atomic_cut_rejected.mir` | negative semantic | pre-cut state remains |
| `CUT-03` | `cut-03-irreversible_effect_in_rollback_rejected.mir` | negative static/runtime | reject/require compensation |
| `CUT-04` | `cut-04-local_save_load_valid.mir` | positive | load restores room-local runtime frontier |
| `CUT-05` | `cut-05-inconsistent_distributed_snapshot_rejected.mir` | negative | invalid cut |
| `CUT-06` | `cut-06-inflight_message_channel_state_valid.mir` | positive | valid cut |
| `CUT-07` | `cut-07-observe_without_publish_rejected.mir` | negative | invalid cut |
| `CUT-08` | `cut-08-witness_use_without_create_rejected.mir` | negative | invalid cut |
| `CUT-09` | `cut-09-hotplug_activation_without_request_rejected.mir` | negative | invalid cut |
| `CUT-10` | `cut-10-load_does_not_resurrect_expired_lease.mir` | negative semantic | invalid or remains expired |
| `CUT-11` | `cut-11-zcycle_checkpoint_invalid.mir` | negative | unusable checkpoint |
| `CUT-12` | `cut-12-forced_checkpoint_breaks_zcycle.mir` | positive if implemented | valid |
| `CUT-13` | `cut-13-durable_cut_deferred_in_mir0.mir` | negative static | rejected/deferred diagnostic |
| `CUT-14` | `cut-14-capability_use_without_grant_rejected.mir` | negative | invalid cut |
| `CUT-15` | `cut-15-auth_evidence_use_without_create_rejected.mir` | negative | invalid cut |
| `CUT-16` | `cut-16-load_does_not_resurrect_stale_witness.mir` | negative semantic | invalid or remains stale |
| `CUT-17` | `cut-17-load_does_not_resurrect_stale_membership.mir` | negative semantic | invalid or remains stale |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars split into:
  - runtime-backed row: `CUT-04`
  - checker-backed rows: `CUT-05/07/08/09/13/14/15`
  - planned-only rows: `CUT-01/02/03/06/10/11/12/16/17`
- `CUT-05` / `07` / `08` / `09` / `13` / `14` / `15` currently carry checker-floor seed rows for the first structural cut-validity/deferred-surface cut.
- the current mixed cut does not yet cover:
  `CUT-11` Z-cycle graph modeling, `CUT-12` communication-induced checkpoint repair, `CUT-10/16/17` load non-resurrection verdict split, or membership-dependent dispatch closure
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/cut-save-load -maxdepth 1 -type f | sort
cargo test -p mirrorea-core --test runtime_substrate
cargo test -p mir-runtime --test alpha_cut_save_load_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
python3 -m unittest \
  scripts.tests.test_alpha_cut_save_load_checker \
  scripts.tests.test_alpha_cut_save_load_samples
```
