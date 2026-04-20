# Problem 2 sample bundle

## この bundle の目的

- Problem 2 の current first line である
  relation decomposition principal / authoritative-room first default / reserve lane split
  を representative pair `p07 / p08` で確認する。
- low-level `memory_order` exact surface、final public witness/provider/artifact contract、
  exhaustive shared-space catalog をここで確定するものではない。

## まず見るサンプル

### `p07-dice-late-join-visible-history`

- representative sample:
  `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
- parser companion:
  `samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt`
- Lean artifact:
  `samples/lean/current-l2/p07-dice-late-join-visible-history/`

### `p08-dice-stale-reconnect-refresh`

- representative sample:
  `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`
- parser companion:
  `samples/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt`
- Lean artifact:
  `samples/lean/current-l2/p08-dice-stale-reconnect-refresh/`

この pair で次を確認する。

- publication / observation / witness / finalization を distinct に保つ
- late join は published history を past として見る
- stale reconnect は fail then refresh として扱う

## 最短 quickstart

### 1. `smoke problem2` で representative pair を一度に確認する

```bash
python3 scripts/current_l2_guided_samples.py smoke problem2
```

見るべき結果:

- `p07` / `p08` の runtime / `matrix problem2` / `bundle problem2` /
  parser companion inspector / `mapping` が順に通る。
- authoritative-room first completion line の representative pair が drift していないことを 1 本で確認できる。

### 2. `matrix problem2` で representative / reserve / negative pair を分けて読む

```bash
python3 scripts/current_l2_guided_samples.py matrix problem2
```

見るべき結果:

- `p07 / p08` が first-line representative として見える。
- `p09` が delegated RNG practical reserve route、
  `p13 / p14` が negative static-stop pair として分かれて見える。

### 3. `bundle problem2` で docs / Lean artifact / anchor spec-report まで一本道で辿る

```bash
python3 scripts/current_l2_guided_samples.py bundle problem2
```

見るべき結果:

- representative pair、reserve route、negative pair、Lean artifact、anchor spec / report が 1 画面で読める。
- final public witness/provider/artifact contract や exhaustive shared-space catalog をまだ確定していない stop line も確認できる。

### 4. parser companion inspector で order-handoff companion surface を直接見る

```bash
cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- \
  samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt \
  --format pretty
```

見るべき結果:

- edge-row principal / stage-block secondary の companion surface が parser-side carrier に戻っていることが分かる。
- final source wording を凍らせず、thin experimental companion として保っている current cut を追える。

## 実行の順番

1. representative pair をそのまま流す

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt \
  --format pretty
```

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt \
  --format pretty
```

2. representative / reserve / negative pair をまとめて見る

```bash
python3 scripts/current_l2_guided_samples.py matrix problem2
```

3. docs / Lean artifact / anchor spec-report まで一本道で辿る

```bash
python3 scripts/current_l2_guided_samples.py bundle problem2
```

4. parser-side companion / mapping まで同じ読みに揃える

```bash
python3 scripts/current_l2_guided_samples.py mapping
```

## reserve / negative sample

- `p09-dice-delegated-rng-provider-placement`
  - delegated RNG practical reserve route
- `p13-dice-late-join-missing-publication-witness`
  - publication witness 欠如 negative
- `p14-dice-late-join-handoff-before-publication`
  - handoff-before-publication negative

これらは current first line を補う reserve / negative sample であり、
distributed fairness theorem や exhaustive shared-space catalog を要求しない first completion line と切り分けて読む。

## 読み方の要点

- `surface_preview`
  - edge-row principal / stage-block secondary / serial-scope reserve
- `authoritative_room_first_scenario_actual_adoption`
  - representative reached pair / reserve route / negative pair の current reading
- `authoritative_room_reserve_strengthening_lane`
  - witness strengthening / delegated RNG / model-check second line の separate status

## 現在の mixed gate 再開点

- `final source-surface handoff wording / final emitted-artifact schema`
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
    と parser companion inspector を合わせて見て、
    edge-row principal / stage-block secondary のまま final source wording と emitted schema を凍らせない current cut を再確認する。
- `final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract`
  - `python3 scripts/current_l2_guided_samples.py matrix problem2`
    で representative / reserve / negative pair の分担を見直しつつ、
    `python3 scripts/current_l2_guided_samples.py bundle problem2`
    で claim / payload split first の current reading を辿る。
- stronger fairness / replay profile は current first completion line の必須条件ではなく、
  representative pair `p07 / p08` と reserve route `p09` の current split を保ったまま later reserve line に残す。
- compressed residual lane は
  `python3 scripts/current_l2_guided_samples.py residuals`
  から、Problem 2 mixed gate lane と true user-spec residual の切り分けを 1 枚で見直す。
- global true user-spec residual は
  `python3 scripts/current_l2_guided_samples.py reopen-map`
  から、exhaustive shared-space catalog / packaging / upper-layer target をまとめて見直す。

## split package status

- `source wording / emitted schema split`
  - close 済み。`python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
    から入り、edge-row principal / stage-block secondary / serial reserve を保ったまま
    source wording / emitted schema residual を witness-provider public-shape residual から切り離して読める。
- `witness-provider public-shape split`
  - close 済み。`python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape`
    から入り、claim / payload split first / route-schema split first を保ったまま
    shared-space public-shape residual を source wording residual から切り離して読める。

## source wording / emitted schema split の入口

```bash
python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema
```

見るべき結果:

- `p07 / p08` representative pair と `p13 / p14` negative pair が、
  source wording / emitted schema residual の representative / supporting set としてまとまって見える。
- `witness-provider public-shape split` が kept separate として表示され、
  source wording / emitted schema residual だけを narrow に読む current cut を確認できる。
- stop line が `final source-surface handoff wording` /
  `final emitted-artifact schema` /
  `final public parser / checker / runtime API`
  に留まり、witness/provider public-shape residual と混ざらないことを確認できる。

## witness-provider public-shape split の入口

```bash
python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape
```

見るべき結果:

- `p07 / p08` representative pair、`p09` reserve route、`p13 / p14` negative pair が、
  witness/provider public-shape residual の representative / supporting set としてまとまって見える。
- `source wording / emitted schema split` が kept separate として表示され、
  witness/provider public-shape residual だけを narrow に読む current cut を確認できる。
- stop line が `final public witness/provider/artifact contract` /
  `stronger fairness / replay profile` /
  `exhaustive shared-space catalog`
  に留まり、source wording / emitted schema residual と混ざらないことを確認できる。

## stop line

- final source-surface handoff wording
- final emitted-artifact / emitted-handoff schema
- final public witness/provider/artifact contract
- stronger fairness / replay profile
- exhaustive shared-space catalog
