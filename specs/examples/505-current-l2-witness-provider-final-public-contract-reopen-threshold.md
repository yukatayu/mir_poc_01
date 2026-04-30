# 505 — current L2 witness/provider final public-contract reopen threshold

## 目的

`specs/examples/493`、
`499`、
`504`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- final public witness/provider contract reopen threshold
- public-schema pair first
- delegated attestation + combined public-contract second
- final emitted-handoff contract third
- exhaustive shared-space catalog later
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで固定するのは、
**witness/provider public-contract / emitted-handoff coupled-later gate と witness/provider schema route actual adoption を保ったまま、remaining shared-space final public-contract gate の reopen 順を helper-local threshold として actualize する current cut**
であり、

- final public witness schema
- final public provider receipt schema
- delegated provider attestation adoption
- combined provider+witness public contract adoption
- final emitted-handoff contract adoption
- exhaustive shared-space catalog adoption

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. witness/provider public-contract / emitted-handoff coupled-later gate
   - claim/payload split first
   - witness/provider route non-collapse
   - repo-local emitted artifact refs first
   - combined public contract later
   - final emitted-handoff contract later
   - next line:
     1. final public witness schema / final public provider receipt schema
     2. delegated provider attestation / combined provider+witness public contract
     3. final emitted-handoff contract / final source-surface handoff wording
     4. exhaustive shared-space catalog
2. witness/provider public-schema coupled-later gate
   - witness-schema candidate only
   - provider-receipt candidate only
   - combined public-contract candidate only
3. witness/provider schema route actual adoption
   - witness-schema candidate keep + witness route first
   - provider-receipt candidate keep + provider route first
   - combined public-contract candidate keep
   - final emitted-handoff contract adjacent keep

したがって current open problem は、
remaining shared-space final public-contract gate を unordered mixed gate として残すことではなく、
**public-schema pair first / delegated-attestation-and-combined-contract second / final emitted-handoff contract third の reopen 順を current recommendation に上げつつ、final public contract adoption そのものは still later に残せるか**
である。

## current threshold cut

current package では、次を採る。

1. first reopen pair は
   - final public witness schema
   - final public provider receipt schema
   に置く
2. second reopen pair は
   - delegated provider attestation
   - combined provider+witness public contract
   に置く
3. third reopen は
   - final emitted-handoff contract
   に置く
4. exhaustive shared-space catalog は threshold の外に kept-later として残す
5. reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | clean near-end order-handoff family の representative runtime inventory を読み、旧 `p07` / `p08` / `p09` reached と `p05` guard-only の threshold reading を doc-level に再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | witness/provider schema route first と combined public-contract candidate keep を読むための canonical runtime inventory を与える |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | combined public-contract later / final emitted-handoff contract later を読むための canonical runtime inventory を与える。reopen-threshold judgment 自体は helper-local / doc-level に残す |

## threshold shape

current helper-local threshold manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `repo_local_emitted_artifact_refs`
- `witness_schema_route_refs`
- `provider_receipt_route_refs`
- `combined_public_contract_keep_refs`
- `final_public_contract_reopen_sequence_refs`
- `threshold_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### reopen sequence refs

current helper-local cut では、reopen 順を

- `witness_provider_final_contract_reopen:<sample_id>:public_schema_pair_first`
- `witness_provider_final_contract_reopen:<sample_id>:delegated_attestation_and_combined_contract_second`
- `witness_provider_final_contract_reopen:<sample_id>:final_emitted_handoff_contract_third`
- `witness_provider_final_contract_reopen:<sample_id>:exhaustive_shared_space_catalog_later`

として actualize する。

これは final public witness/provider contract adoption ではない。

## current recommendation

1. shared-space final public-contract line は、public-schema pair first を current recommendation に上げてよい。
2. current package は
   - public-schema pair first
   - delegated attestation + combined public-contract second
   - final emitted-handoff contract third
   - exhaustive shared-space catalog later
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalog に昇格させない。
5. order-handoff final source-surface wording は shared-space final public-contract reopen threshold に collapse しない。

## retained alternatives

- final public witness schema first single adoption
- final public provider receipt schema first single adoption
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
