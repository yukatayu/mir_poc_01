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

current repo still lacks:

- typed host-I/O direct semantic execution lane
- beginner-facing reproducible README/hands-on for this full workflow

## decisions mirrored from specs/19 / 20 / 22 / 23 / 24

- α-0.5 is local observable runtime, not merely Stage B evidence closeout
- package input -> checker -> runtime plan -> local runtime session -> observe -> save/load is the required path
- observer-safe export is part of completion
- typed host-I/O minimal demo is required
- local save/load remains distinct from distributed durable save/load

## current evidence mapping

| Operational need | Existing evidence | Missing for α-0.5 operational readiness |
|---|---|---|
| package input | `SRC-01..05` | typed host-I/O lane |
| checker | `CHK-*` | typed host-I/O lane over the same session |
| local run | `RUN-01..04` | typed host-I/O lane |
| local save/load | `SL-A1-01/02/03`, `OA05-01/05` | typed host-I/O lane |
| event DAG export | `VIS-A1-01`, `OA05-01` | wider session-bound live devtools families |
| observer-safe view | `VIS-A1-06`, `OA05-01` | wider session-bound live devtools families |
| typed host-I/O | `EXT-03/04` preview only | direct semantic execution lane |

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

add one minimal typed host adapter family:

- `EchoText`
- or `AddOne`

This should route through the session/runtime path, not bypass it.

## required rows

minimum operational α-0.5 matrix:

- accepted local dispatch
- stale membership reject
- missing capability reject
- missing witness reject
- fallback degradation visible event
- local save/load resume
- stale membership non-resurrection
- invalid distributed cut preflight reject
- session-bound event DAG export
- session-bound observer-safe export
- typed host-I/O minimal demo

## validation direction

future validation when executable packages land:

```bash
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha05_session.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha05_session
```

P-A1-19 actualizes the session commands above; `P-A1-20` should add the host-I/O lane and expand validation accordingly.

## deferred

- distributed durable save/load
- same-session hot-plug mutation
- final public host boundary ABI
- final public runtime / devtools ABI

## next reopen point

recommended next reopen point after `P-A1-18`:

- `P-A1-20`
- `P-A1-21` only after `P-A1-20`

`progress.md` / `tasks.md` remain queue authority.
