# hands-on: Sugoroku 04 roll / publish / handoff

## 目的

Sugoroku vertical slice の中心です。
Dice owner の Alice が roll し、結果を publish し、witness を作り、dice owner を Bob へ handoff します。

## 全体像

この sample が示す安全性は次です。

- Running phase でしか roll できない。
- dice owner だけが roll できる。
- active member だけが roll できる。
- handoff は publish の後でなければならない。
- handoff には witness が必要。
- handoff target は active member でなければならない。

## 完全なコード

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

## 行ごとの解説

- `effect roll_dice` は、dice を振る effect です。
- `output draw : Nat` は、自然数の出目を返すことを示します。
- `ensures 1 <= draw` と `ensures draw <= 6` は、出目が 1 から 6 の範囲にあることを示します。
- `cost <= { cpu_steps: 10, remote_calls: 0 }` は、remote call を使わない軽い effect として扱います。
- `transition take_turn_alice by Alice` は、Alice の turn です。
- `requires phase(...) = Running` は、game が開始済みであることを要求します。
- `requires dice_owner(...) = Alice` は、Alice だけがこの turn で roll できることを示します。
- `requires active_member(...)` は、Alice が leave 済みではないことを要求します。
- `perform roll_dice via authority_rng` は、authority_rng 経由で dice を振ります。
- `publish roll_result(...)` は、出目を published history に追加します。
- `produces witness draw_pub` は、publish 証拠を作ります。
- `next_active_after` は、turn_order の次の active member を探します。
- `handoff dice_owner Alice -> next` は、dice owner を次の participant へ渡します。
- `after publish(...)` と `requires witness(draw_pub)` は、公開前 handoff と witness なし handoff を防ぎます。

## キーワード解説

- `effect`: 外部作用または計算 action です。
- `perform`: effect を呼びます。
- `publish`: 後から見える履歴へ書きます。
- `witness`: publish の証拠です。
- `handoff`: ownership / authority を渡します。

## builtin / current companion syntax / user-defined

`effect`、`perform`、`publish`、`handoff`、`after`、`requires witness` は current companion syntax です。
`roll_dice`、`authority_rng`、`roll_result`、`draw_pub` は sample 側の名前です。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --format json
```

## pretty output

```text
TURN Alice
  roll:
    draw = 4
  publish:
    roll_result(Alice, 4)
    witness = draw_pub#1
  handoff:
    from Alice
    to   Bob
    reason: next_active_after(Alice)
  state:
    dice_owner = Bob
    published_rolls = 1
```

## json output

```json
{
  "sample": "03_roll_publish_handoff",
  "static_verdict": "valid",
  "terminal_outcome": "success",
  "roll": {
    "roller": "Alice",
    "draw": 4,
    "published_witness": "draw_pub#1"
  },
  "game": {
    "phase": "Running",
    "dice_owner": "Bob",
    "published_rolls": [{"roller": "Alice", "draw": 4}]
  },
  "properties_passed": [
    "owner_only_rolls",
    "roll_is_published_before_handoff",
    "handoff_target_is_active",
    "no_double_dice_owner"
  ]
}
```

## 何が reject されるか

`04_non_owner_roll_rejected.mir` では Carol が roll しようとして拒否されます。
current dice owner は Bob なので、`dice_owner(SugorokuGame#1) = Carol` が成立しません。

## TODO / future layer

- dice は deterministic draw=4 の helper-local simulation です。
- real RNG provider、provider receipt public schema、final runtime API は未確定です。

