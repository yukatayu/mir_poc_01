# hands-on: Sugoroku 05 join / leave

## 目的

late join、leave、owner leave reassignment を確認します。
membership_epoch と member_incarnation が stale action を拒否するために使われる点が重要です。

## 全体像

Dave は後から join して published history を見られますが、自動で turn_order には入りません。
Carol が leave すると membership_epoch と incarnation が進みます。
Bob が dice owner のまま leave すると、次の active participant へ dice owner が移ります。

この family では、shell-backed parity と game-domain aftermath を分けて読みます。

- shell-backed parity:
  `ParticipantPlace[*]` の登録、`MembershipRegistry` の bootstrap / join / leave、`membership_epoch`、`member_incarnation`
- game-domain aftermath:
  `pending_player`、published-history replay、pending action invalidation、owner reassignment、`turn_order`、`dice_owner`

`P11` current third cut が `mirrorea-core` に actualize しているのは前者だけです。後者は helper-side Sugoroku runtime に残ります。

## 完全なコード

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

## 行ごとの解説

- `principal Dave` は、Dave を player として追加します。
- `add_member WorldMembers Dave` は membership registry に Dave を追加します。
- `increments membership_epoch` は、membership が変わったことを番号で示します。
- `Dave observes ... published_history` は、late joiner が過去の publish を見られることを示します。
- `mark_pending_player` は、Dave がまだ turn_order へ自動挿入されないことを示します。
- `mark_inactive WorldMembers Carol` は Carol を inactive にします。
- `increments member_incarnation(Carol)` は、Carol の古い action を stale として区別できるようにします。
- `invalidate_pending_actions actor Carol` は、Carol の pending action を無効にします。
- `owner_bob_leaves` は、dice owner の Bob が leave する sample です。
- `produces witness membership_updated` は、leave による membership 更新証拠です。
- `handoff dice_owner Bob -> next` は、witness 付きで owner を再割り当てします。
- active next がいなければ、dice owner を clear して phase を Paused にします。

## キーワード解説

- `late join`: game 開始後に入ることです。
- `published_history`: publish 済みの履歴です。
- `pending_player`: turn order への追加判断待ち participant です。
- `member_incarnation`: leave/rejoin で古い action と新しい action を分ける番号です。

## builtin / current companion syntax / user-defined

`add_member`、`observes`、`mark_pending_player`、`mark_inactive`、`invalidate_pending_actions` は current sample companion syntax です。
`Dave`、`WorldMembers`、`ParticipantPlace[Dave]`、`membership_updated` は user-defined sample vocabulary です。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py run 06_leave_non_owner --format json
python3 scripts/sugoroku_world_samples.py run 07_owner_leave_reassign --format json
```

## pretty output

```text
MEMBERSHIP
  epoch: 1
  Alice active incarnation=0 place=ParticipantPlace[Alice]
  Bob active incarnation=0 place=ParticipantPlace[Bob]
  Carol active incarnation=0 place=ParticipantPlace[Carol]
  Dave active incarnation=0 place=ParticipantPlace[Dave]
```

## json output

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

## 何が reject されるか

- inactive member の action。
- 古い incarnation の action。
- leave 後に残った pending action。
- owner leave 後、witness なしで owner を動かす handoff。

## TODO / future layer

- late joiner を turn_order に入れる admin operation は future layer です。
- multi-world movement や portal catalog はまだありません。
