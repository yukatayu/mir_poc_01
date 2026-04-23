# hands-on: Sugoroku 00 overview

## 目的

この文書は、Mir / Mirrorea の Sugoroku vertical slice 全体を最初に読むための入口です。
ここで作ったものは、1 つの OS process の中で複数の logical `Place` を動かす repo-local emulator です。
実ネットワーク、複数 server、永続 commit はまだ実装しません。

## 全体像

この sample family は次の流れを実行できます。

1. 空の world server を作る。
2. Alice / Bob / Carol を参加者として登録する。
3. runtime に SugorokuGame を attach する。
4. Server が Alice を admin に任命する。
5. Alice が admin として game を start する。
6. dice owner の Alice だけが roll できる。
7. roll 結果を publish し、witness を作る。
8. witness を使って dice owner を Bob へ handoff する。
9. Dave が late join し、published history を見る。
10. leave / owner leave / reset / detach TODO / model-check を確認する。

## 完全なコード

この overview の code は sample catalog です。
各 `.mir` の全文は後続 hands-on と `hands_on_sugoroku_detail.md` に載せています。

```text
samples/clean-near-end/sugoroku-world/
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
```

## 行ごとの解説

- `samples/clean-near-end/sugoroku-world/` は active sample family の置き場所です。
- `00_world_bootstrap.mir` は、まだ game が付いていない world を作ります。
- `01_runtime_attach_game.mir` は、runtime で game package を world に attach します。
- `02_admin_start_reset.mir` は、admin だけが start/reset できることを確認します。
- `03_roll_publish_handoff.mir` は、roll -> publish -> witness -> handoff の中心 sample です。
- `04_non_owner_roll_rejected.mir` は、dice owner ではない Carol の roll を拒否します。
- `05_late_join_history_visible.mir` は、Dave が後から参加して published history を見られることを示します。
- `06_leave_non_owner.mir` は、Carol の leave で membership epoch と incarnation が変わることを示します。
- `07_owner_leave_reassign.mir` は、dice owner の Bob が leave したときに owner を再割り当てします。
- `08_reset_interleaving_model_check.mir` は、reset と handoff の interleaving を model-check します。
- `09_detach_todo.mir` は、detach を Mirrorea lifecycle TODO として残します。

## キーワード解説

- `Place` は、実行場所、queue、状態、権限境界、観測 frontier を持つ場所です。
- `Principal` は、Alice や Bob のような actor です。
- `Participant` は、world membership に入る Principal です。
- `WorldServerPlace` は、world 全体の authority を持つ logical Place です。
- `ParticipantPlace[Alice]` は、Alice に対応する logical Place です。Alice 自身ではありません。
- `SugorokuGamePlace#1` は、runtime attach で作られる game object の Place です。
- `membership_epoch` は、join / leave が起きるたびに増える番号です。
- `member_incarnation` は、同じ名前の参加者が古い action を commit しないための番号です。
- `witness` は、publish が起きたことを後続 action が確認するための証拠です。

## builtin / current companion syntax / user-defined

current helper が認識する companion vocabulary には、`module`、`index theory`、`principal`、`world`、`place`、`transition`、`stage`、`publish`、`witness`、`handoff`、`model`、`property` があります。
これは final public parser grammar ではありません。

user-defined な名前は、`GameAuthority`、`GamePhase`、`MembershipStatus`、`EmptyWorld`、`WorldMembers`、`SugorokuGame#1`、`Alice`、`Bob`、`Carol`、`Dave` です。
これらは言語の魔法の組み込みではなく、sample が宣言します。

## コマンド

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py closeout --format json
```

## pretty output

```text
00_world_bootstrap: Bootstrap an empty world server with Alice, Bob, and Carol as active participants.
01_runtime_attach_game: Attach SugorokuGame#1 at runtime and appoint Alice as game admin.
02_admin_start_reset: Accept admin start and reject non-admin reset.
03_roll_publish_handoff: Dice owner Alice rolls, publishes, witnesses, and hands off to Bob.
```

## json output

```json
{
  "sample_count": 10,
  "failed": [],
  "static_checks": ["admin-only start/reset", "owner-only roll"],
  "runtime_guards": ["handoff requires publish witness"],
  "model_check_properties": ["no_double_dice_owner", "owner_only_rolls"]
}
```

## 何が reject されるか

- admin ではない Bob の reset。
- dice owner ではない Carol の roll。
- witness のない handoff。
- leave 後の stale action。
- reset 前 epoch の pending handoff。
- Detached phase の domain action。

## TODO / future layer

- no real network yet。
- no multi-server consensus。
- no durable distributed commit。
- detach is TODO lifecycle boundary。
- final parser grammar remains deferred。
- final public API remains deferred。

