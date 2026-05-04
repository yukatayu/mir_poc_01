# 07 — observability and devtools theory

## principle

Debug / visualization is not optional polish. It is part of the project axis: execution, communication, verification, and visualization must be observable.

Observation is a typed information-bearing effect, not untyped debug leak.

## observation event

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

## labels and redaction

Observer-safe view must not expose:

- raw witness payload
- raw auth evidence
- high-label state
- private capability grants
- secrets or native package trust material

Redaction is monotone:

```text
admin_full >= redacted_admin >= observer_safe >= public_summary
```

A layer may strengthen redaction, but not weaken it transparently.

## retention

Retention policy must be explicit.

```text
Retention = ephemeral | report_local | session_local | durable_audit | none
```

α-0.5 / α-0.8 may use `report_local` / `session_local`. Durable audit is later.

## on-demand trace

Observability must support on-demand activation.

```text
trace disabled -> no high-volume materialization
trace enabled -> typed rows emitted according to policy
```

Debug layer attach may activate trace only after activation cut.

## session-bound observability

Operational readiness requires a session-bound observation path.

```text
runtime event -> telemetry row -> redaction -> export -> viewer
```

Exact report bundles are useful first floors, but α-0.9 requires export from runtime session or clear session carrier.

## devtools minimum panels

α-0.9 minimum:

- event DAG panel
- route trace panel
- membership frontier panel
- witness relation panel
- hot-plug lifecycle panel
- fallback degradation panel
- save/load timeline panel
- observer-safe redacted view

## soundness targets

- every exported row derives from event DAG or declared telemetry effect
- observer-safe view contains no raw high-label data
- retention does not exceed policy
- debug activation itself is audited
- trace starts only after attach activation

## required samples

- local event DAG export
- observer-safe route trace
- debug layer attach then trace starts
- non-admin debug attach reject
- save/load membership timeline
- hot-plug lifecycle export
- fallback degradation export
- retention hit/miss query

## stop line

- exact report bundle = final devtools service と書かない
- observer-safe export に raw witness/auth を入れない
- debug layer を permissionless にしない

