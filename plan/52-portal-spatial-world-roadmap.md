# plan/52 — portal and spatial world roadmap

## 目的

`specs/27` の portal / shard / replication profile line を repository-memory として整理する。

## 決定済み

- portal は discrete world-link handoff から始める
- continuous spatial sync は別 family
- two-shard hard boundary を first promoted model にする
- vector clock default は採らない
- replication profile は必要時に導入する

## package order

1. `P-OPS-06`
   portal / world-link first cut
2. `P-OPS-07`
   two-shard hard-boundary model-check sample
3. post-OPS widening
   gradient observation profile
4. much later
   continuous infinite federation / WAN line

## recommended first model

- finite two-shard grid
- owner shard + owner epoch + sequence
- explicit handoff witness
- config epoch reject
- observer-only gradient view without write authority

## avoid

- portal を transport alias に潰すこと
- shard を continuous sync completion と書くこと
- object replication profile を default 必須扱いすること
- vector clock default を membership freshness に持ち込むこと

## open questions

- portal admission を membership authority と capability authority のどこで分けるか
- shard config epoch と membership epoch をどの payload seam で併置するか
- future replication profile catalog を `specs/27` からどの時点で分離するか
