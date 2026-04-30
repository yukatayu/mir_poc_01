# 533 — current L2 order-handoff / witness-provider public seam helper mirror

## 目的

`specs/examples/505`、
`515`、
`526`
と
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- order-handoff / witness-provider public seam compression
- source wording residual
- emitted-artifact residual
- witness/provider public-contract reopen order
- `run-source-sample` helper summary
- current recommendation
- kept-later public seam

を 1 本に束ねる。

ここで actualize するのは、
**docs 側で source-backed だった Problem 2 / shared-space public seam compression を `run-source-sample` helper summary に narrow に mirror する current cut**
であり、

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact schema
- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted-handoff contract
- authoritative-room `serial` sugar public adoption
- low-level `memory_order` exact source surface
- final modal foundation adoption
- exhaustive shared-space catalog

は still later に残す。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. shared-space reopen order
   - `specs/examples/505`
   - public-schema pair first
   - delegated attestation and combined contract second
   - final emitted-handoff contract third
   - exhaustive shared-space catalog later
2. Problem 2 / shared-space residual compression
   - `specs/examples/515`
   - source wording residual
   - emitted-artifact residual
   - public-schema pair residual
   - delegated attestation and combined contract residual
   - final emitted-handoff residual
3. helper-local order-handoff surface preview
   - `specs/examples/526`
   - principal companion
   - stage-block secondary
   - serial reserve

したがって current open problem は、
source wording / emitted artifact / witness-provider public-contract residual を docs-only compare floor に戻すことではなく、
**Problem 2 / shared-space public seam compression を `run-source-sample` helper summary でも sample-visible に保ち、remaining mixed gate を final public wording / contract adoption だけに narrow に残せるか**
である。

## current helper mirror cut

current package では、次を採る。

1. `order_handoff_witness_provider_public_seam_compression`
   - reached sample は `p07 / p08`
   - `p09` は guard-only
   - `profile_axis_refs`
   - `source_wording_route_refs`
   - `emitted_artifact_candidate_keep_refs`
   - `serial_scope_lines`
   - `witness_schema_route_refs`
   - `provider_receipt_route_refs`
   - `combined_public_contract_keep_refs`
   - `trace_alignment_pair_refs`
   - `public_seam_residual_refs`
2. `serial on ...` は helper-local reserve surface のまま carry-over する
3. final public wording / final public witness-provider-artifact contract adoption には上げない

historical `p07 / p08 / p09` labelsは helper-summary compare-anchor memory として残るが、
current active evidence は Sugoroku late-join slices、`NET-03` reconnect canary、
adjacent clean-near-end order-handoff familyへ分かれている。

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json` | publication / handoff core floor が historical `p07` public-seam mirror reading の current replacement として runnable |
| `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json` | late-join visible-history floor が historical `p07` reading の current replacement として runnable |
| `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json` | stale reconnect / no silent merge canary が historical `p08` reading の current replacement として runnable |
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | adjacent current-L2 order-handoff floor を再確認できる。`05_delegated_rng_service` は witness/provider route の active adjacent evidence、`01` は reached floor、`02/03` は negative pair floor に当たる |
| `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | compatibility front door から current corpus floor を再確認できる。`run-source-sample` helper summary naming 自体は repo-local helper memory に残る |

## current recommendation

1. Problem 2 / shared-space residual mixed gate は、final public seams の docs reading を helper-local operational summary に mirror してよい。
2. principal source wording は edge-row / vertical continuation に据え置く。
3. `serial on ...` は authoritative-room-specific reserve surface のまま carry-over する。
4. `p09` を reached に無理に押し広げず、historical `p07 / p08` reached memory・`p09` guard memory の current cut を保つのが semantically honest である。current active adjacent evidence は Sugoroku / `NET-03` / clean-near-end order-handoff familyへ分けて読む。
5. `run-source-sample` helper summary へ出したからといって、final public wording や final public witness/provider/artifact contract を fixed したとは読まない。

## retained alternatives

- order-handoff source wording residual だけを helper summary に出し、shared-space side は docs-only に残す cut
- witness/provider final public-contract reopen threshold だけを helper summary に出し、order-handoff side residual は docs-only に残す cut
- `serial` reserve surface を public principal へ早期昇格する cut
- final public wording / final public contract adoption を helper summary actualization と一緒に進める cut

## stop line

この package の先で still later に残すものは次である。

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact schema
- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted-handoff contract
- authoritative-room `serial` sugar public adoption
- low-level `memory_order` exact source surface
- final modal foundation adoption
- exhaustive shared-space catalog

## non-goal

この package は、

- final public wording adoption
- final public witness/provider/artifact contract adoption
- final parser/public API adoption

ではない。
