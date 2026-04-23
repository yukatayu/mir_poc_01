# hands-on: Sugoroku detail

## 目的

この文書は Sugoroku vertical slice の evidence detail です。
全 sample、runtime component、debug mode、reject case、TODO を 1 か所で確認します。

## 全体像

runtime は `scripts/sugoroku_world_samples.py` にあります。
source samples は `samples/clean-near-end/sugoroku-world/` にあります。

## 完全なコード

ここでは、実際に runner が読む `.mir` sample code を全て貼ります。

### 00_world_bootstrap.mir

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

index theory MembershipStatus {
  element Active
  element Inactive
  element Pending

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

### 01_runtime_attach_game.mir

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

### 02_admin_start_reset.mir

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

### 03_roll_publish_handoff.mir

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

### 04_non_owner_roll_rejected.mir

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

### 05_late_join_history_visible.mir

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

### 06_leave_non_owner.mir

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

### 07_owner_leave_reassign.mir

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

### 08_reset_interleaving_model_check.mir

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

### 09_detach_todo.mir

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

runner の主要 component は次です。

```python
PlaceRuntime
MessageQueue
WorldServerPlace
ParticipantPlace
SugorokuGamePlace
MembershipRegistry
SugorokuState
RollRecord
ActionRecord
```

## 行ごとの解説

- `PlaceRuntime` は、logical Place と queue と world/game state を保持します。
- `MessageQueue` は、Place ごとの message queue です。
- `WorldServerPlace` は、world authority を持つ Place です。
- `ParticipantPlace` は、participant ごとの logical Place です。
- `SugorokuGamePlace` は、runtime attach で作る game Place です。
- `MembershipRegistry` は、membership_epoch と MemberRecord を持ちます。
- `SugorokuState` は、game_epoch、phase、admin、turn_order、dice_owner、published_rolls、pending_actions を持ちます。
- `RollRecord` は、roller、draw、epoch、witness を持ちます。
- `ActionRecord` は、actor、incarnation、game_epoch、membership_epoch、kind を持ちます。

## キーワード解説

- `static check`: 実行前または commit 前に条件を確認することです。
- `runtime guard`: 実行時 state を見て action を拒否する guard です。
- `model-check property`: 実行順序の組み合わせに対して守りたい性質です。
- `debug summary`: world/game の短い状態表示です。
- `debug turn-trace`: turn の event trace です。
- `debug membership`: membership registry の表示です。
- `debug verification`: static/runtime/model-check の確認項目です。

## builtin / current companion syntax / user-defined

current companion syntax には、`module`、`index theory`、`principal`、`world`、`place`、`transition`、`stage`、`effect`、`perform`、`publish`、`witness`、`handoff`、`model`、`property` があります。
final public parser grammar ではありません。

user-defined vocabulary には、`GameAuthority`、`GamePhase`、`MembershipStatus`、`WorldMembers`、`SugorokuGame#1`、`Alice`、`Bob`、`Carol`、`Dave` があります。

## コマンド

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

## pretty output

```text
WORLD EmptyWorld
  membership_epoch: 0
  active: Alice, Bob, Carol
  inactive: none
  attached:
    SugorokuGame#1 phase=Running epoch=0 admin=Alice dice_owner=Bob
```

## json output

```json
{
  "sample_count": 10,
  "failed": [],
  "model_check_properties": [
    "no_double_dice_owner",
    "owner_only_rolls",
    "roll_is_published_before_handoff",
    "handoff_target_is_active",
    "late_join_sees_published_history",
    "owner_leave_reassigns_or_pauses",
    "stale_action_after_leave_is_rejected",
    "reset_invalidates_pending_actions",
    "detach_rejects_domain_actions",
    "admin_reset_does_not_interleave_with_roll_commit_badly"
  ]
}
```

## 何が reject されるか

- `bad_reset_by_bob`: authority が Player のため admin reset 不可。
- `bad_roll_by_carol`: current dice owner が Bob のため Carol roll 不可。
- stale action: membership_epoch / member_incarnation が古いため commit 不可。
- old epoch handoff: reset 後 game_epoch が古いため commit 不可。
- detached domain action: detach TODO boundary のため reject または todo_deferred。

## TODO / future layer

- no real network yet。
- no multi-server consensus。
- no durable distributed commit。
- detach is TODO lifecycle boundary。
- final parser grammar remains deferred。
- final public API remains deferred。
