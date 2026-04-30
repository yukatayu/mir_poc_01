# 476 — current L2 auditable authority witness strengthening actualization

## 目的

`specs/examples/123`、
`467`、
`471`、
`475`
と
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- `auditable_authority_witness` second strengthening package
- room profile claim / audit payload split
- minimal witness core
- helper-local actualization cut
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**minimal witness core を final public witness schema へ上げずに helper-local strengthening manifest に置く current cut**
であり、

- final public witness schema
- provider receipt / provider attestation public contract
- `delegated_rng_service` practical package
- distributed fairness theorem
- exhaustive shared-space catalog

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 2 actual adoption package
   - authoritative room default profile
   - relation decomposition principal
   - `authority_serial_transition_family` first
2. `auditable_authority_witness` minimal witness core
   - room profile には claim だけを残す
   - audit / receipt side に
     - `witness_kind`
     - `action_ref`
     - `draw_slot`
     - `draw_result`
     を置く
3. authoritative-room vertical-slice actualization
   - `p07` / `p08` reached
   - `p05` guard-only
4. principal theory spine / compatibility metatheory package
   - room-profile claim と witness payload を collapse しない
   - helper-local actualization と final public contract を collapse しない

historical `p07 / p08 / p05` labelsは compare-anchor memory として残るが、
current active evidence は clean-near-end `06_auditable_authority_witness` と
`01_authorized_roll_publish_handoff` を中心に読む。

したがって current open problem は、
`auditable_authority_witness` を discovery することではなく、
**minimal witness core をどこまで helper-local actualization に上げるか**
である。

## current actualization cut

current package では、次を採る。

1. room profile には
   - `fairness_claim = auditable_authority_witness`
   を残す
2. witness payload 自体は audit / receipt side に残す
3. historical package-memory compare floor では `p07` を reached anchor に取りつつ、current runnable evidence は clean-near-end `06_auditable_authority_witness` に置く
4. `p08` は authoritative-room default profile の evidence ではあるが、witness-bearing draw sample ではないので historical guard-only compare anchor に残す
5. `p05` は current default room profile 自体が reached しない historical guard-only compare anchor に残す
6. `draw_result` は current helper-local cut では
   final public scalar serialization ではなく
   action-bound symbolic binding ref として actualize してよい

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 06_auditable_authority_witness --format json` | current clean near-end witness-bearing sample が旧 `p07` reached / `p08` `p05` guard-only の strengthening reading を carry-over していることを再確認する |
| `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_roll_publish_handoff --format json` | witness strengthening が room-default vertical slice の上に乗っていることを再確認する |
| `python3 scripts/clean_near_end_samples.py run order-handoff --format json` | current clean near-end order-handoff family の runnable floor 自体は引き続き green である |

## actualization shape

current helper-local strengthening manifest は少なくとも次を持つ。

- `fairness_claim_refs`
- `witness_core_refs`
- `witness_binding_refs`
- `runtime_evidence_refs`
- `repo_local_emitted_artifact_refs`
- `compare_floor_refs`
- `contrast_refs`
- `guard_refs`
- `kept_later_refs`

### witness core refs

current helper-local cut では、minimal witness core を

- `witness_field:witness_kind`
- `witness_field:action_ref`
- `witness_field:draw_slot`
- `witness_field:draw_result`

として actualize する。

### witness binding refs

`p07` の current helper-local binding は、
少なくとも次で読んでよい。

- `witness_binding:witness_kind:authority_draw_witness`
- `witness_binding:action_ref:handoff_dice_authority@dice_state`
- `witness_binding:draw_slot:primary`
- `witness_binding:draw_result_binding:publish_roll_result@dice_state`

ここで `draw_result_binding` は、
current sample evidence が explicit scalar receipt ではなく action-bound record に留まることを反映した helper-local binding である。
これは final public witness schema ではない。

## current recommendation

1. `auditable_authority_witness` は second strengthening package として close してよい。
2. current cut は
   - room profile claim / payload split を保つ
   - minimal witness core だけを actualize する
   - provider receipt / attestation を later attachment に残す
   に置くのが自然である。
3. historical `p07` reached memory、`p08/p05` guard-only memory の組み合わせは semantically honest であり、current active evidence は clean-near-end `06` / `01` に置く。
4. `draw_result` の helper-local actualization は symbolic binding ref で十分であり、final public scalar serialization を先取りしない。

## retained alternatives

- note-only witness
- expanded attested receipt
- `delegated_rng_service` + unchanged witness core
- distributed fairness protocol
- final public witness schema / exporter schema first

## stop line

current package は次で止める。

- final public witness schema
- provider receipt / provider attestation public contract
- `delegated_rng_service` practical package
- distributed fairness theorem
- exhaustive shared-space catalog
- final replay invalidation protocol taxonomy

## next self-driven line

historical package-local next line としては
`delegated_rng_service` と model-check second-line concretization が compare-anchor memory に残る。

ただし current repo-level queue authority は `progress.md` / `tasks.md` にあり、
2026-04-30 の family-wide wording temperature alignment closeout 後に
この package から追加の self-driven implementation line を promote しない。
