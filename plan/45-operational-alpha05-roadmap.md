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
- first local-runtime floor
- first local save/load floor
- first devtools export floor
- product-preview floor

current repo still lacks:

- unified session carrier
- session-bound observe / save / load workflow
- local missing-capability / missing-witness runtime negatives in the same workflow
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
| package input | `SRC-01..05` | session workflow connection |
| checker | `CHK-*` | session-bound check/run handoff |
| local run | `RUN-01/02` | persistent session carrier |
| local save/load | `SL-A1-01/02/03` | session-bound save/load commands |
| event DAG export | `VIS-A1-01` | session export source |
| observer-safe view | `VIS-A1-06` | session export source |
| typed host-I/O | `EXT-03/04` preview only | direct semantic execution lane |

## recommended implementation order

### 1. `P-A1-19` — session runtime carrier

introduce a coherent session carrier such as:

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

minimum command family may be named differently, but should cover:

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

P-A1-18 does not add these commands; it freezes the requirement only.

## deferred

- distributed durable save/load
- same-session hot-plug mutation
- final public host boundary ABI
- final public runtime / devtools ABI

## next reopen point

recommended next reopen point after `P-A1-18`:

- `P-A1-19` first
- `P-A1-20` immediately after or as a narrowly staged follow-up

`progress.md` / `tasks.md` remain queue authority.
