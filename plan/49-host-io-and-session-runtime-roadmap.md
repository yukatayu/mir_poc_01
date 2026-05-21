# plan/49 — host-I/O and session runtime roadmap

## purpose

この文書は、
operational α-0.5 / α-0.8 / α-0.9 へ進むための
**session runtime carrier**、**typed host-I/O adapter lane**、**session-bound devtools export**
の sequencing を repository memory として固定する。

## current repo state

current practical line already has:

- package input
- checker
- runtime plan
- local runtime report
- save/load report
- exact devtools bundles
- product-preview bundles
- bounded session runtime carrier (`practical_alpha05_session`)
- same-session typed host-I/O against that carrier (`practical_alpha05_host_io`, `OA05-07`)
- same-session hot-plug against that carrier (`practical_alpha08_hotplug_session`, `OA08-01..10`)
- session-bound devtools export over that carrier (`practical_alpha09_devtools`, `OA09-01..09`)

current practical line still lacks:

- broader host adapter families beyond the minimal lane
- final public viewer / telemetry ABI
- durable audit backend / distributed durable save-load

## why this is a separate roadmap

with the bounded session carrier plus one minimal host-I/O lane:

- α-0.5 reaches bounded completion
- α-0.8 same-session hot-plug is actualized through the same session carrier
- α-0.9 session-bound devtools is actualized through the same session carrier

with only the minimal host-I/O lane:

- developer workflow now has one external request/receipt demo
- broader host families, final public viewer ABI, and durable audit remain later
- Mir-owned computation is not proven; `AddOne` here is adapter-owned transform evidence

## recommended order

### 1. `P-A1-19` — session runtime carrier

status:

- actualized

delivered:

- session id
- session state carrier
- observe/save/load against the same carrier
- event DAG visibility against the same carrier

### 2. `P-A1-20` — typed external host-I/O adapter lane

status:

- actualized

delivered:

- one minimal adapter family (`AddOne`)
- input schema
- output schema
- effect / failure row
- authority gate
- observer-safe export
- same-session event DAG request/response nodes

non-claim:

- this does not prove Mir-owned arithmetic or computational-core completion
- pure `add_one` in Mir is tracked separately under `specs/28` and `plan/53`

### 3. `P-A1-21` — same-session hot-plug runtime

status:

- actualized

deliver:

- attach against existing session
- later behavior change in the same session
- lifecycle export against the same session

delivered:

- `attach` subcommand over `practical_alpha05_session`
- `practical_alpha08_hotplug_session`
- `OA08-01..10` same-session operational matrix

### 4. `P-A1-22` — session-bound devtools export

status:

- actualized

delivered:

- `export-devtools` subcommand over `practical_alpha05_session`
- `practical_alpha09_devtools`
- `OA09-01..09` session-bound operational matrix
- non-final static HTML viewer helper
- explicit admin/full debug kept-later marker

## anti-shortcut notes

- do not implement host-I/O as Mir core stdio builtin
- do not treat preview bundle as session runtime
- do not treat exact report recomposition as same-session observation
- do not treat session-local devtools export as final public viewer / telemetry ABI
- do not treat adapter-owned `AddOne` as Mir-owned computation

## validation direction

future packages should each bring their own focused script/test surface.
P-A1-18 only freezes the roadmap and category boundary.

## next reopen point

current recommendation:

- current host-I/O / session runtime line is actualized where scoped
- next computational correction is tracked by `P-COMP-01..04`, not by widening this adapter lane
