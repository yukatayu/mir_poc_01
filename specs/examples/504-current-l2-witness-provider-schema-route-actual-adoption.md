# 504 — current L2 witness/provider schema route actual adoption

## 目的

`specs/examples/489`、
`499`、
`502`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- witness/provider schema route actual adoption
- claim/payload split first
- witness-schema candidate keep + witness route first
- provider-receipt candidate keep + provider route first
- repo-local emitted artifact refs first
- combined public-contract candidate keep
- final emitted-handoff contract adjacent keep
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**witness/provider route actual adoption と witness/provider public-schema coupled-later gate を保ったまま、witness/provider schema route first を helper-local actual adoption として actualize する current cut**
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

1. witness/provider/artifact public-shape actual adoption
   - claim/payload split first
   - witness/provider route non-collapse
   - repo-local emitted artifact refs first
   - combined public contract later
2. witness/provider public-schema coupled-later gate
   - witness-schema candidate only
   - provider-receipt candidate only
   - combined public-contract candidate only
3. witness/provider route actual adoption
   - witness/provider route first
   - public-schema candidate keep
   - combined public-contract later
   - final emitted-handoff contract adjacent keep
4. authoritative-room / delegated-provider evidence
   - `p07` witness-bearing late join
   - `p08` stale reconnect baseline
   - `p09` delegated-provider route

したがって current open problem は、
shared-space line を public-schema coupled-later gate や witness/provider route actual adoption で止め続けることではなく、
**witness/provider schema route first を current recommendation に上げつつ、final public witness schema / provider receipt schema / combined public contract / final emitted-handoff contract を still later に残せるか**
である。

## current actual adoption cut

current package では、次を採る。

1. witness-schema route side は
   - claim/payload split first
   - witness-schema candidate keep
   - witness route first
   - repo-local emitted artifact refs first
   - combined public contract later
   に置く
2. provider-receipt route side は
   - provider-receipt candidate keep
   - provider route first
   - repo-local emitted artifact refs first
   - delegated provider attestation later
   - combined public contract later
   に置く
3. combined keep side は
   - combined public-contract candidate only
   - final emitted-handoff contract adjacent keep
   に置く
4. reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | clean near-end order-handoff family の representative runtime inventory を読み、旧 `p07` / `p08` / `p09` reached と `p05` guard-only の actual-adoption reading を doc-level に再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | witness/provider route first / public-schema candidate keep / combined public-contract later / final emitted-handoff contract adjacent keep を読むための canonical runtime inventory を与える |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | witness-schema / provider-receipt / combined public-contract candidate only を読むための canonical runtime inventory を与える。schema-route actual-adoption judgment 自体は helper-local / doc-level に残す |

## actualization shape

current helper-local actual-adoption manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `repo_local_emitted_artifact_refs`
- `witness_schema_route_refs`
- `provider_receipt_route_refs`
- `combined_public_contract_keep_refs`
- `actual_adoption_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### witness-schema route refs

current helper-local cut では、witness-schema route side を

- `witness_provider_schema_route_actual:<sample_id>:claim_payload_split_first`
- `witness_provider_schema_route_actual:<sample_id>:witness_schema_candidate_keep`
- `witness_provider_schema_route_actual:<sample_id>:witness_route_first`
- `witness_provider_schema_route_actual:<sample_id>:repo_local_emitted_artifact_refs_first`
- `witness_provider_schema_route_actual:<sample_id>:combined_public_contract_later`

として actualize する。

これは final public witness schema adoption ではない。

### provider-receipt route refs

current helper-local cut では、provider-receipt route side を

- `witness_provider_schema_route_actual:<sample_id>:provider_receipt_candidate_keep`
- `witness_provider_schema_route_actual:<sample_id>:provider_route_first`
- `witness_provider_schema_route_actual:<sample_id>:repo_local_emitted_artifact_refs_first`
- `witness_provider_schema_route_actual:<sample_id>:delegated_provider_attestation_later`
- `witness_provider_schema_route_actual:<sample_id>:combined_public_contract_later`

として actualize する。

これは final public provider receipt schema adoption でも delegated provider attestation adoption でもない。

### combined public-contract keep refs

current helper-local cut では、combined keep side を

- `witness_provider_combined_contract_keep:<sample_id>:combined_public_contract_candidate_only`
- `witness_provider_combined_contract_keep:<sample_id>:final_emitted_handoff_contract_adjacent_keep`

として actualize する。

これは combined provider+witness public contract でも final emitted-handoff contract でもない。

## current recommendation

1. shared-space line は、claim/payload split first と repo-local emitted artifact refs first を保ったまま、witness/provider schema route first を current recommendation に上げてよい。
2. current package は
   - witness-schema candidate keep + witness route first
   - provider-receipt candidate keep + provider route first
   - combined public-contract candidate keep
   - final emitted-handoff contract adjacent keep
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalog に昇格させない。

## retained alternatives

- final public witness schema first adoption
- final public provider receipt schema first adoption
- delegated provider attestation first adoption
- combined public-contract first adoption
- final emitted-handoff contract first adoption
- exhaustive shared-space catalog first adoption

## stop line

この package の先でまだ止めるものは次である。

- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted-handoff contract
- exhaustive shared-space catalog

## non-goal

この package は、

- final public witness schema adoption
- final public provider receipt schema adoption
- delegated provider attestation adoption
- combined provider+witness public contract adoption
- final emitted-handoff contract adoption
- exhaustive shared-space catalog adoption

ではない。
