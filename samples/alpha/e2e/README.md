# alpha sample family — End-to-End

- Status: planned skeleton only
- Phase: Phase 8
- Stage: Stage F
- Current runners do not execute this family yet.
- Validation for this package is filesystem/docs integrity only.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `E2E-01` | `e2e-01-local_integrated_sugoroku.mir` | positive | roll/publish/handoff |
| `E2E-02` | `e2e-02-docker_two_node_sugoroku.mir` | positive | envelope over Docker network |
| `E2E-03` | `e2e-03-hotplug_debug_layer_runtime.mir` | positive | trace begins after attach |
| `E2E-04` | `e2e-04-hotplug_ratelimit_runtime.mir` | positive/negative | Reject(RateLimited) |
| `E2E-05` | `e2e-05-avatar_runtime_package.mir` | positive | placeholder/custom avatar |
| `E2E-06` | `e2e-06-local_save_load_continue.mir` | positive | resume local state |
| `E2E-07` | `e2e-07-distributed_inconsistent_save_negative.mir` | negative | rejected |
| `E2E-08` | `e2e-08-reversed_library_seed_demo.mir` | optional/non-completion | no claim full app |
| `E2E-09` | `e2e-09-layer_auth_then_hotplug.mir` | positive/negative | correct contract handling |
| `E2E-10` | `e2e-10-package_missing_runtime_fallback.mir` | positive | placeholder |

`E2E-08` は upper-layer seed demo であり、Stage F Alpha completion の required row ではない。

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/e2e -maxdepth 1 -type f | sort
```
