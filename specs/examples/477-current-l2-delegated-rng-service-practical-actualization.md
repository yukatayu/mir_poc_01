# 477 — current L2 delegated RNG service practical actualization

## 目的

`specs/examples/124`、
`467`、
`471`、
`476`
と
`p09-dice-delegated-rng-provider-placement`
を前提に、

- `delegated_rng_service` second practical package
- provider placement / authority placement split
- provider optional attachment cut
- unchanged witness-core boundary
- helper-local practical actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**provider placement を final public provider receipt schema や delegated attestation へ上げずに helper-local practical manifest と narrow prototype に置く current cut**
であり、

- final public provider receipt schema
- delegated provider attestation public contract
- `delegated_rng_service + auditable_authority_witness` combined public contract
- `distributed_randomness_provider`
- control-plane separated carrier
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 2 actual adoption package
   - authoritative room default profile
   - relation decomposition principal
   - `authority_serial_transition_family` first
2. `delegated_rng_service` provider-placement current recommendation
   - authority placement は `single_room_authority` のまま
   - `fairness_source` だけを provider placement として差し替える
   - provider receipt / attestation は optional attachment として later に残す
3. `auditable_authority_witness` strengthening actualization
   - witness core は room profile claim と collapse しない
   - provider placement と fairness claim strengthening は別軸に保つ
4. runnable prototype / helper-local actualization floor
   - `p07 / p08` authoritative-room baseline evidence
   - `p09` delegated provider practical evidence

したがって current open problem は、
`delegated_rng_service` を discovery することではなく、
**provider placement をどこまで executable / helper-local actualization に上げるか**
である。

## current actualization cut

current package では、次を採る。

1. room profile には
   - `fairness_source = delegated_rng_service`
   - `fairness_claim = opaque_authority_trust`
   を置く current practical cutを first actualization にする
2. authority は
   - request accept / reject
   - publish
   - handoff
   - room state mutation
   の owner のままに保つ
3. provider は draw を返すが、state transition を commit しない
4. `provider_draw_ref` / `provider_receipt_ref` は optional attachment に留める
5. `auditable_authority_witness` との combination は kept-later combination として retained し、provider placement と fairness strengthening を collapse しない
6. actual runnable reached sample は `p09` に取り、`p07 / p08` baseline sample は guard-only contrast に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_delegated_rng_service_practical_actualization` | `p09` reached、`p07/p08` guard-only の provider-placement manifest を machine-check する |
| `p09-dice-delegated-rng-provider-placement` | provider draw は delegated capability が返し、publish / handoff / commit は authority が保持する narrow prototype として使える |
| `current_l2_source_sample_runner` / `current_l2_operational_cli` | prototype explicit path / adjacent host-plan path の runnable floor 自体は引き続き green である |
| `current_l2_authoritative_room_vertical_slice_actualization` / `current_l2_auditable_authority_witness_strengthening` | authority baseline と witness strengthening line を separate に保ったまま provider placement line を追加できることを再確認する |

## actualization shape

current helper-local practical manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `provider_boundary_refs`
- `optional_attachment_refs`
- `runtime_evidence_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `contrast_refs`
- `guard_refs`
- `kept_later_refs`

### provider boundary refs

current helper-local cut では、provider boundary を

- `provider_boundary:placement:delegated_rng_service`
- `provider_boundary:authority_keeps_commit`
- `provider_boundary:provider_returns_draw_not_state_transition`
- `provider_boundary:room_state_mutation_by_authority`
- `provider_boundary:witness_core_unchanged`

として actualize する。

### optional attachment refs

current helper-local cut では、provider attachment を

- `optional_attachment:provider_draw_ref`
- `optional_attachment:provider_receipt_ref`

として actualize する。

ここで optional attachment は current helper-local manifest であり、
final public provider receipt schema ではない。

## current recommendation

1. `delegated_rng_service` は second practical package として close してよい。
2. current cut は
   - provider placement だけを room profile に上げる
   - authority keeps commit を保つ
   - provider receipt / attestation を later attachment に残す
   に置くのが自然である。
3. `p09` reached、`p07/p08` guard-only の組み合わせは semantically honest である。
4. `auditable_authority_witness` との combined public contract を先取りせず、provider placement と fairness strengthening を別 package に保つ。

## retained alternatives

- `authority_rng` baseline
- `delegated_rng_service` + `auditable_authority_witness` combined actualization
- `delegated_provider_attestation`
- `distributed_randomness_provider`
- control-plane separated carrier first

## stop line

current package は次で止める。

- final public provider receipt schema
- delegated provider attestation public contract
- `delegated_rng_service + auditable_authority_witness` combined public contract
- `distributed_randomness_provider`
- control-plane separated carrier
- exhaustive shared-space catalog

## next self-driven line

current package を close した後の queue は引き続き nonzero である。

next self-driven line は、

1. model-check second-line concretization

に残すのが自然である。
