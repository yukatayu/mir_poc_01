# plan/47 — operational alpha-0.9 devtools roadmap

## purpose

この文書は、
`specs/22-observability-devtools-semantics.md`
と `specs/24-operational-alpha05-alpha08-readiness.md`
における α-0.9 session-bound devtools readiness の repository-memory roadmap を置く。

## current repo state

current repo already has useful first-floor export bundles:

- `VIS-A1-01` event DAG
- `VIS-A1-02` route trace
- `VIS-A1-03` membership timeline
- `VIS-A1-04` hot-plug lifecycle
- `VIS-A1-05` fallback degradation
- `VIS-A1-06` redacted observer view
- `VIS-A1-07` report-local retention query

current repo also has useful live/session sources:

- `practical_alpha05_session` observer-safe event DAG / host-I/O summary
- `practical_alpha08_hotplug_session` same-session hot-plug lifecycle summary

current repo still lacks:

- session-bound export source
- explicit witness-relation panel
- explicit save/load timeline panel
- admin/full debug view or explicit kept-later marker in operational workflow

## decisions mirrored from specs/22 / 24

- observability is typed effect, not debug leak
- observer-safe and admin/debug views are distinct
- retention is explicit
- exact report bundle is a first floor, not session-bound operational completion

## current evidence mapping

| Operational need | Existing evidence | Missing for α-0.9 operational readiness |
|---|---|---|
| event DAG | `VIS-A1-01` | session source |
| route trace | `VIS-A1-02` | session source |
| membership timeline | `VIS-A1-03` | session source |
| hot-plug lifecycle | `VIS-A1-04`, `OA08-10` | richer session source / viewer surface |
| fallback degradation | `VIS-A1-05` | session source |
| redacted observer view | `VIS-A1-06` | session source |
| retention/on-demand | `VIS-A1-07` | session source |
| witness relation | partial only | explicit panel |
| save/load timeline | partial only | explicit panel |

## sequencing

α-0.9 depends on:

- `P-A1-19` or equivalent session carrier
- `P-A1-21` or equivalent same-session hot-plug carrier

Without those, widening the viewer alone risks rebranding exact-report recomposition as runtime observability.

## required panel set

minimum operational panel family:

- event DAG
- route trace
- membership timeline
- witness relation
- hot-plug lifecycle
- fallback degradation
- save/load timeline
- observer-safe redacted view

## validation direction

future validation should include session-bound exports, not only exact expected bundle recomposition.

illustrative future commands:

```bash
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha09_devtools.py render-html --format json
python3 -m unittest scripts.tests.test_practical_alpha09_devtools
```

`P-A1-21` still does not create these commands; it only supplies a bounded same-session hot-plug source.

## deferred

- durable audit backend
- final public viewer API
- final public telemetry service
- remote retrieval / retention expiry lifecycle

## next reopen point

α-0.9 should remain downstream of session carrier and same-session hot-plug runtime.
Do not promote it as a standalone viewer-only package first.
