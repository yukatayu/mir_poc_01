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

1. completed docs-first widenings
   gradient observation profile
   portal/shard starter boundary
2. completed semantic/runtime widenings
   broader room-chat lane
   bounded observer-only gradient runtime
3. maintenance closeout
   dashboard freshness
4. completed public-ish adoption probe
   installed binary + native host launch bundle
5. next queue-shaping package
   final grammar / ABI scoping
6. much later
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

- active executable roots は `samples/product-alpha1/operational/two-shard-hard-boundary/` と `samples/product-alpha1/operational/two-shard-gradient-observation/`
- `future/two-shard-hard-boundary/` は blueprint root として維持する
- `future/gradient-observation.profile.json` は paired non-executable profile inventory として維持する
- current runtime evidence は hard-boundary root の offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject と、gradient root の observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject に限る
- current stop line は continuous sync、write-authority gradient runtime、general model-check completion、WAN federation、continuous infinite federation

## P-OPS-11 current scope

- `samples/product-alpha1/operational/future/gradient-observation.profile.json` を docs-first / profile-first inventory として追加する
- `spatial-shard-future.profile.json` から gradient observation profile ref を張る
- observer-only overlap zone、freshness fields、replication non-default reading、fallback behavior を reader-facing docs に明記する
- runtime actualization、model-check completion、portal/shard starter wideningは含めない

## P-OPS-12 current scope

- current line では `portal_worldlink` / `two_shard_hard_boundary` starter duplicates を追加しない decision を docs-first に固定する
- portal/shard authoring は active executable roots を study/copy boundary として扱い、`future/` inventory は non-executable のまま保つ
- later starter reopen condition を active-root sourced / future-blueprint distinct / focused validation required に固定する

## P-OPS-14 current scope

- queue / roadmap / dashboard wording を room-chat widening 後の current state に揃える
- next reopen point を gradient observation runtime first cut に進める
- existing hard-boundary root と `planned_only` profile inventory の boundary は維持する

## P-OPS-15 current scope

- `samples/product-alpha1/operational/two-shard-gradient-observation/` を active executable root として追加する
- `future/gradient-observation.profile.json` は non-executable inventory のまま保持しつつ、paired active runtime root ref を更新する
- observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject evidence を same-session runtime / devtools / helper closeout に actualize する
- continuous sync、write authority、WAN federation は主張しない

## next reopen point

- `P-OPS-17` closeout 後の current recommendation は `final grammar / ABI scoping`
- shard line の immediate reopen は推奨しない。current actualized installed-binary + native host launch bundle probe が入ったので、次 comparison は broader room-chat revisit ではなく、この concrete front door に対する grammar / ABI hardening の順序である
- current recommendation は、portal/shard bounded runtime line を維持したまま grammar / ABI scoping を先に閉じ、その結果を見てから later shard widening の順序を再評価すること

## avoid

- portal を transport alias に潰すこと
- shard を continuous sync completion と書くこと
- object replication profile を default 必須扱いすること
- vector clock default を membership freshness に持ち込むこと

## open questions

- portal admission を membership authority と capability authority のどこで分けるか
- shard config epoch と membership epoch をどの payload seam で併置するか
- future replication profile catalog を `specs/27` からどの時点で分離するか
- portal/shard starter を later に reopen するなら、gradient runtime widening の後でも依然として必要か
