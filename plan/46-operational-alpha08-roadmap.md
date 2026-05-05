# plan/46 — operational alpha-0.8 roadmap

## purpose

この文書は、
`specs/24-operational-alpha05-alpha08-readiness.md`
における α-0.8 same-session hot-plug runtime の repository-memory roadmap を置く。

ここで保持するのは
same-session mutation / observation sequencing であり、
current attach report floor を operational completion と言い換えることではない。

## current repo state

current repo already has:

- practical hotplug plan/report floor
- `HP-A1-*` layer/object/detach-boundary rows
- `AV-A1-*` avatar preview companion floor
- `VIS-A1-04` hot-plug lifecycle export
- `PE2E-*` companion preview bundles
- bounded same-session attach carrier over `practical_alpha05_session`
- `OA08-01..10` same-session operational matrix

current repo still lacks:

- accepted detach execution
- session-bound route trace / membership timeline / witness relation / save-load timeline export
- α-0.9 live viewer/export surface beyond observer-safe session summaries

## decisions mirrored from specs/21 / 22 / 24

- layer is a contract transformer
- auth / rate-limit often require explicit contract update
- debug layer needs authority/redaction/retention
- deferred detach is not accepted detach execution
- α-0.8 operational readiness requires same-session visibility

## current evidence mapping

| Operational need | Existing evidence | Missing for α-0.8 operational readiness |
|---|---|---|
| debug attach accepted | `HP-A1-01`, `LI-01`, `OA08-01` | none within bounded α-0.8 |
| non-admin reject | `HP-A1-02`, `LI-02`, `OA08-02` | none within bounded α-0.8 |
| auth contract update | `HP-A1-03`, `LI-03`, `OA08-03` | richer α-0.9 export source |
| rate-limit declared failure | `HP-A1-04`, `LI-04`, `OA08-04` | post-attach full runtime replay beyond preview remains later |
| incompatible patch reject | `HP-A1-05`, `LI-05`, `OA08-05` | none within bounded α-0.8 |
| stale membership reject | `HP-A1-04B1`, `OA08-06` | none within bounded α-0.8 |
| missing witness reject | `HP-A1-04B2`, `OA08-07` | none within bounded α-0.8 |
| object/avatar visibility | `HP-A1-06`, `AV-A1-01/02/03`, `OA08-08/09` | native execution and richer viewer remain later |
| lifecycle export | `VIS-A1-04`, `OA08-10` | α-0.9 live/session bundle widening |

## recommended sequencing

### promoted package

`P-A1-21` — α-0.8 same-session hot-plug runtime

target command family may be named differently, but should cover:

- `start`
- `attach`
- `observe`
- `run-action`
- `detach`
- `check-all`
- `closeout`

status:

- actualized

delivered:

- attach against a live session carrier
- rejected attach without session mutation in the same workflow
- post-attach observer/runtime behavior change in the same session
- session-visible object preview state transition
- deferred detach minimal contract as a same-session lifecycle event

## required rows

minimum α-0.8 operational matrix:

- debug attach accepted
- non-admin attach rejected
- auth explicit contract update
- rate-limit declared failure with later runtime effect
- incompatible patch rejected
- stale membership attach rejected
- missing witness attach rejected
- visible object attach
- visible placeholder/custom/fallback avatar state
- deferred detach minimal contract as lifecycle event
- session-bound hot-plug lifecycle export

## theoretical constraints

- accepted attach implies activation event
- rejected attach does not mutate session state
- auth/rate-limit contract change is explicit
- layer behavior change is observable after activation
- visibility / redaction / retention remain policy-bounded

## deferred

- accepted detach execution
- durable migration
- distributed activation ordering
- native execution realization
- final public package / avatar ABI

## next reopen point

α-0.8 is now actualized in bounded form.

recommended order:

1. `P-A1-22` α-0.9 session-bound devtools export
2. broader host adapter families only after the α-0.9 line is bounded
