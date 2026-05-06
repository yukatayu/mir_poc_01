# 06 — observability and diagrams

## 1. Principle

Observability is not optional polish.
Operational samples are not complete unless a developer can see what happened.

Every implemented sample should expose:

- what source/package was loaded
- what dependencies/imports were resolved
- what runtime plan was produced
- what places exist
- what messages moved
- what events happened
- what witnesses were created/used
- what hot-plug lifecycle occurred
- what save/load/quiescent-save did
- what was redacted and why

## 2. Required diagrams / panels

### 2.1 Source/import graph

Show:

```text
WorldCore
  <- MembershipChat
       <- SugorokuWorld
```

JSON shape:

```json
{
  "panel": "source_import_graph",
  "nodes": ["world-core", "membership-chat", "sugoroku-world"],
  "edges": [
    {"from": "membership-chat", "to": "world-core", "relation": "imports"},
    {"from": "sugoroku-world", "to": "membership-chat", "relation": "imports"}
  ]
}
```

### 2.2 Package dependency graph

Show versioned package dependencies and schema versions.

### 2.3 Projection target graph

Show server/client intent, even if not emitted binaries.

```text
server target:
  WorldServerPlace
  ChatPlace
  SugorokuGamePlace

participant-client target:
  ParticipantPlace[*]
  ClientViewPlace
```

### 2.4 Place graph

Show runtime places:

- WorldServerPlace
- ChatPlace
- SugorokuGamePlace
- HostAdapterPlace
- ParticipantPlace[Alice]
- ParticipantPlace[Bob]

### 2.5 Message route graph

Show `MessageEnvelope` path:

```text
ParticipantPlace[Alice]
  -> SugorokuGamePlace
  -> publish roll_result
  -> witness draw_pub
  -> ParticipantPlace[Bob] handoff
```

### 2.6 Event DAG

Must include:

- join
- chat send
- roll commit
- publish
- witness
- handoff
- attach request
- verdict
- activation cut
- save / load / quiescent-save
- fallback degradation if present

### 2.7 Membership/config frontier timeline

Show:

- membership_epoch
- member_incarnation
- config_epoch
- stale reject points

### 2.8 Witness timeline

Show:

- witness creation
- witness use
- missing witness rejection
- redacted witness payload in observer-safe view

### 2.9 Hot-plug lifecycle

Show:

```text
AttachRequest -> CompatibilityCheck -> Auth/Capability/WitnessCheck -> Verdict -> ActivationCut -> Mutation or Deferred Boundary
```

### 2.10 Save/load timeline

Show:

- R0 save
- R2 quiescent-save
- NoInFlight
- AllPlacesSealed
- NoPostCutSend
- load
- resumed state

### 2.11 Contract/effect/failure summary

For each package, show:

- effects
- failures
- capabilities
- witnesses
- redaction policy
- retention policy

### 2.12 Observer-safe vs admin/debug view

Every viewer/export must state:

- active view role
- redaction level
- retention scope
- admin/debug full view status
- source report/session backing the panel

## 3. ASCII diagrams in docs

At least include simple diagrams in the hands-on doc:

```text
[WorldCore]
    ^
    |
[MembershipChat]
    ^
    |
[SugorokuWorld]
```

```text
Alice Client -> MessageEnvelope -> SugorokuGamePlace
                                      |
                                      v
                               publish / witness
                                      |
                                      v
                                 Bob Client
```

```text
DebugLayer Attach
  request -> checks -> verdict -> activation_cut -> trace visible
```

## 4. If viewer cannot render all panels

Do not overclaim.

Acceptable current package statement:

```text
JSON export includes the required panel data; HTML viewer rendering for some panels is kept-later.
```

Not acceptable:

```text
Final devtools viewer completed.
```
