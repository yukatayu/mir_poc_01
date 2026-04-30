# 496 — current L2 order-handoff source wording and emitted-artifact coupled later gate

## 目的

`specs/examples/436`、
`442`、
`471`、
`490`、
`493`
と
`p05-dice-owner-guarded-chain`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- source-wording candidate side
- emitted-artifact schema candidate side
- edge-row / vertical continuation principal keep
- readable relation vocabulary keep
- `stage` / `after` / `witness` secondary keep
- thread/node same causal language wording keep
- repo-local emitted artifact refs first
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**order-handoff surface actual adoption と witness/provider public-contract / emitted-handoff coupled later gate を保ったまま、source-wording side と emitted-artifact-schema side を adjacent but distinct later gate として helper-local manifest に actualize する current cut**
であり、

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact schema
- final emitted-handoff contract
- final public witness schema
- final public provider receipt schema
- combined provider+witness public contract
- authoritative-room `serial` sugar adoption
- low-level `memory_order` exact source surface
- final modal foundation adoption
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. order-handoff source-surface wording reserve note
   - snake_case relation family 名 principal
   - plain-language stage wording explanation layer
   - final source syntax still later
2. order-handoff emitted-artifact schema reserve note
   - refs-only reserve schema first
   - consumer-shaped schema / source-surface-first schema still later
3. order-handoff surface actual adoption
   - edge-row principal
   - stage-block secondary keep
   - repo-local emitted artifact refs first
4. witness/provider public-contract / emitted-handoff coupled later gate
   - claim/payload split first
   - witness/provider route non-collapse
   - emitted-handoff contract adjacent but distinct later gate

したがって current open problem は、
order-handoff line を surface actual adoption や witness/provider coupled-later gate で止め続けることではなく、
**source-wording side と emitted-artifact-schema side をどこまで current recommendation に上げるか**
である。

## current coupled-later cut

current package では、次を採る。

1. source-wording side は
   - edge-row / vertical continuation principal
   - readable high-level relation vocabulary keep
   - stage-block secondary keep
   - `thread と node は同じ causal language で書く` keep
   - final source-surface handoff wording later
   に置く
2. emitted-artifact-schema side は
   - repo-local emitted artifact refs first
   - source-surface actual adoption adjacent
   - witness/provider contract adjacent but not collapsed
   - final emitted-artifact schema later
   に置く
3. reached sample は `p07 / p08` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | current clean near-end order-handoff family の representative runtime inventory を読み、旧 `p07` / `p08` reached と `p05` guard-only の coupled-later reading を doc-level に再確認する |
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | principal surface / secondary surface / repo-local emitted artifact refs first を読むための runtime-side compare floor を与える |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | emitted-handoff contract side と public-contract side の adjacent-but-distinct cut を読むための canonical runtime inventory を与える。cut 自体は helper-local / doc-level judgment に残す |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json` | profile axis refs と repo-local emitted artifact refs first を読むための representative order-handoff runtime evidence を与える |

## actualization shape

current helper-local coupled-later manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `repo_local_emitted_artifact_refs`
- `source_wording_candidate_refs`
- `emitted_artifact_schema_candidate_refs`
- `coupled_default_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`

### source wording candidate refs

current helper-local cut では、source-wording side を

- `order_handoff_source_wording_candidate:<sample_id>:edge_row_vertical_continuation_principal`
- `order_handoff_source_wording_candidate:<sample_id>:readable_high_level_relation_vocabulary`
- `order_handoff_source_wording_candidate:<sample_id>:stage_block_secondary_keep`
- `order_handoff_source_wording_candidate:<sample_id>:thread_node_same_causal_language`
- `order_handoff_source_wording_candidate:<sample_id>:final_source_surface_handoff_wording_later`

として actualize する。

これは final source-surface handoff wording ではない。

### emitted-artifact schema candidate refs

current helper-local cut では、emitted-artifact side を

- `order_handoff_emitted_artifact_schema_candidate:<sample_id>:repo_local_emitted_artifact_refs_first`
- `order_handoff_emitted_artifact_schema_candidate:<sample_id>:source_surface_actual_adoption_adjacent`
- `order_handoff_emitted_artifact_schema_candidate:<sample_id>:witness_provider_contract_adjacent_not_collapsed`
- `order_handoff_emitted_artifact_schema_candidate:<sample_id>:final_emitted_artifact_schema_later`

として actualize する。

これは final emitted-artifact schema でも final emitted-handoff contract でもない。

## current recommendation

1. order-handoff line は、source-wording side と emitted-artifact-schema side を coupled-later gate として current recommendation に上げてよい。
2. current package は
   - edge-row principal
   - stage-block secondary keep
   - thread/node same causal language keep
   - repo-local emitted artifact refs first
   - final public wording / final schema later
   に置くのが自然である。
3. `p07 / p08` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current package を final source-surface handoff wording、final emitted-artifact schema、final public witness/provider/artifact contract、final modal foundation adoptionに昇格させない。

## retained alternatives

- stage-block principal promotion
- authoritative-room `serial` sugar promotion
- final public witness/provider contract first adoption
- low-level exact source wording import
- consumer-shaped emitted-artifact schema first adoption

## stop line

current package は次で止める。

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact schema
- final emitted-handoff contract
- final public witness schema
- final public provider receipt schema
- combined provider+witness public contract
- authoritative-room `serial` sugar adoption
- low-level `memory_order` exact source surface
- final modal foundation adoption
- exhaustive shared-space catalog

## current kept-later mixed-gate status

order-handoff / shared-space line の remaining mixed gate は、

1. final source-surface handoff wording
2. final emitted-artifact schema / final emitted-handoff contract
3. final public witness schema / final public provider receipt schema / combined provider+witness public contract
4. authoritative-room `serial` sugar adoption / low-level exact source wording
5. final modal foundation adoption
6. exhaustive shared-space catalog

として kept-later に残す。
