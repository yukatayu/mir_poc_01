# world-core blueprint

## Purpose

`WorldCore` is the minimal server-side virtual-space base.

It is not a game.
It is not final world SDK.
It defines the base surfaces that later packages import.

## Representative source: world-core.mir

```mir
// representative source only; current executable input is package.mir.json
module WorldCore version "0.1.0"

place WorldServerPlace

policy observation {
  default_view = observer_safe
  redaction = redact_raw_auth_and_witness
  retention = report_local
}

capability WorldAdmin
capability JoinWorld
capability ObserveWorld

state membership : MembershipRegistry
state event_log : EventDag

perform admit_participant(principal : Principal)
  require capability(JoinWorld)
  ensure membership.active(principal)
  publish participant_joined(principal)

perform leave_participant(principal : Principal)
  require membership.active(principal)
  ensure membership.epoch_advanced
  publish participant_left(principal)

observe event_log via observer_safe_view
```

## package.mir.json intent

```json
{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "ops.world-core",
  "package_version": "0.1.0",
  "package_kind": "world_core",
  "dependencies": [],
  "effects": ["AdmitParticipant", "LeaveParticipant", "ObserveWorld"],
  "failures": ["MissingCapability", "StaleMembership", "MissingWitness"],
  "capabilities": ["WorldAdmin", "JoinWorld", "ObserveWorld"],
  "contracts": [
    {
      "surface": "world.admit",
      "preconditions": ["has JoinWorld capability"],
      "postconditions": ["membership frontier advances or explicit reject"]
    }
  ],
  "observation_policy": {
    "default_view": "observer_safe",
    "redaction": "redact_raw_auth_and_witness"
  },
  "retention_policy": {
    "scope": "report_local"
  },
  "native_policy": {
    "NativeExecutionPolicy": "Disabled"
  }
}
```

Use actual field names from current schema.

## Expected check result

- accepted
- package kind recognized
- world identity present
- observation policy present
- native execution disabled

## Devtools panels

- package overview
- world place graph
- membership frontier
- event DAG empty/start marker
