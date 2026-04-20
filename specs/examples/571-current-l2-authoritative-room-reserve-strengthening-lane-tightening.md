# 571 — current L2 authoritative-room reserve strengthening lane tightening

## 目的

`specs/examples/476`、
`477`、
`478`、
`570`
と
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- `auditable_authority_witness` second strengthening
- `delegated_rng_service` second practical route
- model-check second-line concretization
- first completion line との boundary
- reserve lane helper summary
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**first completion line を `p07 / p08` representative pair に保ったまま、reserve 側の strengthening / practical / checker-second-line package を `run-source-sample` helper summary の separate status bundle に actualize する current cut**
であり、

- final public witness schema
- final public provider receipt schema
- concrete model-check tool brand / schema
- actual public checker migration
- combined witness/provider public contract
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. first completion line
   - authoritative-room first default profile は `specs/examples/467` と `570` で current recommendation に上がっている
   - representative reached pair は `p07 / p08` に保つ
2. reserve strengthening family
   - `auditable_authority_witness` は `specs/examples/476` で strengthening package として close 済み
   - `delegated_rng_service` は `specs/examples/477` で practical package として close 済み
   - model-check second line は `specs/examples/478` で helper-local second line として close 済み
3. current boundary
   - witness strengthening / provider placement / model-check second line を 1 つの semantic family に collapse しない
   - first completion line と reserve package 群を collapse しない
   - room profile / public contract / checker migration を premature に widen しない

したがって current open problem は、
reserve package を discovery することではなく、
**first completion line を壊さずに reserve package 群をどこまで operational helper summary に actualize してよいか**
である。

## current actualization cut

current package では、次を採る。

1. `run-source-sample` helper summary に
   `authoritative_room_reserve_strengthening_lane`
   を追加する
2. lane 自体は 1 本として見せるが、
   - `witness_strengthening_status`
   - `delegated_rng_service_status`
   - `model_check_second_line_status`
   を separate status のまま保持する
3. `p07`
   - witness strengthening reached
   - delegated RNG guarded
   - model-check second line reached
4. `p08`
   - witness strengthening guarded
   - delegated RNG guarded
   - model-check second line reached
5. `p09`
   - witness strengthening guarded
   - delegated RNG reached
   - model-check second line reached
6. `p05`
   - representative reserve strengthening sample setの外として guard-only に残す
7. first completion line boundary は helper summary に明示し、
   `p07 / p08` pair を principal に保ったまま reserve 側を追加する

## actual runnable evidence

| evidence | current reading |
|---|---|
| `current_l2_operational_cli` | `authoritative_room_reserve_strengthening_lane` を JSON / pretty の両方で inspect できる |
| `p07-dice-late-join-visible-history` | witness strengthening reached + model-check second line reached の representative reserve sample |
| `p08-dice-stale-reconnect-refresh` | reconnect default profile を保ったまま model-check second line only を reached にできる sample |
| `p09-dice-delegated-rng-provider-placement` | delegated RNG practical route reached + model-check second line reached の representative reserve sample |
| `p05-dice-owner-guarded-chain` | reserve strengthening lane 自体を widen しすぎない guard-only sample |

## actualization shape

current helper summary は少なくとも次を持つ。

- `status`
- `lane_kind`
- `witness_strengthening_status`
- `delegated_rng_service_status`
- `model_check_second_line_status`
- `witness_strengthening_refs`
- `delegated_rng_service_refs`
- `model_check_second_line_refs`
- `first_line_boundary_refs`
- `reserve_boundary_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`
- `guard_reason`

### first-line boundary refs

current helper-local cut では、first completion line boundary を

- `first_line_boundary:representative_pair_kept_at_p07_p08`
- `first_line_boundary:authoritative_room_default_profile_stays_principal`
- `first_line_boundary:authority_rng_default_profile_unchanged`

として actualize する。

`p09` では追加で

- `first_line_boundary:delegated_rng_not_promoted_into_default_pair`

を持たせてよい。

### reserve boundary refs

current helper-local cut では、reserve lane boundary を

- `reserve_boundary:auditable_authority_witness_second_strengthening`
- `reserve_boundary:delegated_rng_service_second_practical`
- `reserve_boundary:model_check_second_line_not_room_profile`
- `reserve_boundary:public_checker_contract_kept_later`
- `reserve_boundary:witness_provider_combined_public_contract_later`

として actualize する。

これは final public witness/provider/checker contract ではない。

## current recommendation

1. reserve strengthening lane helper summary tightening は current package として close してよい。
2. current cut は
   - first completion line を `p07 / p08` representative pair に保つ
   - reserve 側の strengthening / practical / model-check second line を separate status のまま operational summary に actualize する
   - final public witness/provider/checker contract を先取りしない
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. current queue は zero ではなく、reserve strengthening lane close 後も docs/report closeout と later mixed gate residual が残る。

## retained alternatives

- witness/provider combined public contract first
- delegated RNG を first completion line に昇格
- model-check second line を room profile line に collapse
- concrete model-check tool brand first
- final public checker migration first

## stop line

current package は次で止める。

- final public witness schema
- final public provider receipt schema
- combined witness/provider public contract
- concrete model-check tool brand / schema
- actual public checker migration
- exhaustive shared-space catalog
- distributed fairness theorem

## next self-driven line

current package を close した後の principal self-driven line は、

1. Package 98 docs / plan / progress / tasks / traceability closeout
2. 二大問題それぞれの簡潔な日本語解説付き sample 追加

に移るのが自然である。
