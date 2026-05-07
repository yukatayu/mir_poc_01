# Operational Product Sample 01 Summary

`P-OPS-01` は、product alpha release-candidate workflow の次に置く
**canonical operational product sample suite** です。

## What Is New

- `samples/product-alpha1/operational/` を追加した
- `WorldCore -> MembershipChat -> SugorokuWorld` の source / package chain を固定した
- `mirrorea-alpha` の current product alpha command familyで再現できる bounded operational workflow を docs と helper script にまとめた
- `projection.profile.json` を schema-backed target / packet / FFI inventory として `check` / runtime plan / devtools に接続した
- current backend comparison inventory keeps `native host launch bundle` as the only actualized path and leaves WASM/LLVM docs-first only
- `portal-worldlink/` bounded same-session discrete handoff root を actualize し、`future/portal-worldlink/` blueprint は保持した
- `two-shard-hard-boundary/` bounded same-session hard-authority handoff root を actualize し、`future/two-shard-hard-boundary/` と `spatial-shard-future.profile.json` は retained blueprint inventory として保持した
- separate `two-shard-gradient-observation/` root を actualize し、observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject evidence を same-session runtime / devtools / helper closeout に接続した
- `gradient-observation.profile.json` は paired non-executable profile として保持した
- validated starter catalog を `SugorokuWorld` で止め、portal/shard authoring は active roots と `future/` inventory を分けて読む decision を docs-first に固定し、helper-reported `portal_shard_starter_scope` で current boundary を machine-readable にした
- current `sugoroku-world` carrier を helper-reported `sugoroku_scope` で machine-readable に固定し、interactive turn choice / broader negative rows / networked multi-participant control は current line では未定義のままとした
- helper-reported `widening_queue_scope` を更新し、current room-chat reopening、portal/shard starter reopening、broader Sugoroku reopening は non-promoted、`later_user_final_distribution_decision` が next promoted comparison であると machine-readable にした

## What Is Runnable Now

- `world-core` / `membership-chat` / `sugoroku-world` の `check`
- `world-core` / `membership-chat` / `sugoroku-world` の `run-local`
- `membership-chat` の bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` host boundary、helper-reported `room_chat_scope`、および observer-safe devtools export
- `sugoroku-world` の bounded same-session roll / publish / witness / handoff / stale membership reject runtime evidence、helper-reported `sugoroku_scope`、および observer-safe devtools export
- `sugoroku-world` の `check` / runtime plan / devtools projection panel から、schema-backed projection target / packet / FFI inventory summary
- `portal-worldlink` の bounded same-session resolve / handoff offer / witness emit / destination admit runtime evidence と observer-safe devtools export
- `two-shard-hard-boundary` の bounded same-session offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject runtime evidence と observer-safe devtools export
- `two-shard-gradient-observation` の bounded same-session observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject runtime evidence と observer-safe devtools export
- `sugoroku-world` の `session`, `attach`, `save`, `quiescent-save`, `transport`, `export-devtools`, `view`, `build-native-bundle`
- `scripts/operational_product_samples.py check-all` と helper-reported `room_chat_scope` / `portal_shard_starter_scope` / `sugoroku_scope` / `widening_queue_scope`

## What Is Still Declared Or Planned

- representative textual `.mir`
- richer server/client binary split realization beyond the current projection inventory summary
- `future/portal-worldlink/` blueprint manifest
- `future/two-shard-hard-boundary/` blueprint manifest
- broader replication profile runtime beyond the bounded observer-only gradient cut
- broader multi-message room-chat surface beyond the helper-reported current non-promoted line
- broader portal/shard starter catalog beyond the current active-root-first boundary
- broader interactive Sugoroku controls and additional negative rows beyond the helper-reported current bounded deterministic carrier

## Why This Matters

`demo/` は release-candidate workflow-ready だが、single demo root である。
`operational/` は、外部開発者が package chain, attach layers, save/load, devtools, transport, host bundle, and future boundary inventory を 1 つの canonical suite として読むための root である。

## Main Non-Claims

- final textual grammar
- final server/client binary split
- direct LLVM backend
- WAN / federation
- distributed durable save/load
- final portal ABI / continuous spatial sync / continuous infinite shard federation / write-authority gradient runtime / general model-check completion

## Entry Points

- hands-on: `../hands_on/operational_product_sample_01.md`
- authoring boundary: `../hands_on/operational_portal_shard_starter_boundary_01.md`
- normative boundary: `../../specs/26-operational-product-sample-suite.md`
- future boundary: `../../specs/27-spatial-portal-and-shard-extension-boundary.md`
- roadmap memory: `../../plan/51-operational-product-sample-roadmap.md`
