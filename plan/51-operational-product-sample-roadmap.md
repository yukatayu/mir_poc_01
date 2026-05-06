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

## P-OPS-05 current scope

- `deployments/projection/projection.profile.json` を `ops-product-projection-v0` schema-backed inventory として formalize
- `crates/mir-ast::product_alpha1` の `check` から projection target / packet / FFI inventory summary を accepted obligation として返す
- `crates/mir-runtime::product_alpha1_session` と `crates/mir-runtime::product_alpha1_devtools` から同 inventory を runtime plan / observer-safe projection panel に反映する
- `scripts/operational_product_samples.py` の `release-check` / `check-all` に projection inventory semantic check を追加する

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

1. `P-OPS-06`
   portal / world-link first cut
2. `P-OPS-07`
   two-shard hard-boundary model-check sample
3. `P-OPS-09`
   external developer package authoring guide

## current recommendation

- `P-OPS-03` で direct text host boundary は `MembershipChat` に narrow `EchoText` lane として actualize 済み
- `P-OPS-04` で `SugorokuWorld` の bounded scenario は current product alpha session carrier に寄せて actualize 済み
- `P-OPS-05` で projection schema と packet / FFI boundary inventory は schema-backed inventory として actualize 済み
- 次は `P-OPS-06` として、portal/world-link discrete handoff を planned-only inventory から bounded runtime or model-check evidence に進める
- product alpha current line の bounded runtime semantics と non-claims を壊さず、two-shard hard-boundary model-check は portal first cut の後段に置く

## open questions

- Sugoroku behavior を current bounded scenario からどこまで interactive / negative-row widening するか
- current projection inventory summary を richer projection IR / placement planner boundary にいつ widen するか
- `MembershipChat` の next widening を room-oriented `ChatText` multi-message lane にするか、`EchoText` のまま最小維持するか
