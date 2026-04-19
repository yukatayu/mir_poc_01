# 512 — current L2 witness/provider emitted-contract representative trace-alignment bridge

## 目的

`specs/examples/493`、
`502`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- witness/provider emitted-contract representative trace alignment
- route actual adoption と coupled-later gate の pair alignment
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**witness/provider route actual adoption と emitted-contract coupled-later gate を representative corpus で pair-align する helper-local trace bridge**
であり、

- final public witness schema
- final public provider receipt schema
- delegated provider attestation adoption
- combined provider+witness public contract
- final emitted-handoff contract
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. witness/provider emitted-contract coupled-later gate
   - claim/payload split first
   - repo-local emitted artifact refs first
   - combined public contract later
   - final emitted-handoff contract later
2. witness/provider route actual adoption
   - route first
   - repo-local emitted artifact refs first
   - public-schema candidate keep
3. representative prototype corpus
   - `p07 / p08 / p09` reached
   - `p05` guard-only

したがって current open problem は、
emitted-contract line を coupled-later gate で止めたままにすることではなく、
**representative reached sample に対して route side と emitted-contract side の pair alignment を helper-local に actualize できるか**
である。

## current trace-alignment cut

current package では、次を採る。

1. pair alignment anchor は repo-local emitted artifact refs に置く
2. route side pair refs と emitted-contract side pair refs は
   `witness_provider_emitted_contract_trace_alignment_pair:<sample_id>:<obligation_kind>`
   の exact match に置く
3. representative reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す
4. alignment bridge は helper-local runtime support / focused runtime test に留める
5. final public witness/provider contract 群には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge` | route actual adoption と coupled-later gate を束ね、representative corpus の pair alignment を helper-local に actualize する runtime support |
| `current_l2_witness_provider_emitted_contract_trace_alignment_bridge` | `p07 / p08 / p09` reached、`p05` guard-only の trace-alignment bridge を machine-check する focused runtime test |
| `witness_provider_emitted_contract_trace_alignment_pair:*` refs | route side と emitted-contract side の alignment anchor を subject-local に固定する repo-local ref family |

## current recommendation

1. witness/provider emitted-contract line は representative trace-alignment bridge を current recommendation に上げてよい。
2. pair alignment predicate は repo-local emitted artifact refs anchored exact-match を first predicate に置く。
3. `p05` は guard-only contrast として保持する。
4. final public witness/provider contract 群と final emitted-handoff contract は still later に残す。

## retained alternatives

- coupled-later gate だけで止める
- final public contract first adoption
- provider attestation first adoption
- final emitted-handoff contract first adoption
- exhaustive shared-space catalog first adoption

## stop line

current package は次で止める。

- final public witness schema
- final public provider receipt schema
- delegated provider attestation adoption
- combined provider+witness public contract
- final emitted-handoff contract
- exhaustive shared-space catalog
