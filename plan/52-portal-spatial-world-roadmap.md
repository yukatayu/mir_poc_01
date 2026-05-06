# plan/52 — portal and spatial world roadmap

## 目的

`specs/27` の portal / shard / replication profile line を repository-memory として整理する。

## 決定済み

- portal は discrete world-link handoff から始める
- `P-OPS-06` で active `portal-worldlink/` root は bounded same-session discrete handoff として actualize 済み
- `P-OPS-07` で active `two-shard-hard-boundary/` root は bounded same-session two-shard hard-authority cut として actualize 済み
- continuous spatial sync は別 family
- two-shard hard boundary を first promoted model にする
- vector clock default は採らない
- replication profile は必要時に導入する

## package order

1. post-OPS widening
   gradient observation profile
2. much later
   continuous infinite federation / WAN line

## current portal cut

- active executable root は `samples/product-alpha1/operational/portal-worldlink/`
- `future/portal-worldlink/` は blueprint root として維持する
- current runtime evidence は resolve / handoff offer / witness emit / destination admit の same-session discrete handoff に限る
- current stop line は WAN federation、continuous spatial sync、final portal ABI

## recommended first model

- finite two-shard grid
- owner shard + owner epoch + sequence
- explicit handoff witness
- config epoch reject
- observer-only gradient view without write authority

## current shard cut

- active executable root は `samples/product-alpha1/operational/two-shard-hard-boundary/`
- `future/two-shard-hard-boundary/` は blueprint root として維持する
- current runtime evidence は offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject の same-session hard-authority cut に限る
- current stop line は gradient observation runtime、general model-check completion、WAN federation、continuous infinite federation

## avoid

- portal を transport alias に潰すこと
- shard を continuous sync completion と書くこと
- object replication profile を default 必須扱いすること
- vector clock default を membership freshness に持ち込むこと

## open questions

- portal admission を membership authority と capability authority のどこで分けるか
- shard config epoch と membership epoch をどの payload seam で併置するか
- future replication profile catalog を `specs/27` からどの時点で分離するか
