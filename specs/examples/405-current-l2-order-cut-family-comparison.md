# 405 — current L2 order / cut family comparison

## 目的

current repo の order / cut line で、

- local finalization
- ordering-only barrier
- observation / snapshot
- commit / evidence-bearing cut

を 1 primitive に潰さずに比較する。

ここで固定するのは **docs-first family decomposition** であり、
final vocabulary や final syntax ではない。

## source-backed floor

- `atomic_cut` は current `place` の rollback frontier だけを確定する local finalizing cut である。
- `barrier` は later ordering primitive candidate である。
- `durable_cut` は later commit-bearing / evidence-bearing cut family である。
- `atomic_cut` は global sync point、distributed agreement point、durable commit ではない。

## comparison family

| family | core meaning | rollback interaction | observation meaning | durability / evidence burden | cross-place burden |
|---|---|---|---|---|---|
| `atomic_cut` | local finalization | current `place` frontier を確定 | local trace 上の cut event | 持たない | 持たない |
| `barrier` | ordering-only candidate | local rollback frontier を直接確定しない | ordering relation のみ | 持たない | optional / later |
| `snapshot_cut` / `consistent_cut` | global observation / snapshot-only candidate | rollback-stop を主目的にしない | consistent global observation | 持たない | 高い |
| `durable_cut` | commit / evidence-bearing cut | rollback frontier の外へ durability を要求 | commit success / failure を観測 | 持つ | 高い |

## current judgment

1. `atomic_cut` は local finalization family に留める。
2. `barrier` は ordering-only candidate として later に残す。
3. `durable_cut` は commit / evidence-bearing family として later に残す。
4. `snapshot_cut` / `consistent_cut` は **comparison candidate** としてだけ扱う。

## current first wording

- local rollback frontier を確定する family
- ordering relation だけを追加する family
- observation / snapshot を作る family
- commit / evidence を伴う family

の 4 つを最低限分けて書く。

## what is not decided here

- `snapshot_cut` または `consistent_cut` を repo の settled vocabulary にすること
- cross-place cut family の final profile 名
- final parser surface
- final runtime implementation
