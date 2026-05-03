# alpha sample family — Visualization / Devtools

- Status: current-scope Stage E closeout surface over the widened subset runner + planned rows
- Phase: Phase 4 / 5
- Stage: Stage E
- `scripts/alpha_visualization_samples.py` actualizes a dedicated non-public subset runner for `VIS-01/02/03/05/06/07/08/10/11`.
- `VIS-04/09/12` remain planned-only.
- `scripts/alpha_visualization_samples.py stage-e-closeout` fixes current-scope Stage E completion over the implemented subset only.

## Rows

| ID | File | Kind | Status | Expected |
|---|---|---|---|
| `VIS-01` | `vis-01-event_dag_export.mir` | positive | implemented-thin-runner | JSON/HTML view |
| `VIS-02` | `vis-02-place_graph_export.mir` | positive | implemented-thin-runner | view |
| `VIS-03` | `vis-03-route_trace_export.mir` | positive | implemented-thin-runner | view |
| `VIS-04` | `vis-04-witness_timeline.mir` | positive | planned-only | view |
| `VIS-05` | `vis-05-membership_timeline.mir` | positive | implemented-thin-runner | view |
| `VIS-06` | `vis-06-hotplug_lifecycle_view.mir` | positive | implemented-thin-runner | view |
| `VIS-07` | `vis-07-fallback_degradation_view.mir` | positive | implemented-thin-runner | view |
| `VIS-08` | `vis-08-observer_redacted_view.mir` | positive | implemented-thin-runner | no raw high-label data |
| `VIS-09` | `vis-09-admin_full_view.mir` | positive | planned-only | full allowed detail |
| `VIS-10` | `vis-10-on_demand_trace_only.mir` | positive/perf | implemented-thin-runner | no trace before attach |
| `VIS-11` | `vis-11-retention_policy_enforced.mir` | negative/positive | implemented-thin-runner | no over-retention |
| `VIS-12` | `vis-12-debug_layer_detach_stops_trace.mir` | positive | planned-only | telemetry stops |

## Policy

- `.mir` files here remain source-ish markers, not parsed inputs to the current runner.
- implemented rows are executed only through the sample-ID keyed thin runner and exact `.expected.json` parity.
- planned-only rows still record intended verdicts and blockers for later checker/runtime work.
- `VIS-02` is a report-local place-catalog projection bundle over `LI-01`; `VIS-05` is a report-local membership epoch/incarnation timeline bundle over `CUT-17`.
- `VIS-04` stays planned-only because current repo wording still keeps `witness_timeline_view` as a later gate; existing witness-order evidence remains derivable but not a distinct actualized row.
- current-scope Stage E closeout does not require `VIS-04/09/12`, and does not imply a final public viewer API or telemetry service.
- this family is not an active runnable root and does not freeze a final public viewer API or telemetry schema.

## Validation anchor for this package

```bash
python3 scripts/alpha_visualization_samples.py list --format json
python3 scripts/alpha_visualization_samples.py check-all --format json
python3 scripts/alpha_visualization_samples.py closeout --format json
python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json
python3 -m unittest scripts.tests.test_alpha_visualization_samples
```
