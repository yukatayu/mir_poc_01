# alpha sample family — Network / Docker

- Status: planned skeleton only
- Phase: Phase 5
- Stage: Stage C
- Current runners do not execute this family yet.
- Validation for this package is filesystem/docs integrity only.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `NET-01` | `net-01-local_queue_parity.mir` | positive | valid |
| `NET-02` | `net-02-docker_two_process_envelope.mir` | positive E2E | envelope delivered |
| `NET-03` | `net-03-stale_membership_rejected.mir` | negative E2E | Reject(StaleMembership) |
| `NET-04` | `net-04-missing_capability_rejected.mir` | negative E2E | Reject(MissingCapability) |
| `NET-05` | `net-05-missing_witness_rejected.mir` | negative E2E | Reject(MissingWitness) |
| `NET-06` | `net-06-route_rebinding_no_shadow.mir` | positive | valid |
| `NET-07` | `net-07-observer_safe_route_trace.mir` | positive | no raw secret/witness |
| `NET-08` | `net-08-network_partition_explicit_failure.mir` | negative/positive | Reject(NetworkUnavailable) or Deferred |
| `NET-09` | `net-09-auth_evidence_lane_preserved.mir` | positive | envelope lanes preserved |
| `NET-10` | `net-10-transport_medium_change_preserves_contract.mir` | positive | valid if contract unchanged |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/network-docker -maxdepth 1 -type f | sort
```
