# Operational Product Sample 01 Summary

`P-OPS-01` は、product alpha release-candidate workflow の次に置く
**canonical operational product sample suite** です。

## What Is New

- `samples/product-alpha1/operational/` を追加した
- `WorldCore -> MembershipChat -> SugorokuWorld` の source / package chain を固定した
- `mirrorea-alpha` の current product alpha command familyで再現できる bounded operational workflow を docs と helper script にまとめた
- `projection.profile.json` を schema-backed target / packet / FFI inventory として `check` / runtime plan / devtools に接続した
- `portal-worldlink/` bounded same-session discrete handoff root を actualize し、`future/portal-worldlink/` blueprint は保持した
- shard を同 suite 内の future boundary inventory として固定した

## What Is Runnable Now

- `world-core` / `membership-chat` / `sugoroku-world` の `check`
- `world-core` / `membership-chat` / `sugoroku-world` の `run-local`
- `membership-chat` の bounded `EchoText("Taro") -> "Hello, Taro!"` direct host boundary と observer-safe devtools export
- `sugoroku-world` の bounded same-session roll / publish / witness / handoff / stale membership reject runtime evidence と observer-safe devtools export
- `sugoroku-world` の `check` / runtime plan / devtools projection panel から、schema-backed projection target / packet / FFI inventory summary
- `portal-worldlink` の bounded same-session resolve / handoff offer / witness emit / destination admit runtime evidence と observer-safe devtools export
- `sugoroku-world` の `session`, `attach`, `save`, `quiescent-save`, `transport`, `export-devtools`, `view`, `build-native-bundle`
- `scripts/operational_product_samples.py check-all`

## What Is Still Declared Or Planned

- representative textual `.mir`
- richer server/client binary split realization beyond the current projection inventory summary
- two-shard hard-boundary future profile
- `future/portal-worldlink/` blueprint manifest
- broader room-chat-oriented `ChatText` lane
- broader interactive Sugoroku controls and additional negative rows beyond the current bounded carrier

## Why This Matters

`demo/` は release-candidate workflow-ready だが、single demo root である。
`operational/` は、外部開発者が package chain, attach layers, save/load, devtools, transport, host bundle, and future boundary inventory を 1 つの canonical suite として読むための root である。

## Main Non-Claims

- final textual grammar
- final server/client binary split
- direct LLVM backend
- WAN / federation
- distributed durable save/load
- final portal ABI / continuous spatial sync / shard runtime implementation

## Entry Points

- hands-on: `../hands_on/operational_product_sample_01.md`
- normative boundary: `../../specs/26-operational-product-sample-suite.md`
- future boundary: `../../specs/27-spatial-portal-and-shard-extension-boundary.md`
- roadmap memory: `../../plan/51-operational-product-sample-roadmap.md`
