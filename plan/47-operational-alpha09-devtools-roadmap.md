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
- `practical_alpha09_devtools` session-bound event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace export

current repo now actualizes:

- `OA09-01..09` as the bounded operational α-0.9 sample matrix
- example command `mir_practical_alpha05_session -- export-devtools <session-path>`
- script command `python3 scripts/practical_alpha09_devtools.py check-all --format json`
- non-final static HTML session viewer via `render-html`
- explicit `admin_debug_view_status = kept_later`

current repo still lacks:

- final public viewer / telemetry ABI
- durable audit backend
- remote retained-artifact retrieval / expiry lifecycle

## decisions mirrored from specs/22 / 24

- observability is typed effect, not debug leak
- observer-safe and admin/debug views are distinct
- retention is explicit
- exact report bundle is a first floor, not session-bound operational completion

## current evidence mapping

| Operational need | Existing evidence | Missing for α-0.9 operational readiness |
|---|---|---|
| event DAG | `VIS-A1-01`, `OA09-01` | none within bounded α-0.9 |
| route trace | `VIS-A1-02`, `OA09-02` | WAN/federation route trace is later |
| membership timeline | `VIS-A1-03`, `OA09-03` | distributed durable membership timeline is later |
| hot-plug lifecycle | `VIS-A1-04`, `OA09-05` | accepted detach execution is later |
| fallback degradation | `VIS-A1-05`, `OA09-06` | native avatar execution is later |
| redacted observer view | `VIS-A1-06`, `OA09-08` | admin/full debug view is kept-later |
| retention/on-demand | `VIS-A1-07`, `OA09-09` | durable audit / remote retrieval is later |
| witness relation | `OA09-04` | raw witness payload export is not allowed |
| save/load timeline | `OA09-07` | distributed durable save/load is later |

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

current commands:

```bash
cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha09_devtools.py render-html --format json
python3 -m unittest scripts.tests.test_practical_alpha09_devtools
```

`P-A1-22` creates these commands and keeps them non-final.

## deferred

- durable audit backend
- final public viewer API
- final public telemetry service
- remote retrieval / retention expiry lifecycle

## next reopen point

α-0.9 is now bounded operational-ready. The safest next package is not a viewer-only package, but a practical α-1 integrated workflow carrier that reuses the bounded α-0.5 / α-0.8 / α-0.9 line without claiming final public product readiness.
