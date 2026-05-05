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

current practical line still lacks:

- host input -> transform -> host output direct execution lane
- same-session typed host-I/O against the new carrier
- session-bound attach mutation

## why this is a separate roadmap

with only the bounded session carrier and no host-I/O lane:

- α-0.5 stays below completion
- α-0.8 same-session hot-plug still lacks an external execution seam
- α-0.9 session-bound devtools still lacks richer live sources

without typed host-I/O lane:

- α-0.5 remains repo-internal runtime evidence
- developer workflow still lacks minimal external request/receipt demo

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

deliver:

- one minimal adapter family (`EchoText` or `AddOne`)
- input schema
- output schema
- effect / failure row
- authority gate
- observer-safe export

### 3. `P-A1-21` — same-session hot-plug runtime

deliver:

- attach against existing session
- later behavior change in the same session
- lifecycle export against the same session

## anti-shortcut notes

- do not implement host-I/O as Mir core stdio builtin
- do not treat preview bundle as session runtime
- do not treat exact report recomposition as same-session observation

## validation direction

future packages should each bring their own focused script/test surface.
P-A1-18 only freezes the roadmap and category boundary.

## next reopen point

current recommendation:

- reopen with `P-A1-20`
- keep `P-A1-21` immediately behind it
- do not jump directly to `P-A1-21`
