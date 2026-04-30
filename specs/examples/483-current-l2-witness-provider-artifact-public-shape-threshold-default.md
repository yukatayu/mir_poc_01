# 483 — current L2 witness-provider-artifact public-shape threshold default

## 目的

`specs/examples/467`、
`471`、
`476`、
`477`
と
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`、
`p05-dice-owner-guarded-chain`
を前提に、

- witness / provider / emitted-artifact public-shape threshold default
- claim / payload split first
- repo-local emitted artifact refs first
- optional attachment refs only
- combined public contract later
- helper-local threshold actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**authoritative-room vertical slice、auditable witness strengthening、delegated provider practical actualization を保ったまま、public witness schema / provider receipt / emitted-artifact reserve shape の current default threshold を helper-local に actualize する current cut**
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

1. Problem 2 actual adoption package
   - authoritative room default profile
   - relation decomposition principal
   - `authority_serial_transition_family` first
2. authoritative-room vertical-slice actualization
   - repo-local emitted artifact refs
   - late join / stale reconnect default profile evidence
3. `auditable_authority_witness` strengthening actualization
   - claim / payload split preserve
   - minimal witness core
   - symbolic binding ref keep
4. `delegated_rng_service` practical actualization
   - provider placement only
   - authority-kept-commit
   - optional provider attachment cut

したがって current open problem は、
public witness/provider/artifact shape を final contract にすること自体ではなく、
**claim / payload split と repo-local emitted artifact refs を崩さずに、どこまで reserve public-shape threshold として actualize するか**
である。

## current actualization cut

current package では、次を採る。

1. room profile には current default profile axis を残す
2. witness side は
   - claim / payload split first
   - witness attachment refs only
   - final public witness schema later
   に分ける
3. provider side は
   - provider placement only
   - optional attachment refs only
   - delegated provider attestation later
   に分ける
4. emitted-artifact side は
   - repo-local emitted artifact refs first
   - final emitted handoff contract later
   に分ける
5. combined provider+witness public contract は still later に残す
6. reached sample は `p07 / p08 / p09` に取り、`p05` は guard-only に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | current clean near-end order-handoff family が旧 `p07` / `p08` / `p09` reached と `p05` guard-only の threshold reading を carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json` | baseline room-default profile と repo-local emitted artifact refs を threshold が carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json` | witness-bearing sample では minimal witness core を threshold が witness attachment reserve として扱うことを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | delegated-provider sample では optional provider attachment refs を threshold が carry-over していることを再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | representative runtime corpus の runnable floor 自体は引き続き green である |

## actualization shape

current helper-local threshold manifest は少なくとも次を持つ。

- `profile_axis_refs`
- `witness_attachment_refs`
- `provider_attachment_refs`
- `emitted_artifact_reserve_refs`
- `threshold_default_refs`
- `compare_floor_refs`
- `contrast_refs`
- `guard_refs`
- `kept_later_refs`

### threshold default refs

current helper-local cut では、threshold default を

- `public_shape_threshold_default:claim_payload_split_first`
- `public_shape_threshold_default:repo_local_emitted_artifact_refs_first`
- `public_shape_threshold_default:optional_attachment_refs_only`
- `public_shape_threshold_default:combined_public_contract_later`

として actualize する。

これは final public witness/provider/artifact contract ではなく、
reserve public-shape threshold の current default である。

### witness/provider attachment refs

current helper-local cut では、

- `p07` は witness attachment reserve を持つ
- `p08` は baseline replay/refresh profile と emitted-artifact reserve だけを持つ
- `p09` は provider attachment reserve を持つ

という非対称 reached setをそのまま保ってよい。

これは final combined public contract ではなく、
sample-driven reserve threshold である。

## current recommendation

1. witness/provider/artifact public-shape threshold package は、この threshold default で close してよい。
2. current default は
   - claim / payload split first
   - repo-local emitted artifact refs first
   - optional attachment refs only
   - combined public contract later
   に置くのが自然である。
3. `p07 / p08 / p09` reached、`p05` guard-only の組み合わせは semantically honest である。
4. threshold manifest を final public witness schema、final public provider receipt schema、final emitted handoff contract に昇格させない。

## retained alternatives

- note-only witness
- delegated provider attestation first
- combined provider+witness public contract first
- scalar receipt first
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

current self-driven queue はこの threshold package で principal line を close してよい。

その後に残る line は、

1. actual discharge transport / public theorem contract actual adoption
2. first settled property language / concrete model-check tool brand actual adoption
3. final public witness/provider/artifact contract actual adoption

の later mixed gate と、
packaging / exhaustive catalog / broader application target の user-spec residual である。
