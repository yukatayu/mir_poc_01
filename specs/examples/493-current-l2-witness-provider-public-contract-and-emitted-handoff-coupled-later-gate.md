# 493 — current L2 witness/provider public-contract and emitted-handoff coupled later gate

## 目的

`specs/examples/471`、
`483`、
`489`、
`490`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- witness/provider public-contract coupled later gate
- emitted-handoff contract coupled later gate
- claim / payload split first
- witness/provider route non-collapse
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**witness/provider/artifact public-shape actual adoption と order-handoff surface actual adoption を保ったまま、public contract side と emitted-handoff contract side を adjacent but distinct later gate として helper-local manifest に actualize する current cut**
であり、

- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted-handoff contract
- final source-surface handoff wording
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. witness/provider/artifact public-shape actual adoption
   - claim / payload split first
   - witness/provider route non-collapse
   - repo-local emitted artifact refs first
   - combined public contract later
2. order-handoff surface actual adoption
   - edge-row principal
   - stage-block secondary keep
   - repo-local emitted artifact refs first
3. authoritative-room vertical-slice actualization
   - profile axis refs
   - repo-local emitted artifact refs
4. delegated-rng-service practical actualization
   - optional provider attachment keep
   - delegated provider attestation later

したがって current open problem は、
public-shape line を actual adoption で止め続けることではなく、
**public contract side と emitted-handoff contract side をどこまで coupled-later gate として current recommendation に上げるか**
である。

## current coupled-later cut

current package では、次を採る。

1. witness public-contract side は
   - claim / payload split first
   - witness route non-collapse
   - final public witness schema later
   - combined public contract later
   に置く
2. provider public-contract side は
   - optional provider attachment keep
   - provider route non-collapse
   - delegated provider attestation later
   - combined public contract later
   に置く
3. emitted-handoff contract side は
   - repo-local emitted artifact refs first
   - public contract adjacent but not collapsed
   - final emitted-handoff contract later
   に置く
4. order-handoff surface actual adoption は compare pressure として再利用してよいが、
   public contract side と collapse しない
5. reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | current clean near-end order-handoff family が旧 `p07` / `p08` / `p09` reached と `p05` guard-only の coupled-later reading を carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | witness/provider route non-collapse と repo-local emitted artifact refs first を compare floor として carry-over する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json` | `p07 / p08` reached sample では emitted-handoff side が source-surface actual adoption pressure を carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json` | profile axis refs と repo-local emitted artifact refs を coupled-later gate が保持することを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | delegated-provider sample では provider public-contract side が practical floor を carry-over していることを再確認する |

## actualization shape

current helper-local coupled-later manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `witness_contract_candidate_refs`
- `provider_contract_candidate_refs`
- `emitted_contract_candidate_refs`
- `coupled_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### witness contract candidate refs

current helper-local cut では、witness side を

- `witness_public_contract_candidate:<sample_id>:claim_payload_split_first`
- `witness_public_contract_candidate:<sample_id>:witness_route_noncollapsed`
- `witness_public_contract_candidate:<sample_id>:final_public_witness_schema_later`
- `witness_public_contract_candidate:<sample_id>:combined_public_contract_later`

として actualize する。

これは final public witness schema ではない。

### provider contract candidate refs

current helper-local cut では、provider side を

- `provider_public_contract_candidate:<sample_id>:optional_provider_attachment_keep`
- `provider_public_contract_candidate:<sample_id>:provider_route_noncollapsed`
- `provider_public_contract_candidate:<sample_id>:delegated_provider_attestation_later`
- `provider_public_contract_candidate:<sample_id>:combined_public_contract_later`

として actualize する。

これは final public provider receipt schema でも delegated provider attestation adoption でもない。

### emitted contract candidate refs

current helper-local cut では、emitted-handoff side を

- `emitted_handoff_contract_candidate:<sample_id>:repo_local_emitted_artifact_refs_first`
- `emitted_handoff_contract_candidate:<sample_id>:public_contract_adjacent_not_collapsed`
- `emitted_handoff_contract_candidate:<sample_id>:final_emitted_handoff_contract_later`

として actualize する。

これは final emitted-handoff contract ではない。

## current recommendation

1. shared-space / order-handoff line は、witness/provider public-contract side と emitted-handoff contract side を coupled-later gate として current recommendation に上げてよい。
2. current package は
   - claim / payload split first
   - repo-local emitted artifact refs first
   - witness/provider route non-collapse
   - combined public contract later
   - final emitted-handoff contract later
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final public witness schema、final public provider receipt schema、combined provider+witness public contract、final emitted-handoff contract、final source-surface handoff wording に昇格させない。

## retained alternatives

- final public witness schema first adoption
- final public provider receipt schema first adoption
- delegated provider attestation first adoption
- final emitted-handoff contract first adoption
- combined provider+witness/emitted unified contract first adoption
- exhaustive shared-space catalog first adoption

## stop line

current package は次で止める。

- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted-handoff contract
- final source-surface handoff wording
- exhaustive shared-space catalog

## next line

shared-space / order-handoff line の remaining mixed gate は、

1. final public witness schema / final public provider receipt schema
2. delegated provider attestation / combined provider+witness public contract
3. final emitted-handoff contract / final source-surface handoff wording
4. exhaustive shared-space catalog

として kept-later に残す。
