# 22 — Observability and Devtools Semantics

## role

この文書は、debug / telemetry / visualization / viewer を
**typed observation effect** として扱う semantics を固定する。

- observability を optional polish にしない
- export を untyped debug leak にしない
- α-0.5 / α-0.8 / α-0.9 readiness に observation requirement を入れる

## decision level

- `L1`
  - observation is a typed information-bearing effect
  - label / authority / redaction / retention are explicit
  - observer-safe view と admin/debug view を分ける
  - exact report bundle と session-bound observability を混同しない
- `L2`
  - observation event carrier
  - current panel inventory
  - on-demand trace boundary

## observation event

minimum carrier:

```text
ObservationEvent = {
  subject_event,
  observer_principal,
  view_label,
  redaction_level,
  retention_scope,
  export_surface,
  proof_or_reason_refs
}
```

exported row は event DAG か declared telemetry effect に由来しなければならない。

## observability pipeline

```text
runtime event
  -> typed telemetry row
  -> authority check
  -> redaction
  -> retention decision
  -> export
  -> viewer / admin / observer surface
```

この pipeline 自体が typed boundary であり、
helper-local debug leak ではない。

## label / authority / redaction / retention

### labels

observer-safe view must not expose:

- raw witness payload
- raw auth evidence
- high-label state
- private capability grants
- secret or trust material

### redaction

redaction is monotone.

```text
admin_full >= redacted_admin >= observer_safe >= public_summary
```

layer may strengthen redaction.
it must not weaken redaction transparently.

### retention

retention policy must be explicit.

```text
none | ephemeral | report_local | session_local | durable_audit
```

α-0.5 / α-0.8 first operational cut may use `report_local` or `session_local`.
`durable_audit` is later.

## observer-safe view vs admin/debug view

- `observer_safe`
  redacted / minimal / no privileged payload
- `admin_debug`
  stronger visibility, but still authority-gated and audited

if admin/full view is not yet actualized,
docs must say `kept_later` explicitly rather than imply parity.

## on-demand trace

on-demand trace is required boundary, not optional convenience.

```text
trace disabled -> no high-volume materialization
trace enabled  -> typed rows emitted according to policy
```

debug layer attach may activate trace only after activation cut.

## session-bound observability

operational α-0.9 requires export source that is session-bound or clearly session-carried.

```text
runtime session
  -> event DAG / route / membership / witness / hot-plug / save-load rows
  -> redaction / retention
  -> JSON export
  -> non-final viewer
```

exact report bundles are useful first floors,
but they do not by themselves satisfy session-bound observability.

## devtools minimum panels

α-0.9 minimum panel family:

- event DAG
- route trace
- membership timeline
- witness relation
- hot-plug lifecycle
- fallback degradation
- save/load timeline
- observer-safe redacted view

## completion-condition boundary

observation is part of readiness, not afterthought.

- α-0.5 operational readiness requires event DAG export and observer-safe debug export
- α-0.8 operational readiness requires attach/reject/defer behavior to become observable in the same session
- α-0.9 readiness requires session-bound export / viewer path, not only exact report recomposition

## current first-floor reading

current repo has useful first-floor observability:

- `VIS-A1-01` event DAG + publication / witness / handoff export
- `VIS-A1-02` observer-safe route trace
- `VIS-A1-03` membership timeline over exact save-load evidence
- `VIS-A1-04` hot-plug lifecycle export over exact hot-plug reports
- `VIS-A1-05` fallback degradation export over exact avatar preview evidence
- `VIS-A1-06` redacted observer view with auth-lane separation
- `VIS-A1-07` report-local retention query export

しかしこれはまだ
session-bound α-0.9 completion を意味しない。

## soundness targets

- every exported row derives from event DAG or declared telemetry effect
- observer-safe view contains no raw privileged payload
- retention does not exceed policy
- debug activation itself is audited
- trace starts only after activation

## required anchors

current freeze で visible であるべき row family:

- `VIS-A1-01`
- `VIS-A1-02`
- `VIS-A1-03`
- `VIS-A1-04`
- `VIS-A1-05`
- `VIS-A1-06`
- `VIS-A1-07`
- `PE2E-07`

future operational anchors:

- session-bound witness relation panel
- session-bound save/load timeline
- explicit admin/debug full view or explicit kept-later marker

## deferred

- final public viewer API
- final telemetry service
- durable audit backend
- detached runtime trace-stop proof

## stop line

- exact report bundle を final devtools service と書かない
- observer-safe export に raw witness/auth payload を入れない
- debug layer を permissionless と書かない
- session-bound export source が無いのに α-0.9 operational-ready と書かない
