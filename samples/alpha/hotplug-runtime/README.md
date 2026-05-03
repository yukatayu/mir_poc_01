# alpha sample family — Hot-plug Runtime

- Status: mixed
- Phase: Phase 4 / 8
- Stage: Stage D
- `HP-02..06` は `../layer-insertion/` に first runtime-sensitive mirrors を持つ。
- `HP-11/12/15` は `mirrorea_alpha_avatar_runtime` と `scripts/alpha_avatar_runtime_samples.py` により runtime-private native-policy subset として実行される。
- `scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout` now treats `HP-11/12/15` as the required package-policy subset for Stage D current-scope closeout.
- この directory 自体は broader hot-plug lifecycle family の planned/sample-mirror authority を保つ。

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
| `HP-10` | `hp-10-runtime_library_hotplug.mir` | planned/positive | accepted |
| `HP-11` | `hp-11-unsigned_native_package_rejected.mir` | negative | rejected |
| `HP-12` | `hp-12-signed_overcapability_package_rejected.mir` | negative | rejected |
| `HP-13` | `hp-13-activation_cut_prevents_hidden_rollback.mir` | planned/negative semantic | no hidden detach |
| `HP-14` | `hp-14-hotplug_verdict_mismatch_rejected.mir` | planned/negative | rejected |
| `HP-15` | `hp-15-revoked_signed_package_rejected.mir` | negative | rejected |

## Policy

- `.mir` files here remain source-ish anchors rather than parsed inputs.
- `.expected.json` sidecars for `HP-11/12/15` are now generated from current runtime-private example output and act as bridge evidence for the native-policy subset runner.
- `HP-02..06` are still not directly executed from this directory in the current repo state; the dedicated Rust attach-time floor lives under `../layer-insertion/`.
- `HP-01/07/08/09/10/13/14` remain planned-only rows.
- Stage D current-scope closeout does not imply `HP-08/09/13/14`, detach runtime, durable migration, distributed activation ordering, or final public hot-plug ABI.
- Promotion to active/runnable root status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
cargo test -p mir-runtime --test alpha_avatar_runtime
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json
find samples/alpha/hotplug-runtime -maxdepth 1 -type f | sort
```
