# 499 — current L2 witness/provider public schema coupled later gate

## 目的

`specs/examples/489`、
`493`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- final public witness schema candidate side
- final public provider receipt candidate side
- combined public-contract candidate side
- emitted-handoff contract adjacent keep
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**witness/provider public-contract / emitted-handoff coupled later gate を保ったまま、final public witness schema side と final public provider receipt side と combined public-contract side を coupled-later candidate として helper-local manifest に actualize する current cut**
であり、

- final public witness schema
- final public provider receipt schema
- delegated provider attestation
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
2. witness/provider public-contract / emitted-handoff coupled later gate
   - witness/provider public-contract side candidate
   - emitted-handoff contract side candidate
   - final public contract later

したがって current open problem は、
shared-space line を public-shape actual adoption や public-contract/emitted-handoff coupled-later gate で止め続けることではなく、
**final public witness schema side と final public provider receipt side と combined public-contract side をどこまで current recommendation に上げるか**
である。

## current coupled-later cut

current package では、次を採る。

1. final public witness schema side は
   - claim/payload split first
   - witness route noncollapsed
   - symbolic binding ref keep
   - final public witness schema later
   に置く
2. final public provider receipt side は
   - provider route noncollapsed
   - optional provider attachment keep
   - delegated provider attestation adjacent keep
   - final public provider receipt schema later
   に置く
3. combined public-contract side は
   - witness/provider routes noncollapsed
   - repo-local emitted artifact refs first
   - combined public-contract candidate only
   - final emitted-handoff contract adjacent keep
   に置く
4. reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | clean near-end order-handoff family の representative runtime inventory を読み、旧 `p07` / `p08` / `p09` reached と `p05` guard-only の coupled-later reading を doc-level に再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | witness/provider public-contract side / emitted-handoff contract side の relation を読むための canonical runtime inventory を与える。coupled-later compare judgment 自体は helper-local / doc-level に残す |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | claim/payload split first / witness-provider route noncollapse / repo-local emitted artifact refs first を読むための shared-space-side runtime inventory を与える |

## actualization shape

current helper-local coupled-later manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `witness_schema_candidate_refs`
- `provider_receipt_candidate_refs`
- `combined_public_contract_candidate_refs`
- `coupled_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### witness schema candidate refs

current helper-local cut では、final public witness schema side を

- `witness_schema_candidate:<sample_id>:claim_payload_split_first`
- `witness_schema_candidate:<sample_id>:witness_route_noncollapsed`
- `witness_schema_candidate:<sample_id>:symbolic_binding_ref_keep`
- `witness_schema_candidate:<sample_id>:final_public_witness_schema_later`

として actualize する。

これは final public witness schema ではない。

### provider receipt candidate refs

current helper-local cut では、final public provider receipt side を

- `provider_receipt_candidate:<sample_id>:provider_route_noncollapsed`
- `provider_receipt_candidate:<sample_id>:optional_provider_attachment_keep`
- `provider_receipt_candidate:<sample_id>:delegated_provider_attestation_adjacent_keep`
- `provider_receipt_candidate:<sample_id>:final_public_provider_receipt_schema_later`

として actualize する。

これは final public provider receipt schema でも delegated provider attestation adoption でもない。

### combined public-contract candidate refs

current helper-local cut では、combined public-contract side を

- `combined_public_contract_candidate:<sample_id>:witness_provider_routes_noncollapsed`
- `combined_public_contract_candidate:<sample_id>:repo_local_emitted_artifact_refs_first`
- `combined_public_contract_candidate:<sample_id>:combined_public_contract_candidate_only`
- `combined_public_contract_candidate:<sample_id>:final_emitted_handoff_contract_adjacent_keep`

として actualize する。

これは combined provider+witness public contract ではない。

## current recommendation

1. shared-space line は、final public witness schema side と final public provider receipt side と combined public-contract side を coupled-later gate として current recommendation に上げてよい。
2. current package は
   - final public witness/provider schema candidate only
   - combined public-contract candidate only
   - final emitted-handoff contract adjacent keep
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalog に昇格させない。

## retained alternatives

- final public witness schema first adoption
- final public provider receipt schema first adoption
- combined public-contract first adoption
- final emitted-handoff contract first adoption
- delegated provider attestation first adoption

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
