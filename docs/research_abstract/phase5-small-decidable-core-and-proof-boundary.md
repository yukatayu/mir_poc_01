# Phase 5 要約 — small decidable core と proof boundary

## 何をした phase か

Phase 5 は、
**current L2 / shared-space line で、何を local / structural / decidable core に残し、何を theorem prover / protocol verifier / runtime policy へ外出しするか**
を切る phase である。

ここでは強い型システムや public checker API をすぐに作るのではなく、
small decidable core の inventory と external handoff の cut を docs-first に整理した。

## current first choice

current package の first choice は次の 4-way split である。

- `core_static_checker`
- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

これにより、same-lineage floor や malformed / underdeclared rejection のような
local / structural / decidable なものだけを core に残し、
canonical normalization や witnessed draw protocol のような global property は外へ残す。

## `atomic_cut` の位置づけ

`atomic_cut` は current では
**place-local finalizing cut の最小核**
に留める。

つまり、

- local rollback frontier の structural floor

までは core checker / runtime representative に残すが、

- authority-serial ordering
- fairness witness
- replay obligation
- event-tree derived execution view

のような higher-level async control は別 boundary に残す。

## proof-obligation matrix

次段では、proof obligation 自体を docs 正本の matrix として見える化した。

代表的には次を分ける。

- fallback chain の canonical normalization / no re-promotion
- `try` / `atomic_cut` の rollback-cut non-interference
- authoritative room の witness / provider receipt consistency
- activation visibility の runtime control-plane policy

ここで重要なのは、
**current helper の row と proof obligation row は同じではない**
という点である。

## external handoff artifact の current cut

current cut では、external handoff artifact を actual API にせず、
**source evidence を参照する narrow row bundle sketch**
に留める。

つまり、

- detached static gate artifact
- `checked_reason_codes`
- shared-space witness bundle

は handoff row そのものではなく、
**proof obligation row が参照する source evidence**
に残す。

次段の threshold comparison では、
**mixed row bundle を current docs-only default に維持し、
boundary-specific handoff artifact と actual emitter は concrete consumer pressure が出たときだけ reopen する**
という cut まで固定した。

さらに current first practical candidate として、
**first concrete external consumer pressure は `theorem_prover_boundary` に置き、
`protocol_verifier_boundary` は second practical candidate、`runtime_policy_boundary` は later candidate**
と読むのが自然だと整理した。

その次段では、mixed row default を壊さずに、
**theorem line だけを `subject_kind + subject_ref + theorem_rows[]` の docs-only projection bundle として切る**
のが current first choice であると整理した。

さらに projection bundle の `evidence_refs` は、
actual path ではなく **`ref_kind + ref_id` の typed symbolic ref family**
へ整えるのが current first choice である。

ただし current phase では、これをまだ public checker API へは上げず、
**concrete theorem consumer pressure が出るまで docs-only bridge に留める**
のが current first choice である。

さらに current later reopen では、concrete theorem consumer bridge に渡してよい
minimum contract row core を
**`obligation_kind + evidence_refs`**
に留め、`goal_text` / `proof_hint` / `consumer_hint` は
consumer-specific attachment として後段に残すのが current first choice である。

その次段では、concrete theorem consumer class の current first practical candidate を
**`proof_notebook`**
に置き、`proof_assistant_adapter` は second practical candidate、
`theorem_export_checker` は later candidate に留めるのが自然だと整理した。
また `proof_notebook` first bridge の current lightweight attachment は
**`goal_text`**
に留め、`proof_hint` / `consumer_hint` はさらに後段へ残すのが current first choice である。
そのうえで current phase では、`proof_notebook` bridge 自体を named artifact family に昇格させず、
**docs-only derived view**
に留めるのが自然であり、stable notebook bridge sketch や actual emitted notebook artifact は concrete notebook workflow pressure が出たときだけ reopen 候補に残す。
さらに current order としては、
**concrete notebook workflow pressure を first practical reopen**
に置き、`proof_assistant_adapter` consumer pressure は second practical candidate に残すのが自然である。
その具体的な first threshold も、
**human review checklist / walkthrough pressure**
に置くのが最小であり、compare / bless-like flow や actual file exchange はさらに後段に残す。
その next cut では、review checklist / walkthrough unit 自体を
**`subject_ref + row(obligation_kind + evidence_refs + goal_text) + checklist` を持つ docs-only named review unit bundle**
へ寄せるところまでは自然だが、stronger notebook bridge sketch や actual emitted artifact はまだ後段に残す。
さらにその次段では、named review unit を複数束ねる
**`bridge_subject_ref + review_units + bridge_goal_text` を持つ docs-only notebook bridge sketch**
まで寄せるところまでは自然だが、compare / bless metadata や retained file policy は still 後段に残す。
さらに current first cut では、bridge sketch に
**`comparison_basis_refs`**
までは足してよく、その次段として
**`bless_decision_state`**
までは足してよく、さらにその次段として
**`review_note_refs`**
までは足してよく、その次段として
**`retained_notebook_ref`**
までは足してよく、そのさらに次段として
**`review_session_ref`**
までは足してよく、そのさらに次段として
**`reviewed_by_ref + reviewed_at_ref`**
までは足してよく、そのさらに次段として
**`review_session_state`**
までは足してよく、そのさらに次段として
**`retention_state`**
までは足してよく、そのさらに次段として
**`retained_path_policy_ref`**
までは足してよく、そのさらに次段として
**`emitted_artifact_ref`**
までは足してよく、そのさらに次段として
**`handoff_emitter_ref`**
までは足してよく、そのさらに次段として
**`consumer_adapter_ref`**
までは足してよく、そのさらに次段として
**`exchange_rule_ref`**
までは足してよく、そのさらに次段として
**`adapter_validation_ref`**
までは足してよく、そのさらに次段として
**`consumer_invocation_surface_ref`**
までは足してよく、そのさらに次段として
**`exchange_rule_body_ref`**
までは足してよく、そのさらに次段として
**`runtime_coupling_ref`**
までは足してよいが、concrete transport protocol / failure body はまだ後段に残す。

その次段では、
**`transport_protocol_ref`**
までは足してよいが、concrete failure body は still 後段に残す。

その次段では、
**`failure_body_ref`**
までは足してよいが、actual runtime invocation protocol / host binding / failure wording は still 後段に残す。

その次段では、
**`actual_invocation_protocol_ref`**
までは足してよいが、consumer-host-specific binding / failure wording は still 後段に残す。

その次段では、
**`consumer_host_binding_ref`**
までは足してよいが、failure wording と actual notebook runtime handoff actualization は still 後段に残す。

その次段では、
**`failure_wording_ref`**
までは足してよいが、actual notebook runtime handoff actualization / emitted invocation receipt / runtime transcript family は still 後段に残す。

**`actual_runtime_handoff_ref`**
までは足してよく、actual handoff step 自体を symbolic ref にできる。

**`emitted_invocation_receipt_ref`**
までは足してよく、receipt row と transcript family はまだ分けて扱う。

**`runtime_transcript_ref`**
までは足してよいが、concrete channel payload / transcript body は still 後段に残す。

**`materialized_runtime_handoff_ref`**
までは足してよいが、concrete payload / transcript body と actual serialized channel body は still 後段に残す。

**`concrete_payload_ref`**
までは足してよいが、concrete transcript body と actual serialized channel body は still 後段に残す。

**`concrete_transcript_body_ref`**
までは足してよいが、actual serialized channel body は still 後段に残す。

**`serialized_channel_body_ref`**
までは足してよいが、actual emitted attachment blob / file materialization は still 後段に残す。

**`emitted_attachment_body_ref`**
までは足してよいが、actual emitted attachment blob / file materialization は still 後段に残す。

**`emitted_attachment_blob_ref`**
までは足してよいが、actual retained file body / archive materialization は still 後段に残す。

**`retained_file_body_ref`**
までは足してよいが、actual archive materialization は still 後段に残す。

**`archive_materialization_ref`**
までは足してよいが、actual archive body / bundle family は still 後段に残す。

**`archive_body_ref`**
までは足してよいが、actual archive bundle family は still 後段に残す。

**`archive_bundle_ref`**
までは足してよいが、actual archive bundle manifest / member family は still 後段に残す。

**`archive_bundle_manifest_ref`**
までは足してよいが、actual archive bundle member family は still 後段に残す。

**`archive_bundle_member_family_ref`**
までは足してよいが、actual archive member body compare は still 後段に残す。

**`archive_member_body_compare_ref`**
までは足してよいが、actual archive bless / update policy は still 後段に残す。

**`archive_bless_update_policy_ref`**
までは足してよいが、retained archive payload は still 後段に残す。

**`retained_archive_payload_ref`**
までは足してよいが、archive retention layout と actual bless-side row / update-side row split は still 後段に残す。

**`archive_retention_layout_ref`**
までは足してよいが、actual retained archive payload body family と actual bless-side row / update-side row split は still 後段に残す。

**`retained_archive_payload_body_family_ref`**
までは足してよいが、retained payload materialization family と actual bless-side row / update-side row split は still 後段に残す。

**`retained_payload_materialization_family_ref`**
までは足してよいが、actual retained payload body materialization detail と actual bless-side row / update-side row split は still 後段に残す。

**`retained_payload_body_materialization_detail_ref`**
までは足してよいが、actual retained payload body materialization payload と actual bless-side row / update-side row split は still 後段に残す。

**`retained_payload_body_materialization_payload_ref`**
までは足してよいが、actual retained payload body materialization carrier detail と actual bless-side row / update-side row split は still 後段に残す。

**`retained_payload_body_materialization_carrier_detail_ref`**
までは足してよいが、actual retained payload body materialization carrier payload と actual bless-side row / update-side row split は still 後段に残す。

**`retained_payload_body_materialization_carrier_payload_ref`**
までは足してよいが、actual bless-side row / update-side row split は still 後段に残す。

**`retained_payload_body_materialization_bless_update_split_ref`**
までは足してよいが、actual bless-side row / update-side row pair は still 後段に残す。

**`retained_payload_body_materialization_bless_update_row_pair_ref`**
までは足してよいが、その次段として
**`retained_payload_body_materialization_bless_update_row_ref_family_ref`**
までは足してよい。そのさらに次段として
**`retained_payload_body_materialization_bless_update_dual_ref_bundle_ref`**
までは足してよい。そのさらに次段として
**`retained_payload_body_materialization_bless_side_row_ref`** と
**`retained_payload_body_materialization_update_side_row_ref`**
までは internal-only strict dual field として足してよい。そのさらに次段として
**`retained_payload_body_materialization_bless_update_pair`**
までは notebook consumer 向け pair surface として足してよい。その次段として
**`retained_payload_body_materialization_boundary_handoff_pair_ref`**
までは symbolic handoff-facing pair ref として足してよい。その次段として
**`retained_payload_body_materialization_boundary_handoff_pair`**
までは actual handoff-facing pair shape として足してよい。その次段として
**`retained_payload_body_materialization_boundary_handoff_artifact_row`**
までは boundary-specific handoff artifact row として足してよい。その次段として
**`retained_payload_body_materialization_external_handoff_row`**
までは external-contract-facing handoff row として足してよい。その次段として
**`retained_payload_body_materialization_external_contract`**
までは actual external contract として足してよい。その次段として
**`retained_payload_body_materialization_external_contract_payload`**
までは consumer-specific external contract payload として足してよい。その次段として
**`retained_payload_body_materialization_external_contract_proof_hint`**
までは `proof_hint` enrichment として足してよい。その次段として
**`retained_payload_body_materialization_external_contract_consumer_hint`**
までは `consumer_hint` enrichment として足してよい。その次段として
**`retained_payload_body_materialization_external_contract_second_consumer_pressure`**
までは symbolic second consumer pressure marker として足してよい。その次段として
**`retained_payload_body_materialization_proof_assistant_adapter_contract`**
までは actual adapter-facing contract として足してよい。その次段として
**`retained_payload_body_materialization_theorem_export_checker_pressure`**
までは checker-facing pressure marker として足してよい。その次段として
**`retained_payload_body_materialization_theorem_export_checker_contract`**
までは actual checker-facing contract として足してよい。ただし exported checker payload は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_payload_pressure`**
までは exported checker payload pressure marker として足してよい。ただし actual exported checker payload は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_payload`**
までは actual exported checker payload として足してよい。ただし checker result materialization family は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_result_materialization_family`**
までは checker result materialization family marker として足してよい。ただし actual checker result payload は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_result_payload`**
までは actual checker result payload として足してよい。ただし checker verdict carrier detail は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail`**
までは checker verdict carrier detail として足してよい。ただし checker verdict payload family / witness / transport は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_payload_family`**
までは checker verdict payload family marker として足してよい。ただし checker verdict witness / transport は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_witness_family`**
までは checker verdict witness family marker として足してよい。ただし checker verdict transport は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_family`**
までは checker verdict transport family marker として足してよい。ただし checker verdict transport carrier detail / payload / receipt は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail`**
までは checker verdict transport carrier detail として足してよい。ただし checker verdict transport payload / receipt / channel body は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload`**
までは checker verdict transport payload として足してよい。ただし checker verdict transport receipt / channel body は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt`**
までは checker verdict transport receipt として足してよい。ただし checker verdict transport channel body は still 後段に残す。
その次段として
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`**
までは checker verdict transport channel body として足してよい。ただし low-level memory-order family は theorem-line retained bridge に入れず、ここを current stop line にする。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_serial_transition_family`**
までは higher-level async-control family の current first cut として足してよい。ただし actual `authority_serial_transition_contract` row、witness / replay / fairness attachment、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_serial_transition_contract`**
までは minimal authority-serial contract row として足してよく、さらに
**`retained_payload_body_materialization_theorem_export_authority_owner_ref`**
までは owner-slot detail として足してよい。ただし transition stage family、authority handoff epoch、witness / replay attachment、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_transition_stage_family`**
までは symbolic stage family として足してよい。ただし explicit stage sequence、authority handoff epoch、witness / replay attachment、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_transition_stage_sequence`**
までは actual ordered stage sequence として足してよい。ただし stage-local obligation family、authority handoff epoch、witness / replay attachment、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_family`**
までは symbolic stage-local obligation family として足してよい。ただし actual stage-local obligation row detail、authority handoff epoch、witness / replay attachment、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_row`**
までは actual stage-local obligation row detail として足してよい。ただし authority handoff epoch、witness / replay attachment、payload / carrier detail、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_authority_handoff_epoch_ref`**
までは symbolic authority handoff epoch ref として足してよい。ただし witness / replay attachment、handoff payload / carrier detail、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_witness_aware_handoff_family`**
までは symbolic witness-aware handoff family として足してよい。ただし actual witness row、replay attachment、handoff payload / carrier detail、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_handoff_witness_row`**
までは actual handoff witness row detail として足してよい。ただし replay attachment、handoff payload / carrier detail、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_handoff_replay_attachment_ref`**
までは symbolic replay attachment ref として足してよい。ただし handoff payload / carrier detail、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_handoff_payload_ref`**
までは symbolic handoff payload ref として足してよい。ただし handoff carrier detail、low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_handoff_carrier_detail`**
までは minimal handoff carrier detail として足してよい。ただし handoff transport family と low-level memory-order family は still 後段に残す。
その次段では、
**`retained_payload_body_materialization_theorem_export_handoff_transport_family`**
までは symbolic handoff transport family として足してよく、その次段として
**`retained_payload_body_materialization_theorem_export_handoff_transport_carrier_detail`**
までは minimal handoff transport carrier detail として足してよく、その次段として
**`retained_payload_body_materialization_theorem_export_handoff_transport_payload`**
までは symbolic handoff transport payload として足してよく、その次段として
**`retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row`**
までは minimal handoff transport receipt row として足してよく、その次段として
**`retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`**
までは minimal handoff transport channel body として足してよい。ただし low-level memory-order family は still 後段に残す。
その次段では、low-level memory-order family を marker としても足さず、theorem-line retained bridge 自体をここで止めるのが current first choice である。したがって Phase 5 closeout 時点の次 line は theorem-line の先へ下ることではなく、small decidable core 側の **docs-only checker-cluster matrix** に戻し、first checker cut の 6 cluster を user-facing に見える形へ整理することであった。そこで current first choice は、minimal checker-cluster row core を **`cluster_kind + checker_subject + decidable_class + external_handoff`** に留め、その次段で **`fixture_evidence_refs`** を row attachment として足し、さらに stable typed static reason actualization がある row に限って **optional `typed_reason_family_hint` attachment** を許し、その payload は **`family_refs[]` minimal bundle** に留め、multi-family row には **lightweight `coverage_state`** だけを追加する cut である。actual kind summary は current checker-cluster matrix には足さず、その次段として actual checker payload family を **`payload_family_kind + source_refs`** minimal bundle まで docs-first bridge として切り、そのさらに次段として checker payload row family を **`payload_family_ref + row_family_kind`**、checker payload row detail を **`payload_row_family_ref + row_source_ref + row_reason_kind`**、checker payload row body を **`row_body` variant-local slot bundle**、checker payload supported kind summary を **`payload_row_family_ref + supported_kind_scope + supported_kind_refs`**、public checker payload schema を **`actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref`** minimal wrapper、public checker API を **`checker_subject + public_checker_payload_schema_ref`** docs-only read relation まで narrow に actualize する。その次段では public checker comparison 専用の entry criteria を別に置き、その minimum を **docs-only minimal API fixed + source-backed family-facade command-surface pressure** に留め、そのさらに次段では public checker command surface を **`command_surface_kind + family_facade_command_refs + public_checker_api_ref`** minimal family-facade bundle、shared output contract を **`output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref`** minimal summary bundle、public checker boundary を **`boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref`** docs-only parser-front bundle、その次段の verifier handoff surface を **`handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode`** docs-only mixed-row bridge に留めるのが current first choice である。actual subject row、boundary-specific handoff artifact、actual emitted verifier handoff artifact、shared generic entry、final parser grammar は still later に残し、Phase 5 verifier handoff gate 自体は fixed 済みである。Phase 5 self-driven closeout 自体は `specs/examples/297...298` で fixed され、その後 `specs/examples/299...304` により Phase 6 parser / checker-runtime / compile-ready formal-hook first tranche も fixed された。

## closeout で固定した current stop line

`specs/examples/297...298` により、Phase 5 self-driven closeout は narrow bundle として固定した。ここで残した current package は、

- checker-side verifier handoff surface を docs-only mixed-row bridge に留めること
- theorem-side retained bridge を `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` stop line に留めること
- proof / protocol / runtime-policy line を 4-way split inventory と priority の形で current package に残すこと

である。

一方で次は retained-later に明示的に残した。

- actual subject row materialization
- boundary-specific handoff artifact family
- actual emitted verifier artifact
- concrete theorem / model-check tool binding
- public checker migration
- low-level memory-order family

この closeout により、Phase 5 の self-driven snapshot は fixed と読める。Phase 6 front-half の parser / checker-runtime / compile-ready formal-hook first tranche も `specs/examples/299...304` で fixed 済みであり、その後 `specs/examples/305...368` により next reopen sequencing、parser second tranche first package、reserve formal tool binding inventory、source-sample path first packages、theorem-first concrete tool pilot、authored-row widen sequencing、bridge-sketch reopen ordering、`e1` / `e21` actualization、`e3` guard comparison、plain bridge sketch actualization、compare-ready bridge sketch second reopen、deferred `e3` actualization reopen timing、actual `e3` authored-row actualization、proof/model-check first concrete tool pilot、second source-sample cluster sequencing、actual `e22` contrast-row source actualization、stable static malformed post-contrast sequencing、parser / checker / runtime public surface inventory、Mirrorea/shared-space docs-first re-entry bundle、model-check/public-checker second reserve inventory、stable-static edge-pair first reopen、public operational surface actualization gate、shared-space identity/auth layering reopen、model-check concrete carrier first actualization gate も fixed 済みである。follow-up maintenance も report-backed に閉じており、repo-level current mainline は **stable malformed broader follow-up inventory** である。Phase 5 は fixed entry criteria と retained-later inventory を渡し終えた状態として読むのが自然である。

## まだやっていないこと

- public checker API の finalization
- theorem prover input schema の finalization
- protocol verifier input schema の finalization
- stable `evidence_refs` family をどこまで actual artifact ref に寄せるか
- concrete notebook workflow pressure を何とみなし、stable notebook bridge sketch や actual emitted notebook artifact をいつ reopen するか
- `authority_serial_transition_contract` の minimal row core をどこまで narrow に retained bridge へ寄せるか
- theorem-line retained bridge から async-control / concurrency vocabulary へ接続する最小 handoff shape
- low-level memory-order family を external verifier vocabulary としてだけ残すか

## この phase が次へ渡すもの

- small decidable core の current boundary
- global proof / protocol / runtime policy を別 line に残す cut
- proof-obligation matrix
- external handoff artifact の minimal sketch

これにより、Phase 6 以降で actual parser / checker / runtime を切るときも、
どこまでを core に ratchet してよいかを見失いにくくなる。
