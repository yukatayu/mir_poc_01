# hands-on: Sugoroku 06 model check

## 目的

reset と roll/handoff が競合しても、古い game_epoch の handoff が commit されないことを model-check second line として確認します。

## 全体像

static check は authority や owner を見る first line です。
model-check は interleaving、つまり AdminReset と OwnerRoll がどの順で混ざるかを探索する second line です。

## 完全なコード

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

## 行ごとの解説

- `model SugorokuResetInterleaving` は、model-check 用の model です。
- `actor Admin = Alice` は、reset できる actor を Alice とします。
- `actor Owner = Bob` は、dice owner を Bob とします。
- `state game_epoch : Nat = 1` は、現在の game 世代を 1 とします。
- `pending_handoff` は、publish されたがまだ handoff commit 前の状態を表します。
- `process OwnerRoll` は、Bob の roll/publish/handoff の流れです。
- `require dice_owner = Owner` は、owner だけが roll する条件です。
- `pending_handoff <- true` は、handoff 前の pending 状態を作ります。
- `process AdminReset` は、Alice の reset です。
- `game_epoch <- game_epoch + 1` は reset により epoch が進むことを示します。
- `pending_handoff <- false` は、old pending handoff を invalid にします。
- `property no_old_epoch_handoff_after_reset` は、守りたい性質です。
- `never (...)` は、その中の状態が起きてはいけないという意味です。

## キーワード解説

- `model`: model-check 対象です。
- `process`: actor の実行列です。
- `interleaving`: 複数 process の操作が混ざる順序です。
- `property`: 守りたい性質です。
- `counterexample`: property を破る実行例です。

## builtin / current companion syntax / user-defined

`model`、`actor`、`state`、`process`、`property`、`never` は current companion vocabulary です。
`SugorokuResetInterleaving`、`OwnerRoll`、`AdminReset`、`no_old_epoch_handoff_after_reset` は user-defined です。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py model-check --format json
```

## pretty output

```text
MODEL CHECK
  result: pass
  property: no_old_epoch_handoff_after_reset
```

## json output

```json
{
  "sample": "08_reset_interleaving_model_check",
  "property": "no_old_epoch_handoff_after_reset",
  "model_check_result": "pass",
  "explanation": "reset invalidates pending handoff from old epoch",
  "broken_variant": {
    "sample": "08b_reset_interleaving_broken",
    "model_check_result": "counterexample"
  }
}
```

## 何が reject されるか

- reset 後の old game_epoch handoff commit。
- stale member incarnation action。
- Detached game domain action。

## TODO / future layer

- concrete external model checker binding はまだありません。
- property language の final public syntax は未確定です。
- CI artifact / verifier handoff schema は future layer です。

