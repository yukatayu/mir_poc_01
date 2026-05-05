# plan/45 — operational alpha-0.5 roadmap

## purpose

この文書は、
`specs/24-operational-alpha05-alpha08-readiness.md`
における α-0.5 operational readiness の repository-memory roadmap を置く。

ここで保持するのは
session-bound local observable runtime への sequencing であり、
current first-floor closeout を operational completion と言い換えることではない。

## current repo state

current repo already has:

- practical package input floor
- first checker floor
- widened first local-runtime floor (`RUN-01..04`)
- first local save/load floor
- first devtools export floor
- product-preview floor
- bounded session runtime carrier (`practical_alpha05_session`)
- minimal typed host-I/O direct execution lane (`practical_alpha05_host_io`, `OA05-07`)

current repo still lacks:

- fuller beginner-facing worked walkthrough beyond the current README / script command references

## decisions mirrored from specs/19 / 20 / 22 / 23 / 24

- α-0.5 is local observable runtime, not merely Stage B evidence closeout
- package input -> checker -> runtime plan -> local runtime session -> observe -> save/load is the required path
- observer-safe export is part of completion
- typed host-I/O minimal demo is required
- local save/load remains distinct from distributed durable save/load

## current evidence mapping

| Operational need | Existing evidence | Residual later gap after bounded α-0.5 |
|---|---|---|
| package input | `SRC-01..05`, `OA05-07` | broader host adapter families |
| checker | `CHK-*`, `OA05-07` | same-session hot-plug over the same carrier |
| local run | `RUN-01..04`, `OA05-07` | same-session hot-plug mutation |
| local save/load | `SL-A1-01/02/03`, `OA05-01/05` | live/session-bound save-load timeline |
| event DAG export | `VIS-A1-01`, `OA05-01` | wider session-bound live devtools families |
| observer-safe view | `VIS-A1-06`, `OA05-01` | wider session-bound live devtools families |
| typed host-I/O | `OA05-07`, `crates/mir-runtime::practical_alpha05_host_io` | broader host families and fuller live devtools surfaces |

## recommended implementation order

### 1. `P-A1-19` — session runtime carrier

actualized:

- `crates/mir-runtime::practical_alpha05_session`
- example `mir_practical_alpha05_session`
- `scripts/practical_alpha05_session.py`
- `OA05-01..06` bounded operational matrix rows

implemented carrier shape:

```text
PracticalAlphaSession {
  session_id
  package_id
  checker_report_ref
  runtime_plan_ref
  runtime_state
  membership_frontier
  witness_store_summary
  visible_history
  event_dag
  attached_layers
  savepoints
}
```

current command family:

- `start`
- `observe`
- `save`
- `load`
- `check-all`
- `closeout`

### 2. `P-A1-20` — typed external host-I/O direct execution lane

actualized:

- `crates/mir-runtime::practical_alpha05_host_io`
- example `mir_practical_alpha05_session -- host-io`
- `samples/practical-alpha1/packages/oa05-07-add-one-host-io`
- `OA05-07`

delivered:

- one minimal typed host adapter family (`AddOne`)
- input schema / output schema
- effect row / failure row
- authority gate
- observer-safe host receipt summary
- event DAG request/response nodes on the same session carrier

## required rows

minimum operational α-0.5 matrix:

- accepted local dispatch
- stale membership reject
- missing capability reject
- missing witness reject
- fallback degradation visible event
- local save/load resume
- stale membership non-resurrection
- session-bound event DAG export
- session-bound observer-safe export
- typed host-I/O minimal demo

current bounded matrix:

- `OA05-01..07`

## validation direction

current validation floor:

```bash
cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha05_session.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha05_session
```

`P-A1-19` actualized the session commands above; `P-A1-20` widened them with the host-I/O lane.

## deferred

- distributed durable save/load
- same-session hot-plug mutation
- final public host boundary ABI
- final public runtime / devtools ABI

## next reopen point

current recommended next reopen point:

- `P-A1-21`

`progress.md` / `tasks.md` remain queue authority.
