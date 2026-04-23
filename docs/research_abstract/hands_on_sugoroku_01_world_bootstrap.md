# hands-on: Sugoroku 01 world bootstrap

## 目的

空の world server を作り、Alice / Bob / Carol を active member として登録する最初の sample を読みます。
ここで一番大事なのは、Participant と Place を混同しないことです。

## 全体像

`00_world_bootstrap.mir` は、まだ SugorokuGame を attach しません。
WorldServerPlace と ParticipantPlace だけを作ります。

## 完全なコード

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

## 行ごとの解説

- `module` は、この sample の名前を付けます。
- `index theory GameAuthority` は、権限の小さな有限世界を sample 側で宣言します。
- `Player <= Admin <= Server` は、Server が最も強い権限を持つという比較です。
- `GamePhase` は、game の状態名です。まだ user-defined です。
- `MembershipStatus` は、member が active / inactive / pending かを表します。
- `principal Server` は actor です。Place ではありません。
- `world EmptyWorld` は、空の仮想 world を宣言します。
- `place WorldServerPlace` は、server authority の logical execution locus です。
- `place ParticipantPlace[Alice]` は、Alice に対応する Place です。Alice そのものではありません。
- `membership_registry WorldMembers` は、member 一覧と epoch を持つ registry です。
- `epoch 0` は、まだ join / leave が起きていない初期状態です。
- `incarnation 0` は、各 member の最初の incarnation です。

## キーワード解説

- `index theory`: 有限の名前と順序を宣言する current companion syntax です。
- `principal`: 行為者です。
- `place`: 実行場所と状態境界です。
- `membership_registry`: join / leave と active 状態を管理する registry です。
- `incarnation`: 古い action を識別して拒否するための世代番号です。

## builtin / current companion syntax / user-defined

`module`、`index theory`、`principal`、`world`、`place`、`membership_registry` は current companion vocabulary です。
final parser grammar ではありません。

`GameAuthority`、`GamePhase`、`MembershipStatus`、`EmptyWorld`、`WorldMembers`、`Alice`、`Bob`、`Carol` は user-defined です。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 00_world_bootstrap
python3 scripts/sugoroku_world_samples.py run 00_world_bootstrap --format json
```

## pretty output

```text
WORLD EmptyWorld
  server place: WorldServerPlace
  membership epoch: 0
  active members:
    - Alice @ ParticipantPlace[Alice] incarnation=0
    - Bob @ ParticipantPlace[Bob] incarnation=0
    - Carol @ ParticipantPlace[Carol] incarnation=0
  attached components:
    none
```

## json output

```json
{
  "sample": "00_world_bootstrap",
  "static_verdict": "valid",
  "terminal_outcome": "success",
  "world": "EmptyWorld",
  "membership_epoch": 0,
  "active_members": ["Alice", "Bob", "Carol"],
  "places": ["WorldServerPlace", "ParticipantPlace[Alice]", "ParticipantPlace[Bob]", "ParticipantPlace[Carol]"]
}
```

## 何が reject されるか

この sample 自体には reject transition はありません。
ただし Participant を Place と同一視する読みは拒否すべき誤読です。

## TODO / future layer

- real network はまだありません。
- ParticipantPlace は single process 内の logical Place です。
- final public syntax は未確定です。

