# alpha sample family — End-to-End

- Status: mixed runner-backed non-public bridge + planned rows
- Phase: Phase 8
- Stage: Stage F
- `python3 scripts/alpha_e2e_samples.py` actualizes `E2E-01/02/03/04/05/07/09/10` as a thin integrated bridge over already-actualized Stage B/C/D/F subset floors.
- `E2E-06` local save/load continue and `E2E-08` Reversed Library seed remain planned-only.
- This family is still not an active runnable root, and Stage F remains incomplete.

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

- `.mir` files here remain source-ish placeholders, not parser-front-door inputs.
- `.expected.json` sidecars now split into:
  - implemented thin integrated bridge rows: `E2E-01/02/03/04/05/07/09/10`
  - planned-only rows: `E2E-06/08`
- `E2E-07` is checker-backed invalid distributed cut evidence only. It must not be read as distributed save/load runtime completion.
- Promotion to the repo's active runnable root still requires dedicated validation commands, report evidence, and snapshot updates beyond this alpha-local bridge.
- `E2E-06` remains the local save/load positive blocker for any future Stage F completion claim.

## Validation anchor for this package

```bash
find samples/alpha/e2e -maxdepth 1 -type f | sort
python3 scripts/alpha_e2e_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py closeout --format json
```
