# alpha sample family — Contract Variance / Layer Compatibility

- Status: mixed scaffold with synthetic negative checker floor and selected helper-local acceptance floor
- Phase: Phase 1 / 4
- Stage: Stage A -> D bridge
- Current runners still do not execute this family as a parser/runtime sample root.
- Current package adds a non-public checker floor for selected negative-static rows via sidecar-declared `expected_static.checked_reason_codes`.
- Current package also adds a helper-local synthetic acceptance floor for `VAR-01/04/06` via sidecar-declared `expected_acceptance.checked_acceptance_rows`.
- `reason_codes_scope = alpha-static-floor` and `acceptance_scope = alpha-acceptance-floor` are distinct carrier boundaries.
- Validation for this package is synthetic helper-local checker/acceptance tests plus filesystem/docs integrity.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `VAR-01` | `var-01-logging_layer_valid.mir` | positive | valid overlay |
| `VAR-02` | `var-02-precondition_strengthening_rejected.mir` | negative static | reject transparent overlay |
| `VAR-03` | `var-03-postcondition_weakening_rejected.mir` | negative static | reject |
| `VAR-04` | `var-04-output_covariance_valid.mir` | positive | valid |
| `VAR-05` | `var-05-mutable_covariance_rejected.mir` | negative static | reject |
| `VAR-06` | `var-06-readonly_covariance_valid.mir` | positive | valid |
| `VAR-07` | `var-07-failure_row_widening_rejected.mir` | negative static | reject |
| `VAR-08` | `var-08-ratelimit_declared_failure_valid.mir` | positive/runtime | valid, runtime Reject(RateLimited) |
| `VAR-09` | `var-09-effect_row_widening_rejected.mir` | negative static | reject |
| `VAR-10` | `var-10-cost_degradation_rejected.mir` | negative static | reject unless declared |
| `VAR-11` | `var-11-redaction_layer_valid.mir` | positive | valid |
| `VAR-12` | `var-12-debug_layer_requires_authority.mir` | negative runtime/static | reject |
| `VAR-13` | `var-13-auth_layer_contract_update_valid.mir` | positive | valid with activation cut |
| `VAR-14` | `var-14-adapter_transform_preserves_contract.mir` | positive | valid |
| `VAR-15` | `var-15-hidden_shadowing_rejected.mir` | negative static | reject |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- `VAR-02` / `03` / `05` / `07` / `09` / `10` / `15` currently carry checker-floor seed rows for the first static diagnostic cut.
- `VAR-01/04/06` currently carry helper-local synthetic acceptance rows only.
- `VAR-08/11/13/14` remain outside the current acceptance floor because they need runtime / layer / adapter semantics beyond this helper-local cut.
- `VAR-08` / `11` / `12` / `13` now have runtime-backed mirrors under `../layer-insertion/`, but this directory remains the planned/sample-mirror authority for the broader variance family.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/contract-variance -maxdepth 1 -type f | sort
python3 -m unittest \
  scripts.tests.test_alpha_contract_variance_checker \
  scripts.tests.test_alpha_contract_variance_acceptance
```
