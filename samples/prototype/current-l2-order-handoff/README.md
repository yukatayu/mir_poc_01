# current-l2 order / handoff / shared-space サンプル

## 要約

- Problem 2 の current first line を追うための prototype 群。
- 入口は `p07` と `p08` の representative pair とし、`p09` を reserve practical route、`p13 / p14` を negative static-stop pair として読む。

## まず見るサンプル

### `p07-dice-late-join-visible-history`

- 役割:
  authoritative-room first default profile の representative first sample。
- ここで確認したいこと:
  - publication / observation / witness / finalization を distinct に保つ current reading
  - late join では published history が past として見える current default
  - `authoritative_room_first_scenario_actual_adoption`
  - `authoritative_room_reserve_strengthening_lane`

### `p08-dice-stale-reconnect-refresh`

- 役割:
  stale reconnect fail-then-refresh を representative pair のもう片側として確認する。
- ここで確認したいこと:
  - stale reconnect は silent merge ではなく fail then refresh として読む
  - first scenario summary と reserve strengthening lane summary が drift なく出る

実行例:

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

## reserve / negative として見るサンプル

### `p09-dice-delegated-rng-provider-placement`

- `delegated_rng_service` practical route の representative sample。
- first completion line に昇格させず、reserve route として読む。

### `p13-dice-late-join-missing-publication-witness`

- publication witness 欠如で止まる negative static-stop pair。

### `p14-dice-late-join-handoff-before-publication`

- handoff-before-publication で止まる negative static-stop pair。

## 読み方

- `surface_preview`
  - edge-row principal / stage-block secondary / serial-scope reserve の current reading を示す。
- `authoritative_room_first_scenario_actual_adoption`
  - representative reached pair / reserve route / negative pair の current reading を示す。
- `authoritative_room_reserve_strengthening_lane`
  - witness strengthening / delegated RNG practical route / model-check second line を separate status のまま示す。

## 注意

- これらは **corrected prototype** であり、final source wording / final public witness-provider-artifact contract / exhaustive shared-space catalog を意味しない。
- `atomic_cut` は place-local finalizing cut nucleus であり、global consistent cut / durable commit と同一視しない。
