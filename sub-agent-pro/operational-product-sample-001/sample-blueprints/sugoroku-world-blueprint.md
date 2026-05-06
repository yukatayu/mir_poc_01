# sugoroku-world blueprint

## Purpose

`SugorokuWorld` imports `MembershipChat` and adds a small game with server-side and participant-side behavior.

It should demonstrate:

- server-side game state
- participant actions
- roll / publish / witness / handoff
- stale membership reject
- missing witness reject
- save/load visibility
- hot-plug layer visibility

## Representative source: sugoroku-world.mir

```mir
// representative source only; current executable input is package.mir.json
import MembershipChat version "^0.1"

module SugorokuWorld version "0.1.0"

place SugorokuGamePlace

capability RollDice
capability PublishRoll
capability HandoffTurn
capability ResetGame

witness draw_pub

state dice_owner : Principal
state game_epoch : Nat
state board : BoardState

perform roll_dice(player : Principal)
  require membership.active(player)
  require player == dice_owner
  require capability(RollDice)
  output draw : Nat
  publish roll_result(player, draw)
  produces witness draw_pub

perform handoff_turn(from : Principal, to : Principal)
  require witness(draw_pub)
  require membership.active(to)
  require capability(HandoffTurn)
  ensure dice_owner = to
  publish handoff(from, to)

perform reset_game(admin : Principal)
  require capability(ResetGame)
  atomic_cut reset_frontier
  ensure game_epoch = game_epoch + 1
  publish game_reset(game_epoch)

observe board via sugoroku_board_panel
observe roll_result via sugoroku_event_panel
```

## package.mir.json intent

```json
{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "ops.sugoroku-world",
  "package_version": "0.1.0",
  "package_kind": "sugoroku_world",
  "dependencies": [
    {
      "package_id": "ops.membership-chat",
      "version_req": "^0.1",
      "path": "../membership-chat"
    }
  ],
  "effects": ["RollDice", "PublishRoll", "HandoffTurn", "ResetGame"],
  "failures": ["StaleMembership", "MissingCapability", "MissingWitness", "NotDiceOwner", "RateLimited"],
  "capabilities": ["RollDice", "PublishRoll", "HandoffTurn", "ResetGame"],
  "witness_requirements": ["draw_pub"],
  "contracts": [
    {
      "surface": "sugoroku.roll",
      "preconditions": ["active membership", "current dice owner", "RollDice capability"],
      "postconditions": ["roll result published", "draw_pub witness created"]
    },
    {
      "surface": "sugoroku.handoff",
      "preconditions": ["draw_pub witness", "active target", "HandoffTurn capability"],
      "postconditions": ["dice owner changes or explicit reject"]
    }
  ],
  "savepoint_policy": {
    "supported": ["R0_Local", "R2_Quiescent"],
    "distributed_durable": false
  },
  "native_policy": {
    "NativeExecutionPolicy": "Disabled"
  }
}
```

## Expected runtime evidence

If executable in P-OPS-01:

- check accepted
- run-local emits session id
- event DAG contains roll/publish/witness/handoff or declared equivalent
- attach debug/auth/rate-limit works against same session
- R0 save and R2 quiescent-save work
- devtools export shows game panel

If only manifest/check is possible:

- document runtime behavior as planned
- do not claim Sugoroku product runtime execution
