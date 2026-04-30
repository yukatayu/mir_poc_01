# 489 — current L2 witness-provider-artifact public-shape actual adoption

## 目的

`specs/examples/471`、
`476`、
`477`、
`483`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- witness / provider / emitted-artifact public-shape actual adoption
- claim / payload split first
- witness route / provider route non-collapse
- repo-local emitted artifact refs first
- optional attachment refs only
- combined public contract later
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**public-shape threshold default を current repo-local actual adoption package に進め、witness route と provider route を helper-local manifest に actualize する current cut**
であり、

- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted handoff contract
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. authoritative-room vertical-slice actualization
   - profile axis refs
   - repo-local emitted artifact refs
2. `auditable_authority_witness` strengthening actualization
   - minimal witness core
   - symbolic binding ref keep
3. `delegated_rng_service` practical actualization
   - provider placement only
   - optional provider attachment cut
4. witness/provider/artifact public-shape threshold default
   - claim / payload split first
   - repo-local emitted artifact refs first
   - optional attachment refs only
   - combined public contract later

したがって current open problem は、
public-shape line を threshold default に留め続けることではなく、
**claim / payload split と repo-local emitted artifact refs を崩さずに、どの repo-local actual adoption package を current recommendation に上げるか**
である。

## current actual adoption cut

current package では、次を採る。

1. witness side は
   - claim / payload split first
   - witness attachment refs only
   - symbolic binding ref keep
   - combined public contract later
   に置く
2. provider side は
   - provider attachment refs only
   - optional provider attachment keep
   - delegated provider attestation later
   - combined public contract later
   に置く
3. emitted-artifact side は
   - repo-local emitted artifact refs first
   - final emitted handoff contract later
   に置く
4. witness route と provider route は adjacent に読むが、collapse しない
5. reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | current clean near-end order-handoff family が旧 `p07` / `p08` / `p09` reached と `p05` guard-only の actual-adoption reading を carry-over していることを再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | threshold default を actual adoption の compare floor として carry-over する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json` | witness-bearing sample では witness route actualization が strengthening floor を carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | delegated-provider sample では provider route actualization が practical floor を carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json` | profile axis refs と repo-local emitted artifact refs を actual adoption が保持することを再確認する |

## actualization shape

current helper-local actual adoption manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `witness_route_refs`
- `provider_route_refs`
- `repo_local_emitted_artifact_refs`
- `actual_adoption_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### witness route refs

current helper-local cut では、witness route を

- `witness_public_shape_route:<sample_id>:claim_payload_split_first`
- `witness_public_shape_route:<sample_id>:witness_attachment_refs_only`
- `witness_public_shape_route:<sample_id>:symbolic_binding_ref_keep`
- `witness_public_shape_route:<sample_id>:combined_public_contract_later`

として actualize する。

これは final public witness schema ではない。

### provider route refs

current helper-local cut では、provider route を

- `provider_public_shape_route:<sample_id>:provider_attachment_refs_only`
- `provider_public_shape_route:<sample_id>:optional_provider_attachment_keep`
- `provider_public_shape_route:<sample_id>:delegated_provider_attestation_later`
- `provider_public_shape_route:<sample_id>:combined_public_contract_later`

として actualize する。

これは final public provider receipt schema でも delegated provider attestation adoption でもない。

## current recommendation

1. witness/provider/artifact public-shape line は、claim/payload split first と repo-local emitted artifact refs first の actual adoption package を current recommendation に上げてよい。
2. current package は
   - witness route / provider route non-collapse
   - optional attachment refs only
   - combined public contract later
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final public witness schema、final public provider receipt schema、final emitted handoff contract に昇格させない。

## retained alternatives

- note-only witness
- scalar receipt first
- delegated provider attestation first
- combined provider+witness public contract first
- exhaustive shared-space catalog first

## stop line

current package は次で止める。

- final public witness schema
- final public provider receipt schema
- delegated provider attestation
- combined provider+witness public contract
- final emitted handoff contract
- exhaustive shared-space catalog

## next line

shared-space line の remaining mixed gate は、

1. final public witness schema / provider receipt schema
2. delegated provider attestation
3. combined provider+witness public contract / final emitted handoff contract
4. exhaustive shared-space catalog

として kept-later に残す。
