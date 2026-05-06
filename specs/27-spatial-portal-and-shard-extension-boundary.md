# 27 — spatial portal and shard extension boundary

## 目的

この文書は、operational product sample suite の後段に来る
portal / world-link / shard / replication profile の future boundary を固定する。

## 決定レベル

- `L1`
  portal は WWW hyperlink 相当の **discrete handoff** として始める
- `L1`
  continuous spatial synchronization は portal と別 family に残す
- `L1`
  shard は first cut では **hard authority boundary** から始める
- `L1`
  membership freshness の default に vector clock を採らない
- `L1`
  replication profile は object replication が必要になった時だけ導入する
- `L2`
  two-shard finite boundary を first promoted model とし、gradient observation はその後段に置く

## portal boundary

`L1`:

- portal / world-link は destination lookup, admission, handoff を分ける
- portal handoff は witness / capability / membership freshness を transport に潰さない
- current line は bounded same-session portal root を actualize してよいが、future blueprint root も並置してよい
- portal current line は WAN federation completion を主張しない

## shard boundary

`L1`:

- shard は owner shard / owner epoch / sequence を持つ single-owner first cut を優先する
- cross-shard authority transfer は explicit handoff と witness を必要とする
- stale config epoch / stale owner epoch write は reject path を持つ
- observer ghost / observation-only copy は write capability を持たない
- current line may actualize one bounded same-session two-shard hard-boundary root so long as it does not claim gradient observation runtime or general model-check completion

## gradient observation profile

`L1`:

- gradient observation は observer-only widening profile としてまず記述する
- overlap zone は ghost / presence / hint view を許してよいが、write authority は与えない
- freshness は `membership_epoch`, `member_incarnation`, `config_epoch`, `owner_epoch`, `sequence` を使い、vector clock default を導入しない
- gradient observation profile は `planned_only` JSON inventory として repo に置いてよいが、runtime actualization と混同してはならない

## authoring boundary

`L2`:

- portal/shard future blueprint inventory does not imply corresponding starter templates
- current line may keep portal/shard authoring on active executable roots while `future/` portal/shard files remain non-executable inventory
- if a later portal/shard starter appears, it must be sourced from the active executable root rather than from the future blueprint/profile inventory

## replication profile

`L1`:

- default profile は single-owner sequence family
- optional future profile として owner-epoch sequence, CRDT family, dotted-version-vector family, interval-tree-clock family を inventory してよい
- optional profile inventory は current default を弱めない

## save/load and federation non-claims

`L1`:

- portal / shard package line は distributed durable save/load R3/R4 completion を主張しない
- continuous infinite federation / WAN federation / gradient sync completion は current package line の外側に残す

## first future evidence target

`L2`:

current bounded two-shard root and future sample / model-check package は少なくとも次を確認する。

- no double owner after handoff
- old owner write rejected after commit
- missing handoff witness rejected
- stale config epoch rejected
- observer-only copy has no write capability
