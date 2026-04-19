# 460 — current L2 order / handoff cut-relation-authority current first-line integration

## 目的

`specs/examples/405`、`406`、`407`、`408`、`411`、`416`、`421`、`428`、`436`、`442`、`448`、
`specs/examples/218`、`219`、`220`、`221...242` と
`p07-dice-late-join-visible-history`、`p08-dice-stale-reconnect-refresh`
を前提に、

- cut family decomposition
- order relation family
- authority-handoff family
- thread / node parity wording
- source wording candidate comparison
- low-level reference mapping
- current recommendation
- retained alternatives
- stop line
- remaining mixed gate

を 1 本の current integration note に束ねる。

ここで fixed するのは **Package C の current first-line / stop-line integration judgment** であり、

- final source-surface handoff syntax
- low-level `memory_order` exact public surface
- final emitted-artifact schema
- final public verifier contract
- final fairness / replay operational profile
- final shared-space catalog

は fixed しない。

## scope

- order / handoff / higher-level async-control line だけを扱う。
- mainline actualization と theory-lab line は分けて書く。
- `p07` / `p08` は corrected runnable prototype として扱い、final replay policy や final source wording と同一視しない。
- typed/theorem/model-check line の current first line は `specs/examples/459` に委ねる。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. cut family decomposition
   - `atomic_cut` は place-local rollback frontier の finalization nucleus
   - `barrier` は ordering-only candidate family
   - `snapshot_cut` / `consistent_cut` は observation-only comparison candidate
   - `durable_cut` は commit-bearing / evidence-bearing family
2. relation decomposition
   - `program_order`
   - `dependency_order`
   - `publication_order`
   - `observation_order`
   - `witness_order`
   - `finalization_order`
   - `scoped_happens_before`
3. candidate reduction
   - `authority_serial_transition_family` first
   - `witness_aware_commit_family` second
   - `event_tree_execution_view` derived/debug
   - low-level `memory_order` / `kill_dependency` family retained-later reference
4. thread / node parity note
   - source-level causal language は parity side に置く
   - lowering / evidence / transport / failure / durability / policy は asymmetry side に残す
5. adequacy corpus / boundary matrix / falsifier loop
   - order / handoff adequacy corpus は positive + negative corpus を already 持つ
   - property-to-boundary matrix は 4-way verifier split と接続済み
6. authority-handoff docs-only ratchet
   - `authority_serial_transition_contract`
   - `authority_owner_ref`
   - `authority_transition_stage_family`
   - `authority_transition_stage_sequence`
   - `authority_transition_stage_local_obligation_family`
   - `authority_transition_stage_local_obligation_row`
   - `authority_handoff_epoch_ref`
   - `witness_aware_handoff_family`
   - `handoff_witness_row`
   - `handoff_replay_attachment_ref`
   - `handoff_payload_ref`
   - `handoff_carrier_detail` still later
7. corrected runnable prototype evidence
   - `p07-dice-late-join-visible-history`
   - `p08-dice-stale-reconnect-refresh`
8. fairness / replay mixed-gate boundary
   - final operational profile / final room policy / final catalog は mixed gate

## property-to-boundary matrix

| property family | principal boundary / source | current reading | prototype / corpus evidence | not promoted here |
|---|---|---|---|---|
| same-owner / same-authority structural floor | `core_static_checker` | structural floor に寄せる | adequacy corpus `1, 11, 15` | replay / fairness / provider protocol への collapse |
| local cut non-interference | `theorem_prover_boundary` | theorem-side first choice | `p07` / `p08` `rollback_cut_non_interference` preview、corpus `3, 4` | durable / snapshot / fairness bundle 化 |
| publication-before-handoff / witness-before-handoff | `protocol_verifier_boundary` | protocol side first choice | corpus `1, 11, 12, 13` | checker floor only への矮小化 |
| replay / duplicate invalidation / late-join safety | `protocol_verifier_boundary` + `runtime_policy_boundary` | protocol/runtime split で持つ | corpus `6, 7, 13, 14`、`p07` / `p08` | checker / theorem sideへの premature import |
| room-level seriality and provider placement | `protocol_verifier_boundary` | authoritative room baseline の first line | corpus `1, 8, 9, 15` | fairness / catalog adoption |
| observation / `emit` ordering / finalization contrast | theorem + protocol + runtime comparison material | relation decomposition で分けて扱う | corpus `2, 5, 10, 17` | low-level fence public API 化 |

## cut family note

| family | current reading | keep | not claimed here |
|---|---|---|---|
| `atomic_cut` | local finalization nucleus | place-local rollback frontier | global consistent cut / durable commit / fence |
| `barrier` | ordering-only candidate | later ordering family | local cut / durable evidence |
| `snapshot_cut` / `consistent_cut` | observation-only comparison candidate | comparison vocabulary | settled repo vocabulary |
| `durable_cut` | commit-bearing / evidence-bearing family | later durable family | local `atomic_cut` synonym |

## relation decomposition and low-level mapping note

| relation family | current reading | retained low-level reference family | current guard |
|---|---|---|---|
| `program_order` | same-place program order | intra-thread / sequenced-before analog | public low-level tokenにしない |
| `dependency_order` | dependency-preserving edge | `std::memory_order::consume` / `std::kill_dependency` reference line | source core に premature import しない |
| `publication_order` | publish / release-like edge | release-like reference family | `observation_order` と collapse しない |
| `observation_order` | observe / acquire-or-consume-like edge | acquire-like / consume-like reference family | `witness_order` と collapse しない |
| `witness_order` | receipt / proof / handoff evidence edge | direct low-level equivalent は置かない | observation / fairness と collapse しない |
| `finalization_order` | local or scoped finalization edge | low-level fence / commit reference material は比較だけ | durable と collapse しない |
| `scoped_happens_before` | scope-aware admissible order relation | scope-aware happens-before / lowering comparison | authority seriality と collapse しない |

current reading は
**reference family yes / direct surface import no**
である。

## thread / node parity note

current principal wording は、

- `thread も node も place として同じ causal language で書く`
- `違いは lowering / evidence / transport / failure / durability / policy に残す`

に置く。

したがって、

- `thread == node`
- `same-process analog == distributed operational contract`

の short-hand は current wording に採らない。

## authority-handoff stage / witness note

current repo が持つ authority-handoff line は、
最初から final public contract を切るのではなく、
**family-first / minimal-row / symbolic-ref-first**
の ratchet で読むのが自然である。

### current first-line ladder

| rung | current role | why it stays |
|---|---|---|
| `authority_serial_transition_family` | first higher-level family | authoritative room baseline と最も自然につながる |
| `authority_serial_transition_contract` | minimal ordering obligation row | owner slot / witness をまだ混ぜない |
| `authority_owner_ref` | owner slot detail | subject ref の再記述を避ける |
| `authority_transition_stage_family` | stage cluster family | explicit stage list を still later に残せる |
| `authority_transition_stage_sequence` | minimal actual ordering row | `lock -> draw -> commit -> publish` を source-backed に見せる |
| `authority_transition_stage_local_obligation_family` | obligation cluster family | actual row detail を later に残せる |
| `authority_transition_stage_local_obligation_row` | minimal stage-local row | handoff / witness / payload をまだ混ぜない |
| `authority_handoff_epoch_ref` | turnover ref | witness / replay / payload を still later に残せる |
| `witness_aware_handoff_family` | witness cluster family | actual witness row や replay attachment より先に置ける |
| `handoff_witness_row` | minimal actual witness row | replay / payload / carrier detail をまだ混ぜない |
| `handoff_replay_attachment_ref` | symbolic replay attachment ref | payload / carrier detail を後段に残せる |
| `handoff_payload_ref` | symbolic payload ref | carrier detail を後段に残せる |
| `handoff_carrier_detail` | still later | runtime carrier と proof boundary を premature に混ぜない |

### current judgment

1. principal family は `authority_serial_transition_family` に置く。
2. `witness_aware_commit_family` は second candidate に留める。
3. stage / handoff / witness / replay / payload は、
   family marker と minimal row を前進させてよいが、
   final emitted-artifact schema や final public contract にしない。
4. fairness / replay final operational profile は shared-space mixed gate に残す。

## source wording candidate comparison

| candidate | current status | current recommendation | retained later as |
|---|---|---|---|
| snake_case relation-name-first wording | source-backed principal | principal wording に採る | working docs vocabulary |
| plain-language stage wording | source-backed explanation layer | explanation layer に採る | example-facing prose |
| low-level fence wording | comparison only | current packageでは採らない | backend-aligned reference wording |
| room-macro wording | comparison only | current packageでは採らない | future room profile sugar candidate |

## `p07` / `p08` の位置づけ

`p07-dice-late-join-visible-history` と
`p08-dice-stale-reconnect-refresh` は、

- order / handoff current first line の corrected runnable prototype
- `rollback_cut_non_interference` preview の sample-visible evidence
- late join / stale reconnect refresh の motivating scenario anchor

である。

current CLI output では、

- `p07` は `publish_roll_result` と `handoff_dice_authority` を経て
  `late_join_view: player_c sees result+owner history`
  まで `success` で到達する
- `p08` は stale reconnect を rollback した後、
  `refresh_owner_snapshot: stale reconnect redirected`
  まで `success` で到達する

したがって `p07` / `p08` は、

- final replay invalidation protocol
- final fairness operational profile
- final source-surface handoff syntax

の evidence ではなく、
**current order/handoff bridge floor が runnable で reached**
ことの evidence と読む。

## current judgment

1. order / handoff の current first line は、
   - cut family decomposition
   - relation decomposition principal
   - `authority_serial_transition_family` first
   - `witness_aware_commit_family` second
   - `event_tree_execution_view` derived/debug
   - snake_case relation-name wording principal + plain-language stage explanation
   - low-level `memory_order` / `kill_dependency` retained-later reference family
   に置く。
2. `atomic_cut` は local finalization nucleus に留める。
3. thread / node は same causal language で書き、差は lowering / evidence / transport / failure / durability / policy に残す。
4. authority-handoff line は family-first / minimal-row / symbolic-ref ratchet で前進させてよいが、final public contract に昇格させない。
5. `p07` / `p08` は corrected runnable prototype に留める。
6. fairness / replay final profile と shared-space final catalog は mixed gate に残す。

## retained alternatives

- `witness_aware_commit_family` stronger-family promotion
- `event_tree_execution_view` primary-control-family promotion
- low-level fence / `memory_order` wording import
- room-macro / transaction-like wording
- `snapshot_cut` / `consistent_cut` settled adoption
- stronger witness / replay / payload / carrier contract

## stop line

current package は次で止める。

- final source-surface handoff syntax
- final public low-level memory-order stance
- final emitted-artifact schema
- final theorem / protocol / runtime handoff contract
- final replay invalidation protocol
- final fairness operational profile
- final room-profile / shared-space catalog
- final carrier / transport detail

## remaining mixed gates

- fairness / replay operational profile gate
- shared-space final catalog gate
- final source-surface handoff syntax gate
- final emitted-artifact / public-contract gate
- low-level memory-order public-stance gate

## current package close

Package C は、cut / relation / authority-handoff / wording / low-level reference / mixed-gate boundary を
`p07` / `p08` evidence と adequacy corpus / falsifier loop の上で再統合した時点で close と読む。

next promoted line は `Package D` syntax / modality である。
