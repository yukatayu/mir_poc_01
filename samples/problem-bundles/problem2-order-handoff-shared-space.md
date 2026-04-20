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

## stop line

- final source-surface handoff wording
- final emitted-artifact / emitted-handoff schema
- final public witness/provider/artifact contract
- stronger fairness / replay profile
- exhaustive shared-space catalog
