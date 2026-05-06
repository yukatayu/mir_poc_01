# 26 — operational product sample suite

## 目的

この文書は、product alpha release-candidate workflow の次に置く
**canonical operational product sample suite** の規範境界を固定する。

対象 package:

- `P-OPS-01 operational product sample suite scaffold and first workflow`

## 決定レベル

- `L1`
  operational sample suite は `samples/product-alpha1/demo/` とは別 root に置く
- `L1`
  current executable input は versioned `package.mir.json` であり、direct textual `.mir` は representative source に留める
- `L1`
  native output は current line では `native host launch bundle` であり、direct Mir-to-machine-code / LLVM backend ではない
- `L1`
  portal / world-link と shard / federation は future boundary を先に固定し、未実装部分を runnable claim に混ぜない
- `L2`
  `WorldCore -> MembershipChat -> SugorokuWorld` import / package chain を first canonical suite とする

## sample root

`L1`:

```text
samples/product-alpha1/operational/
```

この root は次を分けて保持する。

- runnable package roots
  - `world-core/`
  - `membership-chat/`
  - `sugoroku-world/`
- shared attach package roots
  - `packages/debug-layer/`
  - `packages/auth-layer/`
  - `packages/rate-limit-layer/`
  - `packages/placeholder-object/`
  - `packages/custom-avatar-preview/`
- deployment / projection inventory
  - `deployments/local/`
  - `deployments/docker/`
  - `deployments/projection/`
- future boundary inventory
  - `future/portal-worldlink/`
  - `future/two-shard-hard-boundary/`
- expected observer / docs anchor
  - `expected/*.expected.json`

## current executable boundary

`L1`:

- `.mir` files are representative source only
- current executable input is `package.mir.json`
- `deployments/projection/projection.profile.json` may carry schema-backed target / packet / FFI inventory for the runnable root, but it is supplementary inventory rather than executable input
- `world_core` / `membership_chat` / `sugoroku_world` are product alpha package kinds for this line
- dependency chain may be expressed as sibling package paths
- future portal / shard manifests may use richer fields than the current executable schema, but must be marked planned-only and must not be claimed runnable

## package roles

### WorldCore

`L1`:

- server-only world base
- room / world identity
- membership frontier
- event DAG policy
- observer-safe observation / redaction / retention policy
- typed host boundary placeholder

### MembershipChat

`L1`:

- imports `WorldCore`
- adds join / leave / room-message contract surface
- actualizes one bounded `EchoText` request/response lane as current direct text host-boundary evidence
- keeps host I/O at typed external boundary
- does not introduce stdio as Mir core primitive

### SugorokuWorld

`L1`:

- imports `MembershipChat`
- is the runnable root for the first operational workflow
- actualizes one bounded same-session scenario with roll / publish / witness / handoff / stale membership reject evidence
- accepts one schema-backed projection inventory summary from `deployments/projection/projection.profile.json`, surfaced through `check`, runtime plan, and observer-safe devtools projection panels
- keeps save/load visibility, transport visibility, and hot-plug visibility on the same product alpha session carrier
- current direct execution lane remains bounded to the existing typed host-I/O `AddOne` adapter and does not claim final interactive game runtime completion

## attach / transport / devtools / save-load

`L1`:

- auth / debug / rate-limit layers are explicit attach packages, not transparent overlays
- current attach admission is bounded same-session session-carried evidence, not a final external issuer / witness / membership attestation pipeline
- object / avatar preview packages may remain deferred boundary evidence
- local and Docker transport may be used as bounded operational evidence only
- `R0` local save/load and `R2` quiescent-save remain bounded product alpha semantics
- current `R2` success evidence is session-preflight / observer-safe export bounded evidence, not durable distributed proof completion
- viewer rendering may remain non-final; current success condition may be observer-safe JSON export plus static viewer check

## devtools inventory requirement

`L2`:

the suite should expose, through runtime export or documented JSON inventory:

- source/import graph
- package dependency graph
- projection target graph
- Place graph
- server/client/process graph
- MessageEnvelope route graph
- event DAG
- membership/config frontier timeline
- witness timeline
- hot-plug lifecycle
- save/load / quiescent-save timeline
- contract/effect/failure row summary
- observer-safe vs admin/debug split
- portal future panel
- shard-map future panel

If a panel is not renderer-complete, the current package may satisfy the requirement with observer-safe JSON plus explicit kept-later wording.

## completion condition for P-OPS-01

`L1`:

`P-OPS-01` is complete when the following are externally reproducible and documented without overclaim:

- package/source graph for `WorldCore -> MembershipChat -> SugorokuWorld`
- `check` for the three package roots
- `run-local` and `session` for the operational Sugoroku root
- explicit attach workflow for debug / auth / rate-limit and visible deferred object / avatar boundaries
- local transport and, when environment permits, Docker transport
- observer-safe devtools export and static viewer check
- `save` / `quiescent-save`
- native host launch bundle
- release-check helper for the operational suite

helper / sidecar / report / exact expected JSON / first-floor runner remain evidence, not completion by themselves.

## fixed non-claims

`L1`:

- no final textual `.mir` grammar
- no final server/client binary split
- no final public ABI / SDK
- no direct LLVM backend completion
- no WAN / federation completion
- no distributed durable save/load R3/R4
- no arbitrary native package execution
- no portal / shard implementation claim without corresponding runtime evidence
