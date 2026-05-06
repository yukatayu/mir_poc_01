# membership-chat blueprint

## Purpose

`MembershipChat` imports `WorldCore` and adds join/leave/chat behavior.

It demonstrates typed external / room-message boundary without making stdio a Mir core primitive.

## Representative source: membership-chat.mir

```mir
// representative source only; current executable input is package.mir.json
import WorldCore version "^0.1"

module MembershipChat version "0.1.0"

capability SendRoomMessage
capability ModerateRoomMessage

failure RateLimited
failure StaleMembership
failure MissingCapability

perform join(principal : Principal)
  require capability(JoinWorld)
  ensure membership.active(principal)
  publish participant_joined(principal)

perform leave(principal : Principal)
  require membership.active(principal)
  ensure membership.epoch_advanced
  publish participant_left(principal)

perform send_room_message(sender : Principal, body : Text)
  require membership.active(sender)
  require capability(SendRoomMessage)
  may_fail RateLimited
  publish chat_message(sender, body) redacted observer_safe

observe chat_message via room_chat_panel
```

## package.mir.json intent

```json
{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "ops.membership-chat",
  "package_version": "0.1.0",
  "package_kind": "membership_chat",
  "dependencies": [
    {
      "package_id": "ops.world-core",
      "version_req": "^0.1",
      "path": "../world-core"
    }
  ],
  "effects": ["JoinWorld", "LeaveWorld", "SendRoomMessage"],
  "failures": ["StaleMembership", "MissingCapability", "RateLimited"],
  "capabilities": ["JoinWorld", "ObserveWorld", "SendRoomMessage"],
  "contracts": [
    {
      "surface": "chat.send",
      "preconditions": ["active membership", "SendRoomMessage capability"],
      "postconditions": ["chat message published or explicit failure"]
    }
  ],
  "observation_policy": {
    "chat_view": "observer_safe",
    "redaction": "redact_auth_witness"
  },
  "message_recovery_policy": {
    "transport_failures": ["Timeout", "RouteUnavailable"],
    "default": "RejectWithTrace"
  },
  "native_policy": {
    "NativeExecutionPolicy": "Disabled"
  }
}
```

## Current execution note

If product alpha does not yet support text chat host-I/O, do not claim it executes.
Use package check and manifest/dependency evidence, or AddOne as the existing direct host-I/O lane.

## Expected evidence

- dependency on WorldCore represented
- effect/failure/capability rows declared
- observer-safe chat surface declared
- no stdio builtin claim
