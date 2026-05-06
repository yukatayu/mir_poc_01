# 04 — source import and package model

## 1. Current executable surface

Current product alpha executable surface is:

```text
versioned package.mir.json
```

Direct textual `.mir` input remains non-goal for the product alpha CLI.

Therefore this operational sample suite must use:

```text
representative .mir files for explanation
package.mir.json files for executable alpha input
```

## 2. Representative source files

Place these files for human understanding:

```text
world-core/world-core.mir
membership-chat/membership-chat.mir
sugoroku-world/sugoroku-world.mir
```

They should be clearly marked:

```text
# representative source only; current executable input is package.mir.json
```

Do not make `mirrorea-alpha check *.mir` succeed unless final textual grammar support is intentionally implemented in a separate package.

## 3. Import model

Desired source-level imports:

```text
membership-chat imports world-core
sugoroku-world imports membership-chat
```

Current package-level representation:

```json
"dependencies": [
  {
    "package_id": "world-core",
    "version_req": "^0.1",
    "path": "../world-core"
  }
]
```

If current product alpha schema already supports dependencies, use it.
If not, extend schema minimally and add tests.
If dependency execution cannot be implemented safely, mark it as manifest-only and do not claim runtime import resolution.

## 4. Package schema expectations

Each package should include at least:

```text
schema_version
package_id
package_version
package_kind
dependencies
effects
failures
capabilities
witness_requirements
membership_requirements
auth_policy
auth_stack
contracts
observation_policy
redaction_policy
retention_policy
message_recovery_policy
savepoint_policy
native_policy
compatibility
```

Use current product alpha schema names where they already exist. Do not invent incompatible names if the repo schema has established names.

## 5. Boundary types

### WorldCore package

- package kind: `world_core`
- provided surfaces:
  - `world.admit`
  - `world.observe`
  - `world.event_dag`
- failures:
  - `StaleMembership`
  - `MissingCapability`
  - `MissingWitness`

### MembershipChat package

- package kind: `membership_chat`
- dependency: WorldCore
- effects:
  - `JoinWorld`
  - `LeaveWorld`
  - `SendRoomMessage`
- failures:
  - `StaleMembership`
  - `RateLimited`
  - `MissingCapability`
- observation:
  - observer-safe chat messages

### SugorokuWorld package

- package kind: `sugoroku_world`
- dependency: MembershipChat
- effects:
  - `RollDice`
  - `PublishRoll`
  - `HandoffTurn`
  - `ResetGame`
- witnesses:
  - `draw_pub`
  - `handoff_authorized`
- failures:
  - `MissingWitness`
  - `StaleMembership`
  - `NotDiceOwner`
  - `RateLimited`

## 6. Static checks to add or assert

At minimum, product alpha check should identify:

- dependency package exists or is explicitly manifest-only
- dependency version matches
- package kind allowed
- effect row declared
- failure row declared
- capability requirements declared
- witness requirements declared
- observation / redaction / retention policy declared
- native policy disabled unless explicitly allowed

## 7. Diagnostics

If a developer tries unsupported direct `.mir` execution:

```json
{
  "status": "unsupported",
  "diagnostic_code": "direct_mir_non_goal",
  "message": "direct textual .mir input is a product alpha-1 non-goal; use versioned package.mir.json"
}
```

Keep this behavior until textual grammar is intentionally opened.
