# alpha sample family — Hot-plug Runtime

- Status: planned skeleton only
- Phase: Phase 4 / 8
- Stage: Stage D
- Current runners do not execute this family yet.
- Validation for this package is filesystem/docs integrity only.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `HP-01` | `hp-01-attach_sugoroku_to_empty_world.mir` | positive | accepted + activation cut |
| `HP-02` | `hp-02-attach_debug_layer_admin.mir` | positive | accepted |
| `HP-03` | `hp-03-attach_debug_layer_non_admin_rejected.mir` | negative | rejected |
| `HP-04` | `hp-04-attach_auth_layer_contract_valid.mir` | positive | accepted |
| `HP-05` | `hp-05-attach_ratelimit_layer_contract_valid.mir` | positive | accepted |
| `HP-06` | `hp-06-incompatible_patch_rejected.mir` | negative | rejected |
| `HP-07` | `hp-07-missing_capability_hotplug_rejected.mir` | negative | rejected |
| `HP-08` | `hp-08-detach_object_no_dangling.mir` | positive | valid |
| `HP-09` | `hp-09-detach_runtime_with_dependents_rejected.mir` | negative | rejected/deferred |
| `HP-10` | `hp-10-runtime_library_hotplug.mir` | positive | accepted |
| `HP-11` | `hp-11-unsigned_native_package_rejected.mir` | negative | rejected |
| `HP-12` | `hp-12-signed_overcapability_package_rejected.mir` | negative | rejected |
| `HP-13` | `hp-13-activation_cut_prevents_hidden_rollback.mir` | negative semantic | no hidden detach |
| `HP-14` | `hp-14-hotplug_verdict_mismatch_rejected.mir` | negative | rejected |
| `HP-15` | `hp-15-revoked_signed_package_rejected.mir` | negative | rejected |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/hotplug-runtime -maxdepth 1 -type f | sort
```
