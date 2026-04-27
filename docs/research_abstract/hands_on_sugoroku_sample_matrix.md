# hands_on_sugoroku_sample_matrix

この文書は、`samples/clean-near-end/sugoroku-world/` の 10 sample を
「どの phase の何を確認しているのか」という観点で読むための短い案内です。

規範判断の正本は `specs/` です。ここでは active runnable family の読み方だけを整理します。

## 1. まず何が動いているか

この family は `scripts/sugoroku_world_samples.py` で実行します。

```bash
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py closeout --format json
```

この runner は single OS process logical multi-place emulator です。
real network transport や final public runtime API ではありません。

## 2. sample ごとの役割

| ID | 役割 | 最初に見るコマンド | 何が分かるか |
|---|---|---|---|
| `SUG-00` | world bootstrap | `run 00_world_bootstrap --debug summary` | 空 world と初期 membership |
| `SUG-01` | runtime attach | `run 01_runtime_attach_game --debug summary` | game place が runtime attach されること |
| `SUG-02` | admin control | `run 02_admin_start_reset --debug summary` | admin-only start/reset |
| `SUG-03` | turn E2E | `run 03_roll_publish_handoff --debug turn-trace` | roll -> publish -> witness -> handoff |
| `SUG-04` | rejection | `run 04_non_owner_roll_rejected --debug summary` | wrong owner action rejection |
| `SUG-05` | late join | `run 05_late_join_history_visible --debug membership` | late join と published history visibility |
| `SUG-06` | leave invalidation | `run 06_leave_non_owner --debug membership` | leave 後の epoch / incarnation 変化 |
| `SUG-07` | owner leave continuity | `run 07_owner_leave_reassign --debug membership` | owner leave 時の reassignment |
| `SUG-08` | verification bridge | `run 08_reset_interleaving_model_check --debug verification` | reset safety と stale handoff invalidation |
| `SUG-09` | detach stop line | `run 09_detach_todo --debug summary` | detach を完了扱いせず TODO として残す |

## 3. 重要な debug 出力

### `summary`

一番短い全体要約です。sample が success したか rejection したか、主な state 変化は何かを追えます。

### `turn-trace`

`SUG-03` を読むときの主出力です。`roll -> publish -> handoff` の順が human-readable に見えます。

### `membership`

`SUG-05` から `SUG-07` を読むときの主出力です。join/leave、epoch、active/pending distinction を追えます。

### `verification`

`SUG-08` の主出力です。static / runtime / model-check の 3 行をどう繋ぐかを見る最短経路です。

### `signatures`

`SUG-03` の副読本です。`run 03_roll_publish_handoff --debug signatures` を使うと、
effect / transition / witness / relation / property の `TermSignature` inventory を helper-local に確認できます。
これは current `TermSignature` first cut の evidence であり、final public visualization protocol ではありません。

### `layers`

`SUG-03` と `SUG-05..07` の副読本です。`run 03_roll_publish_handoff --debug layers` や
membership sample を使うと、current helper が `verification`、`runtime_trace`、`membership` を
どの current layer として読んでいるかを確認できます。これも helper-local first cut であり、
final public layer law schema ではありません。

これらは helper-local debug output であり、final public visualization protocol ではありません。

## 4. phase ごとの読み分け

- `Phase 4 shared-space membership / room boundary`
  - `SUG-00`, `SUG-02`, `SUG-04`, `SUG-05`, `SUG-06`, `SUG-07`
- `Phase 7 Sugoroku runtime attach`
  - `SUG-01`, `SUG-03`, `SUG-07`, `SUG-08`
- `Phase 14 hot-plug / detach`
  - `SUG-09` は preview / stop-line 用であり completion evidence ではない

## 5. ここで意図的に主張していないこと

- real network transport
- multi-server consensus
- durable distributed commit
- final public auth / visualization / hot-plug API
- detach lifecycle complete

つまり、この sample family は current repo-local vertical slice の evidence であって、
Mirrorea system complete の証拠ではありません。
