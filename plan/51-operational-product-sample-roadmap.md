# plan/51 — operational product sample roadmap

## 目的

`specs/26` の operational product sample suite を repository-memory として整理する。

## 決定済み

- `P-OPS-01` は `samples/product-alpha1/operational/` を新設する
- `demo/` は release-candidate workflow root として残し、operational suite と混ぜない
- current executable input は `package.mir.json`
- representative `.mir` は explanatory source
- runnable root は `WorldCore`、`MembershipChat`、`SugorokuWorld`、`PortalWorldLink`、`TwoShardHardBoundary`
- `future/portal-worldlink/` と shard inventory は同 root に置いてよいが、active portal root と混同せず blueprint / planned-only を維持する

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

## P-OPS-06 current scope

- `samples/product-alpha1/operational/portal-worldlink/` を active executable root として追加する
- `portal_worldlink` package kind を current product alpha executable line に追加する
- bounded same-session discrete handoff evidenceを `run-local` / observer-safe devtools / helper `release-check` / `check-all` から再現する
- `future/portal-worldlink/` blueprint root を残し、active runtime root と明示的に分ける

## P-OPS-07 current scope

- `samples/product-alpha1/operational/two-shard-hard-boundary/` を active executable root として追加する
- `two_shard_hard_boundary` package kind を current product alpha executable line に追加する
- bounded same-session two-shard offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidenceを `run-local` / observer-safe devtools / helper `release-check` / `check-all` から再現する
- `future/two-shard-hard-boundary/` と `spatial-shard-future.profile.json` を retained blueprint inventory として残し、active runtime root と明示的に分ける

## P-OPS-09 current scope

- `samples/product-alpha1/operational/templates/` を template-only authoring starter root として追加する
- `templates/world-core-starter/` を current validated `world_core` starter として `check` / `run-local` で再現可能にする
- `docs/hands_on/operational_package_authoring_01.md` と `docs/research_abstract/operational_package_authoring_01.md` で external developer 向け `author -> check -> run-local -> session -> export-devtools -> view --check` の bounded order を固定する
- template roots を active operational sample roots や generic release helper と混同しない

## P-OPS-08 current scope

- `native host launch bundle` を current actualized backend-adjacent path として明示する
- WASM client host と LLVM/native projection backend を docs-first comparison inventory としてだけ棚卸しする
- packet / FFI / projection boundary と auth/membership/capability/witness lane preservation requirement を backend reopen prerequisite として書き出す
- generic backend build helper や direct codegen claim は追加しない

## P-OPS-01 non-goals

- final textual grammar
- final SDK / ABI
- final server/client split
- LLVM backend
- continuous portal spatial sync
- shard replication actualization
- WAN / federation
- distributed durable save/load

## next packages

1. broader operational template catalog
   next template-only starter widening order

## current recommendation

- `P-OPS-03` で direct text host boundary は `MembershipChat` に narrow `EchoText` lane として actualize 済み
- `P-OPS-04` で `SugorokuWorld` の bounded scenario は current product alpha session carrier に寄せて actualize 済み
- `P-OPS-05` で projection schema と packet / FFI boundary inventory は schema-backed inventory として actualize 済み
- `P-OPS-06` で `PortalWorldLink` bounded same-session discrete handoff root は actualize 済み
- `P-OPS-07` で `TwoShardHardBoundary` bounded same-session hard-authority root は actualize 済み
- `P-OPS-09` で `templates/world-core-starter/` と bounded package authoring guide は actualize 済み
- `P-OPS-08` で current host launch bundle line を保ったまま backend feasibility inventory は docs-first に actualize 済み
- 次は broader operational template catalog として、`world_core` 以外の template-only starter widening 順を整理する

## open questions

- Sugoroku behavior を current bounded scenario からどこまで interactive / negative-row widening するか
- current projection inventory summary を richer projection IR / placement planner boundary にいつ widen するか
- `MembershipChat` の next widening を room-oriented `ChatText` multi-message lane にするか、`EchoText` のまま最小維持するか
- `templates/` を `world_core` 以外の package kinds へどの順で widen するか
- WASM client host comparison を projection inventory の内側へ寄せるか、独立 docs inventory として維持するか
