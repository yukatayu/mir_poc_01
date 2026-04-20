# 574 — current L2 Problem 2 public-shape residual bundle matrix

## 目的

Problem 2 の representative sample bundle `p07 / p08 / p09 / p13 / p14` について、first line / reserve practical route / negative static-stop pair / public-shape residual を helper-local matrix として読めるようにする。

## current recommendation

- `p07 / p08` を authoritative-room first-line representative pair に置く。
- `p09` は delegated RNG practical reserve route に置く。
- `p13 / p14` は negative static-stop pair に置く。
- `order_handoff_source_surface_artifact_actual_adoption`、`authoritative_room_first_scenario_actual_adoption`、`order_handoff_witness_provider_public_seam_compression`、`authoritative_room_reserve_strengthening_lane` は final public contract ではなく helper-local residual matrix として読む。

## actualization

- `scripts/current_l2_guided_samples.py matrix problem2`
  - `p07 / p08` を `first-line representative`
  - `p09` を `reserve practical route`
  - `p13 / p14` を `negative static-stop`
  として sample-facing に表示してよい。
- reserve lane の detail では
  - `p07 = witness + model-check`
  - `p08 = model-check`
  - `p09 = delegated-rng + model-check`
  を separate status のまま保ってよい。

## stop line

- final public witness schema
- final public provider receipt schema
- final emitted-handoff contract
- exhaustive shared-space catalog
- stronger fairness / replay / provider attestation profile

## non-goals

- first completion line と reserve line を collapse しない。
- final source wording / final emitted artifact schema / final public witness-provider-artifact contract を採らない。
- `atomic_cut` を global consistent cut / durable commit と同一視しない。

## evidence

| command | expectation |
|---|---|
| `python3 -m unittest scripts.tests.test_current_l2_guided_samples` | Problem 2 residual row classification と matrix text が固定される |
| `python3 scripts/current_l2_guided_samples.py matrix problem2` | representative pair / reserve route / negative pair が 1 表で見える |

## rationale

- Package 96 / 97 までで first completion line と reserve strengthening lane は helper summary に actualize 済みであり、remaining work は public-shape residual の読み分けである。
- representative sample bundle matrix は final public witness/provider/artifact contract を premature に採らず、current first line と reserve line の boundary を維持するのに十分である。
