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

## まだやっていないこと

- public checker API の finalization
- theorem prover input schema の finalization
- protocol verifier input schema の finalization
- stable `evidence_refs` family をどこまで actual artifact ref に寄せるか
- concrete notebook workflow pressure を何とみなし、stable notebook bridge sketch や actual emitted notebook artifact をいつ reopen するか
- actual archive body / bundle family をどこまで theorem-line bridge に寄せるか
- `proof_assistant_adapter` consumer pressure を notebook line より先に practical reopen へ上げる条件をどう置くか
- low-level memory-order family の導入

## この phase が次へ渡すもの

- small decidable core の current boundary
- global proof / protocol / runtime policy を別 line に残す cut
- proof-obligation matrix
- external handoff artifact の minimal sketch

これにより、Phase 6 以降で actual parser / checker / runtime を切るときも、
どこまでを core に ratchet してよいかを見失いにくくなる。
