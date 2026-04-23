# order_01

## この文書の目的

この文書は、repo にある **Problem 2**

- order / handoff の high-level reading
- authoritative-room first default
- reserve package と negative static-stop

を、実際の sample と実行結果を順に追いながら理解するための入門ガイドである。

この repo では、Problem 2 を今のところ **low-level `memory_order` exact surface** としてではなく、
より実務的な **publish / handoff / observe** の関係として読む。
その current cut を、`p07` から `p14` までの sample で確認する。

## ここで扱う範囲

この文書で扱う主な sample は次の通りである。

- `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
- `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`
- `samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt`
- `samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt`
- `samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt`

扱わないものも先に書いておく。

- low-level `memory_order` exact source surface の確定
- final source wording
- final public witness schema / provider receipt schema
- exhaustive shared-space catalog

## 事前準備

repo root で次を使う。

```bash
python3 --version
cargo --version
```

Problem 2 の基本確認は Rust runtime と helper script だけで追えるので、Lean は必須ではない。

## 最短確認

まず representative bundle が崩れていないことをまとめて見る。

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

2026-04-21 の再実行では `problem2` は `status: "passed"` だった。
詳細を理解したい場合は、以下を順に実行する。

## 1. 代表的な成功例 `p07` を見る

`p07` は「publish 済みの結果を、handoff 後に late joiner が past-visible history として読める」例である。
Problem 2 の first-line representative と考えてよい。

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt \
  --format pretty
```

### 実際の出力例

```text
static_gate: valid
entered_evaluation: true
terminal_outcome: success
steps_executed: 9
```

長い pretty 出力では、さらに次が読める。

```text
- publish_roll_result: player_a -> visible
- handoff_dice_authority: player_a -> player_b
- late_join_view: player_c sees result+owner history
```

つまり current line では、

1. 先に publish する
2. その後に authority を handoff する
3. late joiner は published history を過去として読む

という順序が保たれている。

## 2. stale reconnect を `p08` で見る

`p08` は、古い reconnect message をそのまま成功扱いにせず、
fallback で owner snapshot refresh に回す例である。

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt \
  --format pretty
```

### 実際の出力例

```text
static_gate: valid
terminal_outcome: success
steps_executed: 7
verification_preview:
  formal_hook_status: reached
```

ここでの要点は、「reconnect を success に見せかける」のではなく、
**fail then refresh** として扱うことにある。

## 3. 一見それっぽいが実はダメな 2 つの negative sample を見る

Problem 2 で最も大事なのは `p13` と `p14` である。
どちらも字面だけならもっともらしく見えるが、current line では static stop にする。

### `p13`: publication witness がない

#### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt \
  --format pretty
```

#### 実際の出力例

```text
static_gate: underdeclared
entered_evaluation: false
terminal_outcome: none
steps_executed: 0
```

実際の pretty 出力には次の reason が出る。

```text
missing publication witness before handoff for late-join visibility at root / room / dice_authority
```

これは、「late join で見せたい」と言っているのに、その前提になる publish witness が存在しない、という意味である。

### `p14`: publish より先に handoff している

#### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt \
  --format pretty
```

#### 実際の出力例

```text
static_gate: malformed
entered_evaluation: false
terminal_outcome: none
steps_executed: 0
```

`p13` も `p14` も runtime まで進まない。
これは current line が **順序の破綻を静的に止めている** ことを意味する。

## 4. representative / reserve / negative pair をまとめて materialize する

次の command で、Problem 2 全体を 1 つの output dir にまとめられる。

```bash
python3 scripts/current_l2_guided_samples.py emit-scenario problem2
```

### 実際の出力例

```text
Problem 2 authoritative-room runnable scenario loop
output dir: target/current-l2-guided/problem2-scenario-bundle

- p07-dice-late-join-visible-history: first-line representative
  static_gate: valid
  terminal_outcome: success

- p08-dice-stale-reconnect-refresh: first-line representative
  static_gate: valid
  terminal_outcome: success

- p09-dice-delegated-rng-provider-placement: reserve practical route
  static_gate: valid
  terminal_outcome: success

- p13-dice-late-join-missing-publication-witness: negative static-stop
  static_gate: underdeclared
  terminal_outcome: none

- p14-dice-late-join-handoff-before-publication: negative static-stop
  static_gate: malformed
  terminal_outcome: none
```

この helper は特に便利で、Problem 2 の current cut を

- success pair
- reserve route
- negative pair

の 3 つに分けて一気に見直せる。

## 5. reserve package を順に見る

Problem 2 は representative pair だけでは終わらない。
current line の practical reserve として、witness strengthening と delegated RNG service も separate に置いてある。

### 5-1. auditable authority witness

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness
```

実際の出力例:

```text
auditable authority witness reserve package
output dir: target/current-l2-guided/reserve-packages/auditable-authority-witness

- p07-dice-late-join-visible-history: witness-strengthening reached
  static_gate: valid
  terminal_outcome: success
  witness_strengthening_status: reached

- p08-dice-stale-reconnect-refresh: guard-only non-witness-bearing contrast
  static_gate: valid
  terminal_outcome: success
  witness_strengthening_status: guarded_not_reached
```

ここでは、「authoritative-room first default」は保ったまま、
その上で **auditable witness をどこまで強められるか** を別 lane として見ている。

### 5-2. delegated RNG service

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service
```

実際の出力例:

```text
delegated RNG service reserve package
output dir: target/current-l2-guided/reserve-packages/delegated-rng-service

- p09-dice-delegated-rng-provider-placement: delegated provider placement reached
  static_gate: valid
  terminal_outcome: success
  delegated_rng_service_status: reached

- p07-dice-late-join-visible-history: guard-only authority-rng baseline contrast
  static_gate: valid
  terminal_outcome: success
  delegated_rng_service_status: guarded_not_reached
```

この package の重要点は、**provider placement** と **authority keeps commit** を潰さずに読むことだ。
つまり「乱数を外に出す」と「部屋の状態遷移の責任を渡す」を同じ話にしていない。

## 6. どうして今は low-level `memory_order` を直接出していないのか

Problem 2 を見始めると、「これは結局 memory order の話ではないのか」と感じるはずである。
理解としては半分正しいが、current repo の public-near line はそこを **直接の source surface** にしていない。

今の practical reading は次の通りである。

- publish
- handoff
- observe
- witness
- authoritative-room

を、room-level / scenario-level の関係として扱う。

この読み方の利点は、次の bad pattern を user-visible に説明しやすいことにある。

- publish witness がないのに late join で見せようとする
- publish より先に handoff を置いてしまう
- provider placement と authority placement を混同する

low-level `memory_order` exact source surface は later に残してあり、今この repo で意味がある line は **order / handoff / authoritative-room の high-level reading** である。

## 7. 出力の読み方

Problem 2 の command では、まず次の 4 つを見るとよい。

- `static_gate`
  - `valid` なら static gate を通った
  - `underdeclared` / `malformed` なら static stop
- `entered_evaluation`
  - `true` なら runtime evaluation に入った
  - `false` ならその前で止まった
- `terminal_outcome`
  - `success` なら representative line か reserve line で許可
  - `none` なら runtime まで入っていない
- `steps_executed`
  - 0 なら静的に止まった

補助的に、bundle / reserve helper では次も役立つ。

- `first_line_status`
  - representative first line まで reached しているか
- `reserve_lane_status`
  - reserve package の意味づけまで reached しているか
- `witness_strengthening_status`
  - witness reserve の達成状況
- `delegated_rng_service_status`
  - delegated provider reserve の達成状況

## 8. 一見大丈夫そうだが実はダメな例

初心者が最も引っかかりやすいのは `p14` である。
publish が存在するので、一見すると「必要な perform は全部ある」ように見える。

しかし current line では、

- publish が先
- handoff が後
- observe はそのさらに後

という関係を崩してはいけない。

したがって `p14` はこうなる。

```text
static_gate: malformed
entered_evaluation: false
terminal_outcome: none
```

これは runtime bug ではなく、**設計段階で止めるべき順序違反**として扱っている。

## まとめ

Problem 2 の current line では、すでに次を repo 内で再確認できる。

- `p07 / p08` の representative success pair
- `p09` の delegated RNG reserve route
- `p13 / p14` の negative static-stop pair
- witness strengthening / delegated RNG service の reserve package

ただし、これは low-level `memory_order` exact surface や final public witness/provider contract を確定したことを意味しない。
現在の読みは、**high-level order / handoff / authoritative-room を practical line として成立させ、その上で bad pattern を static stop できることを実演する**ところにある。
