
# Mir / Mirrorea Sugoroku Runtime-Attachment Vertical Slice — CodeX Handoff

**Date:** 2026-04-23  
**Intended reader:** CodeX / sub-agent-pro / implementation agents  
**Goal:** Implement a repo-local runnable Mir / Mirrorea vertical slice where a Sugoroku game is attached to an initially empty world at runtime, played by participants in separate logical Places, and progressively verified through static checking, runtime debugging, and model-checking.

---

## 0. Executive summary

This task implements a new sample family that demonstrates the central Mir / Mirrorea idea:

```text
A world server exists.
Participants already exist in the world.
At runtime, a Sugoroku game package is attached to the world.
The game imports the world membership relation.
The server appoints one participant as game administrator.
The admin starts/resets the game.
The current dice owner rolls, publishes the roll, and hands off dice ownership to the next participant.
Participants may join and leave.
If the dice owner leaves, dice ownership moves to the next active participant or the game pauses.
Game detachment is deferred as a Mirrorea lifecycle TODO.
```

This is not merely a game example. It is a representative sample of:

- runtime attachment of a domain object to a virtual world
- generalized `Place` across physical processes, virtual threads, and authority boxes
- membership inheritance
- authority and capability checks
- publish / witness / handoff ordering
- late join / leave / stale action handling
- model-check second-line properties
- readable debug output
- beginner-friendly hands-on docs

The implementation target is **repo-local alpha-ready current layer**, not final public language completion.

---

## 1. Completion target

### 1.1 In scope

This task should complete the following:

```text
- logical multi-Place emulator
- runtime game attachment
- world membership registry
- Sugoroku game state
- server-appointed admin
- admin-only start/reset
- dice-owner-only roll
- roll -> publish -> witness -> handoff
- late join published-history visibility
- leave and owner-leave reassignment
- reset invalidating old pending actions
- detach TODO
- static checks
- runtime debug output
- model-check second-line scenarios
- hands-on documentation
- repo docs/plan/progress/tasks/report synchronization
```

### 1.2 Out of scope

Do not attempt these in this task:

```text
- real network transport
- physical multi-process deployment
- distributed consensus
- multi-server authority replication
- durable distributed commit
- final parser grammar
- final public parser/checker/runtime/verifier API
- installed binary or packaging
- FFI or game-engine adapter
- exhaustive shared-space catalog
```

Real networking can be implemented later. For now, use logical Places and in-memory queues.

---

## 2. Key concept: Place

Use this definition consistently.

```text
Place =
  physical process / physical machine / OS process / network node
  + virtual thread
  + virtual authority/state/visibility compartment
  + execution locus with local state, queue, effects, capability, and observation frontier
```

A `Place` is not the same as a person.

Use this distinction:

```text
Principal / Participant / Member:
  Who acts or belongs.
  Examples: Alice, Bob, Carol, Dave.

Place:
  Where execution, observation, state, message queue, or authority boundary lives.
  Examples: WorldServerPlace, ParticipantPlace[Alice], SugorokuGamePlace#1.

Capability / Authority:
  What an actor/place is allowed to do.
  Examples: ServerAuthority, GameAdmin, DiceOwner.

Membership:
  Whether a participant is active in the world/game, including epoch and incarnation.

Game object:
  A runtime-attached domain object in the world.
```

Bad wording:

```text
Alice is a Place.
```

Preferred wording:

```text
Alice is a Principal / Participant.
Alice has ParticipantPlace[Alice].
```

---

## 3. Why this sample matters

This sample should demonstrate that Mir / Mirrorea can model:

```text
distributed system over distributed system
runtime world extension
virtual-space object attachment
authority transfer
dynamic membership
causal publication and handoff
verification overlays
```

It is a concrete version of:

```text
Bring a game/gimmick/item into a virtual world server,
attach it at runtime,
inherit the participant relation,
run it safely,
and expose verification without forcing all users to see proof machinery all the time.
```

This sample should be easy enough for a new reader to follow, while still exercising the core architecture.

---

## 4. Architecture of the sample

### 4.1 Places

Use these logical places:

```text
WorldServerPlace
ParticipantPlace[Alice]
ParticipantPlace[Bob]
ParticipantPlace[Carol]
ParticipantPlace[Dave]          # appears later as late joiner
SugorokuGamePlace#1             # created by runtime attach
```

### 4.2 Actors / principals

```text
Server
Alice
Bob
Carol
Dave
```

### 4.3 Membership

Initial world membership:

```text
Alice active
Bob active
Carol active
Dave absent
```

After late join:

```text
Dave active
Dave can observe published history
Dave is not automatically added to turn_order
```

### 4.4 Game roles

```text
Server:
  may attach game
  may appoint game admin

Admin:
  may start/reset game
  initially Alice

Player:
  may play if active and in turn_order

DiceOwner:
  current player who may roll
```

### 4.5 Default turn order

```text
Alice -> Bob -> Carol -> Alice
```

Late joiner does not automatically enter turn order. Add-to-turn-order is a separate future/admin operation.

### 4.6 Detach

Detach should be represented as a TODO:

```text
todo detach_game_from_world
```

Reason:

```text
Detach is a Mirrorea lifecycle operation, not part of the first Sugoroku domain sample.
```

---

## 5. Synchronization model

For the first implementation, use:

```text
single authoritative world server
single authoritative game instance
authoritative serial transition
membership registry as source of truth
published history as append-only under authority
participant views observing published history
membership_epoch and member_incarnation for join/leave/reconnect
```

This is enough for the sample.

Do not attempt:

```text
network partitions
multi-authority merge
consensus
quorum
durable distributed commit
Byzantine behavior
offline reconciliation
```

---

## 6. User-defined index theories

Do not make authority or game phase magical built-ins. Define them in the sample or source-adjacent declaration.

### 6.1 GameAuthority

```mir
index theory GameAuthority {
  element Player
  element Admin
  element Server

  order Player <= Admin
  order Admin <= Server

  law finite_preorder
}
```

Meaning:

```text
Player:
  ordinary participant authority

Admin:
  may start/reset and manage turn participation

Server:
  may attach game and appoint admin
```

### 6.2 GamePhase

```mir
index theory GamePhase {
  element Attached
  element Running
  element Paused
  element Detached

  law finite_set
}
```

Meaning:

```text
Attached:
  game exists but has not started

Running:
  turns are active

Paused:
  game exists but cannot proceed, e.g. no active player

Detached:
  game lifecycle removed; domain actions must not succeed
```

### 6.3 MembershipStatus

```mir
index theory MembershipStatus {
  element Active
  element Inactive
  element Pending

  law finite_set
}
```

### 6.4 Dice ownership

Conceptually:

```mir
capability theory DiceOwnership {
  capability DiceOwner(principal : Principal)
  law unique_owner
}
```

If current parser does not support this syntax, implement an equivalent internal carrier.

---

## 7. Runtime state model

### 7.1 World state

```text
WorldState {
  world_id: WorldId,
  server_place: PlaceId,
  membership_registry: MembershipRegistry,
  attached_components: Map<ComponentId, AttachedComponent>,
  authoritative_event_log: List<EventRecord>
}
```

### 7.2 Membership registry

```text
MembershipRegistry {
  membership_epoch: Nat,
  members: Map<Principal, MemberRecord>
}
```

```text
MemberRecord {
  principal: Principal,
  participant_place: PlaceId,
  active: Bool,
  incarnation: Nat,
  joined_at_epoch: Nat,
  left_at_epoch?: Nat
}
```

### 7.3 Sugoroku state

```text
SugorokuState {
  game_id: GameId,
  attached_world: WorldId,
  game_place: PlaceId,
  game_epoch: Nat,
  phase: GamePhase,
  admin: Principal,
  turn_order: List<Principal>,
  active_players: Set<Principal>,
  pending_players: Set<Principal>,
  dice_owner: Option<Principal>,
  published_rolls: List<RollRecord>,
  pending_actions: Map<ActionId, ActionRecord>
}
```

### 7.4 Roll record

```text
RollRecord {
  roll_id: RollId,
  roller: Principal,
  draw: Nat,
  game_epoch: Nat,
  membership_epoch: Nat,
  published_witness: WitnessId
}
```

### 7.5 Action record

```text
ActionRecord {
  action_id: ActionId,
  actor: Principal,
  actor_incarnation: Nat,
  game_epoch: Nat,
  membership_epoch: Nat,
  kind: ActionKind
}
```

The combination of `game_epoch`, `membership_epoch`, and `actor_incarnation` is essential. It prevents:

```text
- action from old game epoch committing after reset
- stale reconnect committing as current action
- action from inactive member being accepted silently
```

---

## 8. Sample bucket

Create:

```text
samples/clean-near-end/sugoroku-world/
```

Required files:

```text
00_world_bootstrap.mir
01_runtime_attach_game.mir
02_admin_start_reset.mir
03_roll_publish_handoff.mir
04_non_owner_roll_rejected.mir
05_late_join_history_visible.mir
06_leave_non_owner.mir
07_owner_leave_reassign.mir
08_reset_interleaving_model_check.mir
09_detach_todo.mir
README.md
```

Use the exact content below as the starting point. If parser syntax must differ, preserve semantics and document changes.

---

## 9. Sample 00 — world bootstrap

```mir
module CleanNearEnd.SugorokuWorldBootstrap

// GameAuthority is user-defined.
// It is not a built-in privilege hierarchy.
index theory GameAuthority {
  element Player
  element Admin
  element Server

  order Player <= Admin
  order Admin <= Server

  law finite_preorder
}

// GamePhase is also user-defined for this sample.
index theory GamePhase {
  element Attached
  element Running
  element Paused
  element Detached

  law finite_set
}

// Principals are actors.
// They are not Places.
principal Server : GameAuthority.Server
principal Alice  : GameAuthority.Player
principal Bob    : GameAuthority.Player
principal Carol  : GameAuthority.Player

// The world contains Places.
// ParticipantPlace[Alice] is a logical place associated with Alice.
// This emulates a separate process/thread/node without real networking.
world EmptyWorld {
  place WorldServerPlace

  place ParticipantPlace[Alice]
  place ParticipantPlace[Bob]
  place ParticipantPlace[Carol]

  membership_registry WorldMembers {
    epoch 0

    member Alice {
      place ParticipantPlace[Alice]
      status Active
      incarnation 0
    }

    member Bob {
      place ParticipantPlace[Bob]
      status Active
      incarnation 0
    }

    member Carol {
      place ParticipantPlace[Carol]
      status Active
      incarnation 0
    }
  }
}
```

Expected pretty output:

```text
WORLD EmptyWorld
  server place: WorldServerPlace
  membership epoch: 0
  active members:
    - Alice @ ParticipantPlace[Alice] incarnation=0
    - Bob   @ ParticipantPlace[Bob]   incarnation=0
    - Carol @ ParticipantPlace[Carol] incarnation=0
  attached components:
    none
```

Expected JSON shape:

```json
{
  "sample": "00_world_bootstrap",
  "static_verdict": "valid",
  "world": "EmptyWorld",
  "membership_epoch": 0,
  "active_members": ["Alice", "Bob", "Carol"],
  "places": [
    "WorldServerPlace",
    "ParticipantPlace[Alice]",
    "ParticipantPlace[Bob]",
    "ParticipantPlace[Carol]"
  ]
}
```

---

## 10. Sample 01 — runtime attach game

```mir
module CleanNearEnd.SugorokuRuntimeAttach

use CleanNearEnd.SugorokuWorldBootstrap

// SugorokuGame is a game package.
// The package may be checked before runtime.
// Attachment happens at runtime.
game package SugorokuGame {
  requires membership_registry WorldMembers
  requires authority(Server) >= GameAuthority.Server

  state SugorokuState {
    game_epoch : Nat
    phase : GamePhase
    admin : Principal
    turn_order : List[Principal]
    dice_owner : Option[Principal]
    published_rolls : List[RollRecord]
  }
}

// Runtime transition.
// This is not compile-time inclusion.
transition attach_sugoroku_game by Server at WorldServerPlace {
  stage load_package:
    package <- load_game_package SugorokuGame
      ensure package.checked = true

  stage attach:
    game <- attach package to EmptyWorld
      import membership from WorldMembers
      produces place SugorokuGamePlace#1
      produces component SugorokuGame#1

  stage appoint_admin:
    appoint Alice as GameAuthority.Admin
      for SugorokuGame#1
      requires authority(Server) >= GameAuthority.Server

  stage initialize:
    initialize_turn_order SugorokuGame#1 [Alice, Bob, Carol]
    set_dice_owner SugorokuGame#1 Alice
    set_phase SugorokuGame#1 GamePhase.Attached
}
```

Expected pretty output:

```text
ATTACH SugorokuGame#1
  world: EmptyWorld
  package checked: yes
  imported membership epoch: 0
  game place: SugorokuGamePlace#1
  admin: Alice
  turn order: Alice -> Bob -> Carol
  dice owner: Alice
  phase: Attached
```

---

## 11. Sample 02 — admin start/reset

```mir
module CleanNearEnd.SugorokuAdminStartReset

use CleanNearEnd.SugorokuRuntimeAttach

transition start_game by Alice at SugorokuGamePlace#1 {
  requires authority(Alice) >= GameAuthority.Admin
  requires phase(SugorokuGame#1) = GamePhase.Attached

  stage start:
    set_phase SugorokuGame#1 GamePhase.Running

  stage publish_start:
    publish game_started(SugorokuGame#1, game_epoch(SugorokuGame#1))
      to SugorokuGame#1.published_history
      produces witness game_started_witness
}

transition bad_reset_by_bob by Bob at SugorokuGamePlace#1 {
  requires authority(Bob) >= GameAuthority.Admin

  stage reset:
    reset_game SugorokuGame#1
}
```

Expected for `start_game`:

```json
{
  "transition": "start_game",
  "static_verdict": "valid",
  "terminal_outcome": "success",
  "phase_after": "Running",
  "witnesses": ["game_started_witness"]
}
```

Expected for `bad_reset_by_bob`:

```json
{
  "transition": "bad_reset_by_bob",
  "static_verdict": "malformed",
  "reason_family": "authority_preorder_constraint_failed",
  "constraints_failed": [
    "GameAuthority.Player >= GameAuthority.Admin"
  ],
  "entered_evaluation": false
}
```

---

## 12. Sample 03 — roll / publish / handoff

```mir
module CleanNearEnd.SugorokuRollPublishHandoff

use CleanNearEnd.SugorokuAdminStartReset

effect roll_dice {
  output draw : Nat
  ensures 1 <= draw
  ensures draw <= 6
  cost <= { cpu_steps: 10, remote_calls: 0 }
}

transition take_turn_alice by Alice at SugorokuGamePlace#1 {
  requires phase(SugorokuGame#1) = GamePhase.Running
  requires dice_owner(SugorokuGame#1) = Alice
  requires active_member(WorldMembers, Alice)

  stage roll:
    draw <- perform roll_dice via authority_rng
      ensure 1 <= draw
      ensure draw <= 6

  stage publish:
    publish roll_result(SugorokuGame#1, Alice, draw)
      to SugorokuGame#1.published_history
      produces witness draw_pub

  stage handoff:
    next <- next_active_after(Alice, turn_order(SugorokuGame#1))

    handoff dice_owner Alice -> next
      after publish(roll_result(SugorokuGame#1, Alice, draw))
      requires witness(draw_pub)
      ensure dice_owner(SugorokuGame#1) = next
}
```

Expected pretty output:

```text
TURN Alice
  roll:
    draw = 4
  publish:
    roll_result(Alice, 4)
    witness = draw_pub
  handoff:
    from Alice
    to   Bob
    reason: next_active_after(Alice)
  state:
    dice_owner = Bob
    published_rolls = 1
```

---

## 13. Sample 04 — non-owner roll rejected

```mir
module CleanNearEnd.SugorokuNonOwnerRollRejected

use CleanNearEnd.SugorokuRollPublishHandoff

transition bad_roll_by_carol by Carol at SugorokuGamePlace#1 {
  requires phase(SugorokuGame#1) = GamePhase.Running
  requires dice_owner(SugorokuGame#1) = Carol
  requires active_member(WorldMembers, Carol)

  stage roll:
    draw <- perform roll_dice via authority_rng
}
```

Expected:

```json
{
  "sample": "04_non_owner_roll_rejected",
  "static_or_runtime_verdict": "reject",
  "reason_family": "dice_owner_requirement_failed",
  "required": "dice_owner(SugorokuGame#1) = Carol",
  "actual": "Bob"
}
```

If current state is statically known, reject statically. Otherwise reject at runtime guard with audit evidence.

---

## 14. Sample 05 — late join

```mir
module CleanNearEnd.SugorokuLateJoin

use CleanNearEnd.SugorokuRollPublishHandoff

principal Dave : GameAuthority.Player

transition dave_joins_world by Server at WorldServerPlace {
  requires authority(Server) >= GameAuthority.Server

  stage add_member:
    add_member WorldMembers Dave
      place ParticipantPlace[Dave]
      status Active
      increments membership_epoch

  stage observe_game:
    Dave observes SugorokuGame#1.published_history
      after add_member(WorldMembers, Dave)
      ensure visible_to(Dave, SugorokuGame#1.published_history)

  stage turn_policy:
    mark_pending_player SugorokuGame#1 Dave
      reason "late joiner sees history but is not automatically inserted into turn_order"
}
```

Expected:

```json
{
  "sample": "05_late_join_history_visible",
  "terminal_outcome": "success",
  "membership_epoch_incremented": true,
  "Dave": {
    "active": true,
    "published_history_visible": true,
    "in_turn_order": false,
    "pending_player": true
  }
}
```

---

## 15. Sample 06 — leave non-owner

```mir
module CleanNearEnd.SugorokuLeaveNonOwner

use CleanNearEnd.SugorokuLateJoin

transition carol_leaves by Carol at ParticipantPlace[Carol] {
  requires active_member(WorldMembers, Carol)

  stage leave:
    mark_inactive WorldMembers Carol
      increments membership_epoch
      increments member_incarnation(Carol)

  stage invalidate:
    invalidate_pending_actions actor Carol
      reason member_left
}
```

Expected:

```json
{
  "sample": "06_leave_non_owner",
  "terminal_outcome": "success",
  "member": "Carol",
  "active_after": false,
  "pending_actions_invalidated": true
}
```

---

## 16. Sample 07 — owner leave reassign

```mir
module CleanNearEnd.SugorokuOwnerLeaveReassign

use CleanNearEnd.SugorokuRollPublishHandoff

transition owner_bob_leaves by Bob at ParticipantPlace[Bob] {
  requires dice_owner(SugorokuGame#1) = Bob
  requires active_member(WorldMembers, Bob)

  stage leave:
    mark_inactive WorldMembers Bob
      increments membership_epoch
      increments member_incarnation(Bob)
      produces witness membership_updated

  stage reassign_owner:
    next <- next_active_after(Bob, turn_order(SugorokuGame#1))

    if next exists {
      handoff dice_owner Bob -> next
        after leave(Bob)
        requires witness(membership_updated)
        ensure dice_owner(SugorokuGame#1) = next
    } else {
      clear_dice_owner SugorokuGame#1
      set_phase SugorokuGame#1 GamePhase.Paused
    }
}
```

Expected if Alice is next active:

```json
{
  "sample": "07_owner_leave_reassign",
  "terminal_outcome": "success",
  "left_member": "Bob",
  "new_dice_owner": "Alice",
  "membership_epoch_incremented": true,
  "used_witness": "membership_updated"
}
```

---

## 17. Sample 08 — reset interleaving model-check

This is model-check second-line.

```mir
model SugorokuResetInterleaving {
  actor Admin = Alice
  actor Owner = Bob

  state game_epoch : Nat = 1
  state phase : GamePhase = Running
  state dice_owner : Principal = Bob
  state pending_handoff : Bool = false

  process OwnerRoll {
    require dice_owner = Owner
    action roll
    action publish
    pending_handoff <- true
    action handoff_to_next
  }

  process AdminReset {
    require authority(Admin) >= GameAuthority.Admin
    action reset
    game_epoch <- game_epoch + 1
    pending_handoff <- false
    dice_owner <- Alice
  }

  property no_old_epoch_handoff_after_reset:
    never (
      pending_handoff_from_epoch(1)
      and game_epoch = 2
      and commit_handoff_from_epoch(1)
    )
}
```

Expected pass:

```json
{
  "sample": "08_reset_interleaving_model_check",
  "property": "no_old_epoch_handoff_after_reset",
  "model_check_result": "pass",
  "explanation": "reset invalidates pending handoff from old epoch"
}
```

Broken variant expected:

```json
{
  "sample": "08b_reset_interleaving_broken",
  "property": "no_old_epoch_handoff_after_reset",
  "model_check_result": "counterexample",
  "trace": [
    "Owner rolls in epoch 1",
    "Owner publishes in epoch 1",
    "Admin resets game to epoch 2",
    "Old owner handoff from epoch 1 commits after reset"
  ]
}
```

---

## 18. Sample 09 — detach TODO

```mir
module CleanNearEnd.SugorokuDetachTodo

use CleanNearEnd.SugorokuRuntimeAttach

transition request_detach_game by Server at WorldServerPlace {
  requires authority(Server) >= GameAuthority.Server

  stage request:
    request_detach SugorokuGame#1
      reason "server requested game removal"

  stage todo_lifecycle:
    todo detach_game_from_world
      reason "Mirrorea lifecycle operation; not part of first Sugoroku domain sample"
}
```

Expected:

```json
{
  "sample": "09_detach_todo",
  "static_verdict": "valid",
  "terminal_outcome": "todo_deferred",
  "deferred_to": "Mirrorea lifecycle layer"
}
```

---

## 19. Debug output design

Implement readable debug modes:

```text
--debug summary
--debug turn-trace
--debug membership
--debug verification
--format json
```

### Summary

```text
WORLD EmptyWorld
  membership_epoch: 2
  active: Alice, Bob, Dave
  inactive: Carol
  attached:
    SugorokuGame#1 phase=Running epoch=1 admin=Alice dice_owner=Bob
```

### Turn trace

```text
TURN TRACE
  [1] Alice roll draw=4
  [2] publish roll_result(Alice, 4) witness=draw_pub#1
  [3] handoff dice_owner Alice -> Bob using witness=draw_pub#1
```

### Membership

```text
MEMBERSHIP
  epoch: 2
  Alice active incarnation=0 place=ParticipantPlace[Alice]
  Bob   active incarnation=0 place=ParticipantPlace[Bob]
  Carol inactive incarnation=1 place=ParticipantPlace[Carol]
  Dave  active incarnation=0 place=ParticipantPlace[Dave]
```

### Verification

```text
VERIFICATION
  static:
    ✓ authority(Alice) >= Admin
    ✓ dice_owner(game) = Alice
    ✓ handoff has witness(draw_pub)
  runtime:
    ✓ Alice active at membership_epoch 0
    ✓ game_epoch action matches current epoch
  model-check:
    ✓ no_double_dice_owner
    ✓ late_join_sees_published_history
```

Default output should not be too verbose. Full internals should require `--verbose` or `--format json`.

---

## 20. Implementation plan

### A. Add sample files

Add `samples/clean-near-end/sugoroku-world/*`.

### B. Add logical runtime

Implement:

```text
PlaceRuntime
MessageQueue
WorldServerPlace
ParticipantPlace
SugorokuGamePlace
```

### C. Add membership registry

Implement epoch / incarnation.

### D. Add Sugoroku state and transitions

Implement attach / start / reset / roll / publish / handoff / join / leave / owner leave.

### E. Add static checks

Implement admin-only, owner-only, witness, active target, stale epoch checks.

### F. Add model-check scenarios

Implement reset interleaving and owner leave properties.

### G. Add debug output

Implement pretty + JSON output.

### H. Add hands-on docs

Generate staged beginner docs.

### I. Validate and report

Run tests and create report.

---

## 21. Commands

Implement or adapt these commands:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 00_world_bootstrap
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout
```

Run docs validation:

```bash
python3 scripts/validate_docs.py
```

Run targeted tests. Add tests if missing:

```text
sugoroku_world_bootstrap_valid
sugoroku_runtime_attach_valid
sugoroku_admin_start_valid
sugoroku_non_admin_reset_rejected
sugoroku_owner_roll_valid
sugoroku_non_owner_roll_rejected
sugoroku_handoff_requires_witness
sugoroku_handoff_before_publish_rejected
sugoroku_late_join_history_visible
sugoroku_leave_increments_membership_epoch
sugoroku_owner_leave_reassigns
sugoroku_reset_invalidates_pending_actions
sugoroku_detach_todo_deferred
```

---

## 22. Hands-on docs to generate

Create:

```text
docs/research_abstract/hands_on_sugoroku_00_overview.md
docs/research_abstract/hands_on_sugoroku_01_world_bootstrap.md
docs/research_abstract/hands_on_sugoroku_02_runtime_attach.md
docs/research_abstract/hands_on_sugoroku_03_admin_start_reset.md
docs/research_abstract/hands_on_sugoroku_04_roll_publish_handoff.md
docs/research_abstract/hands_on_sugoroku_05_join_leave.md
docs/research_abstract/hands_on_sugoroku_06_model_check.md
docs/research_abstract/hands_on_sugoroku_detail.md
```

Each must include:

```text
- purpose
- full code
- line-by-line explanation
- keyword glossary
- user-defined vs built-in/current companion distinction
- command
- expected pretty output
- expected JSON
- what gets rejected
- what remains TODO/future
```

Beginner-friendly. No undefined terminology.

---

## 23. Docs and plan updates

Update if relevant:

```text
README.md
Documentation.md
progress.md
tasks.md
specs/00-document-map.md
specs/10-open-questions.md
specs/11-roadmap-and-workstreams.md
plan/01-status-at-a-glance.md
plan/12-open-problems-and-risks.md
plan/13-heavy-future-workstreams.md
plan/16-shared-space-membership-and-example-boundary.md
plan/17-research-phases-and-autonomy-gates.md
plan/90-source-traceability.md
```

Only update `specs/12-decision-register.md` if evidence supports a decision row.

---

## 24. Report

Create:

```text
docs/reports/<next>-sugoroku-world-runtime-attachment-vertical-slice.md
```

Report must include:

```text
summary
implemented files
sample list
runtime model
Place model
membership model
static checks
model-check properties
debug output
validation command results
hands-on docs
limitations
next steps
```

Limitations must include:

```text
- no real network yet
- no multi-server consensus
- no durable distributed commit
- detach is TODO lifecycle boundary
- final parser grammar remains deferred
- final public API remains deferred
```

---

## 25. Final CodeX response

Return:

```text
- implemented samples:
- runtime components:
- static checks:
- model-check properties:
- debug output modes:
- generated hands-on docs:
- tests run:
- validation results:
- report path:
- remaining limitations:
- next recommended step:
```

---

# End of handoff
