# plan/51 — operational product sample roadmap

## 目的

`specs/26` の operational product sample suite を repository-memory として整理する。

## 決定済み

- `P-OPS-01` は `samples/product-alpha1/operational/` を新設する
- `demo/` は release-candidate workflow root として残し、operational suite と混ぜない
- current executable input は `package.mir.json`
- representative `.mir` は explanatory source
- runnable root は `WorldCore`、`MembershipChat`、`SugorokuWorld`
- portal / shard は future boundary inventory として同 root に置いてよいが、planned-only を維持する

## P-OPS-01 current scope

- package dependency / import chain の first canonical suite
- same-session / local transport / Docker transport / observer-safe devtools / native host launch bundle の bounded operational replay
- release-check helper
- docs / hands-on / research summary / dashboard sync

## P-OPS-03 current scope

- `MembershipChat` に one-lane `EchoText("Taro") -> "Hello, Taro!"` direct host boundary を actualize
- `run-local` / `session` / `export-devtools` 上で observer-safe host-I/O evidence を再現
- `scripts/operational_product_samples.py` の semantic check に direct text lane を追加

## P-OPS-04 current scope

- `SugorokuWorld` に bounded same-session roll / publish / witness / handoff / stale membership reject scenario を actualize
- `run-local` / `session` / `export-devtools` / `release-check` 上で同じ Sugoroku runtime evidence を再現
- `scripts/operational_product_samples.py` の semantic check に Sugoroku runtime event/route/failure evidence を追加

## P-OPS-01 non-goals

- final textual grammar
- final SDK / ABI
- final server/client split
- LLVM backend
- portal runtime actualization
- shard replication actualization
- WAN / federation
- distributed durable save/load

## next packages

1. `P-OPS-05`
   projection manifest / packet / FFI schema formalization
2. `P-OPS-06`
   portal / world-link first cut
3. `P-OPS-09`
   external developer package authoring guide

## current recommendation

- `P-OPS-03` で direct text host boundary は `MembershipChat` に narrow `EchoText` lane として actualize 済み
- `P-OPS-04` で `SugorokuWorld` の bounded scenario は current product alpha session carrier に寄せて actualize 済み
- 次は `P-OPS-05` として、projection schema と packet / FFI boundary inventory を manifest-only から schema-backed inventory に進める
- product alpha current line の bounded runtime semantics を壊さず、portal first cut は projection schema の後段に置く

## open questions

- Sugoroku behavior を current bounded scenario からどこまで interactive / negative-row widening するか
- projection profile を runtime-plan adjacent field に入れるか、manifest-only に保つか
- `MembershipChat` の next widening を room-oriented `ChatText` multi-message lane にするか、`EchoText` のまま最小維持するか
