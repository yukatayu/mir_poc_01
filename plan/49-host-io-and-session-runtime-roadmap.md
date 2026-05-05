# plan/49 — host-I/O and session runtime roadmap

## purpose

この文書は、
operational α-0.5 / α-0.8 へ進むための
**session runtime carrier** と **typed host-I/O direct execution lane**
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

current practical line still lacks:

- broader host adapter families beyond the minimal lane
- α-0.9 session-bound devtools export over the same carrier

## why this is a separate roadmap

with the bounded session carrier plus one minimal host-I/O lane:

- α-0.5 reaches bounded completion
- α-0.8 same-session hot-plug is now actualized through the same session carrier
- α-0.9 session-bound devtools still lacks richer live sources

with only the minimal host-I/O lane:

- developer workflow now has one external request/receipt demo
- broader host families and same-session attach mutation remain later

## recommended order

### 1. `P-A1-19` — session runtime carrier

status:

- actualized

delivered:

- session id
- session state carrier
- observe/save/load against the same carrier
- event DAG visibility against the same carrier

### 2. `P-A1-20` — typed external host-I/O direct execution lane

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

## anti-shortcut notes

- do not implement host-I/O as Mir core stdio builtin
- do not treat preview bundle as session runtime
- do not treat exact report recomposition as same-session observation

## validation direction

future packages should each bring their own focused script/test surface.
P-A1-18 only freezes the roadmap and category boundary.

## next reopen point

current recommendation:

- reopen with `P-A1-22`
- do not jump directly to final public viewer/telemetry ABI
