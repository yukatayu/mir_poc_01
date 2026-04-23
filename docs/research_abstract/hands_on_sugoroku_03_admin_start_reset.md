# hands-on: Sugoroku 03 admin start/reset

## 目的

admin だけが game を start/reset できることを確認します。
この sample では Alice の start は通り、Bob の reset は権限不足で拒否されます。

## 全体像

Alice は Server から admin に任命されています。
Bob は Player のままです。
そのため `authority(Bob) >= GameAuthority.Admin` は false です。

## 完全なコード

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

## 行ごとの解説

- `transition start_game by Alice` は、Alice が start する処理です。
- `requires authority(Alice) >= GameAuthority.Admin` は、Alice が admin 以上であることを要求します。
- `requires phase(...) = GamePhase.Attached` は、まだ Running ではないことを要求します。
- `set_phase ... Running` は game を開始状態にします。
- `publish game_started(...)` は、開始したことを published history に残します。
- `produces witness game_started_witness` は、開始 publish の witness を作ります。
- `transition bad_reset_by_bob by Bob` は、Bob が reset しようとする失敗例です。
- `requires authority(Bob) >= GameAuthority.Admin` は満たされません。
- `reset_game` の stage には入りません。

## キーワード解説

- `admin`: start/reset できる role です。
- `phase`: game の状態です。
- `publish`: 後から観測できる履歴へ記録します。
- `witness`: publish の証拠です。
- `malformed`: 実行前に拒否されたという verdict です。

## builtin / current companion syntax / user-defined

`transition`、`requires`、`stage`、`publish`、`witness` は current companion vocabulary です。
`GameAuthority.Admin`、`GamePhase.Attached`、`GamePhase.Running` は user-defined sample theory の要素です。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 02_admin_start_reset --format json
```

## pretty output

この sample は mixed result なので、pretty では JSON 形の要約が表示されます。

```text
start_game: valid success
bad_reset_by_bob: malformed authority_preorder_constraint_failed
```

## json output

```json
{
  "sample": "02_admin_start_reset",
  "static_verdict": "mixed",
  "terminal_outcome": "success",
  "transitions": [
    {
      "transition": "start_game",
      "static_verdict": "valid",
      "terminal_outcome": "success",
      "phase_after": "Running",
      "witnesses": ["game_started_witness"]
    },
    {
      "transition": "bad_reset_by_bob",
      "static_verdict": "malformed",
      "reason_family": "authority_preorder_constraint_failed",
      "constraints_failed": ["GameAuthority.Player >= GameAuthority.Admin"],
      "entered_evaluation": false
    }
  ]
}
```

## 何が reject されるか

- Bob の reset。
- admin ではない actor の start/reset。
- Attached ではない phase からの start。

## TODO / future layer

- final public checker API は未確定です。
- ここでは finite authority preorder の helper-local check として扱います。

