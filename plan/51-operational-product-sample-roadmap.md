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

1. `P-OPS-02`
   package dependency and import resolver hardening
2. `P-OPS-03`
   operational chat / direct text host boundary
3. `P-OPS-04`
   Sugoroku behavior widening in the product alpha session/runtime path
4. `P-OPS-05`
   projection manifest / packet / FFI schema formalization
5. `P-OPS-09`
   external developer package authoring guide

## current recommendation

- `P-OPS-01` では runnable root と future inventory を同じ sample suite で見せる
- ただし runtime widening より先に、import/dependency/bundle/devtools inventory が reproducible であることを優先する
- product alpha current line の bounded runtime semantics を壊さずに載せる

## open questions

- `MembershipChat` の text host-I/O lane を `EchoText` / `ChatText` のどちらで widen するか
- Sugoroku behavior を current product alpha session carrier にどこまで直接 actualize するか
- projection profile を runtime-plan adjacent field に入れるか、manifest-only に保つか
