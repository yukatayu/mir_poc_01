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

1. `P-OPS-04`
   Sugoroku behavior widening in the product alpha session/runtime path
2. `P-OPS-05`
   projection manifest / packet / FFI schema formalization
3. `P-OPS-06`
   portal / world-link first cut
4. `P-OPS-09`
   external developer package authoring guide

## current recommendation

- `P-OPS-03` で direct text host boundary は `MembershipChat` に narrow `EchoText` lane として actualize 済み
- 次は `P-OPS-04` として、`SugorokuWorld` の roll / publish / witness / handoff / stale action rows を current product alpha session carrier に寄せる
- product alpha current line の bounded runtime semantics を壊さず、projection schema と portal first cut はその後段に置く

## open questions

- Sugoroku behavior を current product alpha session carrier にどこまで直接 actualize するか
- projection profile を runtime-plan adjacent field に入れるか、manifest-only に保つか
- `MembershipChat` の next widening を room-oriented `ChatText` multi-message lane にするか、`EchoText` のまま最小維持するか
