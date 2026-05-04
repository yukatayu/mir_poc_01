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

current repo still lacks:

- attach against a live session carrier
- rejected attach without session mutation in the same workflow
- post-attach behavior/observation change in the same session
- session-visible object/avatar state transition
- detach lifecycle execution beyond explicit deferred boundary

## decisions mirrored from specs/21 / 22 / 24

- layer is a contract transformer
- auth / rate-limit often require explicit contract update
- debug layer needs authority/redaction/retention
- deferred detach is not accepted detach execution
- α-0.8 operational readiness requires same-session visibility

## current evidence mapping

| Operational need | Existing evidence | Missing for α-0.8 operational readiness |
|---|---|---|
| debug attach accepted | `HP-A1-01`, `LI-01` | same-session state mutation |
| non-admin reject | `HP-A1-02`, `LI-02` | same-session rejection path |
| auth contract update | `HP-A1-03`, `LI-03` | same-session effect on later actions |
| rate-limit declared failure | `HP-A1-04`, `LI-04` | same-session runtime reject after activation |
| incompatible patch reject | `HP-A1-05`, `LI-05` | same-session attach workflow |
| stale membership reject | `HP-A1-04B1` | session attach workflow |
| missing witness reject | `HP-A1-04B2` | session attach workflow |
| object/avatar visibility | `HP-A1-06`, `AV-A1-01/02/03` | actual session representation |
| lifecycle export | `VIS-A1-04` | session-bound export source |

## recommended sequencing

### prerequisite

`P-A1-19` should land first or provide an equivalent session carrier.

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

α-0.8 should not be promoted ahead of α-0.5 session carrier.

recommended order:

1. `P-A1-19`
2. `P-A1-20`
3. `P-A1-21`
