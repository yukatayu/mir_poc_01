# 515 — current L2 order-handoff / witness-provider final public seam compression after reserve actualizations

## 目的

`specs/examples/503`、
`505`、
`511`、
`512`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- order-handoff / witness-provider final public seam compression
- principal source wording route carry-over
- `serial` reserve surface carry-over
- emitted-contract representative trace alignment carry-over
- shared-space final public-contract reopen threshold carry-over
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**order-handoff source-wording route actual adoption、authoritative-room `serial` reserve surface、witness/provider emitted-contract representative trace-alignment bridge、witness/provider final public-contract reopen threshold を保ったまま、remaining Problem 2 / shared-space public seams を helper-local compression manifest に actualize する current cut**
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

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. order-handoff source wording route actual adoption
   - edge-row principal
   - readable relation vocabulary keep
   - stage-block secondary keep
   - final source wording later
2. order-handoff serial-scope reserve surface
   - authoritative-room-specific `serial on ...` reserve
   - principal edge-row surface unchanged
3. witness/provider emitted-contract representative trace-alignment bridge
   - route side / emitted-contract side pair alignment
   - repo-local emitted artifact refs first
4. witness/provider final public-contract reopen threshold
   - public-schema pair first
   - delegated attestation + combined public-contract second
   - final emitted-handoff contract third
   - exhaustive catalog later

したがって current open problem は、
Problem 2 / shared-space mixed gate を source wording side と public-contract side に散らしたまま残すことではなく、
**source wording residual、emitted-artifact residual、public-schema pair residual、delegated attestation + combined contract residual、final emitted-handoff residual を 1 本の current recommendation に圧縮できるか**
である。

## current compression cut

current package では、次を採る。

1. order-handoff side residual は
   - final source-surface handoff wording later
   - final emitted-artifact schema later
   に圧縮する
2. shared-space side residual は
   - public-schema pair first
   - delegated attestation and combined contract second
   - final emitted-handoff contract third
   に圧縮する
3. `serial on ...` は authoritative-room-specific reserve surface のまま carry-over する
4. representative reached sample は `p07 / p08` に取り、`p05` は guard-only に残す
5. final public wording / final public contract adoption 群には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | provider-boundary / witness-provider trace-alignment 側を読むための representative runtime evidence を与える。source-wording route や reserve-surface judgment 自体は含めない |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | order-handoff side と shared-space side を併記した canonical runtime inventory を与える。final public-seam compression judgment 自体は helper-local / doc-level に残す |
| `order_handoff_public_seam_residual:*` refs | order-handoff side final wording / emitted-artifact residual を sample-local に固定する repo-local ref family |
| `shared_space_public_seam_residual:*` refs | shared-space side final public-schema / contract residual を sample-local に固定する repo-local ref family |

## current recommendation

1. Problem 2 / shared-space line は final public seams を compression manifest に actualize してよい。
2. principal source wording は edge-row / vertical continuation に据え置く。
3. `serial on ...` は authoritative-room-specific reserve surface に留める。
4. witness/provider trace alignment は final public contract promotionの代替ではなく、current residual compression の evidence として使う。
5. final public wording / final public witness-provider-artifact contract 群は still later に残す。

## retained alternatives

- order-handoff final wording first reopen
- final emitted-artifact schema first reopen
- final public witness/provider schema first reopen
- delegated provider attestation first reopen
- final emitted-handoff contract first reopen
- `serial` sugar public promotion

## stop line

current package は次で止める。

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
