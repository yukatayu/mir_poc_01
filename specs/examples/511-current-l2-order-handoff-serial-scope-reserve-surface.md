# 511 — current L2 order-handoff serial-scope reserve surface

## 目的

`specs/examples/477`、
`490`、
`503`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- authoritative-room-specific `serial on ...` sugar の helper-local reserve surface
- edge-row principal / stage-block secondary keep
- delegated-provider practical route と serial scope の同居
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**final source wording や final grammar に上げず、authoritative-room-specific `serial on ...` sugar を helper-local reserve surface として narrow に machine-check する current cut**
であり、

- final parser grammar
- final public parser / checker / runtime API
- `serial` sugar の public promotion
- authoritative room 外への `serial` widening
- final source-surface handoff wording
- final emitted-artifact / emitted-handoff contract
- final public witness/provider contract
- low-level `memory_order` exact source surface
- final modal foundation adoption

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. order-handoff source wording route actual adoption
   - edge-row vertical continuation principal
   - readable high-level relation vocabulary
   - stage-block secondary keep
2. delegated-rng-service practical actualization
   - provider placement keep
   - authority-kept-commit split
   - optional provider attachment later
3. witness/provider route actual adoption
   - route first
   - repo-local emitted artifact refs first
   - final public schema later
4. corrected runnable prototype floor
   - `p07 / p08 / p09` reached
   - `p05` guard-only

したがって current open problem は、
`serial` sugar を invent することではなく、
**room-specific reserve surface としてどこまで admissible に actualize し、どこで止めるか**
である。

## current reserve-surface cut

current package では、次を採る。

1. `serial on dice_authority { ... }` は helper-local reserve surface に留める
2. `p07 / p08` は order-handoff source wording route actual adoption の reached floor から actualize する
3. `p09` は delegated-provider practical route と witness/provider route actual adoption の両方を満たす reached floor から actualize する
4. principal source wording は引き続き edge-row / vertical continuation に置く
5. `stage` / `after` / `witness` family は secondary candidate のままに置く
6. `p05` は guard-only に残し、room-specific reserve surface が reached sample を過大に広げないことを保つ

## actual runnable evidence

| evidence | current reading |
|---|---|
| `build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface` | `p07 / p08 / p09` reached、`p05` guard-only の serial-scope reserve surface manifest を helper-local に actualize する runtime support |
| `current_l2_order_handoff_serial_scope_reserve_surface` | source-wording route / delegated-provider route と serial-scope reserve surface の carry-over を machine-check する focused runtime test |
| `serial on dice_authority { ... }` lines | room-specific reserve surface が principal source wording ではなく helper-local reserve surface であることを明示する compare artifact |

## current recommendation

1. `serial on ...` sugar は authoritative-room-specific reserve surface としては actualize してよい。
2. principal source wording は edge-row / vertical continuation に据え置く。
3. `stage-block` family は secondary candidate のまま keep する。
4. provider contract / public schema / final wording は still later に残す。
5. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。

## retained alternatives

- edge-row principal only で止める
- `stage-block` principal promotion
- `serial` sugar の public promotion
- authoritative room 外への `serial` widening
- low-level exact source wording import
- final grammar adoption

## stop line

current package は次で止める。

- final parser grammar
- final public parser / checker / runtime API
- `serial` sugar の public promotion
- authoritative room 外への `serial` widening
- final source-surface handoff wording
- final emitted-artifact / emitted-handoff contract
- final public witness/provider contract
- low-level `memory_order` exact source surface
- final modal foundation adoption
