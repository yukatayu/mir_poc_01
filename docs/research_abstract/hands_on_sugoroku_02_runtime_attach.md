# hands-on: Sugoroku 02 runtime attach

## 目的

空の world に runtime で SugorokuGame を attach する sample を読みます。
compile-time include ではなく、world が動いているところへ game object を追加することが主題です。

## 全体像

`01_runtime_attach_game.mir` は、`SugorokuGamePlace#1` を作り、WorldMembers を import し、Server が Alice を admin に任命します。

## 完全なコード

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

## 行ごとの解説

- `use CleanNearEnd.SugorokuWorldBootstrap` は、world bootstrap の前提を使います。
- `game package SugorokuGame` は、runtime attach できる package を宣言します。
- `requires membership_registry WorldMembers` は、world の membership を使うという要求です。
- `requires authority(Server) >= GameAuthority.Server` は、attach には Server authority が必要という要求です。
- `state SugorokuState` は、game が持つ状態の形です。
- `game_epoch` は reset で増える game 世代です。
- `phase` は Attached / Running / Paused / Detached の状態です。
- `turn_order` は Alice -> Bob -> Carol の順番です。
- `dice_owner` は今 roll できる participant です。
- `published_rolls` は publish 済み roll の履歴です。
- `transition attach_sugoroku_game by Server at WorldServerPlace` は、Server が server place で attach する transition です。
- `load_package` は package を読みます。
- `ensure package.checked = true` は、package が検査済みであることを要求します。
- `attach package to EmptyWorld` は runtime attach です。
- `produces place SugorokuGamePlace#1` は、新しい game Place ができることを示します。
- `appoint Alice as GameAuthority.Admin` は、Alice を admin にします。
- `initialize_turn_order`、`set_dice_owner`、`set_phase` は初期状態を作ります。

## キーワード解説

- `game package`: runtime attach できる domain package です。
- `requires`: 前提条件です。
- `state`: component が持つ状態です。
- `attach`: runtime で world へ component を追加します。
- `produces place`: attach により Place が生まれることを示します。

## builtin / current companion syntax / user-defined

`game package`、`requires`、`state`、`transition`、`stage`、`attach`、`produces place` は current companion syntax です。
final public API ではありません。

`SugorokuGame`、`SugorokuState`、`WorldMembers`、`SugorokuGamePlace#1`、`SugorokuGame#1` は user-defined sample vocabulary です。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --format json
```

## pretty output

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

## json output

```json
{
  "sample": "01_runtime_attach_game",
  "static_verdict": "valid",
  "terminal_outcome": "success",
  "world": "EmptyWorld",
  "package_checked": true,
  "imported_membership_epoch": 0,
  "game": {
    "game_place": "SugorokuGamePlace#1",
    "phase": "Attached",
    "admin": "Alice",
    "turn_order": ["Alice", "Bob", "Carol"],
    "dice_owner": "Alice"
  }
}
```

## 何が reject されるか

- Server authority なしの attach。
- membership registry を import しない package。
- package.checked が false の package。

## TODO / future layer

- real dynamic loading はまだありません。
- attach は Python helper 内の logical runtime operation です。
- Mirrorea lifecycle の detach は別 sample で TODO として扱います。

