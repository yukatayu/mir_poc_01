# alpha sample family — Visualization / Devtools

- Status: planned skeleton only
- Phase: Phase 4 / 5
- Stage: Stage E
- Current runners do not execute this family yet.
- Validation for this package is filesystem/docs integrity only.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `VIS-01` | `vis-01-event_dag_export.mir` | positive | JSON/HTML view |
| `VIS-02` | `vis-02-place_graph_export.mir` | positive | view |
| `VIS-03` | `vis-03-route_trace_export.mir` | positive | view |
| `VIS-04` | `vis-04-witness_timeline.mir` | positive | view |
| `VIS-05` | `vis-05-membership_timeline.mir` | positive | view |
| `VIS-06` | `vis-06-hotplug_lifecycle_view.mir` | positive | view |
| `VIS-07` | `vis-07-fallback_degradation_view.mir` | positive | view |
| `VIS-08` | `vis-08-observer_redacted_view.mir` | positive | no raw high-label data |
| `VIS-09` | `vis-09-admin_full_view.mir` | positive | full allowed detail |
| `VIS-10` | `vis-10-on_demand_trace_only.mir` | positive/perf | no trace before attach |
| `VIS-11` | `vis-11-retention_policy_enforced.mir` | negative/positive | no over-retention |
| `VIS-12` | `vis-12-debug_layer_detach_stops_trace.mir` | positive | telemetry stops |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/visualization -maxdepth 1 -type f | sort
```
